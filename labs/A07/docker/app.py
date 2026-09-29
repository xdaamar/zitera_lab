from flask import Flask, jsonify, render_template_string, request, session, redirect, url_for
import os

app = Flask(__name__)
app.secret_key = "zitera-insecure-lab-session-key"

FLAG = "ZITERA{4uth_f41lur3_brut3_f0rc3_2026}"
ADMIN_USER = "admin"
ADMIN_PIN = "2026"

PAGE_TEMPLATE = """
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>OmniAuth Control Portal</title>
    <style>
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background: #090d16; color: #f1f5f9; margin: 0; padding: 24px; }
        .container { max-width: 600px; margin: 40px auto; background: #131b2e; border: 1px solid #1e293b; border-radius: 10px; padding: 28px; box-shadow: 0 10px 25px rgba(0,0,0,0.5); }
        .header { border-bottom: 1px solid #1e293b; padding-bottom: 16px; margin-bottom: 20px; display: flex; justify-content: space-between; align-items: center; }
        .badge { background: #7c3aed; color: #ede9fe; padding: 4px 10px; border-radius: 4px; font-size: 12px; font-weight: bold; }
        .banner { background: #311042; border-left: 4px solid #a855f7; padding: 12px; margin-bottom: 20px; font-size: 13px; color: #f3e8ff; }
        .field { margin-bottom: 16px; }
        label { display: block; font-size: 13px; color: #94a3b8; margin-bottom: 6px; }
        input[type="text"], input[type="password"] { width: 100%; box-sizing: border-box; padding: 10px; background: #090d16; border: 1px solid #334155; color: white; border-radius: 6px; }
        button { width: 100%; padding: 12px; background: #7c3aed; color: white; border: none; border-radius: 6px; font-weight: bold; cursor: pointer; }
        button:hover { background: #6d28d9; }
        .error { color: #f87171; font-size: 13px; margin-bottom: 12px; }
        .flag-box { background: #0f172a; border: 2px dashed #4ade80; border-radius: 8px; padding: 16px; margin-top: 16px; color: #4ade80; font-family: monospace; font-size: 15px; font-weight: bold; text-align: center; }
        .hint { font-size: 12px; color: #64748b; margin-top: 12px; }
    </style>
</head>
<body>
<div class="container">
    <div class="header">
        <div>
            <h2 style="margin: 0;">OmniAuth // Admin Gateway</h2>
            <div style="color: #94a3b8; font-size: 12px; margin-top: 4px;">OWASP A07:2025 • Authentication Failures</div>
        </div>
        <span class="badge">LAB A07 • PORT 8017</span>
    </div>

    {% if view == 'login' %}
    <div class="banner">
        <strong>SECURITY AUDIT NOTICE:</strong> Notice the absence of rate-limiting, CAPTCHA, or account lockout. Brute-force attacks against weak credentials can succeed rapidly.
    </div>

    {% if error %}
    <div class="error">⚠ {{ error }}</div>
    {% endif %}

    <form method="POST" action="/login">
        <div class="field">
            <label for="username">Administrative Username</label>
            <input type="text" id="username" name="username" value="admin" required>
        </div>
        <div class="field">
            <label for="password">Security PIN / Password (4-Digit Year)</label>
            <input type="password" id="password" name="password" placeholder="Enter PIN..." required autofocus>
        </div>
        <button type="submit">Authenticate</button>
    </form>
    <div class="hint">
        Tip: The administrator configured their 4-digit PIN using the current release cycle year.
    </div>
    {% elif view == 'admin' %}
    <div style="color: #4ade80; font-weight: bold; margin-bottom: 10px;">✓ Authentication Succeeded! Welcome Administrator.</div>
    <p style="color: #94a3b8; font-size: 14px;">
        You have successfully bypassed the authentication barrier due to weak credential requirements and zero brute-force protection.
    </p>

    <div class="flag-box">
        {{ flag }}
    </div>

    <div style="margin-top: 24px;">
        <a href="/logout" style="color: #a855f7; text-decoration: none; font-size: 13px;">← Log Out</a>
    </div>
    {% endif %}
</div>
</body>
</html>
"""

@app.route("/health", methods=["GET"])
def health():
    return jsonify({"status": "ok", "lab": "A07", "port": 8017})

@app.route("/", methods=["GET"])
def index():
    if session.get("authenticated"):
        return redirect(url_for("admin"))
    return redirect(url_for("login"))

@app.route("/login", methods=["GET", "POST"])
def login():
    if request.method == "POST":
        user = request.form.get("username", "").strip()
        pw = request.form.get("password", "").strip()
        if user == ADMIN_USER and pw == ADMIN_PIN:
            session["authenticated"] = True
            return redirect(url_for("admin"))
        return render_template_string(PAGE_TEMPLATE, view="login", error="Invalid credentials. (Attempt logged, zero rate-limit applied)")
    return render_template_string(PAGE_TEMPLATE, view="login", error=None)

@app.route("/admin", methods=["GET"])
def admin():
    if not session.get("authenticated"):
        return redirect(url_for("login"))
    return render_template_string(PAGE_TEMPLATE, view="admin", flag=FLAG)

@app.route("/logout", methods=["GET"])
def logout():
    session.clear()
    return redirect(url_for("login"))

@app.route("/api/login", methods=["POST"])
def api_login():
    data = request.get_json(silent=True) or request.form
    user = (data.get("username") or "").strip()
    pw = (data.get("password") or "").strip()
    if user == ADMIN_USER and pw == ADMIN_PIN:
        return jsonify({"status": "passed", "message": "Authentication successful!", "flag": FLAG})
    return jsonify({"status": "failed", "message": "Invalid credentials"}), 401

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8080)
