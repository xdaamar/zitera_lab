# ZITERA_LAB — A05: Injection (SQL Injection)

[![OWASP](https://img.shields.io/badge/OWASP-A05%3A2025-red)](https://owasp.org/Top10/A03_2021-Injection/)
[![Difficulty](https://img.shields.io/badge/Difficulty-Beginner-blue)]()
[![Runtime](https://img.shields.io/badge/Runtime-Docker-informational)]()

A self-contained cybersecurity training lab for **OWASP A05:2025 — Injection** (SQL Injection focus).

---

## Scope

This lab is **intentionally vulnerable**. All challenge data is synthetic. It contains:
- No real user credentials
- No real database records
- No external network connections
- One deliberately insecure Python Flask application bound only to `127.0.0.1:8015`

**Do not expose this application to an untrusted network.**

---

## Learning Objectives

1. Understand how SQL injection exploits unsanitized user input concatenated into SQL queries
2. Manipulate SQL statements to bypass authentication and extract unauthorized data
3. Identify injection surfaces in login forms, search fields, and dynamic queries
4. Learn how parameterized queries and prepared statements eliminate SQL injection

---

## Challenge

Use SQL injection techniques to extract the hidden admin credentials and retrieve the secret flag in `ZITERA{...}` format.

---

## Lab Contract

| Property | Value |
|---|---|
| Schema Version | 1 |
| Lab ID | A05 |
| OWASP Ref | A05:2025 |
| Port | 8015 (127.0.0.1 only) |
| Runtime | Docker / Python Flask |
| Modes | learn, practice, challenge |
| Engine Compat | ≥0.1.0 |

---

## Installation via ZITERA Engine

```
zitera lab install A05
zitera lab start A05
```

Then open `http://127.0.0.1:8015` in your browser.

---

## Reset

```
zitera lab reset A05
```

This deterministically restores the lab to its initial state.

---

## Stop

```
zitera lab stop A05
```

---

## Manual Docker Usage

```
cd docker
docker compose -f compose.yml -p zitera_a05 up -d
```

---

## Security Expectations

- Container bound to `127.0.0.1:8015` only — not exposed to LAN
- No privileged container mode
- No Docker socket mount
- No host filesystem bind mounts
- Challenge data is 100% synthetic
- Reset is deterministic and non-destructive to any other resource

---

## File Structure

```
manifest.json          — ZITERA lab contract
README.md              — this file
docker/
  Dockerfile           — Python 3.11-slim image definition
  compose.yml          — Docker Compose service definition
  app.py               — Flask application (intentionally vulnerable to SQL injection)
lesson/                — Learning materials
challenge/             — Challenge-specific assets
assets/                — Static assets
```
