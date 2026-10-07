# Windows Distribution Architecture & Decision Record

**Document ID:** ADR-DIST-001  
**Status:** ACCEPTED  
**Date:** 2026-10-07  
**Scope:** Phase 20 (Release Engineering, Distribution & Field Readiness)  

---

## 1. Context and Problem Statement

ZITERA_LAB 2.0 transitions the platform from a developer-facing repository into an enterprise-grade, field-ready Windows educational product. The target users are non-developer computer science students, educators, and enterprise trainees who:
- Do not possess local administrator or elevated UAC credentials on locked-down campus or corporate laptops.
- Do not have developer toolchains installed (Git, Docker, WSL2, Python, Cargo, Rustc).
- Require zero-friction installation, silent campus IT distribution, reliable in-place upgrades, safe uninstall, and deterministic self-healing repair.
- Operate under enterprise security controls, including Windows Defender Application Control (WDAC), Device Guard, and SmartScreen.

This architectural decision record evaluates distribution technologies and formalizes the definitive Windows packaging, filesystem layout, upgrade, and user-data preservation model.

---

## 2. Evaluation Criteria

Distribution technologies were evaluated across seven critical dimensions:

1. **Reliability:** Atomic file replacement, crash-resilience, rollback safety during partial installs.
2. **Windows Compatibility:** Support for Windows 10 1809+ and Windows 11 across standard corporate configurations.
3. **Privilege Boundary:** 100% non-elevated per-user installation (`%LOCALAPPDATA%\Programs`) with zero elevation prompts.
4. **Upgrade & Downgrade Behavior:** In-place binary replacement without locking active sessions; preservation of historical lab versions.
5. **Uninstall Cleanliness & Data Retention:** Complete removal of application binaries while preserving student progress and lab completion history.
6. **Silent Deployment:** Scriptable headless deployment (`/S` / `/VERYSILENT`) for bulk campus laboratory provisioning.
7. **Maintenance & Tooling Cost:** Reproducible CI/CD pipeline scriptability without complex proprietary toolchain dependencies.

---

## 3. Technology Evaluation Matrix

| Technology | Privilege Model | Silent Deploy | Code Signing | Maintenance Cost | Enterprise Adoption | Verdict |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Inno Setup** | Per-User or All-Users | Excellent (`/VERYSILENT`) | Native `SignTool` support | Very Low (Pascal/INI script) | High | **Selected for Primary Installer** |
| **NSIS** | Per-User or All-Users | Excellent (`/S`) | Scripted `SignTool` | Low (Custom script syntax) | High | Viable Alternative |
| **WiX / MSI** | Historically All-Users (Per-user MSI is fragile) | Native Windows Installer | Native | High (Complex XML schemas) | High for GPO | Rejected (Excessive complexity for non-elevated student laptops) |
| **MSIX / AppX** | Per-User Sandboxed | Excellent | Mandatory trusted CA root | High (Strict packaging constraints) | Moderate | Rejected (Conflicts with native AppContainer child brokering) |
| **Portable Zip** | Per-User Zero-Install | Not applicable | Binary-level | Minimal | High for offline labs | **Selected for Secondary Offline Distribution** |

### Decision Rationale

1. **Rejection of MSIX:** MSIX packages enforce containerized virtualization boundaries that interfere with dynamic Windows AppContainer profile creation and nested Job Object resource assignments used by the Zitera Native Runtime.
2. **Rejection of Machine-Wide MSI:** Enterprise campus student workstations typically block non-admin users from invoking Windows Installer elevation.
3. **Selection of Inno Setup (Per-User Non-Admin):**
   - Directly configures per-user installations (`PrivilegesRequired=lowest`) targeting `{localappdata}\Programs\ZiteraLab`.
   - Built-in command-line silent switches (`/VERYSILENT /SUPPRESSMSGBOXES /NORESTART`).
   - Native `SignTool` pipeline integration for Authenticode signing.
   - Small footprint, fast execution, zero external runtime prerequisites.
4. **Selection of Standalone Portable Archive (`.zip`):**
   - Provides a zero-installation alternative for air-gapped forensic or competitive CTF environments where running setup executables is disallowed.

---

## 4. Storage & Filesystem Separation Contract

