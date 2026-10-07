# A01: Broken Access Control

Broken Access Control occupies the #1 spot in OWASP Top 10:2025. It occurs when an application fails to properly enforce restrictions on what authenticated or anonymous users are permitted to do.

## Why It Matters
When access controls fail, attackers can:
- Act as unauthorized users or administrators
- View sensitive files or records (IDOR / BOLA)
- Modify other users' profile data or financial accounts
- Change access rights or bypass security mechanisms

## Authentication vs Authorization
A common beginner mistake is confusing the two:
- Authentication (AuthN): "Who are you?" (e.g. Logging in with username and password).
- Authorization (AuthZ): "What are you allowed to do?" (e.g. User Alice can only see Alice's invoices, not Bob's).

Broken Access Control is almost always a failure of Authorization, not Authentication.
