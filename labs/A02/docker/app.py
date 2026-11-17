from flask import Flask, request, session, redirect, render_template_string, jsonify, send_from_directory, abort
import os

app = Flask(__name__)
app.secret_key = "cloudcore_opsgateway_session_key_production"

BACKUP_DIR = os.path.join(os.path.dirname(__file__), "backups")

PAGE_TEMPLATE = """
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ title }} | CloudCore OpsGateway</title>
    <style>
        :root {
            --bg-primary: #0b0f19;
            --bg-secondary: #111827;
            --bg-tertiary: #1f2937;
            --border: #374151;
            --accent: #3b82f6;
            --accent-hover: #2563eb;
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
            background: linear-gradient(135deg, #0ea5e9, #3b82f6);
            border-radius: 6px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: 900;
            color: white;
            font-size: 0.95rem;
        }
        .nav-links { display: flex; align-items: center; gap: 1.5rem; }
        .nav-link { color: var(--text-muted); text-decoration: none; font-size: 0.875rem; font-weight: 500; transition: color 0.15s; }
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
        .badge-warning { background: rgba(245, 158, 11, 0.15); color: #fbbf24; border: 1px solid rgba(245, 158, 11, 0.3); }
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
            <div class="brand-icon">CC</div>
            <span>CloudCore OpsGateway</span>
        </a>
        <nav class="nav-links">
            <a href="/" class="nav-link">Cluster Nodes</a>
            <a href="/backups/" class="nav-link">Snapshot Storage</a>
            <a href="/debug/vars" class="nav-link">Diagnostic Telemetry</a>
            {% if user %}
            <span style="font-size: 0.825rem; color: var(--text-muted);">Operator: <strong>{{ user }}</strong> ({{ role }})</span>
            <a href="/admin" class="nav-link" style="color: #38bdf8;">Cluster Console</a>
            <a href="/logout" class="nav-link" style="color: #f87171;">Sign Out</a>
            {% else %}
            <a href="/login" class="btn btn-primary" style="padding: 0.35rem 0.85rem;">Admin Sign In</a>
            {% endif %}
        </nav>
    </header>

    <main class="container">
        {% if view == 'home' %}
            <div class="breadcrumbs">
                <a href="/">Home</a>
                <span>/</span>
                <a href="/">Cluster Operations</a>
                <span>/</span>
                <span>Active Infrastructure Mesh</span>
            </div>

            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 1rem; margin-bottom: 1.5rem;">
                <div class="card" style="margin-bottom: 0;">
                    <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Kubernetes Node State</div>
                    <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem; color: var(--success);">All Systems Nominal</div>
                    <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.25rem;">12 Nodes / 48 Pods Active</div>
                </div>
                <div class="card" style="margin-bottom: 0;">
                    <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Cluster Ingress Gateway</div>
                    <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem;">OpsGateway v2.4.0</div>
                    <div style="font-size: 0.8rem; color: #60a5fa; margin-top: 0.25rem;">Port 8012 &bull; Direct Proxy</div>
                </div>
                <div class="card" style="margin-bottom: 0;">
                    <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Automated Snapshots</div>
                    <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem;">Archive Bucket</div>
                    <div style="font-size: 0.8rem; color: var(--warning); margin-top: 0.25rem;">Directory Index Exposed</div>
                </div>
            </div>

            <div class="card">
                <div class="card-header">
                    <div>
                        <h2 class="card-title">Microservices Mesh Health & Metrics</h2>
                        <p style="font-size: 0.825rem; color: var(--text-muted); margin-top: 0.2rem;">Real-time service telemetry synchronized across edge regions</p>
                    </div>
                    <span class="badge badge-success">Mesh Sync: Active</span>
                </div>
                <table>
                    <thead>
                        <tr>
                            <th>Service Identifier</th>
                            <th>Namespace</th>
                            <th>Replicas</th>
                            <th>CPU / Memory</th>
                            <th>Status</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td><code>cloudcore-auth-proxy</code></td>
                            <td>kube-system</td>
                            <td>3 / 3</td>
                            <td>42m / 184Mi</td>
                            <td><span class="badge badge-success">Running</span></td>
                        </tr>
                        <tr>
                            <td><code>cloudcore-telemetry-collector</code></td>
                            <td>observability</td>
                            <td>2 / 2</td>
                            <td>98m / 312Mi</td>
                            <td><span class="badge badge-success">Running</span></td>
                        </tr>
                        <tr>
                            <td><code>cloudcore-backup-manager</code></td>
                            <td>storage</td>
                            <td>1 / 1</td>
                            <td>12m / 64Mi</td>
                            <td><span class="badge badge-warning">Snapshot Ready</span></td>
                        </tr>
                        <tr>
                            <td><code>cloudcore-billing-engine</code></td>
                            <td>production</td>
                            <td>4 / 4</td>
                            <td>120m / 512Mi</td>
                            <td><span class="badge badge-success">Running</span></td>
                        </tr>
                    </tbody>
                </table>
            </div>

            <div class="card">
                <div class="card-header">
                    <h3 class="card-title">OpsGateway Internal System Resources</h3>
                </div>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 1rem;">
                    <div style="background: var(--bg-tertiary); padding: 1rem; border-radius: 6px;">
                        <h4 style="font-size: 0.9rem; margin-bottom: 0.35rem;">Runtime Diagnostics</h4>
                        <p style="font-size: 0.8rem; color: var(--text-muted); margin-bottom: 0.75rem;">Internal debug variables and server environment inspection.</p>
                        <a href="/debug/vars" class="btn btn-outline" style="font-size: 0.8rem;">Inspect /debug/vars &rarr;</a>
                    </div>
                    <div style="background: var(--bg-tertiary); padding: 1rem; border-radius: 6px;">
                        <h4 style="font-size: 0.9rem; margin-bottom: 0.35rem;">Snapshot Archive Storage</h4>
                        <p style="font-size: 0.8rem; color: var(--text-muted); margin-bottom: 0.75rem;">Automated disaster recovery storage bucket for master configuration files.</p>
                        <a href="/backups/" class="btn btn-outline" style="font-size: 0.8rem;">Browse /backups/ &rarr;</a>
                    </div>
                </div>
            </div>

        {% elif view == 'login' %}
            <div class="breadcrumbs">
                <a href="/">Home</a>
                <span>/</span>
                <span>Administrator Authentication</span>
            </div>
            <div style="max-width: 440px; margin: 3rem auto;">
                <div class="card">
                    <div class="card-header" style="text-align: center; display: block; border-bottom: none;">
                        <div class="brand-icon" style="margin: 0 auto 1rem; width: 44px; height: 44px; font-size: 1.25rem;">CC</div>
                        <h2 class="card-title">OpsGateway Cluster Login</h2>
                        <p style="font-size: 0.825rem; color: var(--text-muted); margin-top: 0.25rem;">Administrative Access Console</p>
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
                            <label style="display: block; font-size: 0.825rem; color: var(--text-muted); margin-bottom: 0.35rem;">Console Password</label>
                            <input type="password" name="password" placeholder="••••••••" required>
                        </div>
                        <button type="submit" class="btn btn-primary" style="width: 100%; padding: 0.75rem;">Sign In to Console</button>
                    </form>
                </div>
            </div>

        {% elif view == 'admin' %}
            <div class="breadcrumbs">
                <a href="/">Home</a>
                <span>/</span>
                <a href="/admin">Cluster Operations</a>
                <span>/</span>
                <span>Root Administration Console</span>
            </div>
            <div class="card">
                <div class="card-header">
                    <div>
                        <h2 class="card-title" style="color: #34d399;">Root Cluster Administration Console</h2>
                        <p style="font-size: 0.825rem; color: var(--text-muted); margin-top: 0.2rem;">Supervisory Authority &bull; Session Authenticated as <code>admin</code></p>
                    </div>
                    <span class="badge badge-success">Root Privilege: Granted</span>
                </div>
                <div style="background: var(--bg-tertiary); padding: 1.25rem; border-radius: 6px; margin-bottom: 1.5rem;">
                    <h4 style="font-size: 0.95rem; margin-bottom: 0.5rem;">Disaster Recovery Snapshot Summary</h4>
                    <p style="font-size: 0.85rem; color: var(--text-muted); line-height: 1.5;">
                        Recent automated infrastructure backups are archived in <code>/backups/backup_config.json.bak</code>. 
                        Inspect the configuration archive to view authoritative system parameters and root activation keys.
                    </p>
                </div>
                <div style="display: flex; gap: 1rem;">
                    <a href="/backups/backup_config.json.bak" class="btn btn-primary">Download Backup Archive (.bak)</a>
                    <a href="/logout" class="btn btn-outline">Sign Out</a>
                </div>
            </div>

        {% elif view == 'backups' %}
            <div class="breadcrumbs">
                <a href="/">Home</a>
                <span>/</span>
                <span>Snapshot Bucket Index</span>
            </div>
            <div class="card">
                <div class="card-header">
                    <div>
                        <h2 class="card-title">CloudCore Snapshot Storage / Index of /backups/</h2>
                        <p style="font-size: 0.825rem; color: var(--text-muted); margin-top: 0.2rem;">Automated storage volume &bull; Web server directory browsing enabled</p>
                    </div>
                    <span class="badge badge-warning">Directory Listing Public</span>
                </div>
                <table>
                    <thead>
                        <tr>
                            <th>File Name</th>
                            <th>Resource Type</th>
                            <th>Action</th>
                        </tr>
                    </thead>
                    <tbody>
                        {% for file in files %}
                        <tr>
                            <td><code>{{ file }}</code></td>
                            <td style="color: var(--text-muted);">Configuration Backup Archive (JSON)</td>
                            <td><a href="/backups/{{ file }}" class="btn btn-outline" style="padding: 0.3rem 0.75rem; font-size: 0.8rem;">Download Archive &rarr;</a></td>
                        </tr>
                        {% endfor %}
                    </tbody>
                </table>
                <div style="margin-top: 1.5rem;">
                    <a href="/" class="btn btn-outline">&larr; Return to OpsGateway Dashboard</a>
                </div>
            </div>
        {% endif %}
    </main>

    <footer class="footer">
        <p>&copy; 2026 CloudCore Technologies Inc. All rights reserved. &bull; Enterprise Cloud Orchestration Infrastructure</p>
    </footer>
</body>
</html>
"""

