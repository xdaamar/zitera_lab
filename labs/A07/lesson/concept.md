# Technical Analysis: Authentication Vectors, Keyspace Search, & Rate Limiting

Robust authentication security depends upon two foundational variables: **credential entropy (keyspace complexity)** and the **system's architectural ability to throttle automated candidate submissions (*rate limiting & backoff*)**.

---

## 1. Mathematical Keyspace Feasibility Analysis
When an application restricts credentials to a 4-digit numeric PIN (`0000` through `9999`):
- Total combinatorial keyspace:
  $$10^4 = 10,000\text{ permutations}$$
- **Unthrottled Exhaustive Search Duration**:
  If the HTTP server processes an incoming verification request in 10 milliseconds (100 requests/second), the entire search space of 10,000 candidate PINs completes in:
  $$\frac{10,000}{100} = 100\text{ seconds (under 2 minutes!)}$$
- **Context-Aware Targeted Guessing**:
  If reconnaissance reveals that the administrator chose a PIN corresponding to an active release cycle year (range 2020 through 2030), the search space collapses from 10,000 to just **11 discrete candidates**!

---

## 2. Authentication Attack Taxonomies

### A. Brute-Force Attack
Systematically submitting every possible character sequence against a single target identity (such as `admin`). Extremely lethal when passwords are short or rate-limiting guards are absent.

### B. Dictionary Attack
Evaluating curated lists of statistically frequent human passwords (such as `rockyou.txt` containing hundreds of millions of observed leaked passwords: `password`, `123456`, `admin2026`).

### C. Credential Stuffing
Replaying pre-compiled databases of authentic username/password pairs exfiltrated from previous enterprise breaches against secondary portals, capitalizing on pervasive human password reuse.

### D. Password Spraying
The inverse of targeted brute force: testing 1 or 2 highly probable passwords (e.g., `Winter2026!`) across thousands of distinct enterprise user accounts to remain beneath per-account lockout thresholds.

---

## 3. Session Fixation & Lifecycle Management
Beyond credential evaluation, authentication security governs token lifecycles:
- **Session Fixation**: The adversary initializes an anonymous session and forces a victim to authenticate using that predefined session identifier (via URL query parameters or cookie injection). If the server fails to issue a new session token, the adversary inherits the authenticated state.
- **Mandatory Session Token Regeneration**: Secure architectures **MUST** invalidate the pre-authentication session identifier upon successful credential validation, issuing a newly minted cryptographically random session token.

---

## 4. Code Anatomy of Missing Rate Limiting
Examine this unhardened authentication endpoint:
```python
# VULNERABLE IMPLEMENTATION: Zero throttling, zero progressive delay
@app.route("/api/login", methods=["POST"])
def api_login():
    data = request.get_json()
    user = data.get("username")
    pw = data.get("password")

    # CRITICAL DEFECT: Synchronous evaluation without tracking failed attempts
    if user == ADMIN_USER and pw == ADMIN_PIN:
        return jsonify({"status": "passed", "flag": FLAG})

    # Returns 401 in milliseconds without enforcing a cooldown delay
    return jsonify({"status": "failed", "message": "Invalid credentials"}), 401
```
Because the handler lacks stateful attempt tracking (such as in-memory leaky buckets or Redis counters) and returns immediately without a `Retry-After` enforcement throttle, automated scripts can flood the endpoint continuously.
