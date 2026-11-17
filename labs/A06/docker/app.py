import os
import json
from http.server import HTTPServer, BaseHTTPRequestHandler
import urllib.parse

PORT = 8016

INITIAL_ORDERS = {
    "1001": {"id": "1001", "item": "Standard Developer Cloud Sandbox Tier-1", "amount": 120, "status": "DRAFT", "owner": "alice"},
    "1002": {"id": "1002", "item": "High-Performance AI & GPU Cluster Access", "amount": 15000, "status": "PENDING_APPROVAL", "owner": "bob"},
    "9999": {"id": "9999", "item": "Executive Tier-0 Datacenter Infrastructure Clearance", "amount": 50000, "status": "DRAFT", "owner": "learner"}
}

FLAG = "ZITERA{1n53cur3_d351gn_fl4w3d_w0rkfl0w}"

orders = dict(INITIAL_ORDERS)

class InsecureDesignHandler(BaseHTTPRequestHandler):
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
            
            rows_html = ""
            for oid, o in orders.items():
                badge_class = "badge-pending" if "PENDING" in o['status'] else ("badge-approved" if o['status'] in ["APPROVED", "DISPATCHED"] else "badge-draft")
                rows_html += f"""
                <tr>
                    <td><code>#{o['id']}</code></td>
                    <td><strong>{o['item']}</strong></td>
                    <td style="font-family: monospace; font-weight: 600;">${o['amount']:,}</td>
                    <td>{o['owner']}</td>
                    <td><span class="badge {badge_class}">{o['status']}</span></td>
                </tr>
                """

            html = f"""<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>AuthShield Identity | Enterprise Access Governance &amp; Provisioning</title>
    <style>
        :root {{
            --bg-primary: #0a0d18;
            --bg-secondary: #111728;
            --bg-tertiary: #1c2438;
            --border: #2d3748;
            --accent: #3b82f6;
            --accent-hover: #2563eb;
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
            background: linear-gradient(135deg, #3b82f6, #60a5fa);
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
        .badge-draft {{ background: rgba(156, 163, 175, 0.15); color: #9ca3af; border: 1px solid rgba(156, 163, 175, 0.3); }}
        .badge-pending {{ background: rgba(245, 158, 11, 0.15); color: #fbbf24; border: 1px solid rgba(245, 158, 11, 0.3); }}
        .badge-approved {{ background: rgba(16, 185, 129, 0.15); color: #34d399; border: 1px solid rgba(16, 185, 129, 0.3); }}
        table {{ width: 100%; border-collapse: collapse; font-size: 0.875rem; }}
        th {{ text-align: left; padding: 0.75rem 1rem; background: var(--bg-tertiary); color: var(--text-muted); font-weight: 600; border-bottom: 1px solid var(--border); }}
        td {{ padding: 0.875rem 1rem; border-bottom: 1px solid var(--border); }}
        tr:hover td {{ background-color: rgba(255, 255, 255, 0.02); }}
        code {{ background: var(--bg-tertiary); padding: 2px 6px; border-radius: 4px; font-family: monospace; font-size: 0.825rem; color: #60a5fa; }}
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
            <div class="brand-icon">AS</div>
            <span>AuthShield Identity</span>
        </a>
        <nav class="nav-links">
            <a href="/" class="nav-link">Access Governance</a>
            <a href="/api/orders" class="nav-link">Entitlements API</a>
            <span style="font-size: 0.825rem; color: var(--text-muted);">Session: <strong>Enterprise SSO (SAML 2.0)</strong></span>
        </nav>
    </header>

    <main class="container">
        <div class="breadcrumbs">
            <a href="/">Home</a>
            <span>/</span>
            <a href="/">Identity Governance (IGA)</a>
            <span>/</span>
            <span>Resource Entitlement Provisioning Requests</span>
        </div>

        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 1rem; margin-bottom: 1.5rem;">
            <div class="card" style="margin-bottom: 0;">
                <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Identity Governance Mode</div>
                <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem; color: var(--success);">Zero Trust Lifecycle</div>
                <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.25rem;">Directory Synchronized</div>
            </div>
            <div class="card" style="margin-bottom: 0;">
                <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Approval Threshold Policy</div>
                <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem;">&ge; $500 Dual-Control</div>
                <div style="font-size: 0.8rem; color: #60a5fa; margin-top: 0.25rem;">Executive Signature Required</div>
            </div>
            <div class="card" style="margin-bottom: 0;">
                <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Workflow State Machine</div>
                <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem;">REST API Ingest</div>
                <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.25rem;">Port 8016 &bull; JSON Payloads</div>
            </div>
        </div>

        <div class="card">
            <div class="card-header">
                <div>
                    <h2 class="card-title">Pending Resource Entitlements &amp; Access Requests</h2>
                    <p style="font-size: 0.825rem; color: var(--text-muted); margin-top: 0.2rem;">
                        Workforce privilege elevation requests under organizational review.
                    </p>
                </div>
            </div>

            <table>
                <thead>
                    <tr>
                        <th>Request ID</th>
                        <th>Target Resource Entitlement</th>
                        <th>Cost Allocation</th>
                        <th>Requester Identity</th>
                        <th>Approval State</th>
                    </tr>
                </thead>
                <tbody>
                    {rows_html}
                </tbody>
            </table>
        </div>
    </main>

    <footer class="footer">
        <p>&copy; 2026 AuthShield Identity Technologies Inc. All rights reserved. &bull; Enterprise Access Management &amp; SSO Infrastructure</p>
    </footer>
</body>
</html>"""
            self.wfile.write(html.encode("utf-8"))
        elif parsed.path == "/health":
            self._send_json(200, {"status": "ok", "service": "AuthShield Identity", "port": PORT})
        elif parsed.path == "/practice/verify":
            high_val_bypassed = any(o["amount"] >= 15000 and o["status"] in ["APPROVED", "DISPATCHED"] for o in orders.values())
            if high_val_bypassed:
                self._send_json(200, {
                    "status": "passed",
                    "message": "Insecure design verified: High-value resource entitlement transitioned to approved state without executive validation!"
                })
            else:
                self._send_json(200, {
                    "status": "failed",
                    "message": "Practice target pending: Transition order #1002 or #9999 state machine into APPROVED or DISPATCHED."
                })
        elif parsed.path == "/api/orders":
            self._send_json(200, {"orders": list(orders.values())})
        else:
            self._send_json(404, {"error": "Not found"})

    def do_POST(self):
        global orders
        parsed = urllib.parse.urlparse(self.path)
        length = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(length).decode("utf-8") if length > 0 else "{}"
        try:
            payload = json.loads(body) if body else {}
        except Exception:
            payload = {}

        if parsed.path.startswith("/api/orders/") and "/transition" in parsed.path:
            parts = parsed.path.split("/")
            order_id = parts[3]
            if order_id not in orders:
                self._send_json(404, {"error": "Access request not found"})
                return

            target = payload.get("target_state") or payload.get("status") or "APPROVED"
            # Architectural flaw: Backend accepts client-supplied state transition without enforcing role invariants
            orders[order_id]["status"] = target

            resp = {
                "message": f"Access request {order_id} transitioned to {target}",
                "order": orders[order_id]
            }
            if order_id == "9999" and target in ["APPROVED", "DISPATCHED"]:
                resp["flag"] = FLAG

            self._send_json(200, resp)
        elif parsed.path == "/api/reset":
            orders = {
                "1001": {"id": "1001", "item": "Standard Developer Cloud Sandbox Tier-1", "amount": 120, "status": "DRAFT", "owner": "alice"},
                "1002": {"id": "1002", "item": "High-Performance AI & GPU Cluster Access", "amount": 15000, "status": "PENDING_APPROVAL", "owner": "bob"},
                "9999": {"id": "9999", "item": "Executive Tier-0 Datacenter Infrastructure Clearance", "amount": 50000, "status": "DRAFT", "owner": "learner"}
            }
            self._send_json(200, {"status": "reset", "message": "Access entitlement requests reset to default."})
        else:
            self._send_json(404, {"error": "Endpoint not recognized"})

if __name__ == "__main__":
    server = HTTPServer(("0.0.0.0", PORT), InsecureDesignHandler)
    print(f"AuthShield Identity service running on port {PORT}")
    server.serve_forever()
