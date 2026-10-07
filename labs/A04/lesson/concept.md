# Technical Breakdown: Fast Hashes vs Slow Hashes & Salts

## 1. Fast Hashes vs Password Hashing
Cryptographic hash functions like `MD5` and `SHA-256` were designed for integrity verification (file checksums), not password storage.
- A modern GPU cluster can compute billions of MD5 or SHA-256 hashes per second.
- If a database of unsalted MD5 hashes is leaked, attackers crack the entire list in seconds using precomputed Rainbow Tables or dictionary attacks.

```text
Input: "admin" -> MD5: 21232f297a57a5a743894a0e4a801fc3
Input: "password123" -> MD5: 482c811da5d5b4bc6d497ffa98491e38
```

Because MD5 is deterministic, reversing `21232f297a57a5a743894a0e4a801fc3` requires zero computation—it is looked up instantly in online databases.

## 2. The Purpose of Cryptographic Salts
A `salt` is a cryptographically random, unique sequence of bytes appended to a password before hashing:
```text
Hash = Algorithm(Password + UniqueSalt)
```
- Salts ensure that two users with the exact same password have completely different hashes.
- Salts completely defeat precomputed Rainbow Tables.

## 3. Slow, Adaptive Key Derivation (Argon2 / bcrypt)
Modern password hashing uses algorithms designed to be deliberately slow and resource-intensive (cost factors):
- Requires CPU time and memory space per attempt.
- Throttles brute-force attempts from billions per second down to hundreds per second.
