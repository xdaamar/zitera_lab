# PHASE 20 // RELEASE ENGINEERING & DISTRIBUTION BASELINE

**Document ID:** `PHASE_20_BASELINE.md`
**Program:** ZITERA 2.0 Core Modernization & Productization
**Sprint Phase:** Phase 20 (Release Engineering, Distribution & Field Readiness)
**Date:** October 7, 2026
**Target Platform:** Windows x64 (Native Sandboxed Runtime / AppContainer / Job Object)
**Branch:** `migration/phase-20-release-readiness`
**Phase 19 HEAD Commit:** `27dc426` (`phase19(cp14-20): full regression verification, security matrix, and authoritative sprint reports`)

---

## 1. Executive Summary

Phase 19 closed with complete pedagogical alignment against OWASP Top 10:2025, pure data-driven UI rendering, stable package identity decoupling, an educational in-process terminal, automated CLI validator/scaffolding tooling, and 100% test pass rate across 101 automated cargo tests.

Phase 20 serves as the release engineering, distribution, and field readiness milestone. Its primary objective is to transition ZITERA_LAB into an enterprise-grade Windows product that can be reliably distributed, installed, upgraded, recovered, diagnosed, and operated by non-developer students without requiring Git, Docker, WSL2, Rust, Python, Cargo, or development toolchains.

This document records the exact baseline state of all platform components at the inception of Phase 20.

---

## 2. Environment & Toolchain State

- **Operating System:** Windows 11 Home 64-bit (Build 26200)
- **Rust Toolchain:** `rustc 1.98.1 (48a229cea 2026-09-01)` / Cargo 1.98.1
- **C/C++ Build Environment:** Microsoft Visual Studio Build Tools 2022 (MSVC vcvars64 x64)
- **Flutter Framework:** Flutter 3.29+ (Windows Desktop Runner)
- **Working Tree State:** Clean, branched from `migration/phase-18-full-curriculum` at commit `27dc426`
- **Execution Integrity:** Standard non-elevated user privilege; zero administrator requirements; zero Docker / WSL2 dependencies

---

## 3. Subsystem Baseline Inventory

### 3.1 Core Engine & Native Runtime (`engine/rust/`)
- **Native Sandbox Model:** Low-integrity Windows AppContainers (`S-1-15-2-...`) with zero capabilities granted (`CapabilityCount = 0`).
- **Job Object Containment:** Windows Job Objects configured with `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`, hard memory caps, and process count limits (`active_process_limit = 16`).
- **Local HTTP Broker:** Dynamic loopback proxy (`127.0.0.1:<port>`) with SSRF blocking, host header enforcement, and cryptographic ephemeral session tokens (`/session/<token>/`).
- **Educational Terminal:** In-process terminal session (`TerminalSession`) supporting sandboxed virtual filesystem commands (`ls`, `cat`, `grep`, `pwd`, `cd`, `echo`), process inspection (`ps`), contextual command documentation (`help <cmd>`), and built-in safe localhost `curl` fallback.
- **CLI Subcommands:**
  - `zitera doctor`: Native Windows readiness verification.
  - `zitera lab create <id>`: Scaffolding generator for new labs.
  - `zitera lab validate <id>`: 8-point automated compliance validator.
  - `zitera lab serve <id>`: Background lab process broker launcher.

### 3.2 Curriculum & Package Subsystem
- **Authoritative Modules:** All 10 labs (`A01` through `A10`) fully implemented as native MSVC Rust executables (~300 KB each) with dual-mode practice and challenge mechanics.
- **Package Architecture:** Compressed `.zlab` ZIP packages signed with asymmetric Ed25519 digital signatures.
- **Identity Decoupling:** Stable package identity (`package_id`, e.g. `zitera-lab-a01`) decoupled from dynamic curriculum categories (`category_id`, e.g. `A01`).
- **Anti-Downgrade Protection:** Monotonic `security_version` counter enforcement and semantic version comparisons preventing downgrade attacks.

### 3.3 Flutter Presentation Layer (`ui/flutter/`)
- **Data-Driven Architecture:** Pure generic rendering (`GenericLabCard`, `LabDetailView`, `PracticeModeView`, `ChallengeModeView`) driven by signed catalog metadata without per-lab hardcoded switches.
- **Terminal Console Widget:** Monospace cybernetic dark console (`TerminalConsoleWidget`) with command history and in-place feedback.
- **Update Center:** Platform version card and offline-first catalog update UI in `settings_view.dart`.
- **Progress Model:** Dual-key persistence indexing by `package_id` with backward-compatible lookup by `lab_id`.

---

## 4. Existing Documentation & Test Inventory

### 4.1 Developer Documentation Suite (`/docs/`)
- `docs/architecture.md`: System topography, boundaries, and IPC model.
- `docs/build.md`: Clean-room build instructions for engine, labs, and Flutter UI.
- `docs/security.md`: Threat model, AppContainer isolation, Job Objects, and broker defenses.
- `docs/lab-development.md`: Step-by-step authoring guide for creating and packaging new labs.
- `docs/release.md`: Release engineering guide, versioning semantics, and signing protocol.

### 4.2 Test Suite Verification
- **Automated Engine Unit & Integration Tests:** 101/101 passing (`cargo test --release --bin zitera-engine`).
- **Automated CLI Validator:** 10/10 labs passing 8/8 validation checks (`zitera lab validate A01..A10`).
- **Security Matrices:**
  - `reports/PHASE_16_SECURITY_MATRIX.md`
  - `reports/PHASE_17_SECURITY_MATRIX.md`
  - `reports/PHASE_18B_SECURITY_MATRIX.md`
  - `reports/PHASE_19_SECURITY_MATRIX.md`

---

## 5. Phase 20 Scope & Objectives

The sprint will execute the following checkpoint sequence:
1. **CP00:** Release Baseline Establishment & Toolchain Verification.
2. **CP01:** Full Repository Release Test Matrix (`scripts/verify_release.ps1`).
3. **CP02:** Terminal Security Regression Matrix (`ps`, `help`, `curl`, console).
4. **CP03:** Native Process Isolation Regression.
5. **CP04:** Package Lifecycle, Update & Atomic Rollback Regression.
6. **CP05:** Clean-Machine Installation Proof in Isolated Workspace.
7. **CP06:** Windows Distribution Architecture Decision (`docs/distribution.md`).
8. **CP07:** Deterministic Release Artifact Pipeline.
9. **CP08:** Windows Release Signing Workflow Formalization.
10. **CP09:** Distribution Installer Implementation.
11. **CP10:** User Data & Lab Storage Boundaries Policy.
12. **CP11:** Privacy-Safe Diagnostic Export.
13. **CP12:** Runtime Failure Recovery UX.
14. **CP13:** Legacy State Migration & Progress Preservation.
15. **CP14:** Real User First-Run Workflow Polish.
16. **CP15:** Performance Regression Baseline & Benchmarking.
17. **CP16:** Full Security Release Gate.
18. **CP17:** Release Candidate 1 Preparation (`RC1`).
19. **CP18:** Release Candidate 2 Hardening (`RC2`).
20. **CP19:** Final Documentation & Release Operations.
21. **CP20:** Final Release Gate & Program Closure.
