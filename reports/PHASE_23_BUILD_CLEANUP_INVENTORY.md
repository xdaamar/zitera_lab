# PHASE 23: BUILD CLEANUP INVENTORY
**Document Reference:** `reports/PHASE_23_BUILD_CLEANUP_INVENTORY.md`  
**Phase:** 23 (CP02 — Complete Build Artifact Cleanup)  
**Target Platform:** Windows x64  
**Author:** AI Pair Programmer (Software Craftsmanship Active)  
**Date:** 2026-10-12  
**Status:** AUDITED & SCHEDULED FOR REMOVAL

---

## 1. Executive Summary

In strict accordance with the Phase 23 **MANDATORY STOP-BUILD RULE** and **CP02 Cleanup Policy**, this document records all generated build directories, executables, compiled DLLs, and release archives across the repository scheduled for complete deletion.

All candidates identified below are generated compilation outputs that can be 100% reproduced from source once the curriculum reorganization is approved by the owner. No source files, lesson contents, challenge definitions, schemas, signing keys, or authoritative manifests are targeted.

---

## 2. Pre-Deletion Inventory Table

| Relative Path | Artifact Category | Git Tracked? | Approximate Size | Originating Build Source | Safe to Delete? | Reason & Preserved Source |
|---|---|---|---|---|---|---|
| `dist/ZITERA_LAB_WINDOWS_X64/` | Generated Release Staging | No (Ignored) | ~36.8 MB | `build_distribution_bundle.ps1` | YES | Standalone bundle directory from Phase 22. Reproducible from source. |
| `dist/ZITERA_LAB_WINDOWS_X64.zip` | Release Archive | No (Ignored) | ~24.9 MB | `build_distribution_bundle.ps1` | YES | Phase 22 distribution ZIP. Reproducible from bundle. |
| `dist/ZITERA_LAB_RC1_windows_x64.zip` | Historical Release Archive | No (Ignored) | ~2.47 MB | Historical Phase 20 RC1 | YES | Obsolete intermediate RC archive. |
| `dist/ZITERA_LAB_RC2_windows_x64.zip` | Historical Release Archive | No (Ignored) | ~2.47 MB | Historical Phase 20 RC2 | YES | Obsolete intermediate RC archive. |
| `dist/zitera-lab-2.0.0-rc1-windows-x64-portable.zip` | Historical Release Archive | No (Ignored) | ~724 KB | Historical Phase 20 | YES | Obsolete CLI portable archive. |
| `dist/zitera-lab-2.0.0-windows-x64-portable.zip` | Historical Release Archive | No (Ignored) | ~685 KB | Historical Phase 20 | YES | Obsolete CLI portable archive. |
| `dist/bin/` | Legacy Generated Binaries | No (Ignored) | ~1.9 MB | Historical Phase 20 Build | YES | Contains old compiled `zitera-engine.exe`. Source in `engine/rust/`. |
| `dist/catalog/` | Generated Bundle Catalog Copy | No (Ignored) | ~19 KB | `build_distribution_bundle.ps1` | YES | Copy of authoritative `catalog/` directory. Authoritative source in `catalog/` preserved. |
| `dist/installer/` | Legacy Installer Staging | No (Ignored) | Empty / Intermediate | Historical Phase 20 Build | YES | Staging directory for installer output. |
| `dist/packages/` | Generated Package Copies | No (Ignored) | ~3.8 MB | Historical Packaging Script | YES | Generated `.zlab` copies. Canonical package sources in `labs/` preserved. |
| `dist/checksums.sha256` | Generated Checksums File | No (Ignored) | 198 bytes | Historical Release Script | YES | Checksums for deleted legacy archives. |
| `dist/release_manifest.json` | Generated Release Manifest | No (Ignored) | 1.8 KB | Historical Release Script | YES | Obsolete manifest for Phase 20. |
| `ui/flutter/build/` | Flutter Build Intermediates | No (Ignored) | ~145 MB | Flutter Windows Build | YES | Entire Flutter compilation cache, runner intermediates, C++ obj files. |
| `engine/rust/target/` | Cargo Target Directory | No (Ignored) | ~220 MB | Cargo Build Profile | YES | Compiled Rust engine binaries, lab build artifacts, intermediate deps. |
| `labs/A01/bin/a01-lab.exe` | Compiled Lab Binary | No (Ignored) | 309 KB | `engine/rust/src/bin/a01_lab.rs` | YES | 100% reproducible from Rust source. |
| `labs/A02/bin/a02-lab.exe` | Compiled Lab Binary | No (Ignored) | 303 KB | `engine/rust/src/bin/a02_lab.rs` | YES | 100% reproducible from Rust source. |
| `labs/A03/bin/a03-lab.exe` | Compiled Lab Binary | No (Ignored) | 308 KB | `engine/rust/src/bin/a03_lab.rs` | YES | 100% reproducible from Rust source. |
| `labs/A04/bin/a04-lab.exe` | Compiled Lab Binary | No (Ignored) | 294 KB | `engine/rust/src/bin/a04_lab.rs` | YES | 100% reproducible from Rust source. |
| `labs/A05/bin/a05-lab.exe` | Compiled Lab Binary | No (Ignored) | 618 KB | `engine/rust/src/bin/a05_lab.rs` | YES | 100% reproducible from Rust source. |
| `labs/A06/bin/a06-lab.exe` | Compiled Lab Binary | No (Ignored) | 307 KB | `engine/rust/src/bin/a06_lab.rs` | YES | 100% reproducible from Rust source. |
| `labs/A07/bin/a07-lab.exe` | Compiled Lab Binary | No (Ignored) | 561 KB | `engine/rust/src/bin/a07_lab.rs` | YES | 100% reproducible from Rust source. |
| `labs/A08/bin/a08-lab.exe` | Compiled Lab Binary | No (Ignored) | 325 KB | `engine/rust/src/bin/a08_lab.rs` | YES | 100% reproducible from Rust source. |
| `labs/A09/bin/a09-lab.exe` | Compiled Lab Binary | No (Ignored) | 313 KB | `engine/rust/src/bin/a09_lab.rs` | YES | 100% reproducible from Rust source. |
| `labs/A10/bin/a10-lab.exe` | Compiled Lab Binary | No (Ignored) | 293 KB | `engine/rust/src/bin/a10_lab.rs` | YES | 100% reproducible from Rust source. |

