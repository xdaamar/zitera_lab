# PHASE 21 // FOCUSED LIGHTWEIGHT OPTIMIZATION & BOUNDED EXECUTION
## BASELINE AUDIT REPORT (CP00)

- **Document ID:** `PHASE_21_BASELINE.md`
- **Program:** ZITERA 2.0 Core Modernization & Production Optimization
- **Sprint Phase:** Phase 21 (Focused Lightweight Optimization & Bounded Execution)
- **Date:** October 9, 2026
- **Target Platform:** Windows x64 (`x86_64-pc-windows-msvc`)
- **Starting SHA:** `e854344` (`chore(phase20): finalize release engineering and field readiness`)
- **Branch:** `migration/phase-21-focused-lightweight`
- **Working Tree State:** Clean from modifications (draft Cargo profiles stashed to `stash@{0}`, 6 historical report files preserved untracked)
- **Primary Objective:** Membuat ZITERA_LAB lebih ringan dibuka dan dijalankan, mengurangi beban RAM dan CPU yang tidak perlu, mempercepat workspace Antigravity IDE, serta menjaga arsitektur dan security boundaries yang sudah dibangun.

---

## 1. Executive Summary

Phase 20 menyelesaikan release engineering, distribution packaging, offline installer workflows, privacy-safe diagnostics, failure recovery UX, dan legacy state migration, menghasilkan `RC2` release bundle dengan 100% kepatuhan pada 7 verification stages dan 17 security controls.

Namun, membuka repositori ZITERA_LAB di Antigravity IDE pada perangkat RAM 16 GB terasa sangat berat. Audit awal menemukan akar penyebab nyata:
1. **Lebih dari 1.08 GB dan 2,505 generated files** berada di dalam working tree (`engine/rust/target`, `ui/flutter/build`, `ui/flutter/.dart_tool`).
2. Repositori **tidak memiliki `.vscode/settings.json`** dan `.gitignore` mengabaikan `.vscode/`, sehingga Antigravity IDE / VS Code file watcher dan global search memindai seluruh 2,500+ build artifacts dan intermediate object files secara berulang.
3. Aset gambar pada UI (`ui/flutter/assets`) berukuran **12.68 MB** (dengan `sidebar_bg.mp4` 2.98 MB, `sidebar_bg.webp` 2.54 MB, lima background JPEG resolusi tinggi masing-masing ~1 MB, dan file logo duplikat PNG & JPEG).
4. Tidak ada batasan eksekusi eksplisit pada beberapa workflow automation, yang berisiko memicu infinite loop atau command menggantung.

Dokumen ini mencatat baseline empiris awal sebelum modifikasi optimasi dilakukan pada CP01–CP06.

---

## 2. Environment & Toolchain State

- **Host Operating System:** Microsoft Windows 11 Home 64-bit (NT Build 10.0.26200.0)
- **Hardware Architecture:** x86_64 (12 Logical Cores)
- **Reported RAM:** 16 GB Physical RAM (User hardware baseline)
- **Rust Toolchain:** `rustc 1.98.1 (48a229cea 2026-09-01)`
- **Cargo Toolchain:** `cargo 1.98.1 (797e8a9bc 2026-08-05)`
- **C/C++ Build Environment:** Microsoft Visual Studio Build Tools 2022 (`link.exe`, MSVC MSBuild)
- **Flutter / Dart:** Flutter Windows Desktop Runner source & build tree terkelola di repositori; Flutter CLI tidak terdapat pada host global PATH
- **Version Control:** Git 2.48+, remote origin: `https://github.com/xdaamar/zitera_lab.git`
- **Security Context:** Standard non-elevated user token; zero admin rights; zero Docker/WSL2 dependency; zero telemetry

---

## 3. Repository Disk Footprint Baseline

Pengukuran empiris direktori repositori menggunakan filesystem enumeration:

| Direktori | Jumlah Item | Ukuran (MB) | Ukuran (Bytes) | Deskripsi Komponen |
| :--- | :---: | :---: | :---: | :--- |
| `engine\rust\src` | 41 | 0.77 MB | 809,129 | Source code native Rust engine |
| `engine\rust\target` | 2,047 | 586.01 MB | 614,480,220 | Cargo build outputs & intermediate objects |
| `ui\flutter\lib` | 26 | 0.43 MB | 450,563 | Flutter UI presentation source code |
| `ui\flutter\assets` | 23 | 12.68 MB | 13,295,126 | Backgrounds, stickers, fonts, video |
| `ui\flutter\build` | 365 | 340.78 MB | 357,335,379 | Flutter desktop build artifacts |
| `ui\flutter\.dart_tool` | 93 | 160.56 MB | 168,361,218 | Dart build system cache |
| `labs` | 425 | 3.91 MB | 4,099,165 | 10 canonical OWASP labs (`A01`–`A10`) |
| `dist` | 21 | 11.62 MB | 12,184,786 | Release distributions (`RC1`, `RC2`, portable) |
| `scripts` | 11 | 0.10 MB | 108,969 | Verification, benchmark, & release scripts |
| `docs` | 8 | 0.05 MB | 47,733 | Dokumentasi arsitektur dan operasional |
| `reports` | 19 | 0.15 MB | 159,670 | Security matrices & sprint verification reports |
| `catalog` | 1 | 0.01 MB | 11,214 | Signed curriculum catalog |
| `.git` | 149 | 13.26 MB | 13,908,898 | Version control repository database |
| **Total Tracked Files** | **397** | — | — | Dihitung via `git ls-files` |

