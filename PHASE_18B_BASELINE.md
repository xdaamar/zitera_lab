# PHASE 18B ARCHITECTURAL AND SECURITY HARDENING BASELINE
**Document ID**: `PHASE_18B_BASELINE.md`  
**Execution Context**: Zitera Core Engine (Flutter UI + Rust Host Broker + Native Sandboxed Labs)  
**Platform**: Windows x64 (MSVC toolchain, zero elevation, zero Docker/WSL runtime dependencies)  
**Branch**: `migration/phase-18-full-curriculum`  
**Baseline HEAD**: `781796311f9023d91699b7268bf0323208fc8f77`  

---

## 1. Executive Summary

Phase 18 successfully established the curriculum migration of all 10 labs (A01 through A10) to native sandboxed AppContainer executables. However, before considering the platform fully production-ready, Phase 18B enforces corrective closure, trust chain hardening, package integrity verification, broker confinement, recovery guarantees, and secret hygiene.

This document serves as the formal architectural freeze and baseline audit for Phase 18B.

---

## 2. Core Audit: Five Questions

### A. What Already Exists?

1. **Native Sandboxed Labs (A01–A10):**
   - All 10 lab binaries (`a01_lab.exe` to `a10_lab.exe`) compile natively via Rust and are deployed to `labs/AXX/bin/`.
   - Each lab communicates exclusively with the host broker via deterministic `stdin`/`stdout` JSON lines protocol.
   - CLI argument parsing (`--help`, `-h`, `--version`, `-v`) is enabled across all 10 binaries to guarantee instantaneous response (<50ms) and prevent accidental deadlocks on interactive `stdin`.

2. **Package Subsystem Foundation (`engine/rust/src/package/`):**
   - `archive.rs`: Deterministic `.zlab` package creation and extraction with custom archive magic `ZLAB`, schema verification, CRC32 checks, path traversal mitigation, and strict file count limits.
   - `sha256.rs`: Zero-dependency, purely deterministic SHA-256 and HMAC implementations.
   - `trust.rs`: Ed25519 asymmetric signature generation and verification (`ed25519-dalek 3.0.0`), canonical payload serialization (lexicographically sorted newline-delimited key-value format), and trusted root public keys (`RELEASE_PUBLIC_KEY_HEX`, `DEV_PUBLIC_KEY_HEX`).
   - `installer.rs`: Atomic staging and activation via `active_version.txt`, non-destructive rollback logic, semantic version comparisons, and anti-downgrade checks for `security_version` and version strings.
   - `verifier.rs`: 11-point pre-extraction security verification (archive bounds, manifest schema, lab ID format, runtime isolation model, target architecture, path traversal rejection, content digest computation, package SHA-256 hash check, and cryptographic signature verification).

3. **Signed Catalog Subsystem (`engine/rust/src/catalog.rs`):**
   - Central catalog model with remote fetching capability (`REMOTE_CATALOG_URL`).
   - Cryptographic validation requiring Ed25519 signature binding schema version, lab count, and sorted lab entries.
   - Strict protocol enforcement: rejects `ftp://`, `file://`, `http://`, or UNC paths (`\\\\`).

4. **Terminal and Broker Boundaries:**
   - Strict blacklist blocking arbitrary shells (`cmd.exe`, `powershell.exe`) and interactive tools.
   - Session-bound token authentication for HTTP broker requests (`/session/{token}/...`).

---

### B. What is Incomplete?

1. **Legacy HMAC Fallback in Package Verifier:**
   - In `engine/rust/src/package/verifier.rs` (lines 178–185), a fallback to shared-secret HMAC (`DEV_SIGNING_KEY`, `TEST_SIGNING_KEY`) exists for legacy test packages.
   - In production trust architecture, all `.zlab` packages must strictly require asymmetric Ed25519 signatures signed by the release authority. Shared HMAC secrets must be eliminated from verification logic.

2. **Package Test Suite Modernization:**
   - In `engine/rust/src/package/mod.rs` (lines 36–38), test helpers generate HMAC signatures instead of canonical Ed25519 signatures.

3. **Adversarial Invalid Package Verification Matrix:**
   - While valid package installation and basic tampered package rejection are tested, negative test cases covering the full adversarial matrix (truncated package, corrupt magic, zip bomb/file count exhaustion, path traversal attempt, unsigned package, forged signature, mismatched content digest, architecture mismatch, invalid runtime specification) require comprehensive test coverage.

