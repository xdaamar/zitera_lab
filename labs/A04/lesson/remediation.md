# Remediation Guide: Modern Cryptographic Standards & Implementation

To remediate cryptographic failures and defend sensitive records against compromise, implement these production-grade architectural standards:

---

## 1. Deploy Adaptive, Memory-Hard Password Hashing
Never use MD5, SHA-1, or unsalted SHA-256 for user credentials. Utilize algorithms intentionally engineered to consume dynamic memory and significant CPU cycles to defeat GPU/ASIC acceleration:

### Primary Standard: Argon2id
```python
from argon2 import PasswordHasher

# Initialize with OWASP recommended baseline parameters
# time_cost=3, memory_cost=65536 (64 MB), parallelism=4
ph = PasswordHasher(time_cost=3, memory_cost=65536, parallelism=4)

# Generate secure hash (automatically creates a cryptographically unique salt)
secure_hash = ph.hash("UserSecretPassword123!")

# Verify user password during login
try:
    ph.verify(secure_hash, "UserSecretPassword123!")
    # Verification succeeded
except Exception:
    # Verification failed
    pass
```

### Production Alternative: bcrypt (Cost Factor >= 12)
```python
import bcrypt

# Registration: Hash password with unique salt (12 work rounds)
salt = bcrypt.gensalt(rounds=12)
hashed_password = bcrypt.hashpw(raw_password.encode('utf-8'), salt)

# Authentication: Verify incoming password against stored digest
if bcrypt.checkpw(user_input.encode('utf-8'), hashed_password):
    # Authentication granted
    grant_access()
```

---

## 2. Authenticated Symmetric Encryption at Rest (AEAD Mode)
When encrypting sensitive columns (e.g. government identification, financial tokens, health records) in application datastores:
- Enforce **Authenticated Encryption with Associated Data (AEAD)** ciphers such as `AES-256-GCM` or `ChaCha20-Poly1305`.
- Generate a unique **Nonce / Initialization Vector (IV)** per encryption operation using a CSPRNG.

```python
import os
from cryptography.hazmat.primitives.ciphers.aead import AESGCM

# Generate 256-bit symmetric key (store and manage key within a KMS)
key = AESGCM.generate_key(bit_length=256)
aesgcm = AESGCM(key)

# Generate unique 96-bit nonce for each encryption event
nonce = os.urandom(12)
plaintext = b"Confidential Account Number: 1234-5678-9012"

# Encryption yields ciphertext concatenated with an authentication tag
ciphertext = aesgcm.encrypt(nonce, plaintext, None)

# Decryption validates authenticity before returning cleartext
decrypted = aesgcm.decrypt(nonce, ciphertext, None)
```

---

## 3. Constant-Time Digest Verification
To prevent side-channel timing analysis attacks during HMAC, token, or cryptographic signature comparisons:
```python
import hmac

# NEVER use standard equality operators:
# if user_token == expected_token: ...

# ALWAYS execute constant-time evaluation:
if hmac.compare_digest(user_token, expected_token):
    # Protected against execution duration measurements
    pass
```

---

## 4. Enforce Strict Transport Layer Encryption (TLS 1.3)
- Require **TLS 1.3** across all public interfaces and internal microservice service meshes, permanently disabling SSLv3, TLS 1.0, and TLS 1.1.
- Enable **HTTP Strict Transport Security (HSTS)** headers to ensure web clients communicate exclusively through encrypted tunnels:
  ```http
  Strict-Transport-Security: max-age=63072000; includeSubDomains; preload
  ```

---

## 5. Centralized Key Management Systems (KMS)
- Never embed cryptographic keys, master secrets, or static salts in source code repositories or unencrypted environment files.
- Provision, rotate, and audit encryption keys via dedicated FIPS 140-2 Level 3 Hardware Security Modules or KMS providers (such as HashiCorp Vault, AWS KMS, Google Cloud KMS, or Azure Key Vault).