### Temuan Utama Generated Directories:
Total beban generated directories di workspace adalah **1,087.35 MB (1.08 GB)** yang mencakup **2,505 files**. Folder ini menjadi beban utama file watcher dan indexer jika tidak dikecualikan secara eksplisit di level editor.

---

## 4. Tracked Binary & Large File Inventory

Daftar file biner dan file besar yang di-track oleh Git (`git ls-files`):

| File Path | Ekstensi | Ukuran (KB) | Ukuran (Bytes) | Catatan & Analisis |
| :--- | :---: | :---: | :---: | :--- |
| `ui/flutter/assets/videos/sidebar_bg.mp4` | `.mp4` | 2,977.34 KB | 3,048,798 | Background video; ukuran signifikan |
| `ui/flutter/assets/images/sidebar_bg.webp` | `.webp` | 2,597.95 KB | 2,660,302 | WebP uncompressed/high-res; kandidat optimasi |
| `ui/flutter/assets/images/bg_dashboard.jpg` | `.jpg` | 1,058.61 KB | 1,084,017 | Photographic JPEG background (~1 MB) |
| `ui/flutter/assets/images/bg_tools.jpg` | `.jpg` | 1,038.07 KB | 1,062,987 | Photographic JPEG background (~1 MB) |
| `ui/flutter/assets/images/bg_environment.jpg` | `.jpg` | 1,027.58 KB | 1,052,239 | Photographic JPEG background (~1 MB) |
| `ui/flutter/assets/images/bg_labs.jpg` | `.jpg` | 1,021.40 KB | 1,045,912 | Photographic JPEG background (~1 MB) |
| `ui/flutter/assets/images/bg_settings.jpg` | `.jpg` | 984.89 KB | 1,008,523 | Photographic JPEG background (~1 MB) |
| `ui/flutter/assets/images/zitera_logo.jpg` | `.jpg` | 686.83 KB | 703,315 | Duplikat persis format JPG |
| `ui/flutter/assets/images/zitera_logo.png` | `.png` | 686.83 KB | 703,315 | Duplikat persis format PNG |
| `ui/flutter/assets/fonts/JetBrainsMono.ttf` | `.ttf` | 182.82 KB | 187,208 | UI terminal font |
| `ui/flutter/assets/fonts/PlusJakartaSans.ttf` | `.ttf` | 172.16 KB | 176,288 | UI primary font |
| `ui/flutter/assets/fonts/SpaceGrotesk.ttf` | `.ttf` | 133.47 KB | 136,676 | UI display font |
| `ui/flutter/assets/images/zeta_avatar.png` | `.png` | 127.22 KB | 130,273 | Mascot avatar |
| `engine/rust/src/labs.rs` | `.rs` | 111.54 KB | 114,221 | Rust source code (labs module) |
| `ui/flutter/assets/images/sidebar_bg.jpg` | `.jpg` | 60.00 KB | 61,438 | Legacy background |
| `ui/flutter/assets/images/sticker_shiba.png` | `.png` | 54.99 KB | 56,305 | Sticker asset |
| `ui/flutter/assets/images/sticker_hamster.png` | `.png` | 45.32 KB | 46,405 | Sticker asset |
| `ui/flutter/assets/images/sticker_bunny.png` | `.png` | 44.88 KB | 45,958 | Sticker asset |
| `ui/flutter/assets/images/sticker_duck.png` | `.png` | 35.97 KB | 36,835 | Sticker asset |
| `ui/flutter/assets/images/calico_cat.png` | `.png` | 29.77 KB | 30,485 | Sticker asset |
| `ui/flutter/windows/runner/resources/app_icon.ico` | `.ico` | 10.12 KB | 10,366 | Desktop application icon |
| `ui/flutter/assets/images/sticker_pink.png` | `.png` | 4.63 KB | 4,745 | Sticker asset |
| `ui/flutter/assets/images/sticker_purple.png` | `.png` | 4.55 KB | 4,657 | Sticker asset |
| `ui/flutter/assets/images/sticker_blonde.png` | `.png` | 4.24 KB | 4,342 | Sticker asset |
| `ui/flutter/assets/images/sticker_cat_laptop.png` | `.png` | 4.01 KB | 4,103 | Sticker asset |

**Total Tracked Binary/Large Files:** 25 files (12.80 MB).
**Temuan Asset Duplikat/Unused:**
- `zitera_logo.jpg` dan `zitera_logo.png` identik 703,315 bytes (duplikasi asset).
- `sidebar_bg.jpg` (60 KB) berpotensi redundant terhadap `sidebar_bg.webp` (2.54 MB).

---