@app.route("/")
def index():
    user = session.get("user")
    role = session.get("role", "guest")
    return render_template_string(PAGE_TEMPLATE, title="Cluster Operations", view="home", user=user, role=role)

@app.route("/health")
def health():
    return jsonify({"status": "healthy", "service": "CloudCore-OpsGateway", "version": "2.4.0", "port": 8012}), 200

@app.route("/debug/vars")
def debug_vars():
    return jsonify({
        "app_name": "CloudCore_OpsGateway",
        "version": "2.4.0",
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
    return render_template_string(PAGE_TEMPLATE, title="Storage Snapshots", view="backups", files=files)

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
            session["role"] = "Cluster Administrator"
            return redirect("/admin")
        elif username == "operator" and password == "password123":
            session["user"] = "operator"
            session["role"] = "Ops Operator"
            return redirect("/")
        else:
            error = "Invalid credentials. Authentication rejected by CloudCore Security Gateway."

    return render_template_string(PAGE_TEMPLATE, title="Console Login", view="login", error=error)

@app.route("/admin")
def admin():
    if session.get("user") != "admin":
        return redirect("/login")
    return render_template_string(PAGE_TEMPLATE, title="Root Cluster Console", view="admin")

@app.route("/logout")
def logout():
    session.clear()
    return redirect("/")

@app.route("/reset", methods=["POST"])
def reset():
    session.clear()
    return jsonify({"status": "reset", "message": "CloudCore OpsGateway state reset to default."})

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8012, debug=False)
