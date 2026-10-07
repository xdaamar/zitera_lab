# ZITERA_LAB Phase 18 Cryptographic Trust Model
**Document ID:** ZITERA-P18-TRUST-001
**Program:** ZITERA 2.0 Core Migration
**Phase:** 18 — Trust Chain Hardening, Full Curriculum Migration & Distribution Integrity
**Author:** Core Security Team
**Status:** Approved & Implemented
**Date:** October 2026

---

## 1. Executive Summary

In Phase 17, initial package verification relied upon HMAC-SHA256 with managed development keys to validate the package and atomic rollback pipelines. While HMAC-SHA256 was sufficient for local development sandbox proofing, symmetric authentication is fundamentally unsuitable for public client distribution because symmetric verification requires the verifier to possess the same secret used to sign. Distributing a shared secret within client binaries allows malicious parties to extract the secret and forge arbitrary packages.

Phase 18 establishes a production-grade, asymmetric trust model based on **Ed25519 (RFC 8032)**. This architecture strictly decouples package signing from package verification:

```
DEVELOPER / CI RELEASE PIPELINE
      ↓
[Private Signing Key (32-byte secret)] — NEVER distributed, NEVER committed
      ↓
Canonical Payload Serialization
      ↓
Sign Payload (Ed25519 64-byte signature)
      ↓
Distribute .zlab Package / catalog.json

════════════════════════════════════════════════════════════════════════════════
AIR-GAP BOUNDARY (Public Distribution)
════════════════════════════════════════════════════════════════════════════════

ZITERA 2.0 CLIENT ENGINE
      ↓
Receive Package (.zlab) / Catalog (catalog.json)
      ↓
Construct Deterministic Canonical Payload
      ↓
[Embedded Trusted Public Key (32-byte public)]
      ↓
Ed25519 Cryptographic Verification
      ↓
[PASS] → Atomic Install / Activation
[FAIL] → Immediate Hard Rejection & Rollback
```

---

## 2. Cryptographic Primitives & Justification

### 2.1 Algorithm Selection: Ed25519 (RFC 8032)
ZITERA 2.0 adopts **Ed25519**, the Edwards-curve Digital Signature Algorithm using SHA-512 and Curve25519:
- **Security Level:** ~128-bit cryptographic security level (comparable to RSA-3072 and NIST P-256), with vastly superior resistance to side-channel and implementation timing attacks.
- **Key Sizes:** Public keys are 32 bytes (64 hex characters); private seed keys are 32 bytes.
- **Signature Size:** Fixed 64 bytes (128 hex characters), deterministic with zero requirement for an external entropy source during signature generation.
- **Performance:** Sub-millisecond verification time on modern 64-bit systems with minimal CPU and memory overhead, guaranteeing instantaneous package validation during user interactions.
- **Implementation:** Backed by the audited, pure-Rust, zero-dependency `ed25519-dalek` crate, fully avoiding platform-specific C runtime bindings, unsafe assembly blocks, or OpenSSL dependencies.

