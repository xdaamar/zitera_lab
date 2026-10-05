# Remediation & Modern Cryptographic Standards

To remediate cryptographic failures:

## 1. Use Adaptive, Salted Password Hashing
Never use MD5, SHA-1, or plain SHA-256 for user passwords. Always use modern, adaptive algorithms:
- `Argon2id` (Recommended by OWASP and IETF)
- `bcrypt` (Work factor / cost >= 12)
- `PBKDF2` (With >= 600,000 iterations for SHA-256)

### Python Secure Example:
```python
import bcrypt

# Securely hashing a password
salt = bcrypt.gensalt(rounds=12)
hashed = bcrypt.hashpw(password.encode('utf-8'), salt)

# Verifying a password during login
if bcrypt.checkpw(user_input.encode('utf-8'), stored_hash):
    # Authenticated!
```

## 2. Strong Symmetric Encryption
- Use `AES-256-GCM` or `ChaCha20-Poly1305` for authenticated symmetric encryption.
- Always use a unique, cryptographically random Nonce/IV for every encrypted message.

## 3. Key Management
- Never store cryptographic keys in source code repositories.
- Use dedicated Key Management Systems (KMS), hardware security modules (HSM), or secure environment secrets.
