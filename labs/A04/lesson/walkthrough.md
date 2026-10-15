# Practice Walkthrough: Cryptographic Failures & Unsalted Hash Reversal

This walkthrough adheres to the 4-Tier Mission Progression methodology, balancing **60% manual inspection** and **40% command-line tooling**:

---

## Tier 1: Attack Surface Observation (Weight: 15%)

1. **Verify Lab Container Readiness**:
   Open your browser and navigate to the target address:
   `http://127.0.0.1:8014`

2. **Inspect the CryptoVault Landing Page**:
   Observe the *ZITERA CryptoVault Key Management System* dashboard:
   - Notice the purple *Public Cryptographic Inspection Endpoints* container.
   - Three key routes are listed:
     - `/health` — Service health probe.
     - `/api/audit/hashes` — Legacy audit table exposing user credential hashes.
     - `/vault` — Master cryptographic storage (restricted to SecurityOfficer/admin).
   - Click **Sign In to Vault** to examine the credential form at `/login`.

---

## Tier 2: Endpoint Discovery & Request Injection (Weight: 25%)

1. **Probe Health Status**:
   Verify container availability via terminal or browser:
   ```bash
   curl -s http://127.0.0.1:8014/health
   ```
   *Expected response*: `{"lab":"A04","port":8014,"status":"healthy"}`

2. **Inspect the Audit Hashes Endpoint**:
   Navigate to `http://127.0.0.1:8014/api/audit/hashes` in your browser or issue a curl request:
   ```bash
   curl -s http://127.0.0.1:8014/api/audit/hashes
   ```

3. **Analyze Returned Cryptographic Material**:
   Inspect the JSON response payload:
   ```json
   {
     "records": [
       {"hash": "482c811da5d5b4bc6d497ffa98491e38", "hash_type": "MD5 (Unsalted)", "role": "Analyst", "username": "alice"},
       {"hash": "5ebe2294ecd0e0f08eab7690d2a6ee69", "hash_type": "MD5 (Unsalted)", "role": "Auditor", "username": "bob"},
       {"hash": "21232f297a57a5a743894a0e4a801fc3", "hash_type": "MD5 (Unsalted)", "role": "SecurityOfficer", "username": "admin"}
     ],
     "status": "success"
   }
   ```
   - Administrator Digest: 32-character hexadecimal string (128 bits): `21232f297a57a5a743894a0e4a801fc3`.
   - Vulnerability Indicator: The algorithm is marked explicitly as **MD5 (Unsalted)**.

---

## Tier 3: Weaponization & Secret Recovery (Weight: 35%)

1. **Reverse the Unsalted MD5 Digest**:
   Because the hash lacks cryptographic salting, identical inputs yield identical outputs globally. Test common administrative passwords in the terminal:
   ```bash
   # Test candidate password 'admin'
   echo -n "admin" | md5sum
   ```
   *Output*: `21232f297a57a5a743894a0e4a801fc3` (Exact match with the target administrator digest).

2. **Authenticate into the Portal**:
   Navigate to the login interface at `http://127.0.0.1:8014/login`:
   - **Username**: `admin`
   - **Password**: `admin`
   - Click **Authenticate**.

3. **Capture the Master Vault Secret**:
   The backend matches the MD5 hash and redirects your authenticated session to `/vault`:
   - Authenticated Role: `SecurityOfficer (admin)`.
   - Master Vault Activation Key in standard format: `ZITERA{...}`.
   - Copy the displayed flag token and submit it to the Challenge tab.

---

## Tier 4: Defensive Verification & Remediation Hardening (Weight: 25%)

1. **Decommission Public Telemetry Endpoints**:
   Remove or restrict `/api/audit/hashes` behind strict internal administrative authorization guards.

2. **Migrate to Adaptive Salted Hashing (bcrypt / Argon2id)**:
   Replace deprecated MD5 logic with modern salted hashing:
   ```python
   import bcrypt
   # Generate secure hash with unique salt (cost factor 12)
   salt = bcrypt.gensalt(rounds=12)
   hashed = bcrypt.hashpw("admin".encode('utf-8'), salt)
   ```

3. **Verify Precomputation Resistance**:
   Under bcrypt or Argon2id, every newly generated digest for `"admin"` produces an entirely unique string (e.g. `$2b$12$K...`), neutralizing rainbow table indices completely.