## 5. Existing Workspace Exclusion Configurations

- **`.gitignore` Baseline:**
  - Mengabaikan `target/`, `build/`, `.dart_tool/`, `dist/`, `dev_internal/`.
  - Mengabaikan `.vscode/` secara utuh.
- **`.vscode/settings.json`:**
  - **TIDAK ADA** pada baseline commit `e854344`.
  - Tidak ada konfigurasi `files.exclude`, `search.exclude`, atau `files.watcherExclude`.
  - Dampak: Antigravity IDE / VS Code memproses 2,505 generated files pada setiap startup dan search query.

---

## 6. Existing Build & Verification Scripts

1. `scripts\verify_release.ps1` (6,770 bytes): 7-stage release verification matrix.
2. `scripts\verify_security_gate.ps1` (16,131 bytes): 17-control security gate.
3. `scripts\measure_performance.ps1` (10,395 bytes): 12-point engine performance benchmarking.
4. `scripts\test_phase12_security.ps1` (8,543 bytes): Legacy security checks.

---

## 7. Baseline Execution Times & Latencies

### 7.1 Quality Gate Baselines (Empiris)

| Verification Suite | Cakupan | Durasi Eksekusi | Status Kepatuhan |
| :--- | :--- | :---: | :---: |
| `scripts\verify_release.ps1` | 7 Stages (Broker, Trust, Isolation, Terminal, Lifecycle, Labs, Release) | 13.4s | **PASS (100%)** |
| `scripts\verify_security_gate.ps1` | 17 Controls (Signature, Anti-downgrade, Sandboxing, SSRF, AppContainer, etc.) | 5.4s | **PASS (100%)** |

### 7.2 Native Engine Operation Latency Baseline (5 samples)

| Operasi | Min (ms) | Median (ms) | P95 (ms) | Max (ms) | Status Baseline |
| :--- | :---: | :---: | :---: | :---: | :---: |
| Engine Startup (`--version`) | 59.7 ms | **62.6 ms** | 72.8 ms | 72.8 ms | PASS (<100 ms) |
| Application Startup (`doctor`) | 735.9 ms | **760.3 ms** | 773.7 ms | 773.7 ms | PASS (<1000 ms) |
| Catalog Load (`lab list --json`) | 806.9 ms | **825.2 ms** | 858.9 ms | 858.9 ms | PASS (<1000 ms) |
| Dashboard Load (Doctor + Catalog) | 1,616.3 ms | **1,702.0 ms** | 1,887.0 ms | 1,887.0 ms | Kandidat Optimasi |
| Lab List | 779.0 ms | **811.1 ms** | 884.0 ms | 884.0 ms | PASS (<1000 ms) |
| Lab Detail (`lab status A01`) | 55.4 ms | **59.0 ms** | 61.5 ms | 61.5 ms | PASS (<100 ms) |
| Package Verification (`lab validate A01`) | 59.1 ms | **67.2 ms** | 71.7 ms | 71.7 ms | PASS (<100 ms) |
| Lab Install (`lab install A01`) | 59.7 ms | **60.3 ms** | 86.6 ms | 86.6 ms | PASS (<100 ms) |
| Lab Launch (`lab status A01`) | 55.1 ms | **56.6 ms** | 61.2 ms | 61.2 ms | PASS (<100 ms) |
| Terminal Command (`lab validate A01 --json`) | 63.8 ms | **79.9 ms** | 179.2 ms | 179.2 ms | PASS (<200 ms) |
| Progress Save (JSON atomic persistence) | 4.7 ms | **4.9 ms** | 5.5 ms | 5.5 ms | PASS (<10 ms) |
| Lab Update (`lab update A01`) | 799.3 ms | **877.5 ms** | 1,022.4 ms | 1,022.4 ms | PASS (<1200 ms) |

---

## 8. Baseline Memory Measurements & Boundaries

- **Production Binary Size:** `engine\rust\target\release\zitera-engine.exe` = 1,925,120 bytes (~1.84 MB).
- **Idle Working Set:** < 35 MB RAM per child process.
- **Host Context:** Laptop RAM 16 GB; sandboxed process environment restricts CIM/WMI memory counters; no unauthorized administrative queries executed.
- **Architectural Freeze Verified:** AppContainer, Job Object, Ed25519 signatures, zero-Docker requirement, non-elevated user token fully intact.

---

## 9. Baseline Sign-Off (CP00)

- [x] Repository root, current branch (`migration/phase-21-focused-lightweight`), dan HEAD commit (`e854344`) diverifikasi
- [x] Zero background process aktif dari sprint sebelumnya
- [x] Working tree clean, uncommitted draft profiles tersimpan di `stash@{0}`
- [x] Seluruh ukuran direktori dan item count diukur secara empiris
- [x] Tracked binary dan image assets diinventarisasi
- [x] Verifikasi rilis 7-stage lolos 100% (13.4s)
- [x] Verifikasi security gate 17-control lolos 100% (5.4s)
- [x] 12 operasi engine diukur secara empiris

**CP00 Status:** **COMPLETE & VERIFIED**
