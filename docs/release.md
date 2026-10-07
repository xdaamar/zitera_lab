# ZITERA_LAB // Release Engineering & Operations Guide

This guide describes release management, packaging, signing, distribution, rollback procedures, diagnostics, and publishing operations for ZITERA_LAB.

---

## 1. Versioning Architecture

ZITERA_LAB decouples version dimensions to allow independent courseware updates:

| Dimension | Format | Scope | Example | Description |
| :--- | :--- | :--- | :--- | :--- |
| **Core Version** | SemVer (`X.Y.Z`) | Platform Engine & Runner | `2.0.0` | Tracks engine, broker, and protocol features. |
| **Lab Version** | SemVer (`X.Y.Z`) | Individual Lab Module | `1.0.1` | Tracks changes to lesson text, challenges, or lab code. |
| **Security Version**| Monotonic Integer | Package Verification | `1` | Anti-downgrade counter. Rejects lower versions. |
| **Curriculum Standard**| Canonical Tag | Educational Standard | `owasp-top10` | Governing cybersecurity curriculum standard. |
| **Curriculum Version** | Year Tag | Standard Edition | `2025` | Specific release year/edition of standard. |
| **Contract Version**| Integer Schema | Manifest Contract | `2` | Serialization and IPC schema definition version. |

---

## 2. How to Package Courseware (`.zlab`)

To build an offline, signed laboratory package:

```powershell
# Command syntax:
# zitera-engine.exe package build <source_directory> <output_file.zlab>

# Example: Packaging Lab A01
.\engine\rust\target\release\zitera-engine.exe package build labs/A01 dist/packages/A01.zlab
```

The package builder:
1. Validates entrypoint existence and manifest schema
2. Computes SHA256 digests across all package assets
3. Applies an asymmetric Ed25519 digital signature
4. Produces a tamper-resistant `.zlab` package bundle

---

## 3. How to Assemble a Release

To deterministically compile, sign all 10 laboratories, generate checksums, and assemble the standalone distribution archive:

```powershell
# Build Release Candidate 1:
.\scripts\release\build_rc1.ps1

# Build Hardened Release Candidate 2:
.\scripts\release\build_rc2.ps1
```

### Release Artifacts Produced in `dist/`:
- `ZITERA_LAB_RC2_windows_x64.zip`: Standalone, portable distribution archive
- `bin/zitera-engine.exe`: Core engine standalone binary
- `packages/*.zlab`: All 10 signed Ed25519 lab packages
- `installer/install_user.ps1`: Per-user Windows installer
- `catalog/catalog.json`: Signed curriculum catalog
- `checksums.sha256`: Cryptographic SHA-256 verification manifest
- `release_manifest.json`: Authoritative metadata and artifact inventory

---

## 4. How to Rollback

If an update fails or is interrupted:

### Automatic Engine Rollback
The package installer automatically rolls back to the previous version if manifest validation or digital signature verification fails during extraction:
- Active version pointer (`active_version.txt`) remains locked to the previous known good version.
- Staged temporary files are cleanly discarded.

### Manual Courseware Rollback
To manually switch a lab to a previous installed version:
```powershell
# Inspect available installed versions
Get-ChildItem -Path "labs/A01/versions"

# Re-point active version
Set-Content -Path "labs/A01/active_version.txt" -Value "1.0.0"
```

### Installer Rollback
Running the installer with the previous version automatically performs an in-place downgrade/revert while preserving all student progress in `%LOCALAPPDATA%\ZiteraLab\user\progress.json`.

---

## 5. How to Diagnose

### System Diagnostics Probe
Run the built-in diagnostic probe to inspect environment readiness:
```powershell
.\engine\rust\target\release\zitera-engine.exe doctor
```

### Privacy-Safe Diagnostic Export
To generate a scrubbed diagnostics bundle for technical support:
```powershell
.\engine\rust\target\release\zitera-engine.exe diagnostics export
```
The resulting `.zip` archive in `%LOCALAPPDATA%\ZiteraLab\diagnostics\`:
- Scrubs usernames, computer names, and home directory paths
- Redacts all secret tokens, flags, and sensitive environment variables
- Includes system configuration, non-sensitive logs, and engine status

---

## 6. How to Publish a Release

1. **Verify Git Tree:** Ensure working tree is clean (`git status`).
2. **Execute Full Release Gate:**
   ```powershell
   .\scripts\verify_release.ps1
   .\scripts\verify_security_gate.ps1
   ```
3. **Verify Checksums:**
   ```powershell
   Get-FileHash -Path dist\ZITERA_LAB_RC2_windows_x64.zip -Algorithm SHA256
   ```
4. **Tag Release:**
   ```powershell
   git tag -a v2.0.0-rc2 -m "ZITERA_LAB Release Candidate 2"
   git push origin v2.0.0-rc2
   ```
5. **Publish Artifacts to GitHub Releases:**
   - Upload `dist/ZITERA_LAB_RC2_windows_x64.zip`
   - Upload `dist/checksums.sha256`
   - Attach `reports/PHASE_20_RC2_REPORT.md` and release notes
