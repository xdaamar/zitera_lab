# Remediation Guide: Secure-by-Design Architecture & Authoritative State Machines

To eliminate Insecure Design vulnerabilities at the architectural level, implement this comprehensive **Secure-by-Design** and defense-in-depth framework:

---

## 1. Execute Early-Stage Threat Modeling
Before committing to codebase architecture, perform structured threat modeling using the **STRIDE** methodology:
- **Spoofing**: Is the calling entity authenticated with cryptographic certainty?
- **Tampering**: Can transaction values, amounts, or states be manipulated across boundary payloads?
- **Repudiation**: Are critical state mutations permanently logged in tamper-evident audit streams?
- **Information Disclosure**: Do error responses expose internal workflow topologies or secrets?
- **Denial of Service**: Are rate-limiting and transaction concurrency caps enforced?
- **Elevation of Privilege**: Can unprivileged actors trigger executive workflow actions?

---

## 2. Deploy an Authoritative Finite State Machine (FSM)
Never permit clients to submit raw `target_state` strings. Enforce domain models where state transitions occur strictly through semantic action methods:

### Secure Python Implementation:
```python
class ProcurementOrder:
    ALLOWED_TRANSITIONS = {
        "DRAFT": ["PENDING_APPROVAL", "CANCELLED"],
        "PENDING_APPROVAL": ["APPROVED", "REJECTED"],
        "APPROVED": ["DISPATCHED"],
        "DISPATCHED": ["DELIVERED"],
        "CANCELLED": [],
        "REJECTED": []
    }

    def __init__(self, order_id, amount, owner):
        self.id = order_id
        self.amount = amount
        self.owner = owner
        self.status = "DRAFT"

    def submit_order(self, user_session):
        if self.status != "DRAFT":
            raise ValueError("Only DRAFT orders may be submitted.")

        # Enforce Business Invariant:
        if self.amount < 500:
            self.status = "APPROVED"  # Auto-approve small orders
        else:
            self.status = "PENDING_APPROVAL"  # Mandatory manager approval

    def approve_order(self, user_session):
        if self.status != "PENDING_APPROVAL":
            raise ValueError("Order is not pending approval.")

        # Separation of Duties & Role Enforcement:
        if user_session.get("role") != "ExecutiveManager":
            raise PermissionError("Only ExecutiveManager accounts can approve high-value orders.")

        self.status = "APPROVED"

    def dispatch_order(self, user_session):
        if self.status != "APPROVED":
            raise ValueError("Cannot dispatch an unapproved order.")

        self.status = "DISPATCHED"
```

---

## 3. Enforce Strict Separation of Duties (SoD)
- The creator of an order (*requester*) must never possess permission to approve their own order, regardless of ambient administrative privileges.
- High-value operations must require dual-authorization (*Two-Person Rule / Dual-Control*).

---

## 4. Invariant Enforcement at the Domain Entity Boundary
- Never rely on client-side frontend validation (HTML5 forms, Flutter UI controllers, or JavaScript).
- All boundary limits (e.g., non-negative financial values, discount caps, quantity ceilings) must be enforced rigidly within server-side domain entities.

---

## 5. Tamper-Evident Security Audit Telemetry
Every transaction transition must generate an immutable audit record:
- Authenticated operator identity and calling IP address.
- High-precision UTC timestamp.
- Originating state and resulting target state.
- Associated authorization tokens or business rationale.
