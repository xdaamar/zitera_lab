# Security Architecture: Layered Authorization Enforcement

To permanently eradicate Broken Access Control vulnerabilities across enterprise web architectures, systems must not rely on single ad-hoc checks scattered throughout controller functions. The architecture must enforce a layered **Policy Enforcement Point (PEP)** and **Policy Decision Point (PDP)** model.

---

### End-to-End Authorization Architecture Flow

```
[ CLIENT / BROWSER ]
       │  HTTP Request: GET /invoice/2
       ▼
┌────────────────────────────────────────────────────────┐
│ 1. API Gateway / Reverse Proxy                         │
│    - Rate Limiting                                     │
│    - Token & Session Signature Validation              │
└──────────────────────┬─────────────────────────────────┘
                       │
                       ▼
┌────────────────────────────────────────────────────────┐
│ 2. Authentication Middleware (AuthN)                   │
│    - Inspects Cookie / Authorization Bearer header     │
│    - Establishes verified identity: user_id=1          │
└──────────────────────┬─────────────────────────────────┘
                       │
                       ▼
┌────────────────────────────────────────────────────────┐
│ 3. Policy Enforcement Point (PEP)                      │
│    - Checks whether user_id=1 is permitted to read    │
│      target resource invoice_id=2                      │
│    - Sends context to Policy Decision Point (PDP)      │
└──────────────────────┬─────────────────────────────────┘
                       │  Authorization DENIED (Not Owner)
                       ├────────────────────────► [ 403 / 404 Deny ]
                       │  Authorization GRANTED
                       ▼
┌────────────────────────────────────────────────────────┐
│ 4. Data Access Layer (DAL)                             │
│    - Scoped multi-tenant queries (Tenant Scoping)      │
│    - SELECT * FROM invoices WHERE id=2 AND owner_id=1  │
└──────────────────────┬─────────────────────────────────┘
                       │
                       ▼
[ ISOLATED DATABASE ENGINE ]
```

---

### Three Core Pillars of Secure Authorization Architecture
1. **Deny by Default:**
   Every new endpoint, method, or resource must be blocked by default unless explicit authorization policies are declared and verified.
2. **Session-Bound Ownership:**
   User identity must never be extracted from untrusted client parameters (`request.args['user_id']` or request body), but must always be resolved from securely signed, server-side session stores.
3. **Indirect Reference Maps:**
   Never expose sequential primary keys (`1, 2, 3...`) in public APIs. Replace them with cryptographically random UUIDv4 identifiers or per-session ephemeral token lookup maps, preventing trivial enumeration attacks.

---

### Role-Based Access Control (RBAC) Matrix

| Role | Read Own Invoices | Read Others' Invoices | Download Master Invoices | Modify System Rates |
| :--- | :---: | :---: | :---: | :---: |
| **Anonymous (Guest)** | ❌ | ❌ | ❌ | ❌ |
| **Standard User (Alice/Bob)** | ✅ | ❌ | ❌ | ❌ |
| **Auditor / Finance Staff** | ✅ | ✅ (Read-Only) | ❌ | ❌ |
| **System Administrator** | ✅ | ✅ | ✅ | ✅ |
