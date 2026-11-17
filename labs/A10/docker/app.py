import os
import json
from http.server import HTTPServer, BaseHTTPRequestHandler
import urllib.parse

PORT = 8020
FLAG = "ZITERA{f41l_0p3n_3xc3pt10n5_un4uth0r1z3d}"

fail_open_triggered = False

class ExceptionalConditionHandler(BaseHTTPRequestHandler):
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

            gate_status = "FAIL-OPEN ACTIVE (UNRESTRICTED)" if fail_open_triggered else "STRICT ENFORCEMENT"
            gate_badge = "badge-warning" if fail_open_triggered else "badge-success"

            html = f"""<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>TemplateCloud Studio | SaaS Document &amp; PDF Generation Engine</title>
    <style>
        :root {{
            --bg-primary: #0a0d18;
            --bg-secondary: #111728;
            --bg-tertiary: #1c2438;
            --border: #2d3748;
            --accent: #f43f5e;
            --accent-hover: #e11d48;
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
            background: linear-gradient(135deg, #f43f5e, #fb7185);
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
        table {{ width: 100%; border-collapse: collapse; font-size: 0.875rem; }}
        th {{ text-align: left; padding: 0.75rem 1rem; background: var(--bg-tertiary); color: var(--text-muted); font-weight: 600; border-bottom: 1px solid var(--border); }}
        td {{ padding: 0.875rem 1rem; border-bottom: 1px solid var(--border); }}
        tr:hover td {{ background-color: rgba(255, 255, 255, 0.02); }}
        code {{ background: var(--bg-tertiary); padding: 2px 6px; border-radius: 4px; font-family: monospace; font-size: 0.825rem; color: #fb7185; }}
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
            <div class="brand-icon">TC</div>
            <span>TemplateCloud Studio</span>
        </a>
        <nav class="nav-links">
            <a href="/" class="nav-link">Template Library</a>
            <a href="/" class="nav-link">Rendering Workers</a>
            <span style="font-size: 0.825rem; color: var(--text-muted);">SaaS Tier: <strong>Enterprise Cloud PDF v4.2</strong></span>
        </nav>
    </header>

    <main class="container">
        <div class="breadcrumbs">
            <a href="/">Home</a>
            <span>/</span>
            <a href="/">Document Engine</a>
            <span>/</span>
            <span>Rasterization Gate &amp; SaaS Execution Pipeline</span>
        </div>

        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 1rem; margin-bottom: 1.5rem;">
            <div class="card" style="margin-bottom: 0;">
                <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Rendering Cluster</div>
                <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem; color: var(--success);">8 Headless Nodes</div>
                <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.25rem;">Chromium V8 Engine</div>
            </div>
            <div class="card" style="margin-bottom: 0;">
                <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Authorization Gate Policy</div>
                <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem;"><span class="badge {gate_badge}">{gate_status}</span></div>
                <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.25rem;">Upstream Exception Catch</div>
            </div>
            <div class="card" style="margin-bottom: 0;">
                <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">API Gate Port</div>
                <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem;">8020</div>
                <div style="font-size: 0.8rem; color: #60a5fa; margin-top: 0.25rem;">/api/gate/verify</div>
            </div>
        </div>

        <div class="card">
            <div class="card-header">
                <div>
                    <h2 class="card-title">Production Document Templates &amp; Dispatch Pipeline</h2>
                    <p style="font-size: 0.825rem; color: var(--text-muted); margin-top: 0.2rem;">
                        Commercial transactional invoice and statement generators.
                    </p>
                </div>
            </div>

            <table>
                <thead>
                    <tr>
                        <th>Template ID</th>
                        <th>Document Title</th>
                        <th>Format</th>
                        <th>Compilation Gate</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td><code>TPL-INV-001</code></td>
                        <td><strong>B2B Enterprise Tax Invoice</strong></td>
                        <td>PDF / A4</td>
                        <td><span class="badge badge-success">Authorized</span></td>
                    </tr>
                    <tr>
                        <td><code>TPL-STM-002</code></td>
                        <td><strong>Monthly Custody Account Statement</strong></td>
                        <td>PDF / Letter</td>
                        <td><span class="badge badge-success">Authorized</span></td>
                    </tr>
                    <tr>
                        <td><code>TPL-NDA-003</code></td>
                        <td><strong>Confidential Mutual Non-Disclosure Agreement</strong></td>
                        <td>PDF / Legal</td>
                        <td><span class="badge badge-success">Authorized</span></td>
                    </tr>
                </tbody>
            </table>
        </div>

        <div class="card">
            <div class="card-header">
                <h3 class="card-title">Upstream Token Authorization Evaluator</h3>
            </div>
            <p style="font-size: 0.85rem; color: var(--text-muted); margin-bottom: 1rem;">
                Evaluate client document rendering tokens via <code>POST /api/gate/verify</code>.
            </p>
            <div style="background: var(--bg-tertiary); padding: 1rem; border-radius: 6px; font-family: monospace; font-size: 0.825rem; color: #94a3b8;">
                curl -X POST http://127.0.0.1:{PORT}/api/gate/verify \\<br>
                &nbsp;&nbsp;-H "Content-Type: application/json" \\<br>
                &nbsp;&nbsp;-d '{{"simulate_exception": "UPSTREAM_TIMEOUT"}}'
            </div>
        </div>
    </main>

    <footer class="footer">
        <p>&copy; 2026 TemplateCloud Studio Inc. All rights reserved. &bull; Enterprise SaaS Document Generation &amp; Rendering Platform</p>
    </footer>
</body>
</html>"""
            self.wfile.write(html.encode("utf-8"))
        elif parsed.path == "/health":
            self._send_json(200, {"status": "ok", "service": "TemplateCloud Studio", "port": PORT})
        elif parsed.path == "/practice/verify":
            if fail_open_triggered:
                self._send_json(200, {
                    "status": "passed",
                    "message": "Exceptional condition failure reproduced: Upstream exception caused the gate to fail-open and authorize transaction!"
                })
            else:
                self._send_json(200, {
                    "status": "failed",
                    "message": "Practice target pending: Induce an upstream parsing/timeout exception via POST /api/gate/verify to trigger fail-open behavior."
                })
        else:
            self._send_json(404, {"error": "Not found"})

    def do_POST(self):
        global fail_open_triggered
        parsed = urllib.parse.urlparse(self.path)
        length = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(length).decode("utf-8") if length > 0 else "{}"
        try:
            payload = json.loads(body) if body else {}
        except Exception:
            payload = {}

        if parsed.path == "/api/gate/verify":
            # Exceptional condition evaluation
            try:
                if payload.get("simulate_exception") or "trigger_error" in payload or payload.get("token") is None:
                    raise RuntimeError("Simulated Upstream Gateway Timeout / Malformed Exception")

                token = payload.get("token", "")
                if token == "super_authorized_token":
                    authorized = True
                else:
                    authorized = False
            except Exception as e:
                # Flawed exception handling: Fail-Open default!
                print(f"[WARN] Caught exception: {e}. Falling back to OPEN state!")
                authorized = True
                fail_open_triggered = True

            resp = {
                "authorized": authorized,
                "fail_open_state": fail_open_triggered,
                "message": "Access granted via fail-open fallback." if fail_open_triggered else ("Access granted" if authorized else "Access denied")
            }
            if fail_open_triggered:
                resp["flag"] = FLAG

            self._send_json(200, resp)
        elif parsed.path == "/api/reset":
            fail_open_triggered = False
            self._send_json(200, {"status": "reset", "message": "Transaction gate state reset to fail-closed default."})
        else:
            self._send_json(404, {"error": "Endpoint not recognized"})

if __name__ == "__main__":
    server = HTTPServer(("0.0.0.0", PORT), ExceptionalConditionHandler)
    print(f"TemplateCloud Studio service running on port {PORT}")
    server.serve_forever()
