# Cryptographic Architecture: Key Management & Envelope Encryption

## 1. Lab A04 Architecture Topology (CryptoVault KMS)

The A04 environment models an enterprise credential and key validation service (*CryptoVault KMS*) running on port 8014:

```text
[ Browser / Audit Client ]
            │
            ▼  (HTTP GET / POST port 8014)
┌──────────────────────────────────────────────────────────┐
│                   CRYPTOVAULT SERVICE                    │
│                                                          │
│  ┌──────────────────────┐      ┌──────────────────────┐  │
│  │  Public Audit Route  │      │ Authentication Gate  │  │
│  │  /api/audit/hashes   │      │ /login (MD5 Match)   │  │
│  └──────────┬───────────┘      └──────────┬───────────┘  │
│             │                             │              │
│             ▼                             ▼              │
│  ┌──────────────────────┐      ┌──────────────────────┐  │
│  │ User Database Hashes │      │ Master Cryptographic │  │
│  │ (Alice, Bob, Admin)  │      │ Vault Storage (/vault│  │
│  │ MD5 without Salt     │      │ Session: Role Admin  │  │
│  └──────────────────────┘      └──────────┬───────────┘  │
│                                           │              │
│                                           ▼              │
│                                [ Target Flag Exposed ]   │
└──────────────────────────────────────────────────────────┘
```

---

## 2. Vulnerable Component Analysis
The target architecture exhibits severe interconnected design failures:
1. **Deterministic Credential Storage (Unsalted MD5)**: User passwords are stored as bare MD5 digests lacking salting (`21232f297a57a5a743894a0e4a801fc3` for `admin`).
2. **Public Audit Telemetry Exposure**: The diagnostic endpoint `/api/audit/hashes` is exposed over public HTTP without authentication, returning user account digests in cleartext JSON.
3. **Absence of Rate-Limiting Controls**: The login handler executes without backoff delays or lockout triggers, permitting rapid credential validation attempts.

---

## 3. High-Assurance Enterprise Cryptographic Architecture

Production-grade architectures isolate cryptographic primitives using **Envelope Encryption** and a dedicated **Key Management System (KMS)**:

```text
┌──────────────────────────────────────────────────────────────┐
│                HARDWARE SECURITY MODULE (HSM)                │
│                                                              │
│  ┌────────────────────────────────────────────────────────┐  │
│  │ Master Key (KMS Root Key) — Never Leaves HSM Boundary │  │
│  └───────────────────────────┬────────────────────────────┘  │
└──────────────────────────────┼───────────────────────────────┘
                               │ (Encrypt / Decrypt DEK)
                               ▼
┌──────────────────────────────────────────────────────────────┐
│                    APPLICATION RUNTIME                       │
│                                                              │
│  Plaintext Data  ──▶  [ AES-256-GCM ]  ──▶  Encrypted Data   │
│                              ▲                               │
│                              │                               │
│            Data Encryption Key (DEK) - Ephemeral             │
│                                                              │
│  Password Ingestion:                                         │
│  User Password   ──▶  [ Argon2id + Salt ]  ──▶ Secure Digest │
│                        (Memory: 64MB, t=3, p=4)              │
└──────────────────────────────────────────────────────────────┘
```

### Core Architecture Principles:
- **Envelope Encryption**: Sensitive data payloads are encrypted using an ephemeral *Data Encryption Key (DEK)*. The DEK itself is encrypted under a centralized *Key Encryption Key (KEK)* managed within a tamper-resistant Hardware Security Module (HSM).
- **Cryptographic Agility**: Serialized credential digests incorporate algorithm identifier tags explicitly (e.g. `$argon2id$v=19$m=65536,t=3,p=4$...`). This enables seamless in-flight re-hashing when users authenticate without requiring complex offline batch migrations.
- **Strict Audit Log Hygiene**: Logs must record security events and identity context without exposing raw digests, tokens, or credential payloads.
