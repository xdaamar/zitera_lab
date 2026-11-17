import json
import os
from flask import Flask, jsonify, render_template_string, request

app = Flask(__name__)

FLAG = "ZITERA{5upply_ch41n_p0150n1ng_d3p_2026}"
PACKAGES_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "packages")

HTML_TEMPLATE = """
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>PackNexus Supply Hub | Enterprise Artifact Registry</title>
    <style>
        :root {
            --bg-primary: #0a0e1a;
            --bg-secondary: #111827;
            --bg-tertiary: #1f2937;
            --border: #374151;
            --accent: #6366f1;
            --accent-hover: #4f46e5;
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
            background: linear-gradient(135deg, #6366f1, #8b5cf6);
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
        .badge-verified { background: rgba(16, 185, 129, 0.15); color: #34d399; border: 1px solid rgba(16, 185, 129, 0.3); }
        .badge-untrusted { background: rgba(239, 68, 68, 0.15); color: #f87171; border: 1px solid rgba(239, 68, 68, 0.3); }
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
        input[type=text] {
            width: 100%;
            padding: 0.65rem 0.85rem;
            background: var(--bg-primary);
            border: 1px solid var(--border);
            color: white;
            border-radius: 6px;
            font-size: 0.9rem;
        }
        code { background: var(--bg-tertiary); padding: 2px 6px; border-radius: 4px; font-family: monospace; font-size: 0.825rem; color: #818cf8; }
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
            <div class="brand-icon">PN</div>
            <span>PackNexus Supply Hub</span>
        </a>
        <nav class="nav-links">
            <a href="/" class="nav-link">Artifact Repositories</a>
            <a href="/packages/audit/package_lock_audit.json" class="nav-link">Lockfile Audit API</a>
            <a href="/api/packages" class="nav-link">Catalog API</a>
            <span style="font-size: 0.825rem; color: var(--text-muted);">Pipeline: <strong>CI/CD Worker Cluster 04</strong></span>
        </nav>
    </header>

    <main class="container">
        <div class="breadcrumbs">
            <a href="/">Home</a>
            <span>/</span>
            <a href="/">Supply Chain Hub</a>
            <span>/</span>
            <span>B2B Package Registry &amp; Dependency Integrity</span>
        </div>

        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 1rem; margin-bottom: 1.5rem;">
            <div class="card" style="margin-bottom: 0;">
                <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Private Registry Status</div>
                <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem; color: var(--success);">Operational</div>
                <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.25rem;">1,420 Artifacts Cataloged</div>
            </div>
            <div class="card" style="margin-bottom: 0;">
                <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Build Dependency Firewall</div>
                <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem;">Active Attestation</div>
                <div style="font-size: 0.8rem; color: #818cf8; margin-top: 0.25rem;">Sigstore / SLSA Level 3</div>
            </div>
            <div class="card" style="margin-bottom: 0;">
                <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Lockfile Integrity Scanner</div>
                <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem; color: var(--warning);">Anomaly Flagged</div>
                <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.25rem;">1 Unpinned Dependency</div>
            </div>
        </div>

        <div class="card">
            <div class="card-header">
                <div>
                    <h2 class="card-title">Active Registry Dependencies &amp; Provenance</h2>
                    <p style="font-size: 0.825rem; color: var(--text-muted); margin-top: 0.2rem;">
                        Inspection of automated build manifest and resolved transitive packages.
                    </p>
                </div>
                <a href="/packages/audit/package_lock_audit.json" class="btn btn-outline" style="font-size: 0.8rem;">
                    Download package_lock_audit.json &rarr;
                </a>
            </div>

            <table>
                <thead>
                    <tr>
                        <th>Package Identifier</th>
                        <th>Resolved Version</th>
                        <th>Upstream Registry Origin</th>
                        <th>Integrity &amp; Provenance</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td><code>express</code></td>
                        <td>4.19.2</td>
                        <td>https://registry.npmjs.org/</td>
                        <td><span class="badge badge-verified">Verified (Sigstore)</span></td>
                    </tr>
                    <tr>
                        <td><code>lodash</code></td>
                        <td>4.17.21</td>
                        <td>https://registry.npmjs.org/</td>
                        <td><span class="badge badge-verified">Verified (Sigstore)</span></td>
                    </tr>
                    <tr>
                        <td><code>apex-internal-telemetry</code></td>
                        <td>0.1.0-alpha.build.88</td>
                        <td>http://127.0.0.1:8013/packages/download/</td>
                        <td><span class="badge badge-untrusted">Untrusted / Unpinned Origin</span></td>
                    </tr>
                </tbody>
            </table>
        </div>

        <div class="card">
            <div class="card-header">
                <h3 class="card-title">CI/CD Pipeline Attestation &amp; Deployment Key Verifier</h3>
            </div>
            <p style="font-size: 0.85rem; color: var(--text-muted); margin-bottom: 1rem;">
                Submit an exfiltrated pipeline token or deployment secret recovered during the dependency audit to verify build authorization.
            </p>
            <form method="POST" action="/api/supply-chain/verify" style="display: flex; gap: 0.75rem;">
                <input type="text" name="token" placeholder="ZITERA{...}" required style="flex: 1;">
                <button type="submit" class="btn btn-primary" style="white-space: nowrap;">Verify Attestation</button>
            </form>
        </div>
    </main>

    <footer class="footer">
        <p>&copy; 2026 PackNexus Inc. All rights reserved. &bull; Enterprise Software Supply Chain &amp; Secure Build Infrastructure</p>
    </footer>
</body>
</html>
"""

@app.route("/health", methods=["GET"])
def health():
    return jsonify({"status": "ok", "service": "PackNexus Supply Hub", "port": 8013})

@app.route("/", methods=["GET"])
def home():
    return render_template_string(HTML_TEMPLATE)

@app.route("/packages/audit/package_lock_audit.json", methods=["GET"])
def get_audit_file():
    audit_path = os.path.join(PACKAGES_DIR, "package_lock_audit.json")
    if os.path.exists(audit_path):
        with open(audit_path, "r", encoding="utf-8") as f:
            data = json.load(f)
        return jsonify(data)
    return jsonify({"error": "Audit manifest not found"}), 404

@app.route("/api/packages", methods=["GET"])
def list_packages():
    return jsonify({
        "registry": "PackNexus Supply Hub",
        "packages": [
            {"name": "express", "version": "4.19.2", "verified": True},
            {"name": "lodash", "version": "4.17.21", "verified": True},
            {"name": "apex-internal-telemetry", "version": "0.1.0-alpha.build.88", "verified": False}
        ]
    })

@app.route("/api/supply-chain/verify", methods=["POST", "GET"])
def verify_token():
    token = request.form.get("token") or (request.get_json(silent=True) or {}).get("token")
    if token == FLAG:
        return jsonify({
            "status": "passed",
            "message": "Token verified! Supply chain backdoored dependency successfully audited.",
            "flag": FLAG
        })
    return jsonify({
        "status": "failed",
        "message": "Invalid deployment token. Review /packages/audit/package_lock_audit.json for compromised package metadata."
    }), 400

if __name__ == "__main__":
    port = int(os.environ.get("PORT", 8080))
    app.run(host="0.0.0.0", port=port)
