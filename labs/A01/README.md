# ZITERA_LAB: A01 — Broken Access Control

- **Category:** OWASP Top 10:2025 A01
- **Difficulty:** Beginner
- **Runtime:** Docker / Python Flask
- **Default Port:** 8011 (Bound strictly to `127.0.0.1`)

## Learning Objectives
- Differentiate between Authentication (AuthN) and Authorization (AuthZ).
- Understand how Insecure Direct Object References (IDOR) allow horizontal and vertical privilege escalation.
- Learn server-side authorization validation patterns.

## Challenge
Find the unauthorized master billing record and retrieve the secret flag (`ZITERA{...}`).
