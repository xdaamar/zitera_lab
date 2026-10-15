# A04: Cryptographic Failures

## 1. What are Cryptographic Failures?
In the OWASP Top 10:2025 standard, **Cryptographic Failures** (previously referred to as *Sensitive Data Exposure*) encompasses defects related to cryptography—or the complete omission thereof—that result in the compromise, disclosure, or manipulation of sensitive assets.

Cryptography serves as the ultimate line of defense: when perimeter protections (such as firewalls, web application proxies, or network segmentation) are bypassed, robust encryption and cryptographically secure hashing guarantee that exfiltrated records remain mathematically indecipherable and tamper-proof.

## 2. Why Cryptographic Failures Occur
The vast majority of cryptographic incidents do not stem from adversaries breaking the underlying mathematical foundations of modern ciphers, but rather from engineering and implementation blunders:
- **Deployment of Deprecated and Broken Algorithms**: Utilizing obsolete functions such as MD5 or SHA-1 for credentials, or legacy ciphers like DES and RC4.
- **Unsalted Password Storage**: Storing deterministic digests susceptible to precomputed lookup tables (*Rainbow Tables*).
- **Insecure Key Management**: Hardcoding master cryptographic secrets within source code repositories or configuration files.
- **Missing Transit or Rest Encryption**: Transmitting credentials over cleartext protocols (HTTP, Telnet) or persisting sensitive datastores without authenticated volume or column-level encryption.

## 3. Real-World Business & Operational Impact
When cryptographic controls fail:
1. **Identity & Credential Harvest**: Attackers reverse leaked password hashes to plaintext credentials, gaining authentic administrative access across enterprise perimeters.
2. **Data Tampering & Forgery**: Without authenticated encryption (AEAD) or digital signatures, network interceptors alter payload bodies in transit without detection.
3. **Severe Regulatory Non-Compliance**: Crushing legal liabilities under GDPR, HIPAA, PCI-DSS, or regional data privacy laws.

## 4. Learning Objectives in Lab A04
Through the *CryptoVault* hands-on environment on port 8014, you will:
1. Inspect public audit endpoints exposing user password hashes.
2. Analyze the structural characteristics of 128-bit MD5 digests (32-character hexadecimal strings).
3. Reverse an unsalted administrator hash using deterministic dictionary lookups.
4. Authenticate into the administrative vault and recover key in standard `ZITERA{...}` format.
5. Implement modern, adaptive, memory-hard password hashing architectures using `Argon2id` and `bcrypt`.
