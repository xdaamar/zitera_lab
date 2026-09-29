# A04: Cryptographic Failures

Cryptographic Failures occupies the **#4 spot in OWASP Top 10:2025** (previously known as *Sensitive Data Exposure*). It focuses on failures related to cryptography, which often lead to sensitive data exposure or complete system compromise.

## Core Problem
Cryptography is math, but cryptographic security is implementation. Common flaws occur when developers:
- Transmit sensitive data in cleartext (HTTP, FTP, Telnet).
- Rely on legacy, broken, or deprecated cryptographic algorithms (MD5, SHA-1, DES, RC4).
- Use default, weak, or hardcoded cryptographic keys embedded in source code.
- Store passwords using fast unsalted hashes instead of adaptive, memory-hard hashing algorithms (Argon2, bcrypt, PBKDF2).
- Reuse Initialization Vectors (IVs) or nonce values in stream ciphers.

## Real-World Impact
When cryptography fails, attackers can decrypt confidential user data, forge administrative session tokens, crack password databases using rainbow tables, and impersonate authorized entities.
