# Security Architecture: Layered Authorization Enforcement

To permanently eradicate Broken Access Control vulnerabilities across enterprise web architectures, systems must not rely on single ad-hoc checks scattered throughout controller functions. The architecture must enforce a layered **Policy Enforcement Point (PEP)** and **Policy Decision Point (PDP)** model.

---

### End-to-End Authorization Architecture Flow

```
[ CLIENT / BROWSER ]
       â”‚  HTTP Request: GET /invoice/2
       â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 1. API Gateway / Reverse Proxy                         â”‚
â”‚    - Rate Limiting                                     â”‚
â”‚    - Token & Session Signature Validation              â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                       â”‚
                       â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 2. Authentication Middleware (AuthN)                   â”‚
â”‚    - Inspects Cookie / Authorization Bearer header     â”‚
â”‚    - Establishes verified identity: user_id=1          â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                       â”‚
                       â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 3. Policy Enforcement Point (PEP)                      â”‚
â”‚    - Checks whether user_id=1 is permitted to read    â”‚
â”‚      target resource invoice_id=2                      â”‚
â”‚    - Sends context to Policy Decision Point (PDP)      â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                       â”‚  Authorization DENIED (Not Owner)
                       â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º [ 403 / 404 Deny ]
                       â”‚  Authorization GRANTED
                       â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ 4. Data Access Layer (DAL)                             â”‚
â”‚    - Scoped multi-tenant queries (Tenant Scoping)      â”‚
â”‚    - SELECT * FROM invoices WHERE id=2 AND owner_id=1  â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                       â”‚
                       â–¼
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
| **Anonymous (Guest)** | âŒ | âŒ | âŒ | âŒ |
| **Standard User (Alice/Bob)** | âœ… | âŒ | âŒ | âŒ |
| **Auditor / Finance Staff** | âœ… | âœ… (Read-Only) | âŒ | âŒ |
| **System Administrator** | âœ… | âœ… | âœ… | âœ… |
