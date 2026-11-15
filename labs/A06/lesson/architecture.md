# System Architecture: Trust Boundaries & Authoritative State Machines

## 1. Lab A06 Architecture Topology (Zitera Procurement Portal)

The A06 environment models an enterprise procurement portal (*Zitera Procurement Workflow*) hosted on port 8016:

```text
[ Browser / API Client ]
            │
            ▼  (HTTP GET / POST port 8016)
┌──────────────────────────────────────────────────────────┐
│              ZITERA PROCUREMENT SERVICE RUNTIME          │
│                                                          │
│  ┌────────────────────────────────────────────────────┐  │
│  │ HTTP Request Dispatcher                            │  │
│  │ • GET /                        (HTML Dashboard)    │  │
│  │ • GET /health                  (Health Probe)      │  │
│  │ • GET /practice/verify         (Invariant Checker) │  │
│  │ • POST /api/orders/<id>/transition (FLAWED ENDPOINT)│  │
│  └────────────────────────┬───────────────────────────┘  │
│                           │                              │
│                           ▼ (Client Dictated State)      │
│  ┌────────────────────────────────────────────────────┐  │
│  │ In-Memory Orders State Store                       │  │
│  │ • Order 1001 ($120)  — Alice  (DRAFT)              │  │
│  │ • Order 1002 ($15000)— Bob    (PENDING_APPROVAL)   │  │
│  │ • Order 9999 ($50000)— Learner(DRAFT)              │  │
│  └────────────────────────┬───────────────────────────┘  │
│                           │                              │
│                           ▼                              │
│  ┌────────────────────────────────────────────────────┐  │
│  │ Business Invariant Verification Engine             │  │
│  │ (Detects high-value bypass >= $15,000 -> Releases) │  │
│  └────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘
```

---

## 2. Analysis of the Collapsed Trust Boundary
In this vulnerable architecture, critical design failures compound across three dimensions:
1. **External Control Ingestion**: The `target_state` variable originates directly within client JSON payloads. Application state should never be dictated directly by untrusted consumers.
2. **Missing Domain Invariant Guards**: The business logic fails to enforce domain rules: `IF order.amount >= 500 AND new_state == "APPROVED" THEN REQUIRE manager_signature`.
3. **Absence of Role Context Binding**: The state mutation handler performs zero verification of the caller's session role or permission tier.

---

## 3. High-Assurance Authoritative State Architecture (Domain State Pattern)

Resilient enterprise architectures isolate workflow logic within an authoritative **Domain State Machine**:

```text
[ Incoming Request ]
        │
        ▼
┌──────────────────────────────────────────────────────────────┐
│                  DOMAIN ENTITY WORKFLOW GUARD                │
│                                                              │
│  ┌────────────────────────────────────────────────────────┐  │
│  │ 1. Current State Invariant Check                       │  │
│  │    (Can current state transition to next state?)       │  │
│  │    DRAFT -> [PENDING_APPROVAL, CANCELLED] ONLY         │  │
│  └───────────────────────────┬────────────────────────────┘  │
│                              │                               │
│                              ▼                               │
│  ┌────────────────────────────────────────────────────────┐  │
│  │ 2. Policy Enforcement Point (PEP)                      │  │
│  │    • If amount >= $500 -> Enforce manager approval gate│  │
│  │    • Verify caller identity against required role      │  │
│  └───────────────────────────┬────────────────────────────┘  │
│                              │                               │
│                              ▼                               │
│  ┌────────────────────────────────────────────────────────┐  │
│  │ 3. Atomic State Mutation                               │  │
│  │    Apply change to datastore with audit logging        │  │
│  └────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────┘
```

### Core Architecture Pillars:
- **Action-Oriented Intent Handlers**: Clients submit semantic intents (e.g. `submit()`, `approve()`, `cancel()`), never direct state assignments (`order.status = "APPROVED"`).
- **Authoritative Transition Matrix**: The backend evaluates state changes against a rigid directed graph. Attempting to leapfrog from `DRAFT` directly to `DISPATCHED` triggers an immediate validation exception.
- **Role-Bound Invariant Enforcers**: Invariants governing financial thresholds and data transitions are enforced strictly at the domain entity boundary, immune to frontend omission.
