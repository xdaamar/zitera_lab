from flask import Flask, request, session, redirect, render_template_string, jsonify
import hashlib

app = Flask(__name__)
app.secret_key = "safekey_corporate_vault_production_secret"

USERS_DATABASE = {
    "alice": {
        "id": 1,
        "full_name": "Alice Vance",
        "role": "Cryptographic Analyst",
        "department": "Security Architecture",
        "md5_hash": hashlib.md5("password123".encode()).hexdigest(), # 482c811da5d5b4bc6d497ffa98491e38
    },
    "bob": {
        "id": 2,
        "full_name": "Robert Sterling",
        "role": "HSM Security Auditor",
        "department": "Internal Risk Control",
        "md5_hash": hashlib.md5("secret".encode()).hexdigest(),      # 5ebe2294ecd0e0f08eab7690d2a6ee69
    },
    "admin": {
        "id": 99,
        "full_name": "Marcus Reid",
        "role": "Chief Information Security Officer",
        "department": "Executive Security Governance",
        "md5_hash": hashlib.md5("admin".encode()).hexdigest(),       # 21232f297a57a5a743894a0e4a801fc3
    }
}

FLAG = "ZITERA{cryp70_f41lur35_w34k_k3y_2026}"

PAGE_TEMPLATE = """
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ title }} | SafeKey Corporate Vault</title>
    <style>
        :root {
            --bg-primary: #0a0b14;
            --bg-secondary: #121324;
            --bg-tertiary: #1d1e38;
            --border: #2e3157;
            --accent: #7c3aed;
            --accent-hover: #6d28d9;
            --text-main: #f9fafb;
            --text-muted: #9ca3af;
            --success: #10b981;
            --warning: #f59e0b;
            --danger: #ef4444;
        }
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: var(--bg-primary);
            color: var(--text-main);
            min-height: 100vh;
            display: flex;
            flex-direction: column;
        }
        .navbar {
            background: var(--bg-secondary);
            border-bottom: 1px solid var(--border);
            padding: 0.75rem 2rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        .brand {
            display: flex;
            align-items: center;
            gap: 0.75rem;
            text-decoration: none;
            color: var(--text-main);
            font-weight: 700;
            font-size: 1.15rem;
        }
        .brand-icon {
            width: 32px;
            height: 32px;
            background: linear-gradient(135deg, #7c3aed, #a855f7);
            border-radius: 6px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: 900;
            color: white;
            font-size: 0.95rem;
        }
        .nav-links { display: flex; align-items: center; gap: 1.5rem; }
        .nav-link { color: var(--text-muted); text-decoration: none; font-size: 0.875rem; font-weight: 500; }
        .nav-link:hover { color: var(--text-main); }
        .container { max-width: 1100px; width: 100%; margin: 2rem auto; padding: 0 1.5rem; flex: 1; }
        .breadcrumbs { display: flex; align-items: center; gap: 0.5rem; font-size: 0.825rem; color: var(--text-muted); margin-bottom: 1.5rem; }
        .breadcrumbs a { color: var(--text-muted); text-decoration: none; }
        .breadcrumbs a:hover { color: var(--accent); }
        .card {
            background: var(--bg-secondary);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 1.5rem;
            margin-bottom: 1.5rem;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.3);
        }
        .card-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 1.25rem;
            padding-bottom: 0.75rem;
            border-bottom: 1px solid var(--border);
        }
        .card-title { font-size: 1.125rem; font-weight: 600; }
        .badge {
            display: inline-block;
            padding: 0.25rem 0.6rem;
            border-radius: 9999px;
            font-size: 0.75rem;
            font-weight: 600;
            text-transform: uppercase;
        }
        .badge-success { background: rgba(16, 185, 129, 0.15); color: #34d399; border: 1px solid rgba(16, 185, 129, 0.3); }
        .badge-hsm { background: rgba(124, 58, 237, 0.15); color: #c084fc; border: 1px solid rgba(124, 58, 237, 0.3); }
        .btn {
            display: inline-block;
            padding: 0.5rem 1rem;
            border-radius: 6px;
            font-size: 0.875rem;
            font-weight: 500;
            text-decoration: none;
            cursor: pointer;
            border: 1px solid transparent;
            transition: all 0.15s;
        }
        .btn-primary { background: var(--accent); color: white; }
        .btn-primary:hover { background: var(--accent-hover); }
        .btn-outline { background: transparent; border-color: var(--border); color: var(--text-main); }
        .btn-outline:hover { background: var(--bg-tertiary); }
        input[type=text], input[type=password] {
            width: 100%;
            padding: 0.65rem 0.85rem;
            background: var(--bg-primary);
            border: 1px solid var(--border);
            color: white;
            border-radius: 6px;
            font-size: 0.9rem;
        }
        code { background: var(--bg-tertiary); padding: 2px 6px; border-radius: 4px; font-family: monospace; font-size: 0.825rem; color: #c084fc; }
        .footer {
            background: var(--bg-secondary);
            border-top: 1px solid var(--border);
            padding: 1.5rem 2rem;
            font-size: 0.8rem;
            color: var(--text-muted);
            text-align: center;
        }
    </style>
</head>
<body>
    <header class="navbar">
        <a href="/" class="brand">
            <div class="brand-icon">SK</div>
            <span>SafeKey Corporate Vault</span>
        </a>
        <nav class="nav-links">
            <a href="/" class="nav-link">Cryptographic Status</a>
            <a href="/api/audit/hashes" class="nav-link">Audit Hashes API</a>
            {% if user %}
            <span style="font-size: 0.825rem; color: var(--text-muted);">Operator: <strong>{{ user }}</strong> ({{ role }})</span>
            <a href="/vault" class="nav-link" style="color: #c084fc;">Master Key Vault</a>
            <a href="/logout" class="nav-link" style="color: #f87171;">Sign Out</a>
            {% else %}
            <a href="/login" class="btn btn-primary" style="padding: 0.35rem 0.85rem;">Vault Login</a>
            {% endif %}
        </nav>
    </header>

    <main class="container">
        {% if view == 'home' %}
            <div class="breadcrumbs">
                <a href="/">Home</a>
                <span>/</span>
                <a href="/">Banking Infrastructure</a>
                <span>/</span>
                <span>Hardware Security Module (HSM) Enclave</span>
            </div>

            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 1rem; margin-bottom: 1.5rem;">
                <div class="card" style="margin-bottom: 0;">
                    <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Cryptographic Appliance</div>
                    <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem; color: #c084fc;">SafeKey HSM-8800</div>
                    <div style="font-size: 0.8rem; color: var(--success); margin-top: 0.25rem;">&bull; FIPS 140-2 Level 3 Validated</div>
                </div>
                <div class="card" style="margin-bottom: 0;">
                    <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Key Rotation Lifecycle</div>
                    <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem;">Automated (90 Days)</div>
                    <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.25rem;">Next Rotation in 42 Days</div>
                </div>
                <div class="card" style="margin-bottom: 0;">
                    <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Interbank Clearing TLS</div>
                    <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem; color: var(--success);">Mutual TLS v1.3</div>
                    <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.25rem;">SWIFT Gateway Connected</div>
                </div>
            </div>

            <div class="card">
                <div class="card-header">
                    <div>
                        <h2 class="card-title">HSM Master Key Management &amp; Access Controls</h2>
                        <p style="font-size: 0.825rem; color: var(--text-muted); margin-top: 0.2rem;">
                            Root cryptographic secrets and operational access control policies.
                        </p>
                    </div>
                    <span class="badge badge-hsm">HSM Security Officer Scope</span>
                </div>

                {% if user %}
                <div style="background: var(--bg-tertiary); padding: 1.25rem; border-radius: 6px; margin-bottom: 1.5rem;">
                    <p style="font-size: 0.9rem; margin-bottom: 0.5rem;">
                        Logged in as <strong>{{ user }}</strong> ({{ role }}). Your credential clearance provides direct access to the Master Cryptographic Vault.
                    </p>
                    <a href="/vault" class="btn btn-primary">Open Master Key Vault &rarr;</a>
                </div>
                {% else %}
                <p style="font-size: 0.875rem; color: var(--text-muted); margin-bottom: 1.25rem;">
                    Access to master encryption keys, root certificates, and interbank transaction signing authorities requires authentication with Security Officer clearance.
                </p>
                <div style="display: flex; gap: 1rem;">
                    <a href="/login" class="btn btn-primary">Authenticate to SafeKey Vault</a>
                    <a href="/api/audit/hashes" class="btn btn-outline">Inspect Legacy Hash Audit Export</a>
                </div>
                {% endif %}
            </div>

        {% elif view == 'login' %}
            <div class="breadcrumbs">
                <a href="/">Home</a>
                <span>/</span>
                <span>Security Officer Authentication</span>
            </div>
            <div style="max-width: 440px; margin: 3rem auto;">
                <div class="card">
                    <div class="card-header" style="text-align: center; display: block; border-bottom: none;">
                        <div class="brand-icon" style="margin: 0 auto 1rem; width: 44px; height: 44px; font-size: 1.25rem;">SK</div>
                        <h2 class="card-title">SafeKey Vault Sign In</h2>
                        <p style="font-size: 0.825rem; color: var(--text-muted); margin-top: 0.25rem;">Banking Hardware Security Module</p>
                    </div>

                    {% if error %}
                    <div style="background: rgba(239, 68, 68, 0.15); border: 1px solid var(--danger); color: #fca5a5; padding: 0.75rem; border-radius: 6px; font-size: 0.825rem; margin-bottom: 1rem;">
                        {{ error }}
                    </div>
                    {% endif %}

                    <form method="POST" action="/login">
                        <div style="margin-bottom: 1rem;">
                            <label style="display: block; font-size: 0.825rem; color: var(--text-muted); margin-bottom: 0.35rem;">Operator Account</label>
                            <input type="text" name="username" placeholder="e.g. admin" required>
                        </div>
                        <div style="margin-bottom: 1.25rem;">
                            <label style="display: block; font-size: 0.825rem; color: var(--text-muted); margin-bottom: 0.35rem;">Vault Passphrase</label>
                            <input type="password" name="password" placeholder="••••••••" required>
                        </div>
                        <button type="submit" class="btn btn-primary" style="width: 100%; padding: 0.75rem;">Authenticate to HSM</button>
                    </form>

                    <div style="margin-top: 1.5rem; padding-top: 1rem; border-top: 1px solid var(--border); font-size: 0.8rem; color: var(--text-muted);">
                        Audit compliance export available at <a href="/api/audit/hashes" style="color: #c084fc;">/api/audit/hashes</a>.
                    </div>
                </div>
            </div>

        {% elif view == 'vault' %}
            <div class="breadcrumbs">
                <a href="/">Home</a>
                <span>/</span>
                <a href="/">HSM Enclave</a>
                <span>/</span>
                <span>Master Cryptographic Key Vault</span>
            </div>

            <div class="card">
                <div class="card-header">
                    <div>
                        <h2 class="card-title" style="color: #34d399;">Master Hardware Security Key Enclave</h2>
                        <p style="font-size: 0.825rem; color: var(--text-muted); margin-top: 0.2rem;">
                            Authorized Session: <strong>Marcus Reid</strong> &bull; Chief Information Security Officer
                        </p>
                    </div>
                    <span class="badge badge-success">Clearance: Root Key Holder</span>
                </div>

                <div style="background: var(--bg-tertiary); padding: 1.5rem; border-radius: 6px; border-left: 4px solid var(--accent); margin-bottom: 1.5rem;">
                    <div style="font-size: 0.8rem; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.05em;">Master Banking Key Vault Authorization Seal:</div>
                    <div style="font-family: monospace; font-size: 1.25rem; color: #38bdf8; font-weight: 700; margin: 0.75rem 0;">
                        {{ flag }}
                    </div>
                    <div style="font-size: 0.8rem; color: var(--text-muted);">
                        Cryptographic Hash: <code>21232f297a57a5a743894a0e4a801fc3</code> (Legacy un-salted MD5 implementation).
                    </div>
                </div>

                <div style="display: flex; gap: 1rem;">
                    <a href="/" class="btn btn-outline">&larr; Return to HSM Dashboard</a>
                    <a href="/logout" class="btn btn-outline">Sign Out</a>
                </div>
            </div>
        {% endif %}
    </main>

    <footer class="footer">
        <p>&copy; 2026 SafeKey Technologies Inc. All rights reserved. &bull; Banking Cryptography &amp; Key Governance Appliance</p>
    </footer>
</body>
</html>
"""

