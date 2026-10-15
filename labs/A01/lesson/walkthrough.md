# Practice Walkthrough: Investigating & Proving IDOR

In this guided lab exercise, you will conduct a systematic security audit against the internal billing portal running at `http://127.0.0.1:8011`. This exercise follows a target composition of **60% manual analytical techniques** and **40% terminal-based tool execution**.

---

### Stage 1: Observation & System Mapping (15% â€” Manual)
The first step of an authorized assessment is establishing a baseline of normal application behavior.

1. Open your web browser and navigate to the target billing portal:
   `http://127.0.0.1:8011`
2. Authenticate using the standard test account:
   - **Username:** `alice`
   - **Password:** `password123`
3. Click on the **"My Invoices"** section. Notice that the web UI exclusively renders Alice's legitimate invoices.
4. Open your browser Developer Tools (press **F12** or `Ctrl + Shift + I`) and switch to the **Network** tab.
5. Click on Alice's invoice and inspect the raw HTTP request details:
   - **Request URL:** `http://127.0.0.1:8011/invoice/1`
   - **Method:** `GET`
   - **Headers:** Notice the session cookie `session_id=...` that identifies your session as Alice.

---

### Stage 2: Hypothesis Testing & Fault Injection (25% â€” Manual)
Now test the core hypothesis: *Does the server validate object ownership against Alice's authenticated session?*

1. In the browser address bar, locate the current URL.
2. Alter the path parameter from:
   `http://127.0.0.1:8011/invoice/1`
   to:
   `http://127.0.0.1:8011/invoice/2`
3. Press **Enter** and analyze the resulting output:
   - **Observation:** The page renders successfully with an HTTP `200 OK` status.
   - **Evidence Harvested:** The invoice displayed on your screen belongs to another user: **Bob**!
   - **Critical Analysis:** The request carried Alice's valid session cookie, but the backend returned Bob's private financial invoice without enforcing authorization. This confirms an active IDOR vulnerability.

---

### Stage 3: Controlled Exploitation & Weaponization (35% â€” Tools: cURL)
Leverage cURL from the terminal to evaluate the full scope of data exposure and discover high-value assets (*master invoice*).

1. Open a terminal inside the ZITERA_LAB workspace.
2. Extract Alice's session cookie from the browser Network tab.
3. Craft an explicit cURL command to inspect unauthorized records:
   ```bash
   curl -s -i "http://127.0.0.1:8011/invoice/2" -H "Cookie: session_id=YOUR_ALICE_SESSION"
   ```
4. Perform systematic inspection against reserved or administrative identifiers (such as `id=0`, `id=3`, or admin records):
   ```bash
   # Probe initial indices or special administrative records
   curl -s "http://127.0.0.1:8011/invoice/0" -H "Cookie: session_id=YOUR_ALICE_SESSION"
   ```
5. Observe how the response body leaks the hidden master billing record, which is never linked in standard navigation menus.

---

### Stage 4: Remediation & Verification (25% â€” Manual & Tools)
The final step is verifying that defensive remediations succeed deterministically.

1. Enforce server-side ownership checks on the backend controller:
   ```python
   # Verify that invoice.owner_id == session['user_id']
   ```
2. Re-execute the cURL probe against invoice #2 using Alice's credentials:
   ```bash
   curl -s -i "http://127.0.0.1:8011/invoice/2" -H "Cookie: session_id=YOUR_ALICE_SESSION"
   ```
3. **Success Criteria:** The server must return an HTTP **`403 Forbidden`** or **`404 Not Found`** status, conclusively proving that unauthorized cross-user access is permanently blocked.
