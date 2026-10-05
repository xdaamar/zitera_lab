# ZITERA_LAB — A05: Injection (SQL Injection)

[![OWASP](https://img.shields.io/badge/OWASP-A05%3A2025-red)](https://owasp.org/Top10/A03_2021-Injection/)
[![Difficulty](https://img.shields.io/badge/Difficulty-Beginner-blue)]()
[![Runtime](https://img.shields.io/badge/Runtime-Native%20Sandboxed-informational)]()

A self-contained cybersecurity training lab for OWASP A05:2025 — Injection (SQL Injection focus).

---

## Scope

This lab is intentionally vulnerable. All challenge data is synthetic. It contains:
- No real user credentials
- No real database records
- No external network connections
- One deliberately insecure hardware catalog query service bound locally to ephemeral session port via stdio broker

Do not expose this application to an untrusted network.

---

## Learning Objectives

1. Understand how SQL injection exploits unsanitized user input concatenated into SQL queries
2. Manipulate SQL statements to bypass authentication and extract unauthorized data
3. Identify injection surfaces in login forms, search fields, and dynamic queries
4. Learn how parameterized queries and prepared statements eliminate SQL injection

---

## Challenge

Use SQL injection techniques (UNION-based injection) to extract the hidden records from the `vault_secrets` table and retrieve the secret flag in `ZITERA{...}` format.

---

## Lab Contract

| Property | Value |
|---|---|
| Schema Version | 1 |
| Lab ID | A05 |
| OWASP Ref | A05:2025 |
| Port | 8015 (Ephemeral brokered session) |
| Runtime | Native Sandboxed (`AppContainer + JobObject + stdio broker`) |
| Modes | learn, practice, challenge |
| Engine Compat | ≥0.1.0 |

---

## Installation via ZITERA Engine

```bash
zitera lab install A05
zitera lab start A05
```

Access the application in your browser via the displayed ephemeral brokered session URL.

---

## Reset

```bash
zitera lab reset A05
```

This deterministically restores the lab to its initial state.

---

## Stop

```bash
zitera lab stop A05
```

---

## Running Standalone

```bash
cargo run --bin a05_lab
```

---

## Security Expectations

- Process isolated inside Windows AppContainer profile `ZITERA_LAB_A05`
- Strict JobObject resource enforcement
- No Docker/WSL or administrative elevation required
- Challenge data is 100% synthetic
- Reset is deterministic and non-destructive to any other resource

---

## File Structure

```
manifest.json          — ZITERA lab contract
README.md              — this file
bin/
  a05-lab.exe          — Native sandboxed lab binary
lesson/                — Learning materials
challenge/             — Challenge-specific assets
```
