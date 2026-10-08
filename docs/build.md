# ZITERA_LAB // Build & Verification Guide

This guide details the complete developer build, compilation, test, and verification procedures for ZITERA_LAB.

---

## 1. Prerequisites & Toolchain

### Developer Workstation Requirements
- **Operating System:** Windows 10 (Build 19041+) or Windows 11 x64
- **Rust Toolchain:** `stable-x86_64-pc-windows-msvc` (Rust 1.75+)
- **C++ Build Tools:** Visual Studio 2022 Build Tools (MSVC v143+ `cl.exe`, `link.exe`)
- **Flutter SDK:** Flutter 3.19+ (Channel `stable`, desktop enabled)
- **PowerShell:** PowerShell 5.1+ or PowerShell 7+

> [!NOTE]
> **End-User Zero-Developer Guarantee:** Non-developer students and educational users do **NOT** require Git, Python, Rust, Cargo, Node.js, Docker, WSL2, or Visual Studio Build Tools. The distribution artifact is a completely self-contained Windows per-user runtime.

---

## 2. Setting Up the Build Environment

Launch a PowerShell prompt and initialize the MSVC x64 build environment:

```powershell
# Set deterministic release flag (required for WDAC & reproducible builds)
$env:CARGO_INCREMENTAL = "0"

# Optional: Initialize MSVC vcvars64 if cl.exe is not in PATH
$vcvars = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
if (Test-Path $vcvars) {
    cmd.exe /c "call `"$vcvars`" && set" | ForEach-Object {
        if ($_ -match '^(.*?)=(.*)$') {
            [System.Environment]::SetEnvironmentVariable($matches[1], $matches[2], "Process")
        }
    }
}
```

---

## 3. How to Build

### 3.1 Building the Native Core Engine
To compile the high-performance release binary of the Rust orchestration engine:

```powershell
Set-Location engine\rust
cargo build --release --bin zitera-engine
```
The output binary is produced at:
`engine\rust\target\release\zitera-engine.exe`

### 3.2 Building Laboratory Executables
Each of the 10 hands-on laboratories compiles to a standalone, sandboxed Windows binary:

```powershell
# Example: Building Lab A01 (Broken Access Control)
cargo build --release --bin a01_lab

# Deploy binary to the lab workspace directory
New-Item -ItemType Directory -Path ..\..\labs\A01\bin -Force | Out-Null
Copy-Item target\release\a01_lab.exe ..\..\labs\A01\bin\a01-lab.exe -Force
```

### 3.3 Building the Flutter Desktop UI
To build the native Windows Flutter presentation bundle:

```powershell
Set-Location ..\..\ui\flutter
flutter pub get
flutter analyze
flutter build windows --release
```
The output desktop executable bundle is placed in:
`ui\flutter\build\windows\x64\runner\Release\`

---

## 4. How to Test & Verify

### 4.1 Running Core Rust Tests
```powershell
Set-Location engine\rust
# Run full unit and integration test matrix (101 tests)
cargo test --release --bin zitera-engine
```

### 4.2 Running Full Automated Release Gate
To run the automated, 7-stage release qualification gate:
```powershell
Set-Location ..\..
.\scripts\verify_release.ps1
```
This executes:
1. Environment and toolchain readiness
2. Core engine release compilation
3. Rust security and isolation test matrix
4. Curriculum validation across all 10 labs (`A01` through `A10`)
5. Distribution installer lifecycle tests
6. Clean machine zero-dependency proof
7. Release qualification summary

### 4.3 Running Security Gate Verification
To run the automated, 17-control security release gate:
```powershell
Set-Location ..\..
.\scripts\verify_security_gate.ps1
```
Evaluates all 17 defense-in-depth controls (AppContainer isolation, Job Object limits, loopback proxy binding, anti-downgrade counter, path canonicalization, PII scrubbing) with zero warnings.

### 4.4 Running Performance Benchmarking
To benchmark startup, catalog load, probe latency, and save operations:
```powershell
.\scripts\benchmark_baseline.ps1
```
Ensures sub-second responsiveness across all core orchestration operations.

---

## 5. Building Release Candidates

### 5.1 Building Release Candidate 1 (RC1)
```powershell
.\scripts\release\build_release.ps1 -Version "2.0.0-rc1"
```
Compiles `zitera-engine.exe` with non-incremental determinism (`CARGO_INCREMENTAL=0`), executes release test suites, computes SHA256 checksums, and produces `dist/release_manifest.json`.

### 5.2 Building Hardened Release Candidate 2 (RC2)
```powershell
.\scripts\release\build_rc2.ps1 -Version "2.0.0-rc2"
```
Evaluates all 9 enterprise edge cases (path spaces, v2 schema persistence, file locking, corruption repair, clean uninstall), hashes outputs, and generates `dist/ZITERA_LAB_RC2_windows_x64.zip`.

---

## 6. Development Mode (Live Iteration)

To run the application with Flutter hot-reload during local development:
```powershell
Set-Location ui\flutter
flutter run -d windows
```
The Flutter frontend automatically connects to the compiled `zitera-engine.exe` in `engine\rust\target\release\` or in the workspace root.
