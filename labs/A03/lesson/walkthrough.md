# Practice Walkthrough: Lockfile Audit Investigation & Dependency Exfiltration

This walkthrough follows the 4-Tier Mission Progression methodology, balancing **60% manual inspection** and **40% command-line tooling**:

---

## Tier 1: Attack Surface Observation (Weight: 15%)

1. **Verify Lab Container Status**:
   Open your web browser and navigate to the lab target:
   `http://127.0.0.1:8013`

2. **Inspect the Registry Interface**:
   Examine the *ApexCorp // Internal Package Registry* dashboard:
   - Review the *Active Registry Dependencies* table.
   - Observe row 3: the package `apex-internal-telemetry` (version `0.1.0-alpha.build.88`) displays a prominent warning status: `UNTRUSTED / UNPINNED`.
   - The resolved repository source points to a local address (`http://127.0.0.1:8013/packages/download/`) rather than the official npm registry.
   - Note the link to the full manifest report: `/packages/audit/package_lock_audit.json`.

---

## Tier 2: Endpoint Discovery & Request Injection (Weight: 25%)

1. **Probe Health Endpoint**:
   Verify service readiness using terminal or browser:
   ```bash
   curl -s http://127.0.0.1:8013/health
   ```
   *Expected response*: `{"lab":"A03","port":8013,"status":"ok"}`

2. **Manual Inspection of the Lockfile Audit Manifest**:
   Open the audit link in your browser or run curl to retrieve the raw JSON document:
   ```bash
   curl -s http://127.0.0.1:8013/packages/audit/package_lock_audit.json
   ```

3. **Analyze the Document Structure**:
   Notice that the document follows standard npm lockfile schema (`lockfileVersion: 3`) and contains a `dependencies` dictionary:
   - `express` (`4.19.2`): Fetched from `https://registry.npmjs.org/` with verified sha512 integrity.
   - `lodash` (`4.17.21`): Verified with authentic cryptographic integrity hash.
   - `apex-internal-telemetry`: Features irregular and high-risk metadata properties.

---

## Tier 3: Weaponization & Secret Exfiltration Recovery (Weight: 35%)

1. **Dissect Suspicious Package Metadata**:
   Review the JSON payload for `apex-internal-telemetry`:
   ```json
   "apex-internal-telemetry": {
     "version": "0.1.0-alpha.build.88",
     "resolved": "http://127.0.0.1:8013/packages/download/apex-internal-telemetry.tgz",
     "author": "shadow-developer-anonymous",
     "integrity": "sha512-poisoned_build_hash_xyz",
     "postinstall_script": "node ./scripts/send_env.js",
     "exfiltrated_pipeline_token": "ZITERA{...}",
     "warning": "CRITICAL: Package not signed with corporate GPG key; untrusted origin."
   }
   ```

2. **Identify the Attack Indicators**:
   - `author`: `shadow-developer-anonymous` (untrusted publisher identity).
   - `postinstall_script`: `node ./scripts/send_env.js` (unauthorized lifecycle script executing upon package installation).
   - `exfiltrated_pipeline_token`: Harvested CI/CD runner secret captured during automated build execution.

3. **Validate the Retrieved Flag**:
   Submit the exfiltrated pipeline token via the web verification form at `/api/supply-chain/verify` or via curl:
   ```bash
   curl -s -X POST http://127.0.0.1:8013/api/supply-chain/verify -d "token=<RECOVERED_FLAG_TOKEN>"
   ```
   *Expected response*:
   ```json
   {
     "status": "passed",
     "message": "Flag verified! Supply chain backdoored dependency successfully audited.",
     "flag": "ZITERA{...}"
   }
   ```

---

## Tier 4: Defensive Verification & Remediation Hardening (Weight: 25%)

1. **Disable Lifecycle Script Execution**:
   Enforce non-executing installation flags inside automated CI/CD pipelines:
   ```bash
   npm ci --ignore-scripts
   ```

2. **Audit Lockfile Dependencies**:
   Perform automated dependency vulnerability scanning prior to packaging:
   ```bash
   npm audit --audit-level=high
   ```

3. **Enforce Private Scoped Namespaces**:
   Configure `.npmrc` to strictly route internal corporate scopes (e.g. `@apexcorp/`) exclusively to authenticated private registries.
