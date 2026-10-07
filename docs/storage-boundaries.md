# Zitera Storage Boundaries & Data Retention Policy

## 1. Overview & Architectural Principles

ZITERA_LAB is designed for educational security laboratories, classroom deployment, and individual student training. To ensure enterprise robustness and zero accidental loss of student coursework, the filesystem architecture strictly bifurcates immutable application binaries from user state, lab packages, disposable cache, execution logs, and diagnostic exports.

### Core Guarantees:
1. **Application Updates Never Destroy Progress:** Updating the desktop UI shell or engine replaces only application binaries in `%LOCALAPPDATA%\Programs\ZiteraLab\`. Student progress, solved flags, notes, and preferences in `%LOCALAPPDATA%\ZiteraLab\user\` are never modified, overwritten, or deleted during updates or rollbacks.
2. **Deterministic Lab Isolation:** Lab packages reside in `%LOCALAPPDATA%\ZiteraLab\labs\`, each indexed by ID with semantic versioning and atomic activation markers (`active_version.txt`).
3. **Safe Uninstall Defaults:** Standard uninstallation removes the application binaries and disposable caches. Student coursework and completed challenge records are preserved by default, preventing accidental data loss when reinstalling or upgrading. Full removal of user data requires explicit user opt-in (`/PURGEUSERDATA`).
4. **100% Non-Elevated Execution:** All directory tiers reside strictly within user profile boundaries (`%LOCALAPPDATA%`), requiring zero administrative elevation (no UAC prompts).

---

## 2. Directory Layout & Classification Matrix

```text
%LOCALAPPDATA%\
  ├── Programs\ZiteraLab\                     [TIER 1: IMMUTABLE APPLICATION BINARIES]
  │     ├── zitera.exe                         Desktop UI shell (Flutter Windows runner)
  │     ├── bin\zitera-engine.exe              Rust core orchestration engine & CLI
  │     ├── catalog\catalog.json               Curriculum index & metadata
  │     ├── install_manifest.json              Release build manifest & signature hashes
  │     └── unins000.exe                       Uninstaller binary
  │
  └── ZiteraLab\                              [TIERS 2-6: MUTABLE RUNTIME & DATA STORAGE]
        ├── user\                              [TIER 2: STUDENT COURSEWORK & PREFERENCES]
        │     ├── progress.json                Completed lessons, practice, flags, streaks
        │     ├── preferences.json             Theme, font size, terminal keybindings
        │     └── history.json                 Session launch history & timestamps
        │
        ├── labs\                              [TIER 3: VERSIONED LAB PACKAGES]
        │     └── <LAB_ID>\                    e.g., A01, A02
        │           ├── active_version.txt     Pointer to active semantic version
        │           ├── .runtime.json          Ephemeral broker port and PID marker
        │           └── versions\
        │                 ├── 1.0.0\           Archived previous version
        │                 └── 1.1.0\           Currently active versioned payload
        │
        ├── cache\                             [TIER 4: DISPOSABLE TEMPORARY BUFFERS]
        │     └── staging\                     Pre-extraction package buffer & downloads
        │
        ├── logs\                              [TIER 5: LOCAL EXECUTION LOGS]
        │     └── session_<date>.log           Console stdout/stderr and audit trails
        │
        └── diagnostics\                       [TIER 6: SANITIZED DIAGNOSTIC EXPORTS]
              └── zitera_diagnostics_<date>.zip Redacted diagnostic bundles
```

---

## 3. Path Resolution Hierarchy & Overrides

ZITERA_LAB determines directory paths using a deterministic priority cascade. This enables headless testing, isolated CI validation, and portable lab deployments without modifying global system state:

| Storage Tier | Canonical Default Path | Environment Override |
|---|---|---|
| **Application Files** | `%LOCALAPPDATA%\Programs\ZiteraLab` | `ZITERA_APP_DIR` |
| **User Data Root** | `%LOCALAPPDATA%\ZiteraLab` | `ZITERA_DATA_DIR` |
| **User Progress** | `%LOCALAPPDATA%\ZiteraLab\user` | `ZITERA_USER_DIR` |
| **Lab Packages** | `%LOCALAPPDATA%\ZiteraLab\labs` | `ZITERA_LABS_DIR` |
| **Cache Buffer** | `%LOCALAPPDATA%\ZiteraLab\cache` | `ZITERA_CACHE_DIR` |
| **Execution Logs** | `%LOCALAPPDATA%\ZiteraLab\logs` | `ZITERA_LOGS_DIR` |
| **Diagnostics** | `%LOCALAPPDATA%\ZiteraLab\diagnostics` | `ZITERA_DIAGNOSTICS_DIR` |

When running in portable mode or local development without `%LOCALAPPDATA%`, paths gracefully fall back relative to the executable location.

---

## 4. Lifecycle Data Retention Policy Matrix

The table below defines the required disposition for every storage tier across all installer and runtime lifecycle actions:

| Lifecycle Action | Application Files | Lab Packages | User Progress | Cache | Logs | Diagnostics |
|---|---|---|---|---|---|---|
| **Fresh Install** | Write / Verify | Preserve / Init | Preserve / Init | Create Empty | Create Empty | Create Empty |
| **App Update** | **Replace** | **Preserve** | **Preserve** | **Purge** | **Preserve** | **Preserve** |
| **App Repair** | **Replace Corrupt** | **Preserve** | **Preserve** | **Preserve** | **Preserve** | **Preserve** |
| **Uninstall (Default)** | **Purge** | **Preserve** | **Preserve** | **Purge** | **Preserve** | **Preserve** |
| **Uninstall (Full Purge)** | **Purge** | **Purge** | **Purge** | **Purge** | **Purge** | **Purge** |
| **Lab Update** | Unchanged | Replace Version | **Preserve** | Purge Staging | Unchanged | Unchanged |
| **Lab Reset** | Unchanged | Reset State | **Preserve** | Unchanged | Unchanged | Unchanged |

---

## 5. Security & Isolation Boundary Verification

1. **Disjoint Filesystem Trees:** The engine verifies via `verify_boundary_isolation()` that `user_dir` and `labs_dir` are strictly disjoint from `app_dir`. An uninstaller or updater deleting `app_dir` cannot traverse into or delete `user_dir`.
2. **Zero Secret Leakage:** Student progress files (`progress.json`) track task completion states, attempt counts, and timestamps. Challenge secret flags are verified dynamically against lab runtime verifiers and are NEVER written to disk in plain text.
3. **Cache Purging Safety:** Cache purging operations (`zitera storage purge-cache`) operate strictly within `%LOCALAPPDATA%\ZiteraLab\cache` and cannot escape to parent directories.
