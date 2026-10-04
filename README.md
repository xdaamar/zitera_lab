# ZITERA_LAB

> A personal cyber security learning laboratory desktop application for Windows.

## Overview

ZITERA_LAB is an educational cyber security laboratory platform combining:
- **Interactive vulnerable web application labs** (OWASP Top 10:2025 baseline)
- **Guided practice & concept analogies** (Learn Mode)
- **Investigation walkthroughs** (Practice Mode)
- **CTF-style assessments** (Challenge Mode)
- **Smart system diagnostics & tool manager** (Setup Engine)
- **Local runtime** (Windows Native Runtime Baseline — legacy Docker/WSL deprecated for ZITERA 2.0)
- **Lightweight systems engine** (Rust)
- **Polished desktop interface** (Flutter)

---

## Architectural Layers

```text
┌──────────────────────────────────────────────┐
│            ZITERA_LAB UI (Flutter)           │
│  Layar, Navigasi, Markdown/Lesson, Wizard    │
└──────────────────────┬───────────────────────┘
                       │ Local Process (JSON IPC)
                       ▼
┌──────────────────────────────────────────────┐
│          ZITERA ENGINE / CLI (Rust)          │
│  zitera-engine.exe / zitera doctor / lab ... │
└──────────────┬───────────────┬───────────────┘
               │               │
       ┌───────┴───────┐       ▼
       ▼               ▼     GitHub (Catalog & Lab Repos)
      Git     [OLD] Docker/WSL (Deprecated)
              [NEW] Windows AppContainer / Job Object (Phase 16+)
                       │
                       ▼
               Lab Runtime (127.0.0.1)
```

---

## Repository Structure

```text
zitera_lab/
├── ui/
│   └── flutter/        # Flutter Windows Desktop Application
├── engine/
│   └── rust/           # Rust Core Engine & CLI (zitera)
├── prd/
│   ├── PRD.md          # Product Requirements Document
│   ├── architecture.md # Architecture & Layer Boundaries
│   ├── lab_specification.md # Standard Lab Contract & Modes
│   └── security.md     # Security Boundaries & Isolation Policy
├── catalog/
│   └── catalog.json    # Centralized Lab Discovery Catalog
└── branding/           # Vector Assets & Visual Identity
```

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
