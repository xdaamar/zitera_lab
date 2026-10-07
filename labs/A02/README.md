# ZITERA_LAB — A02: Security Misconfiguration

This lab contains the official ZITERA_LAB implementation for OWASP A02:2025 — Security Misconfiguration.

## Overview
- Category: A02:2025 — Security Misconfiguration
- Default Port: `8012`
- Runtime: Native Sandboxed (`AppContainer + JobObject + stdio broker`)
- Local Binding: `127.0.0.1` (Ephemeral brokered session)

## Modes Supported
- Learn: Core concepts, analogy, technical root causes, remediation, and architectural principles.
- Practice: Hands-on guided discovery of default credentials, debug dumps, and directory indexing.
- Challenge: CTF scenario extracting hidden master configuration keys from exposed backup files.

## Running Standalone
```bash
cargo run --bin a02_lab
```
Or via Zitera Engine:
```bash
zitera lab start A02
```
