# ZITERA_LAB — A04: Cryptographic Failures

This repository contains the official ZITERA_LAB implementation for **OWASP A04:2025 — Cryptographic Failures**.

## Overview
- **Category:** A04:2025 — Cryptographic Failures
- **Default Port:** `8014`
- **Runtime:** Docker (WSL2 / Docker Desktop)
- **Local Binding:** `127.0.0.1:8014` (Host-isolated, non-routable)

## Modes Supported
- **Learn:** Core concepts, analogy, technical root causes, remediation, and architectural principles.
- **Practice:** Hands-on guided analysis of unsalted MD5 password digests.
- **Challenge:** CTF scenario cracking administrator hash to access the master cryptographic vault.

## Running Standalone
```bash
cd docker
docker compose up -d --build
```

Access the application in your browser at `http://127.0.0.1:8014`.
