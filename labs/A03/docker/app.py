import json
import os
from flask import Flask, jsonify, render_template_string, request, send_from_directory

app = Flask(__name__)

FLAG = "ZITERA{5upply_ch41n_p0150n1ng_d3p_2026}"
PACKAGES_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "packages")

HTML_TEMPLATE = """
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>ApexCorp — Supply Chain & Package Registry</title>
    <style>
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background: #0f172a; color: #f8fafc; margin: 0; padding: 24px; }
        .container { max-width: 900px; margin: 0 auto; }
        .header { border-bottom: 2px solid #334155; padding-bottom: 16px; margin-bottom: 24px; display: flex; justify-content: space-between; align-items: center; }
        .badge { background: #0369a1; color: #e0f2fe; padding: 4px 10px; border-radius: 4px; font-size: 12px; font-weight: bold; }
        .card { background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 20px; margin-bottom: 20px; }
        .banner { background: #1e1b4b; border-left: 4px solid #6366f1; padding: 12px; margin-bottom: 16px; font-size: 13px; color: #e0e7ff; }
        table { width: 100%; border-collapse: collapse; margin-top: 12px; }
        th, td { text-align: left; padding: 10px; border-bottom: 1px solid #334155; font-size: 13px; }
        th { background: #0f172a; color: #94a3b8; }
        .code { font-family: monospace; background: #0f172a; padding: 2px 6px; border-radius: 4px; color: #38bdf8; }
        a { color: #38bdf8; text-decoration: none; }
        a:hover { text-decoration: underline; }
        .tag-warn { color: #f59e0b; font-weight: bold; }
    </style>
</head>
<body>
<div class="container">
    <div class="header">
        <div>
            <h2>ApexCorp // Internal Package Registry</h2>
            <div style="color: #94a3b8; font-size: 13px;">Software Supply Chain Dependency Inspector • v2.4.1</div>
        </div>
        <span class="badge">LAB A03 • PORT 8013</span>
    </div>

    <div class="banner">
        <strong>SECURITY NOTICE:</strong> Modern applications incorporate hundreds of external packages. Always audit lockfiles and verify signature provenance before allowing packages into automated production builds.
    </div>

    <div class="card">
        <h3>Active Registry Dependencies</h3>
        <p style="color: #94a3b8; font-size: 13px;">
            The CI/CD pipeline recently completed an automated dependency audit. Download and inspect the full lockfile audit manifest:
            <br>
            <a href="/packages/audit/package_lock_audit.json" target="_blank" class="code">GET /packages/audit/package_lock_audit.json</a>
        </p>

        <table>
            <thead>
                <tr>
                    <th>Package Name</th>
                    <th>Resolved Version</th>
                    <th>Registry Source</th>
                    <th>Status</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td><code>express</code></td>
                    <td>4.19.2</td>
                    <td>https://registry.npmjs.org/</td>
                    <td style="color: #4ade80;">Verified</td>
                </tr>
                <tr>
                    <td><code>lodash</code></td>
                    <td>4.17.21</td>
                    <td>https://registry.npmjs.org/</td>
                    <td style="color: #4ade80;">Verified</td>
                </tr>
                <tr>
                    <td><code>apex-internal-telemetry</code></td>
                    <td>0.1.0-alpha.build.88</td>
                    <td>http://127.0.0.1:8013/packages/download/</td>
                    <td class="tag-warn">UNTRUSTED / UNPINNED</td>
                </tr>
            </tbody>
        </table>
    </div>

    <div class="card">
        <h3>Pipeline Verification Terminal</h3>
        <p style="color: #94a3b8; font-size: 13px;">
            Use the form below or <code>curl</code> to verify an exfiltrated pipeline token recovered from the compromised dependency manifest:
        </p>
        <form method="POST" action="/api/supply-chain/verify" style="display: flex; gap: 8px;">
            <input type="text" name="token" placeholder="ZITERA{...}" style="flex: 1; padding: 10px; background: #0f172a; border: 1px solid #334155; color: white; border-radius: 4px;" required>
            <button type="submit" style="padding: 10px 18px; background: #6366f1; color: white; border: none; border-radius: 4px; cursor: pointer; font-weight: bold;">Verify Token</button>
        </form>
    </div>
</div>
</body>
</html>
"""

@app.route("/health", methods=["GET"])
def health():
    return jsonify({"status": "ok", "lab": "A03", "port": 8013})

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
            "message": "Flag verified! Supply chain backdoored dependency successfully audited.",
            "flag": FLAG
        })
    return jsonify({
        "status": "failed",
        "message": "Invalid deployment token. Review /packages/audit/package_lock_audit.json for compromised package metadata."
    }), 400

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8080)
