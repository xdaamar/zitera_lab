# Architectural Principles: Cryptographic Agility & Forward Secrecy

Cryptographic agility is the ability of an application or security architecture to smoothly transition between cryptographic algorithms without redesigning the underlying system.

### Principles:
1. Never Invent Your Own Crypto: Always rely on vetted, peer-reviewed, industry-standard cryptographic libraries (libsodium, OpenSSL, WebCrypto).
2. Algorithm Identifier in Storage: Store the algorithm identifier alongside the hash (e.g. `$argon2id$v=19$m=65536,t=3,p=4$...`). This enables seamless password rehashing upon user login when upgrading security standards.
3. Transport Layer Security: Enforce TLS 1.3 with modern cipher suites and HTTP Strict Transport Security (HSTS) across all endpoints.

### Zero-Recompile Verification
This educational content is ingested dynamically by the ZITERA_LAB engine without requiring application rebuilds.
