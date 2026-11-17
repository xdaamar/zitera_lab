import os
import json
from http.server import HTTPServer, BaseHTTPRequestHandler
import urllib.parse

PORT = 8018
FLAG = "ZITERA{1nt3gr1ty_f41lur3_un51gn3d_p4ck4g3}"

DEFAULT_CONFIG = {
    "version": "1.0.0",
    "signature_verified": True,
    "signer": "dataflow-root-authority",
    "params": {
        "telemetry_level": "standard",
        "enforce_mtls": True,
        "worker_parallelism": 16,
        "checkpoint_interval_ms": 5000
    }
}

active_config = dict(DEFAULT_CONFIG)
tampered_accepted = False

class IntegrityFailureHandler(BaseHTTPRequestHandler):
    def _send_json(self, status, payload):
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Access-Control-Allow-Origin", "*")
        self.end_headers()
        self.wfile.write(json.dumps(payload).encode("utf-8"))

    def do_GET(self):
        parsed = urllib.parse.urlparse(self.path)
        if parsed.path in ["/", "/index.html"]:
            self.send_response(200)
            self.send_header("Content-Type", "text/html; charset=utf-8")
            self.end_headers()
            
            sig_badge = "badge-success" if active_config.get("signature_verified") else "badge-warning"
            sig_text = "Signature Validated" if active_config.get("signature_verified") else "Unsigned / Tampered"

            html = f"""<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>DataFlow Analytics | Enterprise BI &amp; Stream Processing Engine</title>
    <style>
        :root {{
            --bg-primary: #0a0d18;
            --bg-secondary: #111728;
            --bg-tertiary: #1c2438;
            --border: #2d3748;
            --accent: #10b981;
            --accent-hover: #059669;
            --text-main: #f9fafb;
            --text-muted: #9ca3af;
            --success: #10b981;
            --warning: #f59e0b;
            --danger: #ef4444;
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: var(--bg-primary);
            color: var(--text-main);
            min-height: 100vh;
            display: flex;
            flex-direction: column;
        }}
        .navbar {{
            background: var(--bg-secondary);
            border-bottom: 1px solid var(--border);
            padding: 0.75rem 2rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }}
        .brand {{
            display: flex;
            align-items: center;
            gap: 0.75rem;
            text-decoration: none;
            color: var(--text-main);
            font-weight: 700;
            font-size: 1.15rem;
        }}
        .brand-icon {{
            width: 32px;
            height: 32px;
            background: linear-gradient(135deg, #10b981, #06b6d4);
            border-radius: 6px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: 900;
            color: white;
            font-size: 0.95rem;
        }}
        .nav-links {{ display: flex; align-items: center; gap: 1.5rem; }}
        .nav-link {{ color: var(--text-muted); text-decoration: none; font-size: 0.875rem; font-weight: 500; }}
        .nav-link:hover {{ color: var(--text-main); }}
        .container {{ max-width: 1100px; width: 100%; margin: 2rem auto; padding: 0 1.5rem; flex: 1; }}
        .breadcrumbs {{ display: flex; align-items: center; gap: 0.5rem; font-size: 0.825rem; color: var(--text-muted); margin-bottom: 1.5rem; }}
        .breadcrumbs a {{ color: var(--text-muted); text-decoration: none; }}
        .breadcrumbs a:hover {{ color: var(--accent); }}
        .card {{
            background: var(--bg-secondary);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 1.5rem;
            margin-bottom: 1.5rem;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.3);
        }}
        .card-header {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 1.25rem;
            padding-bottom: 0.75rem;
            border-bottom: 1px solid var(--border);
        }}
        .card-title {{ font-size: 1.125rem; font-weight: 600; }}
        .badge {{
            display: inline-block;
            padding: 0.25rem 0.6rem;
            border-radius: 9999px;
            font-size: 0.75rem;
            font-weight: 600;
            text-transform: uppercase;
        }}
        .badge-success {{ background: rgba(16, 185, 129, 0.15); color: #34d399; border: 1px solid rgba(16, 185, 129, 0.3); }}
        .badge-warning {{ background: rgba(245, 158, 11, 0.15); color: #fbbf24; border: 1px solid rgba(245, 158, 11, 0.3); }}
        pre {{
            background: var(--bg-primary);
            border: 1px solid var(--border);
            padding: 1rem;
            border-radius: 6px;
            overflow-x: auto;
            color: #34d399;
            font-family: monospace;
            font-size: 0.85rem;
            line-height: 1.5;
        }}
        .btn {{
            display: inline-block;
            padding: 0.5rem 1rem;
            border-radius: 6px;
            font-size: 0.875rem;
            font-weight: 500;
            text-decoration: none;
            cursor: pointer;
            border: 1px solid transparent;
            transition: all 0.15s;
        }}
        .btn-primary {{ background: var(--accent); color: white; }}
        .btn-primary:hover {{ background: var(--accent-hover); }}
        .btn-outline {{ background: transparent; border-color: var(--border); color: var(--text-main); }}
        .btn-outline:hover {{ background: var(--bg-tertiary); }}
        .footer {{
            background: var(--bg-secondary);
            border-top: 1px solid var(--border);
            padding: 1.5rem 2rem;
            font-size: 0.8rem;
            color: var(--text-muted);
            text-align: center;
        }}
    </style>
</head>
<body>
    <header class="navbar">
        <a href="/" class="brand">
            <div class="brand-icon">DF</div>
            <span>DataFlow Analytics</span>
        </a>
        <nav class="nav-links">
            <a href="/" class="nav-link">Cluster Health</a>
            <a href="/api/config" class="nav-link">Active Config API</a>
            <span style="font-size: 0.825rem; color: var(--text-muted);">Cluster ID: <strong>DF-PROD-EAST-02</strong></span>
        </nav>
    </header>

    <main class="container">
        <div class="breadcrumbs">
            <a href="/">Home</a>
            <span>/</span>
            <a href="/">Stream Engine</a>
            <span>/</span>
            <span>Cluster Orchestration &amp; Runtime Configuration</span>
        </div>

        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 1rem; margin-bottom: 1.5rem;">
            <div class="card" style="margin-bottom: 0;">
                <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Streaming Throughput</div>
                <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem; color: var(--success);">48,200 msg/sec</div>
                <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.25rem;">Kafka Topics Synchronized</div>
            </div>
            <div class="card" style="margin-bottom: 0;">
                <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Cluster Workers</div>
                <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem;">16 Executors</div>
                <div style="font-size: 0.8rem; color: #34d399; margin-top: 0.25rem;">Zero Task Failures</div>
            </div>
            <div class="card" style="margin-bottom: 0;">
                <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Configuration Authority</div>
                <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem;"><span class="badge {sig_badge}">{sig_text}</span></div>
                <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.25rem;">Signer: {active_config.get("signer", "Unknown")}</div>
            </div>
        </div>

        <div class="card">
            <div class="card-header">
                <div>
                    <h2 class="card-title">Active Streaming Engine Configuration</h2>
                    <p style="font-size: 0.825rem; color: var(--text-muted); margin-top: 0.2rem;">
                        Current runtime parameters ingested into cluster orchestrator memory.
                    </p>
                </div>
                <a href="/api/config" class="btn btn-outline" style="font-size: 0.8rem;">GET /api/config &rarr;</a>
            </div>
            <pre>{json.dumps(active_config, indent=2)}</pre>
        </div>

        <div class="card">
            <div class="card-header">
                <h3 class="card-title">Cluster Configuration Ingestion Endpoint</h3>
            </div>
            <p style="font-size: 0.85rem; color: var(--text-muted); margin-bottom: 1rem;">
                Ingest updated cluster configuration packages via HTTP POST to <code>/api/deploy_update</code>.
            </p>
            <div style="background: var(--bg-tertiary); padding: 1rem; border-radius: 6px; font-family: monospace; font-size: 0.825rem; color: #94a3b8;">
                curl -X POST http://127.0.0.1:{PORT}/api/deploy_update \\<br>
                &nbsp;&nbsp;-H "Content-Type: application/json" \\<br>
                &nbsp;&nbsp;-d '{{"version": "2.0.0", "signature_verified": false, "params": {{"debug_backdoor": true}}}}'
            </div>
        </div>
    </main>

    <footer class="footer">
        <p>&copy; 2026 DataFlow Analytics Inc. All rights reserved. &bull; Enterprise Big Data &amp; Stream Ingestion Infrastructure</p>
    </footer>
</body>
</html>"""
            self.wfile.write(html.encode("utf-8"))
        elif parsed.path == "/health":
            self._send_json(200, {"status": "ok", "service": "DataFlow Analytics", "port": PORT})
        elif parsed.path == "/practice/verify":
            if tampered_accepted:
                self._send_json(200, {
                    "status": "passed",
                    "message": "Software integrity failure demonstrated: Cluster orchestrator accepted unverified/tampered configuration package into active runtime!"
                })
            else:
                self._send_json(200, {
                    "status": "failed",
                    "message": "Practice target pending: Deploy an unsigned or tampered configuration package via POST /api/deploy_update."
                })
        elif parsed.path == "/api/config":
            self._send_json(200, active_config)
        else:
            self._send_json(404, {"error": "Not found"})

    def do_POST(self):
        global active_config, tampered_accepted
        parsed = urllib.parse.urlparse(self.path)
        length = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(length).decode("utf-8") if length > 0 else "{}"
        try:
            payload = json.loads(body) if body else {}
        except Exception:
            payload = {}

        if parsed.path == "/api/deploy_update":
            # Defect: Ingests configuration without cryptographic signature verification
            active_config = {
                "version": payload.get("version", "2.0.0-custom"),
                "signature_verified": payload.get("signature_verified", False),
                "signer": payload.get("signer", "unverified-external-source"),
                "params": payload.get("params", {"debug_backdoor": True})
            }
            tampered_accepted = True
            resp = {
                "status": "applied",
                "message": "Configuration package deployed directly into cluster memory.",
                "active_config": active_config
            }
            if active_config.get("params", {}).get("debug_backdoor") is True or not active_config["signature_verified"]:
                resp["flag"] = FLAG
            self._send_json(200, resp)
        elif parsed.path == "/api/reset":
            active_config = dict(DEFAULT_CONFIG)
            tampered_accepted = False
            self._send_json(200, {"status": "reset", "message": "Cluster configuration reset to factory signed defaults."})
        else:
            self._send_json(404, {"error": "Endpoint not recognized"})

if __name__ == "__main__":
    server = HTTPServer(("0.0.0.0", PORT), IntegrityFailureHandler)
    print(f"DataFlow Analytics service running on port {PORT}")
    server.serve_forever()
