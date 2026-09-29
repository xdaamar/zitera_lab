from flask import Flask, request, session, redirect, render_template_string, jsonify, send_from_directory, abort
import os

app = Flask(__name__)
app.secret_key = "zitera_a02_insecure_default_secret_key"

BACKUP_DIR = os.path.join(os.path.dirname(__file__), "backups")

PAGE_TEMPLATE = """
<!DOCTYPE html>
<html>
<head>
    <title>ZITERA OpsGateway — A02 Security Misconfiguration</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #0f172a; color: #f8fafc; margin: 0; padding: 40px; }
        .card { background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 24px; max-width: 650px; margin: 0 auto; box-shadow: 0 4px 12px rgba(0,0,0,0.3); }
        .badge { display: inline-block; background: #f59e0b; color: #000; padding: 4px 8px; border-radius: 4px; font-size: 11px; font-weight: bold; margin-bottom: 12px; }
        .banner { background: #451a03; border-left: 4px solid #f59e0b; padding: 12px; margin-bottom: 16px; font-size: 13px; color: #fef3c7; }
        input[type=text], input[type=password] { width: 100%; padding: 10px; margin: 8px 0; background: #0f172a; border: 1px solid #475569; color: white; border-radius: 4px; box-sizing: border-box; }
        button { background: #f59e0b; color: black; border: none; padding: 10px 18px; border-radius: 4px; cursor: pointer; font-weight: bold; }
        button:hover { background: #d97706; }
        a { color: #38bdf8; text-decoration: none; }
        a:hover { text-decoration: underline; }
        ul { padding-left: 20px; }
        li { margin-bottom: 8px; }
        code { background: #334155; padding: 2px 6px; border-radius: 4px; font-family: monospace; font-size: 12px; color: #38bdf8; }
    </style>
</head>
<body>
    <div class="card">
        <span class="badge">OWASP A02:2025 LAB</span>
        <h2>ZITERA OpsGateway Management Console</h2>
        <div class="banner">
            ⚠️ <strong>Notice:</strong> This environment contains intentional security misconfigurations for training.
        </div>
        
        {% if view == 'home' %}
            <p>Welcome to the Internal Operations Gateway Portal.</p>
            {% if user %}
                <p>Logged in as: <strong>{{ user }}</strong> (Role: {{ role }})</p>
                <p><a href="/admin">Go to Administrator Console</a> | <a href="/logout">Logout</a></p>
            {% else %}
                <p>Please authenticate to access maintenance controls:</p>
                <p><a href="/login"><button>Login to Portal</button></a></p>
                <p style="font-size: 12px; color: #94a3b8;">
                    <em>Hint for learners: Many systems in development leave default factory credentials enabled or unlinked administrative diagnostic routes active.</em>
                </p>
            {% endif %}
            <div style="margin-top: 20px; padding: 12px; background: #0f172a; border-radius: 6px;">
                <p style="margin: 0; font-size: 12px; font-weight: bold; color: #f59e0b;">Public Diagnostic Endpoints:</p>
                <ul style="font-size: 12px; color: #cbd5e1; margin-top: 6px;">
                    <li><a href="/health">/health</a> — Service health probe</li>
                    <li><a href="/debug/vars">/debug/vars</a> — Runtime debug variables (unprotected!)</li>
                    <li><a href="/backups/">/backups/</a> — Static configuration directory (directory listing enabled!)</li>
                </ul>
            </div>
        {% elif view == 'login' %}
            <h3>Administrative Login</h3>
            {% if error %}
                <p style="color: #ef4444; font-size: 13px;">{{ error }}</p>
            {% endif %}
            <form method="POST" action="/login">
                <label style="font-size: 13px;">Username</label>
                <input type="text" name="username" placeholder="e.g. admin" required>
                <label style="font-size: 13px;">Password</label>
                <input type="password" name="password" placeholder="••••••••" required>
                <button type="submit" style="margin-top: 10px;">Sign In</button>
            </form>
            <p style="margin-top: 16px;"><a href="/">← Return to Portal</a></p>
        {% elif view == 'admin' %}
            <h3 style="color: #4ade80;">System Administration Console</h3>
            <p>Authentication Succeeded: Logged in as default administrator.</p>
            <div style="background: #0f172a; padding: 16px; border-radius: 6px; border-left: 4px solid #4ade80;">
                <p style="margin: 0; font-weight: bold;">System Diagnostic Summary:</p>
                <p style="font-size: 13px; color: #94a3b8;">
                    A recent automated backup was saved to <code>/backups/backup_config.json.bak</code>.
                </p>
                <p style="font-size: 13px; color: #94a3b8;">
                    Inspect the backup archive to retrieve the master configuration activation flag.
                </p>
            </div>
            <p style="margin-top: 16px;"><a href="/">← Return to Portal</a> | <a href="/logout">Logout</a></p>
        {% elif view == 'backups' %}
            <h3>Index of /backups/</h3>
            <p style="font-size: 12px; color: #94a3b8;">Directory browsing is enabled by default on this web server instance.</p>
            <ul>
                {% for file in files %}
                    <li><a href="/backups/{{ file }}"><code>{{ file }}</code></a></li>
                {% endfor %}
            </ul>
            <p><a href="/">← Return Home</a></p>
        {% endif %}

        <hr style="border: 0; border-top: 1px solid #334155; margin-top: 24px;">
        <p style="font-size: 11px; color: #94a3b8; text-align: center;">
            ZITERA_LAB Runtime Isolation: Local-Only 127.0.0.1:8012
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
    return jsonify({"status": "healthy", "lab": "A02", "port": 8012}), 200

@app.route("/debug/vars")
def debug_vars():
    return jsonify({
        "app_name": "ZITERA_OpsGateway",
        "version": "1.0.0",
        "debug_mode": True,
        "default_credentials_enabled": True,
        "backup_storage_path": "/app/backups/",
        "server_banner": "Werkzeug/3.1.8 Python/3.11.16",
        "exposed_routes": ["/", "/health", "/login", "/admin", "/debug/vars", "/backups/"]
    })

@app.route("/backups/")
@app.route("/backups")
def list_backups():
    files = []
    if os.path.exists(BACKUP_DIR):
        files = os.listdir(BACKUP_DIR)
    return render_template_string(PAGE_TEMPLATE, view="backups", files=files)

@app.route("/backups/<path:filename>")
def download_backup(filename):
    if not os.path.exists(os.path.join(BACKUP_DIR, filename)):
        abort(404)
    return send_from_directory(BACKUP_DIR, filename, as_attachment=False)

@app.route("/login", methods=["GET", "POST"])
def login():
    error = None
    if request.method == "POST":
        username = request.form.get("username", "").strip()
        password = request.form.get("password", "").strip()
        
        if username == "admin" and password == "admin":
            session["user"] = "admin"
            session["role"] = "Administrator"
            return redirect("/admin")
        elif username == "operator" and password == "password123":
            session["user"] = "operator"
            session["role"] = "Operator"
            return redirect("/")
        else:
            error = "Invalid credentials. (Hint: Did you check for default administrator credentials?)"

    return render_template_string(PAGE_TEMPLATE, view="login", error=error)

@app.route("/admin")
def admin():
    if session.get("user") != "admin":
        return redirect("/login")
    return render_template_string(PAGE_TEMPLATE, view="admin")

@app.route("/logout")
def logout():
    session.clear()
    return redirect("/")

@app.route("/reset", methods=["POST"])
def reset():
    session.clear()
    return jsonify({"status": "reset", "message": "A02 environment reset to default seed state."})

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8012, debug=False)
