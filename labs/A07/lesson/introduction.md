# Introduction to Authentication Failures

## What is an Authentication Failure?
Authentication is the process of confirming that someone is who they claim to be. If an application fails to properly verify user identity, an attacker can impersonate legitimate users, escalate privileges, and seize administrative control.

**OWASP A07:2025: Authentication Failures** (previously Identification and Authentication Failures) addresses flaws in how systems establish, maintain, and terminate digital user identities.

## Common Symptoms
- Permitting automated brute-force attacks and credential stuffing.
- Permitting weak, short, or widely compromised default passwords.
- Failing to invalidate session tokens upon logout or after idle timeouts.
- Exposing session IDs in URLs or unencrypted channels.
