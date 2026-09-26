# ZITERA_LAB — Architecture Specification

## 1. Purpose

This document defines the permanent technical boundaries of ZITERA_LAB.

The PRD defines what the product must do.
This document defines how the major technical parts are separated.

Any implementation that conflicts with these boundaries requires an ADR before coding.

---

## 2. System Boundary

```text
ZITERA_LAB
│
├── UI
│   └── Flutter / Dart
│
├── ENGINE
│   └── Rust
│
├── LAB RUNTIME
│   ├── Docker
│   └── WSL2
│
├── DISTRIBUTION
│   ├── GitHub Catalog
│   └── Lab Repositories
│
└── LOCAL DATA
    ├── Config
    ├── Progress
    ├── Logs
    └── Lab Workspace
```

---

## 3. Flutter Boundary

Flutter owns:

- screens
- navigation
- presentation
- lesson rendering
- challenge interface
- setup UX
- progress UX
- settings
- user-facing logs/status

Flutter must NOT own:

- Docker implementation
- Git implementation
- WSL implementation
- arbitrary shell orchestration
- tool installation logic
- lab lifecycle internals
- security policy enforcement

Flutter may request these operations from the Rust engine.

---

## 4. Rust Engine Boundary

Rust owns:

- system detection
- process execution
- command execution
- Git operations
- Docker operations
- WSL inspection
- tool detection
- version comparison
- lab lifecycle
- catalog retrieval
- manifest validation
- lab update/reset/remove
- structured logging
- CLI

Rust must expose predictable machine-readable results.

The engine should be usable without Flutter.

---

## 5. Process Boundary

Preferred V1 communication:

```text
Flutter
  ↓
start engine process
  ↓
arguments / JSON
  ↓
engine
  ↓
JSON result
```

The engine should terminate after a command when practical.

Do not introduce a resident daemon unless a concrete requirement proves it necessary.

---

## 6. CLI Contract

The CLI should support:

```text
zitera doctor
zitera lab list
zitera lab install <id>
zitera lab start <id>
zitera lab stop <id>
zitera lab status <id>
zitera lab reset <id>
zitera lab update <id>
zitera lab remove <id>
zitera tool status
```

Machine-readable mode:

```text
zitera --json doctor
zitera --json lab status A01
```

The Flutter layer should consume stable output contracts rather than scraping human-readable terminal text.

---

## 7. Catalog Architecture

The application reads a central catalog.

```text
catalog.json
     ↓
Flutter / Engine
     ↓
lab metadata
     ↓
repository + version
```

The catalog is content/discovery metadata.

It must not contain executable arbitrary instructions.

---

## 8. Lab Architecture

Each lab is independently versioned.

```text
Lab Repository
│
├── manifest.json
├── lesson/
├── challenge/
├── docker/
├── assets/
└── README.md
```

The Rust engine interprets the manifest and performs only supported operations.

A lab repository must not be allowed to redefine core engine behavior.

---

## 9. Runtime Architecture

Default runtime:

```text
Windows
  ↓
Docker Desktop
  ↓
WSL2 backend
  ↓
Lab containers
```

Lab services should bind to localhost by default.

Each lab should have a predictable lifecycle:

```text
install
verify
start
status
stop
reset
update
remove
```

---

## 10. Data Separation

Separate:

```text
core application
lab content
runtime state
user progress
logs
cache
```

Do not mix generated runtime files with source/content files.

---

## 11. Update Separation

### Core update

Changes:

- Flutter
- Rust engine
- core contracts
- core UI

Requires application rebuild/release.

### Lab/content update

Changes:

- lesson
- challenge
- assets
- lab runtime
- lab metadata

Must not require core rebuild.

---

## 12. Dependency Principle

Prefer external tools already designed to solve the problem:

- Git
- Docker
- WSL2
- official OS/package installers

Do not recreate them.

---

## 13. Engineering Principle

Prefer:

```text
clear boundaries
small modules
explicit contracts
predictable errors
simple processes
```

Avoid:

```text
premature abstractions
hidden magic
global state
large multipurpose classes/files
unnecessary services
```

---

## 14. Architectural Change Rule

If a sprint wants to change:

- Flutter ↔ Rust boundary
- lab contract
- runtime model
- update model
- security boundary
- storage architecture

the change must first be recorded as an ADR.

---

## 15. Quality Rule

Every architecture component must be independently testable where practical.

The engine must be testable without the Flutter UI.

The lab contract must be testable without the entire application.

---

# End
