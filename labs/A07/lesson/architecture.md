# System Architecture: Authentication Defense & Resilient Session Topologies

## 1. Lab A07 Architecture Topology (OmniAuth Admin Gateway)

The A07 target environment simulates an administrative single sign-on ingress portal (*OmniAuth Admin Gateway*) running on port 8017:

```text
[ Browser / Brute-Force Bot ]
             │
             ▼  HTTP GET / POST (Port 8017)
┌──────────────────────────────────────────────────────────┐
│              OMNIAUTH RUNTIME SERVICE                    │
│                                                          │
│  ┌────────────────────────────────────────────────────┐  │
│  │ HTTP Request Dispatcher                            │  │
│  │ • GET  /health          (Health Check)             │  │
│  │ • GET  /login           (HTML Login View)          │  │
│  │ • POST /login           (Form Authentication)      │  │
│  │ • POST /api/login       (JSON API Credential Check)│  │
│  │ • GET  /admin           (Protected View & Flag)    │  │
│  │ • GET  /logout          (Session Teardown)         │  │
│  └────────────────────────┬───────────────────────────┘  │
│                           │                              │
│                           ▼ (Synchronous Pin Matching)   │
│  ┌────────────────────────────────────────────────────┐  │
│  │ Unthrottled Evaluation Engine                      │  │
│  │ • Target: admin:2026                               │  │
│  │ • ZERO Rate Limiting / ZERO Cooldown Delay         │  │
│  └────────────────────────┬───────────────────────────┘  │
│                           │                              │
│                           ▼                              │
│  ┌────────────────────────────────────────────────────┐  │
│  │ Authenticated Administrative Session Context       │  │
│  │ • session["authenticated"] = True                  │  │
│  │ • Exposes Master Challenge Flag                    │  │
│  └────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘
```

---

## 2. Vulnerable Architectural Analysis
The service exhibits multiple architectural security anti-patterns:
1. **Omission of Rate Limiting Guards**: Both web forms and JSON API routes execute immediate synchronous credential evaluations without querying client submission frequencies.
2. **Monolithic Single-Factor Dependency**: High-privilege administrative access relies completely upon a low-entropy single factor (4-digit numeric PIN) without secondary challenge requirements.
3. **Sub-Millisecond Rejection Latency**: The application returns rejection codes instantaneously (< 10ms), providing ideal conditions for parallel automated socket bombardment.

---

## 3. High-Assurance Identity Architecture (Centralized IdP & Multi-Layered Throttling)

Production architectures decouple network ingress from credential evaluation through isolated layers:

```text
[ External Traffic ]
        │
        ▼
┌──────────────────────────────────────────────────────────────┐
│                    API GATEWAY / REVERSE PROXY               │
│                                                              │
│  ┌────────────────────────────────────────────────────────┐  │
│  │ Adaptive Rate Limiting (Token Bucket / Redis Cluster)   │  │
│  │ • Max 5 failed attempts per IP per 15 minutes          │  │
│  │ • Exponential Backoff Delay (1s, 2s, 4s, 8s, 16s)      │  │
│  └───────────────────────────┬────────────────────────────┘  │
└──────────────────────────────┼───────────────────────────────┘
                               │ (Clean / Throttled Traffic)
                               ▼
┌──────────────────────────────────────────────────────────────┐
│             CENTRALIZED IDENTITY PROVIDER (IdP)              │
│                                                              │
│  User Verification  ──▶  [ Argon2id / bcrypt Hash ]          │
│                                      │ (Success)             │
│                                      ▼                       │
│  Second Factor Gate ──▶  [ FIDO2 / WebAuthn / TOTP Token ]   │
│                                      │ (Success)             │
│                                      ▼                       │
│  Session Engine     ──▶  [ Cryptographic Token Rotation ]    │
│                          • Invalidate pre-login session ID   │
│                          • Mint high-entropy HttpOnly cookie │
└──────────────────────────────────────────────────────────────┘
```

### Core Architecture Pillars:
- **Perimeter Rate Limiting**: Request limits are enforced at the network ingress proxy using the *Token Bucket* or *Sliding Window Counter* algorithms backed by low-latency datastores.
- **Adaptive Exponential Backoff**: Injecting synthetic response delays that increase exponentially following sequential failed authentication attempts.
- **Hardware-Backed MFA**: Requiring FIDO2/WebAuthn physical authenticators or cryptographic TOTP app challenges across administrative domains.
- **Strict Session Cookie Isolation**: Emitting session cookies strictly with `HttpOnly`, `Secure`, and `SameSite=Strict` attributes, invalidating pre-authentication session tokens upon identity verification.