### 2.2 Rejection of Weak Alternatives
- **RSA-2048 / RSA-4096:** Rejected due to excessive key and signature sizes, vulnerability to padding oracle attacks (e.g. PKCS#1 v1.5), and slow key generation/verification performance.
- **ECDSA (secp256r1 / P-256):** Rejected due to complex curve constants, reliance on high-quality random number generation during signing (fragile to biased nonces), and implementation vulnerabilities.
- **Custom Cryptography:** Prohibited by Rule 1. No homebrewed cryptographic schemes are permitted in ZITERA.

---

## 3. Signing and Verification Identities

### 3.1 Roles and Separation of Duties

| Identity | Role | Access Level | Stored Location |
| :--- | :--- | :--- | :--- |
| **Release Authority (CI / Release Lead)** | Generates official release signatures for packages and catalogs. | Possesses Private Signing Key (`ed25519_sk`). | Secure HSM / Encrypted CI Secret Store. Air-gapped from source code. |
| **Development Authority** | Signs local development packages during testing and automated CI test runs. | Possesses Dev Private Signing Key. | Ephemeral developer environment / test harness fixtures. |
| **Client Verifier (ZITERA Engine)** | Verifies incoming `.zlab` packages and `catalog.json` before any filesystem extraction or execution. | Possesses ONLY Trusted Public Keys (`ed25519_pk`). | Hardcoded within Rust Engine binary in `TRUSTED_PUBLIC_KEYS`. |
| **Untrusted Actor / Network Attacker** | May attempt package tampering, replay, downgrade, or unauthorized distribution. | Possesses zero keys. | Untrusted network, mirrors, or local filesystem. |

### 3.2 Key Ownership & Leakage Prevention Rules
The Private Signing Key must **NEVER** exist in:
1. ZITERA client applications (Flutter UI, Rust engine, terminal).
2. Public Git repositories or commit history.
3. Flutter bundled assets or distributed files.
4. `.zlab` package archives or `catalog.json` metadata.
5. Runtime application logs, telemetry, or debug crash dumps.
6. Developer workstation project files outside secure personal keyrings.

---

## 4. Deterministic Canonical Payload

Signing arbitrary JSON strings or compressed zip archives directly creates vulnerability to non-deterministic serialization, whitespace changes, key ordering drift, and zip file format malleability.

To eliminate these attack surfaces, ZITERA 2.0 defines an explicit **Canonical Payload Representation**.

### 4.1 Package Canonical Payload Specification
The canonical package payload is a strict, newline-delimited (`\n`), ASCII-encoded key-value block with lexicographically sorted keys:

```
architecture:<arch>
content_digest:<sha256_of_archive_payload_excluding_manifest>
entrypoint:<normalized_relative_entrypoint>
id:<LAB_ID_UPPERCASE>
minimum_core_version:<semver_string>
runtime:<declared_runtime>
security_version:<integer>
version:<semver_string>
```

#### Field Definitions and Normalization:
1. `architecture`: Normalized target architecture string (e.g. `x86_64`). Lowercase, trimmed.
2. `content_digest`: 64-character lowercase hexadecimal SHA-256 digest of the uncompressed, sorted archive contents (excluding `manifest.json`).
3. `entrypoint`: Relative forward-slash path to binary (e.g. `bin/a01-lab.exe`). Backslashes strictly normalized to `/`.
4. `id`: Lab identifier in uppercase ASCII (e.g. `A01`, `A06`).
5. `minimum_core_version`: Minimum supported engine version (e.g. `2.0.0`).
6. `runtime`: Execution runtime identifier (e.g. `native_sandboxed`).
7. `security_version`: Monotonically increasing unsigned 32-bit integer for anti-downgrade evaluation.
8. `version`: Semantic version string of the lab package (e.g. `1.0.1`).

### 4.2 Invariant Guarantees
- **No Whitespace Drift:** Extra spaces, tabs, carriage returns (`\r`), or JSON formatting changes have zero effect on the canonical representation.
- **No Key Ordering Discrepancies:** Keys are deterministically sorted by ASCII byte value.
- **Zero Ambiguous Types:** Numbers are represented as base-10 ASCII digits without leading zeros or floating-point representations.

---

## 5. Signature Encoding and Structure

- **Algorithm Identifier:** `ed25519`
- **Encoding:** Standard lowercase hexadecimal string (128 hex characters representing 64 raw bytes).
- **Public Key Encoding:** Standard lowercase hexadecimal string (64 hex characters representing 32 raw bytes).
- **Manifest Field:**
  ```json
  "signature": {
    "key_id": "zitera-release-2026-v1",
    "algorithm": "ed25519",
    "sig": "3a48e7...128 hex chars...c901"
  }
  ```

---

## 6. Zero-Tolerance Failure Policy

If **ANY** step in the verification pipeline fails, the package is immediately rejected with a hard error and deleted from any staging directory:

1. **Missing Signature:** Packages without valid cryptographic signatures are rejected unconditionally. No hash-only fallback.
2. **Corrupted/Invalid Format:** Signatures that are not exactly 64 bytes (128 hex chars) or public keys not exactly 32 bytes (64 hex chars) fail with format violation errors.
3. **Cryptographic Mismatch:** If `ed25519_verify(public_key, canonical_payload, signature)` returns false, the package is rejected as tampered.
4. **Content Digest Tampering:** If any file inside the `.zlab` package is added, removed, or modified, `content_digest` recalculation fails to match the signed metadata, triggering rejection.
5. **Untrusted Key ID:** If `key_id` is unknown or revoked, the package is rejected.
6. **Security Version Downgrade:** If `incoming.security_version < installed.security_version`, update is rejected to prevent rollback attacks.
7. **Clean Rollback Guarantee:** Any verification failure during an update triggers immediate atomic purge of staging artifacts. The previously installed, active version remains completely untouched and healthy.

---

## 7. Key Provisioning and Rotation Architecture

### 7.1 Key Ring Schema
ZITERA 2.0 implements a future-compatible, lightweight `KeyRing` structure:

```rust
pub struct TrustedKey {
    pub key_id: &'static str,
    pub public_key_hex: &'static str,
    pub status: KeyStatus,
}

pub enum KeyStatus {
    Active,
    Deprecated,
    Revoked,
}
```

### 7.2 Phase 18 Active Keys

1. **Production Release Key (`zitera-release-2026-v1`):**
   - **Key ID:** `zitera-release-2026-v1`
   - **Role:** Production releases, public curriculum packages, signed official catalogs.
   - **Status:** Active.
   - **Public Key (Hex):** Embedded in `engine/rust/src/package/keys.rs`.

2. **Development / Test Key (`zitera-dev-key-v1`):**
   - **Key ID:** `zitera-dev-key-v1`
   - **Role:** CI test automation, local lab builder test suites, integration fixtures.
   - **Status:** Active (for development & test builds).

### 7.3 Rotation Lifecycle Procedure
When a key reaches the end of its operational lifecycle or requires replacement:
1. **Introduction:** A new key (`zitera-release-2027-v1`) is added to `TRUSTED_KEYS` as `Active` in an engine update.
2. **Transition:** The preceding key (`zitera-release-2026-v1`) is marked `Deprecated` (valid for existing installed packages, rejected for new incoming packages).
3. **Revocation:** If a key is compromised, its status is updated to `Revoked` in the core engine, causing immediate hard rejection of any package signed by it.
4. **Simplicity:** The key ring is intentionally minimal (~2-3 keys maximum) without introducing complex PKI, X.509 certificates, or online OCSP network requests, ensuring 100% offline autonomy.

---

## 8. Summary Table of Checkpoint 1 Verification Gate

| Security Requirement | Phase 17 Legacy State | Phase 18 Hardened State | Status |
| :--- | :--- | :--- | :--- |
| **Authentication Scheme** | Symmetric HMAC-SHA256 | Asymmetric Ed25519 (RFC 8032) | **SECURED** |
| **Private Key Exposure** | Shared dev key in engine | Private key strictly separated | **SECURED** |
| **Signed Payload** | Ambiguous string (`manifest.id`) | Deterministic Canonical Payload | **SECURED** |
| **Downgrade Defense** | None (version string only) | Explicit monotonic `security_version` | **SECURED** |
| **Catalog Authenticity** | Unsigned JSON | Signed canonical catalog with Ed25519 | **SECURED** |
| **Offline Independence** | Fully offline | Fully offline with embedded public keys | **VERIFIED** |
