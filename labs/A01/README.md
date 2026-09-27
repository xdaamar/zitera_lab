# ZITERA_LAB — A01: Broken Access Control

[![OWASP](https://img.shields.io/badge/OWASP-A01%3A2025-red)](https://owasp.org/Top10/A01_2021-Broken_Access_Control/)
[![Difficulty](https://img.shields.io/badge/Difficulty-Beginner-blue)]()
[![Runtime](https://img.shields.io/badge/Runtime-Docker-informational)]()

A self-contained cybersecurity training lab for **OWASP A01:2025 — Broken Access Control**.

---

## Scope

This lab is **intentionally vulnerable**. All challenge data is synthetic. It contains:
- No real user credentials
- No real financial data
- No external network connections
- One deliberately insecure Python Flask application bound only to `127.0.0.1:8011`

**Do not expose this application to an untrusted network.**

---

## Learning Objectives

1. Differentiate between Authentication (AuthN) and Authorization (AuthZ)
2. Understand how Insecure Direct Object References (IDOR) allow horizontal and vertical privilege escalation
3. Identify authorization flaws by inspecting HTTP requests
4. Learn server-side ownership-based authorization validation patterns

---

## Challenge

Log in as a regular user and discover the unauthorized master billing record.
Retrieve the secret flag in `ZITERA{...}` format.

---

## Lab Contract

| Property | Value |
|---|---|
| Schema Version | 1 |
| Lab ID | A01 |
| OWASP Ref | A01:2025 |
| Port | 8011 (127.0.0.1 only) |
| Runtime | Docker / Python Flask |
| Modes | learn, practice, challenge |
| Engine Compat | ≥0.1.0 |

---

## Installation via ZITERA Engine

```
zitera lab install A01
zitera lab start A01
```

Then open `http://127.0.0.1:8011` in your browser.

---

## Reset

```
zitera lab reset A01
```

This deterministically restores the lab to its initial state (removes and recreates Docker volumes).

---

## Stop

```
zitera lab stop A01
```

---

## Manual Docker Usage

```
cd docker
docker compose -f compose.yml -p zitera_a01 up -d
```

---

## Security Expectations

- Container bound to `127.0.0.1:8011` only — not exposed to LAN
- No privileged container mode
- No Docker socket mount
- No host filesystem bind mounts
- Challenge data is 100% synthetic (no real credentials or financial records)
- Reset is deterministic and non-destructive to any other resource

---

## File Structure

```
manifest.json          — ZITERA lab contract
README.md              — this file
docker/
  Dockerfile           — Python 3.11-slim image definition
  compose.yml          — Docker Compose service definition
  app.py               — Flask application (intentionally vulnerable)
lesson/                — Learning materials
challenge/             — Challenge-specific assets
assets/                — Static assets
```