---

## 3. Explicitly Preserved Files (Not Targeted for Removal)

The following files are strictly preserved to safeguard curriculum integrity and future build reproducibility:
1. **Source Code & Manifests:**
   - `catalog/catalog.json`, `catalog/catalog.signature`
   - `labs/A01/manifest.json` through `labs/A10/manifest.json`
   - All `labs/A*/lesson/`, `labs/A*/challenge/`, `labs/A*/hints/`, and `labs/A*/metadata/` files.
   - All Rust source files under `engine/rust/src/` (including all `src/bin/a01_lab.rs` .. `a10_lab.rs`).
   - All Flutter Dart code under `ui/flutter/lib/` and test suites.
2. **Authoritative Branding & Configuration:**
   - `branding/logo.png`, `branding/app_icon.ico`
   - `ui/flutter/windows/runner/resources/app_icon.ico`, `Runner.rc`, `main.cpp`
   - `Cargo.lock`, `Cargo.toml`, `pubspec.lock`, `pubspec.yaml`
3. **Distribution Documentation & Schemas:**
   - `dist/README.md`
   - `dist/release_manifest.schema.json`
4. **Historical Documentation & Reports:**
   - All reports in `reports/` and `dev_internal/`.

---

## 4. Deletion Safety Execution

All deletions will be executed strictly within the repository workspace using bounded relative PowerShell paths. No files outside the repository root will be touched.
