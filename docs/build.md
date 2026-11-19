# ZITERA_LAB // Build & Verification Guide

This guide details the complete developer build, compilation, test, and verification procedures for ZITERA_LAB (Modern Tauri v2 Desktop & Native Sandboxed Rust Architecture).

---

## 1. Prerequisites & Toolchain

### Developer Workstation Requirements
- **Operating System:** Windows 10 (Build 19041+) or Windows 11 x64
- **Rust Toolchain:** `stable-x86_64-pc-windows-msvc` (Rust 1.75+)
- **Node.js & npm:** Node.js 18+ & npm 9+ (for Tauri v2 frontend)
- **C++ Build Tools:** Visual Studio 2022 Build Tools (MSVC v143+ `cl.exe`, `link.exe`)
- **PowerShell:** PowerShell 5.1+ or PowerShell 7+

> [!NOTE]
> **End-User Zero-Developer Guarantee:** Non-developer students and educational users do **NOT** require Git, Python, Rust, Cargo, Node.js, Docker, WSL2, or Visual Studio Build Tools. The distribution artifact in `ui/tauri/run/` is completely self-contained and ready to execute immediately.

---

## 2. Setting Up the Build Environment

Launch a PowerShell prompt and initialize the MSVC x64 build environment:

```powershell
# Set deterministic release flag (required for reproducible builds)
$env:CARGO_INCREMENTAL = "0"
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
`engine\rust\target\release\zitera-engine.exe` (and copied to `ui\tauri\run\zitera-engine.exe`).

### 3.2 Building Laboratory Executables (A01 - A10)
Each of the 10 hands-on laboratories compiles to a standalone, sandboxed native Windows binary:

```powershell
Set-Location engine\rust
# Example: Building Lab A01 through A10
cargo build --release --bin a01_lab
cargo build --release --bin a02_lab
cargo build --release --bin a03_lab
cargo build --release --bin a04_lab
cargo build --release --bin a05_lab
cargo build --release --bin a06_lab
cargo build --release --bin a07_lab
cargo build --release --bin a08_lab
cargo build --release --bin a09_lab
cargo build --release --bin a10_lab

# Deploy binary to respective lab directories
Copy-Item target\release\a01_lab.exe ..\..\labs\A01\bin\a01-lab.exe -Force
# (Repeated for A02-A10)
```

### 3.3 Building the Tauri v2 Desktop Application
To compile the Tauri v2 desktop user interface (React 19 + TypeScript + Vite + Tailwind CSS):

```powershell
Set-Location ui\tauri
npm install
npm run build
npx tauri build --no-bundle
```
The production standalone executable will be located at:
`ui\tauri\src-tauri\target\release\app.exe` (or `ui\tauri\run\app.exe`).

---

## 4. How to Test & Verify

### 4.1 Running All 10 Lab Integration & Exploitation Tests
```powershell
Set-Location engine\rust
cargo test --release test_a
```
All 10 labs (`test_a01` through `test_a10`) run automated lifecycle spawning, reverse proxy routing, vulnerability parameter tampering, flag extraction, and scoring validation.

### 4.2 Running Full Core Rust Engine Tests
```powershell
Set-Location engine\rust
cargo test --release --bin zitera-engine
```

### 4.3 Running the Application Directly
Double-click `ui\tauri\run\app.exe` or execute:
```powershell
Set-Location ui\tauri\run
.\app.exe
```
This runs the full standalone bootcamp environment:
- Animated anime sidebar & ambient cyberpunk laboratory themes
- 10 interactive SaaS labs with commercial aesthetics
- Integrated Tilix command-line terminal emulator
- YouTube Video Bootcamp theater drawer with custom URL settings
- HackerOne / Bugcrowd bug bounty blueprints and automated 4-pillar triage grading engine
- Verifiable Z-CWPT cryptographic certification generator
