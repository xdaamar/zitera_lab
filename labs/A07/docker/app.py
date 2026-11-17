from flask import Flask, jsonify, render_template_string, request, session, redirect, url_for
import os

app = Flask(__name__)
app.secret_key = "nexhealth_portal_clinical_session_secret"

FLAG = "ZITERA{4uth_f41lur3_brut3_f0rc3_2026}"
ADMIN_USER = "admin"
ADMIN_PIN = "2026"

PAGE_TEMPLATE = """
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ title }} | NexHealth Hospital Clinical Portal</title>
    <style>
        :root {
            --bg-primary: #070d19;
            --bg-secondary: #0f1a2e;
            --bg-tertiary: #182844;
            --border: #23385d;
            --accent: #0284c7;
            --accent-hover: #0369a1;
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
            background: linear-gradient(135deg, #0ea5e9, #0284c7);
            border-radius: 6px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: 900;
            color: white;
            font-size: 1.1rem;
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
        .badge-clinical { background: rgba(14, 165, 233, 0.15); color: #38bdf8; border: 1px solid rgba(14, 165, 233, 0.3); }
        table { width: 100%; border-collapse: collapse; font-size: 0.875rem; }
        th { text-align: left; padding: 0.75rem 1rem; background: var(--bg-tertiary); color: var(--text-muted); font-weight: 600; border-bottom: 1px solid var(--border); }
        td { padding: 0.875rem 1rem; border-bottom: 1px solid var(--border); }
        tr:hover td { background-color: rgba(255, 255, 255, 0.02); }
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
        code { background: var(--bg-tertiary); padding: 2px 6px; border-radius: 4px; font-family: monospace; font-size: 0.825rem; color: #38bdf8; }
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
            <div class="brand-icon">+</div>
            <span>NexHealth Portal</span>
        </a>
        <nav class="nav-links">
            <a href="/" class="nav-link">Clinical EMR</a>
            <a href="/" class="nav-link">Operating Theater</a>
            <span style="font-size: 0.825rem; color: var(--text-muted);">Regional Medical Center #04</span>
            {% if session.get('authenticated') %}
            <a href="/logout" class="nav-link" style="color: #f87171;">Sign Out</a>
            {% else %}
            <a href="/login" class="btn btn-primary" style="padding: 0.35rem 0.85rem;">Clinical Sign In</a>
            {% endif %}
        </nav>
    </header>

    <main class="container">
        {% if view == 'login' %}
            <div class="breadcrumbs">
                <a href="/">Home</a>
                <span>/</span>
                <span>Clinical Practitioner Sign In</span>
            </div>

            <div style="max-width: 440px; margin: 3rem auto;">
                <div class="card">
                    <div class="card-header" style="text-align: center; display: block; border-bottom: none;">
                        <div class="brand-icon" style="margin: 0 auto 1rem; width: 44px; height: 44px; font-size: 1.5rem;">+</div>
                        <h2 class="card-title">NexHealth Staff EMR Gateway</h2>
                        <p style="font-size: 0.825rem; color: var(--text-muted); margin-top: 0.25rem;">Electronic Health Records &bull; Authorized Physicians Only</p>
                    </div>

                    {% if error %}
                    <div style="background: rgba(239, 68, 68, 0.15); border: 1px solid var(--danger); color: #fca5a5; padding: 0.75rem; border-radius: 6px; font-size: 0.825rem; margin-bottom: 1rem;">
                        {{ error }}
                    </div>
                    {% endif %}

                    <form method="POST" action="/login">
                        <div style="margin-bottom: 1rem;">
                            <label style="display: block; font-size: 0.825rem; color: var(--text-muted); margin-bottom: 0.35rem;">Staff Identifier / Username</label>
                            <input type="text" name="username" value="admin" required>
                        </div>
                        <div style="margin-bottom: 1.25rem;">
                            <label style="display: block; font-size: 0.825rem; color: var(--text-muted); margin-bottom: 0.35rem;">Security PIN / Password (4-Digit)</label>
                            <input type="password" name="password" placeholder="••••" required autofocus>
                        </div>
                        <button type="submit" class="btn btn-primary" style="width: 100%; padding: 0.75rem;">Access Clinical Systems</button>
                    </form>

                    <div style="margin-top: 1.5rem; padding-top: 1rem; border-top: 1px solid var(--border); font-size: 0.775rem; color: var(--text-muted); line-height: 1.4;">
                        <strong>Regulatory Notice:</strong> Unlawful access to electronic protected health information (ePHI) is strictly audited under HIPAA &amp; HITECH Act statutory guidelines.
                    </div>
                </div>
            </div>

        {% elif view == 'admin' %}
            <div class="breadcrumbs">
                <a href="/">Home</a>
                <span>/</span>
                <a href="/admin">Clinical Systems</a>
                <span>/</span>
                <span>Inpatient Triage &amp; Chief Medical Officer Ledger</span>
            </div>

            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 1rem; margin-bottom: 1.5rem;">
                <div class="card" style="margin-bottom: 0;">
                    <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Inpatient Census</div>
                    <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem; color: var(--success);">142 Beds Occupied</div>
                    <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.25rem;">88% Facility Capacity</div>
                </div>
                <div class="card" style="margin-bottom: 0;">
                    <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Active Physician Authority</div>
                    <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem; color: #38bdf8;">Chief Medical Officer</div>
                    <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.25rem;">Session: Superuser Access</div>
                </div>
                <div class="card" style="margin-bottom: 0;">
                    <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Audit Logging Policy</div>
                    <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem; color: var(--warning);">Rate Limiter Inactive</div>
                    <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.25rem;">Clinical Ward Port 8017</div>
                </div>
            </div>

            <div class="card">
                <div class="card-header">
                    <div>
                        <h2 class="card-title">Confidential Patient Medical Records (EMR)</h2>
                        <p style="font-size: 0.825rem; color: var(--text-muted); margin-top: 0.2rem;">
                            Live triage chart notes and clinical practitioner assignments.
                        </p>
                    </div>
                    <span class="badge badge-clinical">HIPAA Protected PHI</span>
                </div>

                <table>
                    <thead>
                        <tr>
                            <th>Medical Record #</th>
                            <th>Patient Full Name</th>
                            <th>Clinical Ward</th>
                            <th>Primary Diagnosis</th>
                            <th>Attending Physician</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><code>MRN-449102</code></td>
                            <td><strong>Sarah Connor</strong></td>
                            <td>ICU-3A</td>
                            <td>Acute Cardiac Telemetry Stabilization</td>
                            <td>Dr. Marcus Vance, MD</td>
                        </tr>
                        <tr>
                            <td><code>MRN-449103</code></td>
                            <td><strong>Jonathan Harker</strong></td>
                            <td>SURG-1B</td>
                            <td>Post-Operative Orthopedic Rehabilitation</td>
                            <td>Dr. Robert Sterling, MD</td>
                        </tr>
                    </tbody>
                </table>
            </div>

            <div class="card">
                <div class="card-header">
                    <h3 class="card-title">Chief Medical Officer Executive Telemetry Seal</h3>
                </div>
                <div style="background: var(--bg-tertiary); padding: 1.25rem; border-radius: 6px; border-left: 4px solid #38bdf8;">
                    <div style="font-size: 0.8rem; color: var(--text-muted); text-transform: uppercase;">HIPAA System Authorization Validation Token:</div>
                    <div style="font-family: monospace; font-size: 1.2rem; color: #34d399; font-weight: 700; margin: 0.75rem 0;">
                        {{ flag }}
                    </div>
                    <p style="font-size: 0.8rem; color: var(--text-muted); margin: 0;">
                        Account authentication verified without rate-limit delay or lockout counter.
                    </p>
                </div>
                <div style="margin-top: 1.5rem;">
                    <a href="/logout" class="btn btn-outline">&larr; Sign Out of Clinical Session</a>
                </div>
            </div>
        {% endif %}
    </main>

    <footer class="footer">
        <p>&copy; 2026 NexHealth Systems Inc. All rights reserved. &bull; Clinical Healthcare Informatics &amp; Electronic Medical Records</p>
    </footer>
</body>
</html>
"""

@app.route("/health", methods=["GET"])
def health():
    return jsonify({"status": "ok", "service": "NexHealth Portal", "port": 8017})

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
        return render_template_string(PAGE_TEMPLATE, title="Clinical Sign In", view="login", error="Invalid physician credentials. PIN verification failed.")
    return render_template_string(PAGE_TEMPLATE, title="Clinical Sign In", view="login", error=None)

@app.route("/admin", methods=["GET"])
def admin():
    if not session.get("authenticated"):
        return redirect(url_for("login"))
    return render_template_string(PAGE_TEMPLATE, title="Clinical Systems", view="admin", flag=FLAG)

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
        return jsonify({"status": "passed", "message": "Clinical authentication successful!", "flag": FLAG})
    return jsonify({"status": "failed", "message": "Invalid credentials"}), 401

if __name__ == "__main__":
    port = int(os.environ.get("PORT", 8080))
    app.run(host="0.0.0.0", port=port)
