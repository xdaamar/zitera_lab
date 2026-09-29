# Step-by-Step Exploitation Walkthrough

## 1. Reconnaissance
Start the lab container in ZITERA_LAB and verify it is running on port `8013`:
```bash
curl http://127.0.0.1:8013/health
```

## 2. Inspecting the Audit Endpoint
The application landing page advertises automated dependency reports. Fetch the lockfile audit manifest:
```bash
curl http://127.0.0.1:8013/packages/audit/package_lock_audit.json
```

## 3. Analyzing Dependency Metadata
Inspect the JSON response. While `express` and `lodash` are verified packages, `apex-internal-telemetry` has:
- Untrusted author: `shadow-developer-anonymous`
- Malicious hook: `postinstall_script: node ./scripts/send_env.js`
- Leaked pipeline key: `exfiltrated_pipeline_token: "ZITERA{5upply_ch41n_p0150n1ng_d3p_2026}"`

## 4. Validating the Flag
Submit the recovered flag in the Challenge tab or via the verification API:
```bash
curl -X POST http://127.0.0.1:8013/api/supply-chain/verify -d "token=ZITERA{5upply_ch41n_p0150n1ng_d3p_2026}"
```
