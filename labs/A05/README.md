# ZITERA_LAB: A05 — Injection

- **Category:** OWASP Top 10:2025 A05
- **Difficulty:** Beginner
- **Runtime:** Docker / Python Flask / SQLite
- **Default Port:** 8015 (Bound strictly to `127.0.0.1`)

## Learning Objectives
- Understand how query/data separation breaks under string concatenation.
- Learn to test for SQL injection syntax errors and boolean conditions.
- Extract data using UNION-based injection.
- Master parameterized query / prepared statement remediation.

## Challenge
Extract the secret project key stored in the private `vault_secrets` table and retrieve the flag (`ZITERA{...}`).
