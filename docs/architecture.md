# ZITERA_LAB Architecture Specification

## 1. System Overview

ZITERA_LAB is an offline-first, native cybersecurity learning platform designed to provide realistic hands-on vulnerability laboratories without requiring virtualization, administrative privileges, or containerization daemons.

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

## 2. Component Decoupling & Principles

### A. Presentation Layer (Flutter Desktop)
- **Generic Data-Driven Architecture**: The UI contains zero hardcoded `if (lab == 'A01')` logic. All views (Lab Detail, Overview, Learn, Practice, Challenge, Hints, Progress) render dynamically from the canonical metadata contract and catalog.
- **Zero Raw Markdown Leakage**: All educational content passes through high-fidelity styled renderers with syntax-highlighted cyber terminals and formatted tables.
- **Human-Actionable Error System**: Errors communicate clear diagnostics (*What happened? Why? What can I do?*) instead of raw technical codes.

### B. Core Orchestration Engine (Rust)
- **Single Source of Truth**: Metadata derives strictly from `catalog.json` and laboratory `manifest.json` files.
- **Stable Package Identity**: Package IDs (e.g., `zitera-lab-a01`) remain permanent across updates, while curriculum mappings (e.g., OWASP Top 10:2025 category `A01`) update transparently without breaking user progress.
- **Unified Error Taxonomy**: Machine-readable codes paired with contextual remediation instructions.

### C. Security Broker & Reverse Proxy
- **Localhost Isolation**: Lab servers bind to internal loopback interfaces (`127.0.0.1`).
- **Session Authentication**: Every running lab receives an ephemeral, high-entropy cryptographic session token (`/session/<token>/`). Requests lacking valid tokens are dropped.
- **SSRF Neutralization**: Outbound requests are filtered against internal metadata addresses (e.g. `169.254.169.254`), host networking ranges, and loopback escapes.

### D. Educational Sandboxed Terminal
- **Pure In-Process VFS**: Simulates Unix commands (`pwd`, `ls`, `cd`, `cat`, `grep`, `find`, `echo`, `mkdir`, `touch`, `cp`, `mv`, `rm`, `ps`, `curl`, `whoami`, `uname`) inside an in-memory virtual tree.
- **Zero Raw Shell Passthrough**: Rejects all shell metacharacters (`;`, `&&`, `|`, `` ` ``, `$()`), forbidding `cmd.exe`, `powershell.exe`, or host process execution.
- **Interactive Help System**: Provides comprehensive educational usage guides (`help <command>`) explaining cybersecurity relevance and attack surface analysis.
