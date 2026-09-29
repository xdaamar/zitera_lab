from flask import Flask, request, session, redirect, render_template_string, jsonify
import hashlib

app = Flask(__name__)
app.secret_key = "zitera_a04_static_flask_session_key"

USERS_DATABASE = {
    "alice": {
        "id": 1,
        "role": "Analyst",
        "md5_hash": hashlib.md5("password123".encode()).hexdigest(), # 482c811da5d5b4bc6d497ffa98491e38
    },
    "bob": {
        "id": 2,
        "role": "Auditor",
        "md5_hash": hashlib.md5("secret".encode()).hexdigest(),      # 5ebe2294ecd0e0f08eab7690d2a6ee69
    },
    "admin": {
        "id": 99,
        "role": "SecurityOfficer",
        "md5_hash": hashlib.md5("admin".encode()).hexdigest(),       # 21232f297a57a5a743894a0e4a801fc3
    }
}

FLAG = "ZITERA{cryp70_f41lur35_w34k_k3y_2026}"

PAGE_TEMPLATE = """
<!DOCTYPE html>
<html>
<head>
    <title>ZITERA CryptoVault — A04 Cryptographic Failures</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #0f172a; color: #f8fafc; margin: 0; padding: 40px; }
        .card { background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 24px; max-width: 650px; margin: 0 auto; box-shadow: 0 4px 12px rgba(0,0,0,0.3); }
        .badge { display: inline-block; background: #8b5cf6; color: white; padding: 4px 8px; border-radius: 4px; font-size: 11px; font-weight: bold; margin-bottom: 12px; }
        .banner { background: #2e1065; border-left: 4px solid #8b5cf6; padding: 12px; margin-bottom: 16px; font-size: 13px; color: #e9d5ff; }
        input[type=text], input[type=password] { width: 100%; padding: 10px; margin: 8px 0; background: #0f172a; border: 1px solid #475569; color: white; border-radius: 4px; box-sizing: border-box; }
        button { background: #8b5cf6; color: white; border: none; padding: 10px 18px; border-radius: 4px; cursor: pointer; font-weight: bold; }
        button:hover { background: #7c3aed; }
        a { color: #38bdf8; text-decoration: none; }
        a:hover { text-decoration: underline; }
        table { width: 100%; border-collapse: collapse; margin-top: 14px; font-size: 12px; }
        th, td { border: 1px solid #334155; padding: 8px 12px; text-align: left; }
        th { background: #0f172a; color: #8b5cf6; }
        code { background: #334155; padding: 2px 6px; border-radius: 4px; font-family: monospace; font-size: 12px; color: #38bdf8; }
    </style>
</head>
<body>
    <div class="card">
        <span class="badge">OWASP A04:2025 LAB</span>
        <h2>ZITERA CryptoVault Key Management System</h2>
        <div class="banner">
            🔒 <strong>Notice:</strong> This lab demonstrates cryptographic weaknesses: legacy broken hash algorithms (unsalted MD5) and sensitive data exposure.
        </div>
        
        {% if view == 'home' %}
            <p>Welcome to CryptoVault, the enterprise token and credential validation service.</p>
            {% if user %}
                <p>Authenticated Session: <strong>{{ user }}</strong> (Role: {{ role }})</p>
                <p><a href="/vault">Access Master Crypto Vault</a> | <a href="/logout">Logout</a></p>
            {% else %}
                <p>Please authenticate to access cryptographic vault storage:</p>
                <p><a href="/login"><button>Sign In to Vault</button></a></p>
            {% endif %}

            <div style="margin-top: 20px; padding: 12px; background: #0f172a; border-radius: 6px;">
                <p style="margin: 0; font-size: 12px; font-weight: bold; color: #8b5cf6;">Public Cryptographic Inspection Endpoints:</p>
                <ul style="font-size: 12px; color: #cbd5e1; margin-top: 6px;">
                    <li><a href="/health">/health</a> — Service health probe</li>
                    <li><a href="/api/audit/hashes">/api/audit/hashes</a> — Legacy audit log exposing unsalted MD5 password hashes</li>
                    <li><a href="/vault">/vault</a> — Protected Vault storage (Requires Administrator login)</li>
                </ul>
            </div>
        {% elif view == 'login' %}
            <h3>CryptoVault Sign In</h3>
            {% if error %}
                <p style="color: #ef4444; font-size: 13px;">{{ error }}</p>
            {% endif %}
            <form method="POST" action="/login">
                <label style="font-size: 13px;">Username</label>
                <input type="text" name="username" placeholder="e.g. admin" required>
                <label style="font-size: 13px;">Password</label>
                <input type="password" name="password" placeholder="••••••••" required>
                <button type="submit" style="margin-top: 10px;">Authenticate</button>
            </form>
            <p style="font-size: 12px; color: #94a3b8; margin-top: 14px;">
                <em>Hint: Have you checked <code>/api/audit/hashes</code> for password hashes? Modern rainbow tables or hash lookups reverse unsalted MD5 instantly.</em>
            </p>
            <p><a href="/">← Return to Portal</a></p>
        {% elif view == 'vault' %}
            <h3 style="color: #4ade80;">Master Cryptographic Vault</h3>
            <p>Authentication Succeeded: Logged in as <strong>SecurityOfficer (admin)</strong>.</p>
            <div style="background: #0f172a; padding: 18px; border-radius: 6px; border-left: 4px solid #8b5cf6;">
                <p style="margin: 0; font-size: 13px; font-weight: bold; color: #a78bfa;">CLASSIFIED VAULT SECRET ACTIVATION KEY:</p>
                <p style="font-family: monospace; font-size: 16px; color: #38bdf8; font-weight: bold; margin: 10px 0;">
                    {{ flag }}
                </p>
                <p style="margin: 0; font-size: 11px; color: #94a3b8;">
                    Root Cause: The administrator's password ('admin') was stored as an unsalted MD5 hash (<code>21232f297a57a5a743894a0e4a801fc3</code>).
                </p>
            </div>
            <p style="margin-top: 18px;"><a href="/">← Return to Portal</a> | <a href="/logout">Logout</a></p>
        {% endif %}

        <hr style="border: 0; border-top: 1px solid #334155; margin-top: 24px;">
        <p style="font-size: 11px; color: #94a3b8; text-align: center;">
            ZITERA_LAB Runtime Isolation: Local-Only 127.0.0.1:8014
        </p>
    </div>
</body>
</html>
"""

