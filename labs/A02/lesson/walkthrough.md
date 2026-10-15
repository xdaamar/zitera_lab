# Practice Walkthrough: Auditing Security Misconfigurations

In this guided exercise, you will audit an internal web portal running at `http://127.0.0.1:8012`. This exercise follows a target composition of **60% manual analytical techniques** and **40% terminal-based tool execution**.

---

### Stage 1: Observation & Response Header Inspection (15% â€” Manual)
The initial stage of a configuration audit involves scrutinizing server banner disclosures and evaluating missing defensive headers.

1. Open your web browser and navigate to the target portal:
   `http://127.0.0.1:8012`
2. Open Developer Tools (press **F12** or `Ctrl + Shift + I`), then select the **Network** tab.
3. Reload the page and select the root document request (`/`):
   - **Inspect Response Headers:**
     - Does the server leak underlying software versions (e.g. `Server: Werkzeug/...` or `Server: nginx/...`)?
     - Are critical browser defense headers such as `Content-Security-Policy`, `X-Frame-Options`, and `X-Content-Type-Options` missing?
4. Inspect the administrative authentication portal:
   - Test standard vendor credentials such as `admin` / `admin`. Observe whether default credentials remain enabled or blocked.

---

### Stage 2: Hypothesis Testing & Route Discovery (25% â€” Manual)
Test the core hypothesis: *Did the system administrator leave maintenance or static backup folders publicly accessible?*

1. System administrators frequently park backup archives under common naming conventions such as `/backup/`, `/backups/`, `/data/`, or `/debug/`.
2. In the browser address bar, enter the path:
   `http://127.0.0.1:8012/backups/`
3. Press **Enter** and evaluate the response:
   - **Observation:** The server does not reject the request with `403 Forbidden` or `404 Not Found`; instead, it serves an active directory listing titled **"Index of /backups/"**.
   - **Evidence Harvested:** An abandoned configuration archive is visible:
     `backup_config.json.bak`
   - **Critical Analysis:** The web server's *autoindex* feature was left active, allowing unauthenticated external visitors to browse and download server-side backup files.

---

### Stage 3: Controlled Extraction & Weaponization (35% â€” Tools: cURL)
Utilize terminal tooling to fetch the exposed backup archive and parse confidential secrets contained within the payload.

1. Open a terminal inside the ZITERA_LAB workspace.
2. Download the exposed backup archive directly using cURL:
   ```bash
   curl -s "http://127.0.0.1:8012/backups/backup_config.json.bak" -o backup_config.json.bak
   ```
3. Inspect the contents of the downloaded configuration backup:
   ```bash
   # Display the raw JSON backup payload
   cat backup_config.json.bak
   ```
4. **Payload Analysis:** Locate sensitive operational parameters such as `admin_system_key`, internal database connection strings, and system activation flags disclosed within the unencrypted text document.

---

### Stage 4: Remediation & Verification (25% â€” Manual & Tools)
Apply system hardening controls and verify that the vulnerability is closed deterministically.

1. Disable directory browsing in the web server configuration:
   - Ensure the `/backups/` route enforces `autoindex off;` and returns an HTTP `404` or `403`.
   - Relocate all backup archives to encrypted storage repositories residing strictly outside the public web root.
2. Re-test the endpoint using cURL from your terminal:
   ```bash
   curl -s -i "http://127.0.0.1:8012/backups/"
   ```
3. **Success Criteria:** The server must refuse directory listings and return an HTTP **`403 Forbidden`** or **`404 Not Found`** status.
