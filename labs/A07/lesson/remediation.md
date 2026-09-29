# Remediation and Defense Strategies

## 1. Multi-Factor Authentication (MFA)
Require MFA (TOTP, FIDO2/WebAuthn) for all administrative and user logins. MFA mitigates over 99% of automated credential stuffing attacks.

## 2. Rate Limiting and Account Lockout
- Enforce IP and username-based rate limits (e.g. maximum 5 failed attempts per 15 minutes).
- Implement exponential backoff delays on consecutive failed logins.

## 3. Strict Password Requirements
- Enforce minimum length (NIST recommends at least 8 to 12 characters).
- Check submitted passwords against known compromised password lists (e.g. HaveIBeenPwned database).

## 4. Secure Session Management
- Generate high-entropy session IDs using cryptographically secure PRNGs.
- Invalidate sessions immediately upon logout.
- Set `Secure`, `HttpOnly`, and `SameSite=Lax/Strict` attributes on session cookies.
