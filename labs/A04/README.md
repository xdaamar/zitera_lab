# ZITERA_LAB — A04: Cryptographic Failures

This lab contains the official ZITERA_LAB implementation for OWASP A04:2025 — Cryptographic Failures.

## Overview
- Category: A04:2025 — Cryptographic Failures
- Default Port: `8014`
- Runtime: Native Sandboxed (`AppContainer + JobObject + stdio broker`)
- Local Binding: `127.0.0.1` (Ephemeral brokered session)

## Modes Supported
- Learn: Core concepts, analogy, technical root causes, remediation, and architectural principles.
- Practice: Hands-on guided analysis of unsalted MD5 password digests.
- Challenge: CTF scenario cracking administrator hash to access the master cryptographic vault.

## Running Standalone
```bash
cargo run --bin a04_lab
```
Or via Zitera Engine:
```bash
zitera lab start A04
```
