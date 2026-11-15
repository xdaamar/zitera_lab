# Technical Analysis: Insecure Design & Finite State Machine Invariants

Insecure Design flaws originate from an architectural failure to articulate, enforce, and defend **security invariants** across the lifecycle of business data and application states.

---

## 1. Business Workflow Modeling (Finite State Machine / FSM)
In enterprise transaction and procurement systems, order states should strictly follow a deterministic directed graph:

```text
[ DRAFT ] ──▶ (Submit Order) ──▶ [ PENDING_APPROVAL ] ──▶ (Executive Signature) ──▶ [ APPROVED ] ──▶ [ DISPATCHED ]
   │                                    ▲
   │ (Order < $500)                     │ (Order >= $500: Mandatory Review)
   └────────────────────────────────────┘
```

### Intended Architectural Invariants:
1. Low-value orders (< $500) may automatically transition into `APPROVED`.
2. High-value orders ($\ge$ $500) **MUST** enter `PENDING_APPROVAL` and require cryptographically authenticated approval by an authorized `ExecutiveManager`.
3. An order may never transition directly from `DRAFT` or `PENDING_APPROVAL` to `DISPATCHED` without entering `APPROVED`.

---

## 2. The Architectural Flaw at the API Boundary
Examine the vulnerable backend dispatch logic:
```python
# FLAWED ARCHITECTURAL DESIGN
@app.route("/api/orders/<order_id>/transition", methods=["POST"])
def transition_order(order_id):
    payload = request.get_json()
    target_state = payload.get("target_state")
    action = payload.get("action")

    order = orders.get(order_id)
    if not order:
        return jsonify({"error": "Order not found"}), 404

    # DESIGN DEFECT: Trusts client to dictate target state directly!
    if target_state:
        order["status"] = target_state
    elif action == "force_dispatch":
        order["status"] = "DISPATCHED"

    return jsonify({"status": "updated", "order": order})
```

### Why This Represents a Critical Design Flaw:
1. **Unconstrained State Mutation**: The endpoint blindly ingests `target_state` from untrusted client requests without checking the valid transition matrix.
2. **Missing Invariant Validation**: The backend performs zero checks on order thresholds. A $50,000 procurement order (e.g. Order #9999) can bypass the entire managerial approval lifecycle.
3. **Absence of Role Authority Controls**: Any client with network access to the API can issue `{"action": "force_dispatch"}`, collapsing enterprise segregation of duties.

---

## 3. Common Design Anti-Patterns
- **Client-Driven State Pattern**: Delegating state transition enforcement to frontend JavaScript rather than server-side domain models.
- **Happy Path Bias**: Engineering workflows exclusively around cooperative user behaviors, failing to model malicious, out-of-order API invocations.
- **Lack of Abuse Cases in Specifications**: Writing functional specifications without documenting adversary capabilities, abuse cases, and negative testing scenarios.