@app.route("/")
def index():
    user = session.get("user")
    role = session.get("role", "guest")
    return render_template_string(PAGE_TEMPLATE, view="home", user=user, role=role)

@app.route("/health")
def health():
    return jsonify({"status": "healthy", "lab": "A04", "port": 8014}), 200

@app.route("/api/audit/hashes")
def audit_hashes():
    audit_data = [
        {"username": "alice", "role": "Analyst", "hash_type": "MD5 (Unsalted)", "hash": USERS_DATABASE["alice"]["md5_hash"]},
        {"username": "bob", "role": "Auditor", "hash_type": "MD5 (Unsalted)", "hash": USERS_DATABASE["bob"]["md5_hash"]},
        {"username": "admin", "role": "SecurityOfficer", "hash_type": "MD5 (Unsalted)", "hash": USERS_DATABASE["admin"]["md5_hash"]},
    ]
    return jsonify({
        "status": "success",
        "description": "Legacy authentication audit table. Warning: MD5 is cryptographically broken and vulnerable to lookup tables / rainbow tables.",
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
            error = "Authentication failed: User does not exist."

    return render_template_string(PAGE_TEMPLATE, view="login", error=error)

@app.route("/vault")
def vault():
    if session.get("user") != "admin":
        return redirect("/login")
    return render_template_string(PAGE_TEMPLATE, view="vault", flag=FLAG)

@app.route("/logout")
def logout():
    session.clear()
    return redirect("/")

@app.route("/reset", methods=["POST"])
def reset():
    session.clear()
    return jsonify({"status": "reset", "message": "A04 environment reset to default seed state."})

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8014, debug=False)