4. **Clean-Room Installation & Offline First Proof:**
   - Formal verification that the application operates deterministically in an isolated environment without network connectivity when packages are pre-cached, and cleanly fails with explanatory diagnostics when an un-cached package requires network access without connectivity.

---

### C. What is Only Claimed but Not Proven?

1. **Anti-Downgrade & Anti-Rollback Attack Resistance:**
   - The code contains checks for `security_version` decreasing, but needs dedicated automated tests proving an attacker cannot install a previously valid package with an older `security_version` or older semantic version over an updated installation.

2. **Broker Confinement & Resource Limits Under Abnormal Termination:**
   - Verified in unit tests, but requires evidence of rapid broker recovery when a lab process abruptly crashes or exits with abnormal status codes.

3. **Release Reproducibility & Secret Cleanliness:**
   - No private keys or development seeds must leak into release binaries or production artifacts.

---

### D. What Must Be Changed?

1. **Remove HMAC Shared-Secret Fallback in `verifier.rs` and `package/mod.rs`:**
   - Remove `DEV_SIGNING_KEY` and `TEST_SIGNING_KEY` fallback from `verify_package()`.
   - Update tests to sign packages exclusively using Ed25519 canonical signatures (`sign_payload` with dev seed).

2. **Implement Adversarial Package Verification Test Matrix:**
   - Add explicit negative test cases for every failure class:
     - Missing signature / unsigned package.
     - Corrupted or forged Ed25519 signature.
     - Tampered manifest or content payload.
     - Manipulated content digest.
     - Out-of-bounds archive structure / traversal paths.
     - Incompatible architecture (`arm64` on `x86_64`) and unsupported runtime.

3. **Harden Anti-Downgrade Suite:**
   - Add tests explicitly validating rejection of downgrade attempts for both `security_version` and semantic versions.

4. **Harden Atomic Rollback on Extraction Interruption:**
   - Verify that simulated disk failure or invalid payload during extraction leaves the currently active version 100% operational and undisturbed.

---

### E. What Must NOT Be Changed?

1. **Curriculum Logic & Content (A01–A10):**
   - The educational markdown, challenge solutions, flags, and vulnerabilities in labs A01 through A10 are verified and complete. Do not alter lab challenge scenarios.
2. **Zero Host Elevation & Zero Docker/WSL Invariant:**
   - The core runtime must remain 100% native on Windows without requiring Administrator privileges, Docker Desktop, or WSL2.
3. **No Unnecessary Framework Abstractions or External Dependencies:**
   - Do not introduce complex third-party frameworks, network daemons, or unnecessary crates. Keep standard Rust and MSVC toolchains.
4. **No TLS Bypasses:**
   - Never introduce `sslVerify=false` or `--insecure` flags anywhere in scripts or code.

---

## 3. Toolchain & Quality Gate Baseline

| Gate | Tool / Command | Baseline Result |
|---|---|---|
| Git Formatting | `git diff --check` | **PASS** (Clean whitespace, no trailing lines) |
| Rust Compilation | `cargo check` (MSVC x64) | **PASS** (Zero warnings, zero errors) |
| Rust Package Tests | `cargo test --bin zitera-engine package` | **PASS** (10/10 tests passed) |
| Lab Lifecycle Tests | `test_a01`, `test_a10` | **PASS** (Native sandboxed lifecycle verified) |
| Security Hygiene | Zero `sslVerify=false`, Zero `cmd.exe`/`powershell` execution | **PASS** (Clean audit) |

---

## 4. Next Checkpoint Action Plan

- **Checkpoint 1 & 2**: Migrate Package Trust Model to pure asymmetric Ed25519 signing (deprecate HMAC fallback in `verifier.rs` and update test package creation).
- **Checkpoint 3 & 4**: Implement signed catalog validation proofs and anti-downgrade test matrix.
- **Checkpoint 5 & 6**: Harden atomic installation, staging, and rollback recovery.
- **Checkpoint 7 & 8**: Broker security, origin checks, and lifecycle recovery tests.
- **Checkpoint 9–17**: Release binary proof, full regression, offline behavior, security matrix, and comprehensive reports.
