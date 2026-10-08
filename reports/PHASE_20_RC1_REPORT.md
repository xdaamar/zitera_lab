# PHASE 20 // RELEASE CANDIDATE 1 (RC1) SPECIFICATION & DISTRIBUTION REPORT

- **Document ID:** `PHASE_20_RC1_REPORT.md`
- **Product:** ZITERA_LAB (Enterprise Cyber Range Runtime)
- **Candidate Version:** `2.0.0-rc1`
- **Sprint Phase:** Phase 20 (Checkpoint 17 / Push #18)
- **Build Timestamp (UTC):** 2026-10-08T15:40:32Z
- **Commit SHA:** `b9e01d112a80cc90b0999d0b964bb408f4b2abac`
- **Target Platform:** Windows x64 Native (`x86_64-pc-windows-msvc` / Windows 10 & 11)
- **Host System Tested:** `Microsoft Windows NT 10.0.26200.0`
- **Distribution Model:** Zero-Install Standalone & Portable Distribution Archive

---

## 1. Executive Summary

ZITERA_LAB Release Candidate 1 (`v2.0.0-rc1`) represents the first complete, production-ready candidate distribution of the ZITERA 2.0 native Windows architecture. All legacy dependencies (Docker, WSL2, Python virtual environments, Rust toolchain, Cargo, npm, administrative UAC prompts) have been eradicated from the runtime and delivery path.

RC1 was deterministically assembled and validated using `scripts/release/build_release.ps1` with non-incremental compilation (`CARGO_INCREMENTAL=0`) and strict MSVC link flags compliant with Windows Defender Application Control (WDAC). All release gate stages and unit test suites passed with zero failures.

---

## 2. Release Artifact Inventory & Cryptographic Checksums

The release pipeline produced the following distribution artifacts located in the `dist/` directory:

| Artifact Name | Artifact Type | File Size | SHA256 Cryptographic Checksum | Description |
| :--- | :--- | :--- | :--- | :--- |
| `zitera-engine.exe` | Native PE Binary | 1,925,120 B (~1.84 MB) | `BB98FB7B11AA007E6A1237F1227115E7C037BCA842038B0104212047772702A1` | Standalone zero-elevation Windows x64 CLI & Broker engine |
| `zitera-lab-2.0.0-rc1-windows-x64-portable.zip` | Portable Distribution | 724,559 B (~707 KB) | `FC3BFBFF2ED08D9DB30441C39A538C2680A21BFA4B46C6FBB908B5CDBCDEAC1C` | Self-contained student bundle with engine, catalog & guide |
| `checksums.sha256` | Checksum Manifest | 224 B | `9A8C5E94848356EAE48FDF670DFB52701B4219A44CA559D65E193EB7D6EF98B1` | Standard SHA256 checksums file for SHA256 utility verification |
| `release_manifest.json` | JSON Manifest | 1,489 B | `32FA5EE43542E5CA3D67BCED3623D72ED4AE36D611CE7B4A091A80860A74CE5B` | Machine-readable deterministic release envelope |

### 2.1 Release Manifest (`release_manifest.json`)
```json
{
  "schema_version": 1,
  "product": "ZITERA_LAB",
  "version": "2.0.0-rc1",
  "commit_sha": "b9e01d112a80cc90b0999d0b964bb408f4b2abac",
  "build_timestamp": "2026-10-08T15:40:32Z",
  "platform": "windows",
  "architecture": "x86_64",
  "target_triple": "x86_64-pc-windows-msvc",
  "artifacts": [
    {
      "artifact_name": "zitera-engine.exe",
      "artifact_type": "binary",
      "artifact_size": 1925120,
      "sha256": "BB98FB7B11AA007E6A1237F1227115E7C037BCA842038B0104212047772702A1",
      "description": "ZITERA Core Engine Standalone Windows x64 Executable"
    },
    {
      "artifact_name": "zitera-lab-2.0.0-rc1-windows-x64-portable.zip",
      "artifact_type": "portable_archive",
      "artifact_size": 724559,
      "sha256": "FC3BFBFF2ED08D9DB30441C39A538C2680A21BFA4B46C6FBB908B5CDBCDEAC1C",
      "description": "ZITERA Complete Portable Zero-Install Distribution Archive"
    }
  ],
  "verification": {
    "verified_by": "verify_release.ps1",
    "all_tests_passed": true,
    "contract_version": "V3"
  },
  "deterministic_guarantee": {
    "content_digest_independent_of_timestamp": true,
    "zero_developer_dependency": true,
    "zero_elevation_required": true
  }
}
```

---

## 3. Pre-Release Test Verification Results

During the RC1 build pipeline execution, the comprehensive release test matrix was evaluated under `release` optimization with non-incremental determinism (`CARGO_INCREMENTAL=0`):

| Test Suite / Category | Tests Run | Passed | Failed | Status |
| :--- | :--- | :--- | :--- | :--- |
| Package Archive Format & CRC32 Validation | 3 | 3 | 0 | PASS |
| Cryptographic Hash & HMAC Known Vectors | 2 | 2 | 0 | PASS |
| Canonical Payload Determinism & Ed25519 Trust Matrix | 2 | 2 | 0 | PASS |
| Core Version Compatibility & Anti-Downgrade Rejection | 2 | 2 | 0 | PASS |
| Package Verifier & Tamper Detection Suite | 2 | 2 | 0 | PASS |
| Clean Room Installation Proof & Storage State Machine | 2 | 2 | 0 | PASS |
| Crash Recovery, Incomplete Transaction Reconciliation | 1 | 1 | 0 | PASS |
| Atomic Package Update & Instant Rollback Matrix | 1 | 1 | 0 | PASS |
| Full Release Lifecycle & Adversarial Package Defense | 2 | 2 | 0 | PASS |
| **Total Release Suite** | **17** | **17** | **0** | **100% PASS** |

Pipeline execution time: **10.49 seconds**.

---

## 4. Field Readiness Evaluation Criteria

RC1 was evaluated against the strict non-developer classroom readiness requirements:

1. **Zero Prerequisites:**
   - Package requires no pre-installed Git, Docker Desktop, WSL2, Visual Studio, Python, Rust, Cargo, or Node.js.
   - Operates on standard non-admin Windows accounts (`Standard User`).
2. **Zero Network Requirement at Runtime:**
   - All cryptographic keys, manifests, catalog entries, lessons, challenges, and hints operate completely offline.
   - Offline verification uses embedded Ed25519 public keys.
3. **Data Preservation & Auto-Migration:**
   - User progress from legacy versions (`zitera_progress.json`) is seamlessly migrated on first run to `progress.json` without data loss or user intervention.
4. **Resilient Failure Recovery UX:**
   - Any runtime failure displays structured diagnostic guidance answering "What happened?", "Why?", and "What can I do?".

---

## 5. Conclusion & Release Gate Recommendation

ZITERA_LAB `v2.0.0-rc1` has successfully passed all compilation, integrity, testing, packaging, and security gates. It is designated as **APPROVED FOR FIELD TESTING & HARDENING (RC2)**.