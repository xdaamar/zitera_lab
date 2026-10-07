# ZITERA_LAB Release & Versioning Policy

## 1. Versioning Architecture

To enable autonomous laboratory updates without destabilizing the core platform, ZITERA_LAB separates version dimensions:

| Dimension | Format | Scope | Example | Description |
| :--- | :--- | :--- | :--- | :--- |
| **Core Version** | SemVer (`X.Y.Z`) | Platform Engine & Runner | `2.0.0` | Tracks engine, broker, and UI protocol features. |
| **Lab Version** | SemVer (`X.Y.Z`) | Individual Lab Module | `1.2.0` | Tracks changes to lesson text, challenges, or lab code. |
| **Security Version** | Monotonic Integer | Package Verification | `3` | Anti-downgrade counter. Rejects packages with lower security versions. |
| **Curriculum Standard** | Canonical Tag | Educational Framework | `owasp-top10` | The governing cybersecurity curriculum framework. |
| **Curriculum Version** | Year / Tag | Standard Edition | `2025` | The specific release year or edition of the standard. |
| **Contract Version** | Integer Schema | Manifest Contract | `2` | Serialization and IPC schema definition version. |

---

## 2. Platform Compatibility Matrix

The engine enforces runtime compatibility between the installed Core release and laboratory packages:

| Core Engine | Lab Minimum Version | Contract Version | Support Status | Notes |
| :--- | :--- | :--- | :--- | :--- |
| **Core 2.0.x** | `>= 2.0.0` | `Schema v2` | **Fully Supported** | Primary Phase 19 baseline. |
| **Core 2.0.x** | `>= 1.0.0` | `Schema v1` | **Legacy Compatible** | Automatically adapts to v2 contract. |
| **Core 1.x** | Any | `Schema v2` | **Incompatible** | Engine rejects launch (Core Too Old). |

If an installed laboratory requires a higher core version than currently running, the engine displays an actionable prompt:
*"This lab requires ZITERA_LAB Core 2.1.0 or newer. Please update the application from the Update Center."*

---

## 3. Release Quality Gate Checklist

Before publishing any release candidate:

1. **Rust Engine Verification**:
   ```cmd
   cargo fmt -- --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --release --bin zitera-engine
   cargo build --release --bin zitera-engine
   ```
2. **Laboratory Validation**:
   ```cmd
   zitera lab validate A01
   zitera lab validate A02
   ...
   zitera lab validate A10
   ```
3. **UI Code Integrity**:
   ```cmd
   cd ui/flutter
   flutter analyze
   flutter test
   flutter build windows --release
   ```
4. **Clean Machine Zero-Dependency Proof**:
   Verify the binary runs on a clean standard Windows workstation without Visual Studio, Git, or Python.
