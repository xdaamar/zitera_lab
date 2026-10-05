# Target Lab Architecture

## System Diagram

```
[Attacker / User]
       │
       ▼
[Web Server (OmniAuth Gateway)] ─── (Serves http://127.0.0.1:8017)
       │
       ├── /login (Unthrottled endpoint, no lockout)
       ├── /api/login (JSON API endpoint)
       └── /admin (Protected view containing the flag)
```

## Vulnerable Design Points
- Zero Throttling: The server accepts hundreds of consecutive failed attempts per second without returning HTTP 429 Too Many Requests.
- Weak Password Policy: The administrator account (`admin`) accepts a trivial 4-digit PIN (`2026`).
- No Multi-Factor Authentication (MFA): Access relies purely on a single factor.
