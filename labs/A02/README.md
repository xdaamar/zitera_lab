# ZITERA_LAB — A02: Security Misconfiguration

This repository contains the official ZITERA_LAB implementation for **OWASP A02:2025 — Security Misconfiguration**.

## Overview
- **Category:** A02:2025 — Security Misconfiguration
- **Default Port:** `8012`
- **Runtime:** Docker (WSL2 / Docker Desktop)
- **Local Binding:** `127.0.0.1:8012` (Host-isolated, non-routable)

## Modes Supported
- **Learn:** Core concepts, analogy, technical root causes, remediation, and architectural principles.
- **Practice:** Hands-on guided discovery of default credentials, debug dumps, and directory indexing.
- **Challenge:** CTF scenario extracting hidden master configuration keys from exposed backup files.

## Running Standalone
```bash
cd docker
docker compose up -d --build
```

Access the application in your browser at `http://127.0.0.1:8012`.
