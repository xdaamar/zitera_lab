# PHASE 22 RUNTIME DEPENDENCY MATRIX

**Program:** ZITERA 2.0  
**Phase:** 22 (First-Run Readiness, Branding & Flutter + Rust Bundling)  
**Checkpoint:** CP01 (Push #2)  
**Platform:** Windows x64  
**Bundle Target:** `<REPOSITORY_ROOT>/dist/ZITERA_LAB_WINDOWS_X64/`  
**Distribution Archive:** `<REPOSITORY_ROOT>/dist/ZITERA_LAB_WINDOWS_X64.zip`  

---

## 1. Executive Summary

This document defines the complete, authoritative Windows runtime dependency matrix for ZITERA_LAB. It inventories the actual binary, dynamic link library, resource, cryptographic, and filesystem dependencies required for the portable release distribution.

In strict compliance with ZITERA 2.0 architecture and the Phase 22 mandate:
- **No compiler toolchains (`rustc`, `cargo`, `flutter`, `dart`) are required or evaluated** on the learner's machine.
- **No container or VM virtualization platforms (`docker`, `dockerd`, `wsl`, `hyper-v`) are required**.
- **No administrative elevation is required**; the application executes under a standard Windows user token with native AppContainer DACL sandboxing and Job Object containment.

---

## 2. Dynamic Library (DLL) Dependency Matrix

Every executable in the distribution was forensically analyzed for dynamic PE imports via PE header inspection.

| Binary | Origin / Role | Required Dynamic DLLs | DLL Classification | Bundle Action |
|---|---|---|---|---|
| **`zitera_lab.exe`** | Flutter Windows Runner Executable | `flutter_windows.dll`<br>`vcruntime140.dll`<br>`vcruntime140_1.dll`<br>`msvcp140.dll`<br>`kernel32.dll`, `user32.dll`, `shell32.dll`, `ole32.dll`, `advapi32.dll`, `dwmapi.dll` | Flutter Engine<br>MSVC CRT<br>MSVC CRT<br>MSVC C++ STL<br>Windows Core System | Place next to executable<br>Bundle in root folder<br>Bundle in root folder<br>Bundle in root folder<br>Provided by Windows OS |
| **`flutter_windows.dll`** | Flutter Embedder Engine (x64) | `kernel32.dll`, `user32.dll`, `gdi32.dll`<br>`ole32.dll`, `oleaut32.dll`, `shlwapi.dll`<br>`ws2_32.dll`, `imm32.dll`, `bcrypt.dll`<br>`crypt32.dll`, `propsys.dll`, `rpcrt4.dll`<br>`iphlpapi.dll`, `dbghelp.dll`, `dwmapi.dll` | Windows Core System & Win32 Media / Crypto APIs | Bundled with release runner in root folder |
| **`zitera-engine.exe`** | Compiled Core Rust Engine & Broker | `vcruntime140.dll`<br>`kernel32.dll`, `ws2_32.dll`<br>`userenv.dll`, `advapi32.dll`<br>`ole32.dll`, `bcrypt.dll`<br>`ntdll.dll`, `dbghelp.dll` | MSVC CRT<br>Win32 Core & Sockets<br>User Profile & AppContainer API<br>Security & Crypto API<br>NT Core Subsystem | Bundle in root/engine folder<br>Provided by Windows OS<br>Provided by Windows OS (Win 10/11)<br>Provided by Windows OS<br>Provided by Windows OS |
| **`a01-lab.exe` ... `a10-lab.exe`** | Sandboxed Curriculum Lab Binaries | `vcruntime140.dll`<br>`kernel32.dll`, `ntdll.dll`<br>`dbghelp.dll` | MSVC CRT<br>Win32 Core & NT API<br>Debug API | Inherits CRT from bundle directory<br>Provided by Windows OS<br>Provided by Windows OS |

---

## 3. Microsoft Visual C++ Runtime (MSVC CRT) Strategy

### Analysis
Both `zitera_lab.exe` and `zitera-engine.exe` link against the MSVC x64 C/C++ Universal CRT:
- `vcruntime140.dll` (178,616 bytes)
- `vcruntime140_1.dll` (50,112 bytes)
- `msvcp140.dll` (643,512 bytes)

### Deployment Method: Application-Local Deployment (Recommended for Zero-Elevation Portable Distribution)
Under Windows DLL Search Order rules:
1. The directory containing the executing application (`dist/ZITERA_LAB_WINDOWS_X64/`) is searched **before** system folders (`C:\Windows\System32`).
2. Bundling `vcruntime140.dll`, `vcruntime140_1.dll`, and `msvcp140.dll` directly in the root distribution folder guarantees that ZITERA_LAB launches successfully on clean Windows 10/11 workstations without requiring administrative elevation, UAC prompts, or installer execution.
3. This eliminates dependency failures on machines where the Microsoft Visual C++ 2015–2022 Redistributable has not been system-installed.

---

## 4. Flutter Runtime & Asset Inventory

The compiled Flutter release package requires the following filesystem components in `data/`:

| Path | Purpose | File Type | Criticality |
|---|---|---|:---:|
| `data/icudtl.dat` | International Components for Unicode (ICU) data table | Binary data (~10.3 MB) | **Mandatory** |
| `data/app.so` | Flutter Dart AOT compiled application code | ELF/PE shared library | **Mandatory** |
| `data/flutter_assets/` | Asset bundle root directory | Directory | **Mandatory** |
| `data/flutter_assets/AssetManifest.bin` | Binary asset inventory index | Indexed binary | **Mandatory** |
| `data/flutter_assets/FontManifest.json` | Font definitions (SpaceGrotesk, JetBrainsMono) | JSON manifest | **Mandatory** |
| `data/flutter_assets/NOTICES.Z` | Open-source license attributions | Compressed archive | **Mandatory** |
| `data/flutter_assets/fonts/` | Packaged typefaces (`SpaceGrotesk`, `JetBrainsMono`) | TTF font files | **Mandatory** |
| `data/flutter_assets/assets/images/`| Packaged vector/raster images and badges | PNG/SVG assets | **Mandatory** |

---

## 5. ZITERA Engine & Platform Component Inventory

| Component | Path in Bundle | Origin / Source | Verification Contract |
|---|---|---|---|
| **Compiled Engine** | `engine/zitera-engine.exe` (or root `zitera-engine.exe`) | `engine/rust/target/release/zitera-engine.exe` | Executable exists, responds to `--json doctor`, `lab validate`, `catalog` |
| **Signed Catalog** | `catalog/catalog.json` | `catalog/catalog.json` | Ed25519 signature valid, checksum matches |
| **Catalog Signature** | `catalog/catalog.signature` | Generated canonical signature | Signed with trusted authority key |
| **Lab Packages (A01–A10)** | `packages/A01.zlab` ... `A10.zlab` | `dist/packages/*.zlab` | Ed25519 package signature, CRC32 archive integrity |
| **Branding Master** | `branding/logo.png` | `ui/flutter/assets/images/logo.png` | High-res PNG (2.17 MB), transparent alpha |
| **App Launcher Icon** | Embedded PE resource | `ui/flutter/windows/runner/resources/app_icon.ico` | Multi-resolution ICO (16, 32, 48, 64, 128, 256) |

---

## 6. IPC Contract & Argument Matrix

Communication between Flutter UI and compiled Rust engine occurs via stdio JSON IPC and subprocess execution:

| Operation | Command Invocation | Expected Output | Error Handling |
|---|---|---|---|
| **Health Check** | `zitera-engine.exe --json doctor` | JSON: `{"success": true, "data": {"all_ready": true, ...}}` | Parse error or missing binary triggers First-Run repair |
| **Catalog Query** | `zitera-engine.exe --json catalog` | JSON: `{"success": true, "data": {"labs": [...]}}` | Offline cache fallback if engine response fails |
| **Lab Status** | `zitera-engine.exe --json lab status <LAB_ID>` | JSON: `{"installed": bool, "running": bool, "port": int}` | Status polling bounded to timeout |
| **Lab Start** | `zitera-engine.exe --json lab start <LAB_ID>` | JSON: `{"success": true, "data": "Lab ... started at http://..."}` | Background broker spawned, bounded poll up to 10s |
| **Lab Stop** | `zitera-engine.exe --json lab stop <LAB_ID>` | JSON: `{"success": true, "data": "Lab ... stopped"}` | Process tree terminated via Job Object kill-on-close |
| **Lab Install** | `zitera-engine.exe --json lab install <LAB_ID>` | JSON: `{"success": true, "data": "Lab ... installed"}` | Extracts `.zlab`, verifies Ed25519, applies DACL |
| **Diagnostics Export** | `zitera-engine.exe --json diagnostics export` | JSON: `{"success": true, "data": {"path": "..."}}` | Secrets sanitized, writes to `%LOCALAPPDATA%\ZiteraLab\diagnostics` |

---

## 7. Learner Environment Hard Boundary Checklist

- [x] **No Rust Toolchain:** Zero checks for `rustc`, `cargo`, `rustup`, or LLVM.
- [x] **No Developer Workspace Assumption:** All executable paths resolved via relative installation directory (`Platform.resolvedExecutable`).
- [x] **No Docker / Virtualization:** Zero container or WSL dependencies.
- [x] **No Elevation Requirement:** Standard user privileges only; AppContainer profiles created per-user via `CreateAppContainerProfile`.
- [x] **Offline-First:** All 10 canonical packages and catalog included locally in distribution.
- [x] **Memory & Buffer Boundaries:** All log displays bounded to in-memory buffers; terminal emulation limits line count and payload size.