To guarantee that application binary updates never destroy student coursework, challenge flags, or custom settings, ZITERA_LAB enforces a strict three-tier directory hierarchy:

```text
%LOCALAPPDATA%\
  ├── Programs\ZiteraLab\                     [TIER 1: IMMUTABLE APPLICATION BINARIES]
  │     ├── zitera.exe                         (Flutter desktop UI shell)
  │     ├── engine\zitera-engine.exe           (Rust core broker & runtime engine)
  │     ├── resources\                         (Curriculum catalog, icon assets, fonts)
  │     ├── install_manifest.json              (Build SHA, release version, manifest)
  │     └── unins000.exe                       (Uninstaller executable)
  │
  └── ZiteraLab\                              [TIER 2 & 3: USER MUTABLE STATE & LAB STORAGE]
        ├── user\                              [TIER 2: STUDENT PROGRESS & PREFERENCES]
        │     ├── progress.json                (Completed lessons, earned flags, timestamps)
        │     ├── preferences.json             (Theme, font size, terminal keybindings)
        │     └── history.json                 (Session logs, launch timestamps)
        │
        ├── labs\                              [TIER 3: VERSIONED LAB PACKAGES]
        │     └── <LAB_ID>\                    (e.g., A01, A02)
        │           ├── active_version.txt     (Points to currently active semantic version)
        │           ├── .runtime.json          (Ephemeral active broker port and pid)
        │           └── versions\
        │                 ├── 1.0.0\           (Preserved historical version)
        │                 └── 1.0.1\           (Currently active version)
        │
        ├── diagnostics\                       [PRIVACY-SAFE DIAGNOSTIC LOGS]
        │     └── zitera_diagnostics_<date>.zip
        │
        └── cache\                             [PURGEABLE CACHE]
              └── staging\                     (Temporary pre-extraction package buffer)
```

### Invariants:
1. **Application Upgrades** replace ONLY contents of `%LOCALAPPDATA%\Programs\ZiteraLab\`.
2. **User Data** in `%LOCALAPPDATA%\ZiteraLab\user\` is NEVER overwritten or deleted during application upgrades or rollbacks.
3. **Uninstallation** by default purges `%LOCALAPPDATA%\Programs\ZiteraLab\` but offers the user a prompt (or command-line flag `/PURGEUSERDATA`) to either retain or remove `%LOCALAPPDATA%\ZiteraLab\user\`.

---

## 5. Upgrade, Rollback & Repair Mechanics

### Atomic Application Upgrade Lifecycle

```text
Check Remote Release Metadata -> Verify Release Hash & Signature
      ↓
Download Update Bundle to Cache Staging
      ↓
Verify Authenticode Signature on zitera-engine.exe & zitera.exe
      ↓
Ensure Active Lab Sessions are Gracefully Stopped (SIGTERM -> Stop Broker)
      ↓
Replace Application Binaries in %LOCALAPPDATA%\Programs\ZiteraLab\
      ↓
Execute Self-Check Diagnostic (zitera-engine doctor --json)
      ↓
Restart Application Shell with Previous User Session Restored
```

### Self-Healing & Repair

If an application binary or lab package suffers corruption on disk:
1. **Core Verification:** Running `zitera doctor` detects missing dependencies, broken permissions, or corrupted binaries.
2. **Package Reconciliation:** The engine's `reconcile_lab_version()` auto-detects corrupted or deleted `active_version.txt` markers and deterministically restores the highest valid installed version from the `versions/` directory.
3. **Atomic Rollback:** If an in-progress lab update fails signature verification or archive extraction, the pre-existing lab version remains 100% active and functional.

---

## 6. Enterprise & Non-Developer Machine Readiness

- **Zero Administrative Elevation:** Both installer and application run strictly under standard user privileges.
- **Zero Raw Shell Invocation:** All terminal interactions are parsed and brokered through the internal VFS and sandboxed execution engine.
- **Zero Cloud/Telemetry Outbound Traffic:** The application never dials external servers for licensing, usage tracking, or analytics.
- **Network Confinement:** Sandboxed child processes bind exclusively to `127.0.0.1` loopback endpoints.
- **Device Guard & WDAC Support:** All shipped executables and dynamic libraries must be Authenticode-signed with a valid code-signing certificate (detailed in CP08).