@app.route("/")
def index():
    user = session.get("user")
    role = session.get("role", "guest")
    return render_template_string(PAGE_TEMPLATE, title="HSM Enclave", view="home", user=user, role=role)

@app.route("/health")
def health():
    return jsonify({"status": "healthy", "service": "SafeKey Corporate Vault", "port": 8014}), 200

@app.route("/api/audit/hashes")
def audit_hashes():
    audit_data = [
        {"username": "alice", "role": "Cryptographic Analyst", "hash_type": "MD5 (Legacy Migration)", "hash": USERS_DATABASE["alice"]["md5_hash"]},
        {"username": "bob", "role": "HSM Security Auditor", "hash_type": "MD5 (Legacy Migration)", "hash": USERS_DATABASE["bob"]["md5_hash"]},
        {"username": "admin", "role": "Chief Information Security Officer", "hash_type": "MD5 (Legacy Migration)", "hash": USERS_DATABASE["admin"]["md5_hash"]},
    ]
    return jsonify({
        "status": "success",
        "description": "SafeKey HSM credential migration audit export.",
        "records": audit_data
    })

@app.route("/login", methods=["GET", "POST"])
def login():
    error = None
    if request.method == "POST":
        username = request.form.get("username", "").strip()
        password = request.form.get("password", "").strip()

        if username in USERS_DATABASE:
            input_hash = hashlib.md5(password.encode()).hexdigest()
            if input_hash == USERS_DATABASE[username]["md5_hash"]:
                session["user"] = username
                session["role"] = USERS_DATABASE[username]["role"]
                if username == "admin":
                    return redirect("/vault")
                return redirect("/")
            else:
                error = "Authentication failed: Password hash mismatch."
        else:
            error = "Authentication failed: Identity record not found."

    return render_template_string(PAGE_TEMPLATE, title="Vault Sign In", view="login", error=error)

@app.route("/vault")
def vault():
    if session.get("user") != "admin":
        return redirect("/login")
    return render_template_string(PAGE_TEMPLATE, title="Master Vault Key", view="vault", flag=FLAG)

@app.route("/logout")
def logout():
    session.clear()
    return redirect("/")

@app.route("/reset", methods=["POST"])
def reset():
    session.clear()
    return jsonify({"status": "reset", "message": "SafeKey Vault session reset to default."})

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8014, debug=False)
