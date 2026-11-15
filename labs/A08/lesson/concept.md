# Core Concepts: Software and Data Integrity Failures

Integrity is a foundational pillar of the CIA triad (Confidentiality, Integrity, Availability). In software engineering, integrity guarantees that source code, executable binaries, third-party libraries, dynamic plugins, and operational configurations remain **authentic, uncorrupted, and verifiably unmodified** since their creation by authorized signers.

When integrity verification mechanisms are absent, misconfigured, or bypassed, adversaries can inject malicious logic into runtime environments without needing to crack credentials or exploit low-level memory corruption bugs.

---

## 1. Cryptographic Primitives: Encryption vs Hashes vs Digital Signatures

A widespread engineering misconception is that encrypting communications over HTTPS/TLS inherently protects data integrity end-to-end. In reality, each cryptographic primitive serves distinct operational functions:

| Cryptographic Primitive | Primary Objective | What Is Guaranteed? | What Is NOT Guaranteed? |
| :--- | :--- | :--- | :--- |
| **Symmetric Encryption / TLS** | Confidentiality in transit | Data cannot be eavesdropped on wire by intermediary observers. | Does not guarantee that client-submitted data is trustworthy or non-malicious. |
| **Cryptographic Hash (SHA-256)** | Accidental Tamper Detection | Generates a fixed-size deterministic digest. | Does not prove *who* generated the hash (an adversary can recompute valid hashes for malicious payloads). |
| **Digital Signatures (Ed25519/RSA)** | Authenticity, Integrity, Non-Repudiation | Cryptographically proves data was signed by private key holder and unaltered. | Private signing keys must be securely guarded outside the application perimeter. |

```
Plaintext Data ───> [ SHA-256 Hash ] ───> [ Encrypt with Private Key ] ───> Digital Signature
                                                                                  │
                                                                                  ▼
Receiver: Verify Signature with Public Key ─────────────────────> [ VALID / REJECT ]
```

---

## 2. Taxonomy of Integrity Failures (CWE Mapping)

OWASP A08:2025 consolidates several primary Common Weakness Enumerations (CWEs) centered around integrity:

### A. CWE-494: Download of Code Without Integrity Check
An application downloads executable binaries, configuration bundles, or dynamic extensions across a network without validating asymmetric digital signatures or cryptographic hashes against a trusted root of trust. Example: Fetching auto-update installers over plaintext HTTP or unpinned HTTPS.

### B. CWE-502: Deserialization of Untrusted Data
The system reconstructs serialized objects (such as Python `pickle`, Java serialized streams, PHP `unserialize()`, or Ruby unsafe YAML tags) supplied by untrusted clients. During deserialization, execution gadgets trigger arbitrary code execution inside the runtime process.

### C. Client-Side Integrity Assertion (Target Lab Vulnerability)
A fatal architectural flaw where backend ingestion logic delegates integrity validation to the caller. The endpoint ingests payloads declaring attributes like:
```json
{
  "signature_verified": false,
  "signer": "external"
}
```
and directly merges them into active operational memory. This directly violates the core security principle: **Integrity assertions must always be evaluated server-side using trusted public keys or certificates**.

---

## 3. Modern Industry Integrity Frameworks

To defend software ecosystems at enterprise scale, industry security consortia mandate the following frameworks:

1. **The Update Framework (TUF)**: A resilient security specification for software update systems designed to survive key compromises, rollback attacks, and freeze attacks. TUF underpins Docker Content Trust and modern automotive OTA systems.
2. **SLSA (Supply-chain Levels for Software Artifacts)**: A multi-tier framework (Levels 1 through 4) that guarantees software provenance from isolated build environments to cryptographically attested distribution artifacts.
3. **Sigstore & Cosign**: A modern keyless signing ecosystem leveraging short-lived OIDC certificates, transparency logs (Rekor), and public key infrastructure to sign and verify container images and release blobs.
