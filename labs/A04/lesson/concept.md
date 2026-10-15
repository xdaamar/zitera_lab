# Technical Analysis: Hashing, Salting, & Modern Cryptographic Primitives

Robust cryptographic implementations rely upon a clear understanding of the distinct roles played by two-way encryption algorithms versus one-way cryptographic hash functions.

---

## 1. Fundamental Distinction: Encryption vs Hashing

| Dimension | Symmetric / Asymmetric Encryption | Cryptographic Hash Functions |
| :--- | :--- | :--- |
| **Mathematical Nature** | Two-way (*reversible* using private keys) | One-way (*strictly irreversible*) |
| **Core Objective** | Preserve confidentiality of data that must later be recovered | Validate integrity or verify credentials without storing plaintext |
| **Common Use Cases** | Payment card numbers (AES-256-GCM), TLS session transport | Credential storage (Argon2id), artifact checksum verification (SHA-256) |
| **Key Vulnerability** | Key exposure decrypts all historical records | Fast algorithms allow rapid offline dictionary cracking |

---

## 2. Why Fast Hashes (MD5, SHA-1, SHA-256) Fail for Credentials
General-purpose hash algorithms such as MD5 and SHA-256 were designed to compute checksums over gigabytes of file data at maximum throughput. This exact high-speed property makes them disastrously unsuitable for password storage:
- **GPU Cluster Throughput**: A commodity workstation GPU cluster can compute over 100 billion MD5 digests per second.
- **Trivial Offline Brute-Force**: Standard 8-character passwords can be exhaustively searched within minutes.
- **Cryptographic Collisions**: MD5 collision attacks were proven in 2004 (Wang et al.), and SHA-1 collisions were generated in 2017 (Google SHAttered attack).

---

## 3. Deterministic Hashing & Rainbow Table Lookups
Without a unique cryptographic salt, hash algorithms remain strictly **deterministic**: the identical input always generates the identical output:
$$\text{MD5}(\text{"admin"}) = \text{21232f297a57a5a743894a0e4a801fc3}$$

Adversaries exploit this determinism by precomputing **Rainbow Tables** (compressed multi-terabyte search indices) mapping trillions of known passwords to their resulting digests. When a database is exfiltrated, attackers do not need to compute hashes—they simply query the index like a dictionary.

---

## 4. Salting Mechanics & Adaptive Memory-Hard Algorithms
To neutralize precomputed tables and hardware-accelerated attacks:

### A. Cryptographic Salt
A cryptographically secure pseudo-random string (minimum 16 bytes from a CSPRNG) is concatenated with the password prior to hashing:
$$\text{Hash} = \text{Algorithm}(\text{Password} + \text{Unique\_Salt})$$
Even if 1,000 users share the identical password `"password123"`, every stored digest is mathematically distinct because each user holds a unique salt. Rainbow tables become completely obsolete.

### B. Adaptive Memory-Hard Algorithms
Modern algorithms enforce memory-hard computation and multi-round iteration factors to throttle hardware acceleration:
1. **Argon2id**: Winner of the Password Hashing Competition (PHC). Defeats GPU and ASIC attacks by requiring large dynamic memory allocations.
2. **bcrypt**: Based on the Blowfish cipher with an adjustable work factor. A cost parameter of 12 requires ~250ms of CPU compute per attempt.
3. **PBKDF2**: NIST standard applying repeated HMAC rounds (recommended minimum: 600,000 iterations for HMAC-SHA-256).

---

## 5. Authenticated Encryption with Associated Data (AEAD)
For data at rest:
- **Avoid Legacy ECB / CBC Modes**: Vulnerable to padding oracle attacks and bit-flipping manipulation.
- **Deploy AEAD Ciphers (AES-256-GCM or ChaCha20-Poly1305)**: Combines symmetric encryption with an integrated cryptographic authentication tag. Any unauthorized ciphertext modification immediately aborts decryption safely.
- **Strict Nonce / IV Uniqueness**: Reusing an Initialization Vector (IV/Nonce) with the same AES-GCM key allows attackers to recover the authentication key and forge messages.
