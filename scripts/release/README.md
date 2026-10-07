# ZITERA_LAB Release Pipeline

This directory contains scripts and tooling for generating deterministic, production-ready release distributions of ZITERA_LAB.

## Pipeline Architecture

```text
SOURCE SHA -> BUILD -> TEST -> PACKAGE -> HASH -> SIGN -> MANIFEST -> DIST
```

1. **SOURCE SHA:** Extracts immutable Git commit hash and build environment metadata.
2. **BUILD:** Invokes MSVC Developer Command Prompt with `CARGO_INCREMENTAL=0` to enforce WDAC-compliant deterministic release binaries.
3. **TEST:** Validates release tests across core, package, security, and lab lifecycles.
4. **PACKAGE:** Packages the standalone binary and complete zero-install portable zip bundle.
5. **HASH:** Generates standard SHA-256 checksums (`dist/checksums.sha256`).
6. **SIGN:** Prepares Authenticode code signing hooks and verifies Ed25519 lab package signatures.
7. **MANIFEST:** Generates canonical release manifest (`dist/release_manifest.json`) containing commit SHA, versions, timestamps, and artifact hashes.
8. **DIST:** Emits all artifacts into the `dist/` directory.

## Usage

```powershell
# Run full release pipeline with verification
.\scripts\release\build_release.ps1 -Version "2.0.0"

# Quick packaging (skipping test suite)
.\scripts\release\build_release.ps1 -Version "2.0.0" -SkipTests
```

## Security Guarantees

- **No Private Keys in Repository:** The release pipeline consumes external signing keys via environment variables or hardware tokens; zero private keys are embedded in source control.
- **Deterministic Content Digest:** Manifest timestamps are isolated to the release manifest envelope and never contaminate internal deterministic package content digests.
