# ZITERA_LAB Build & Environment Guide

## 1. Prerequisites

### For Platform Developers:
- **Operating System**: Windows 10 (Build 19041+) or Windows 11 (x64)
- **Rust Toolchain**: `stable-x86_64-pc-windows-msvc` (Rust 1.75+)
- **Build Tools**: Visual Studio 2022 C++ Build Tools (MSVC v143+)
- **Flutter SDK**: Flutter 3.19+ (Channel stable, desktop enabled)

### For End Users:
- **Zero Developer Dependencies**: End users do NOT need Git, Python, Rust, Cargo, Node.js, Docker, WSL2, or Visual Studio Build Tools. The release distribution is a self-contained portable package.

---

## 2. Building the Rust Orchestration Engine

From repository root:

```cmd
:: 1. Initialize MSVC Environment
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"

:: 2. Set cargo incremental build flag
set CARGO_INCREMENTAL=0

:: 3. Run unit and integration tests (Release Mode)
cd engine\rust
cargo test --release --bin zitera-engine

:: 4. Build optimized release binary
cargo build --release --bin zitera-engine
```

The resulting binary will be located at:
`engine/rust/target/release/zitera-engine.exe`

---

## 3. Building the Flutter Desktop Application

From `ui/flutter/`:

```cmd
:: 1. Fetch dependencies
flutter pub get

:: 2. Analyze code quality
flutter analyze

:: 3. Build release Windows desktop executable
flutter build windows --release
```

The compiled application bundle will be created at:
`ui/flutter/build/windows/x64/runner/Release/`

---

## 4. Running Locally in Development Mode

To run the application with hot reload:

```cmd
cd ui/flutter
flutter run -d windows
```

The Flutter app will automatically detect and pair with `engine/rust/target/release/zitera-engine.exe` or `zitera-engine.exe` in the workspace.
