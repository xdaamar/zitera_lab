# ZITERA_LAB: Lab A07 — Authentication Failures

[![OWASP](https://img.shields.io/badge/OWASP-A07%3A2025-blue)](https://owasp.org/Top10/)
[![Difficulty](https://img.shields.io/badge/Difficulty-Beginner-green)](#)
[![Runtime](https://img.shields.io/badge/Runtime-Native%20Sandboxed-informational)](#)

## Overview
This laboratory illustrates common authentication vulnerabilities, focusing on missing rate limiting, weak password/PIN requirements, and susceptibility to credential brute-forcing.

## Target Environment
- Local Host URL: `http://127.0.0.1:8017` (Ephemeral brokered session)
- Runtime: Native Sandboxed (`AppContainer + JobObject + stdio broker`)
- Binary: `bin/a07-lab.exe`

## Objectives
1. Perform reconnaissance on the admin login endpoint at `/login` and `/api/login`.
2. Discover that the portal enforces no rate-limiting or lockout mechanisms.
3. Test credentials for `admin` using common password patterns / 4-digit PINs (`2026`).
4. Gain access to the admin portal and retrieve the challenge flag.

## Running Standalone
```bash
cargo run --bin a07_lab
```
Or via Zitera Engine:
```bash
zitera lab start A07
```
