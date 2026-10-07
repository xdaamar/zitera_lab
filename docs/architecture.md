# ZITERA_LAB // System Architecture Specification

## 1. Architectural Philosophy

ZITERA_LAB is an offline-first, native cybersecurity learning platform designed to provide authentic, exploit-vulnerable laboratory environments on Windows workstations with **zero virtualization, zero Docker/WSL2 daemons, and zero administrative elevation**.

```text
┌────────────────────────────────────────────────────────┐
│                   Flutter Desktop UI                   │
│  - Generic Data-Driven Views (Dashboard, Labs, Learn)  │
│  - Educational Terminal Console (Interactive VFS)      │
│  - Progress Manager & Version Center                   │
└───────────────────────────┬────────────────────────────┘
                            │ Standard IPC (JSON CLI / Subprocess)
┌───────────────────────────▼────────────────────────────┐
│              ZITERA Rust Orchestration Engine          │
│  - Curriculum & Manifest Catalog Engine                │
│  - Package Installer & Ed25519 Cryptographic Verifier  │
│  - Security Broker / Reverse Proxy (SSRF Guard)        │
│  - Educational In-Process Virtual Filesystem (VFS)     │
└───────────────────────────┬────────────────────────────┘
                            │ Win32 Native API
┌───────────────────────────▼────────────────────────────┐
│              Windows Native Sandboxed Runtime          │
│  - Windows Job Objects (KillOnJobClose, Resource Caps) │
│  - Low-Privilege Token / AppContainer Isolation        │
│  - Loopback-Only Ephemeral Socket Binding              │
│  - Strictly Zero Docker, Zero WSL2, Zero Host Shell    │
└────────────────────────────────────────────────────────┘
```

---

## 2. Layer Decoupling & Boundaries

### 2.1 Presentation Layer (Flutter Desktop)
- **Generic Data-Driven Views:** Contains zero hardcoded laboratory identifiers (`if (lab == 'A01')`). Every screen (Overview, Learn, Practice, Challenge, Hints, Progress) renders dynamically from canonical curriculum contracts (`manifest.json` and `catalog.json`).
- **Human-Actionable Failure Recovery:** Runtime errors are mapped to three human questions:
  1. *What happened?* (Failure summary)
  2. *Why did it happen?* (Underlying cause)
  3. *What can I do?* (Actionable remediation button)

### 2.2 Orchestration Engine (Rust Native)
- **Unified IPC Contract:** Standardized JSON envelope across all CLI commands:
  ```json
  {
    "success": true,
    "action": "lab.start",
    "data": { ... },
    "error": null
  }
  ```
- **Error Classifier:** Categorizes faults into 7 minimal recovery states:
  `labCrash`, `runtimeUnavailable`, `packageVerificationFailure`, `updateInterrupted`, `rollbackOccurred`, `storageFailure`, `brokerUnavailable`.

### 2.3 Security Broker & Reverse Proxy
- **Loopback Enforcement:** Binds strictly to `127.0.0.1`.
- **Ephemeral Session Tokens:** Generates cryptographic session tokens (`/session/<token>/`) ensuring only authorized local requests reach the target server.
- **SSRF Neutralization:** Filters and drops requests targeting cloud metadata (`169.254.169.254`) or private intranet ranges.

### 2.4 Educational Sandboxed Terminal
- **Pure In-Process VFS:** Virtualizes standard Unix utilities inside memory.
- **Shell-Less Execution:** Syntactic parser rejects shell metacharacters (`;`, `&&`, `|`, `` ` ``, `$()`), preventing arbitrary command injection.
- **Localhost Curl Proxy:** In-process curl implementation confined strictly to local laboratory endpoints.

---

## 3. Storage Hierarchy (6-Tier Separation)

Storage is segregated into 6 strict isolation tiers under `%LOCALAPPDATA%`:

```text
%LOCALAPPDATA%\
├── Programs\ZiteraLab\               [Tier 1: Immutable Application Binaries]
│   ├── bin\zitera-engine.exe
│   ├── catalog\catalog.json
│   └── install_manifest.json
│
└── ZiteraLab\
    ├── user\                         [Tier 2: Student Progress & Data (Guaranteed Preservation)]
    │   ├── progress.json             (Canonical v2 Schema)
    │   └── preferences.json
    │
    ├── labs\                         [Tier 3: Lab Sandboxes & Packages]
    │   └── A01\
    │       ├── active_version.txt
    │       └── versions\1.0.1\
    │
    ├── cache\                        [Tier 4: Disposable Cache (Purged on maintenance)]
    ├── logs\                         [Tier 5: Process & Error Logs (Size-capped)]
    └── diagnostics\                  [Tier 6: Privacy-Scrubbed Diagnostics]
```

---

## 4. Courseware Lifecycle State Machine

Each courseware package transitions through deterministic states:

```text
[ Remote Feed / .zlab ]
         │
         ▼ (Verify Ed25519 & Anti-Downgrade)
    [ STAGING ] ──────────── (Checksum Mismatch) ──► [ ROLLBACK / PURGE ]
         │
         ▼ (Atomic Extraction)
  [ VERSIONS/<V> ]
         │
         ▼ (Write active_version.txt)
    [ ACTIVE ]
         │
         ▼ (Spawn AppContainer / Job Object)
    [ RUNNING ] ─────────── (Process Crash / Exit) ──► [ STOPPED / RESET ]
```
