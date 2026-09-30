import os
import json
from http.server import HTTPServer, BaseHTTPRequestHandler
import urllib.parse

PORT = 8016

INITIAL_ORDERS = {
    "1001": {"id": "1001", "item": "Office Stationery", "amount": 120, "status": "DRAFT", "owner": "alice"},
    "1002": {"id": "1002", "item": "High-Performance AI Server", "amount": 15000, "status": "PENDING_APPROVAL", "owner": "bob"},
    "9999": {"id": "9999", "item": "Executive Datacenter Cluster", "amount": 50000, "status": "DRAFT", "owner": "learner"}
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
            html = f'''<!DOCTYPE html>
<html>
<head>
    <title>Zitera Corp - Procurement Portal (A06)</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: #0f111a; color: #f0f0f0; margin: 0; padding: 24px; }}
        .container {{ max-width: 900px; margin: 0 auto; background: #161926; border: 1px solid #282d45; border-radius: 8px; padding: 24px; }}
        h1 {{ color: #7aa2f7; font-family: monospace; }}
        table {{ width: 100%; border-collapse: collapse; margin-top: 16px; }}
        th, td {{ border: 1px solid #282d45; padding: 10px; text-align: left; }}
        th {{ background: #1f2335; }}
        .badge {{ padding: 4px 8px; border-radius: 4px; font-weight: bold; font-size: 12px; }}
        .DRAFT {{ background: #565f89; }}
        .PENDING_APPROVAL {{ background: #e0af68; color: #111; }}
        .APPROVED {{ background: #9ece6a; color: #111; }}
        .DISPATCHED {{ background: #bb9af7; color: #111; }}
        .card {{ background: #1f2335; padding: 16px; border-radius: 6px; margin-top: 20px; }}
        button {{ background: #7aa2f7; color: #111; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; font-weight: bold; }}
        pre {{ background: #11131c; padding: 12px; border-radius: 4px; overflow-x: auto; color: #7dcfff; }}
    </style>
</head>
<body>
<div class="container">
    <h1>[A06:2025] Zitera Procurement Workflow</h1>
    <p>Target Scenario: Business Logic & Workflow State Machine Insecure Design.</p>
    <div class="card">
        <h3>System Business Rules (Intended vs Designed)</h3>
        <ul>
            <li><strong>Rule 1:</strong> Orders &lt; $500 auto-approve upon submission.</li>
            <li><strong>Rule 2:</strong> Orders &gt;= $500 require executive manager signature.</li>
            <li><strong>Design Defect:</strong> The client API endpoint accepts arbitrary state transition payloads (e.g. <code>target_state</code>) or bypasses the approval phase directly to DISPATCHED!</li>
        </ul>
    </div>
    <h3>Active Procurement Orders</h3>
    <table>
        <tr><th>ID</th><th>Item</th><th>Amount</th><th>Owner</th><th>Current Status</th></tr>
        '''
            for oid, o in orders.items():
                html += f"<tr><td>{o['id']}</td><td>{o['item']}</td><td>${o['amount']}</td><td>{o['owner']}</td><td><span class='badge {o['status']}'>{o['status']}</span></td></tr>"
            html += f'''
    </table>
    <div class="card">
        <h3>Workflow API Endpoints</h3>
        <p>Transition order state: <code>POST /api/orders/&lt;id&gt;/transition</code></p>
        <p>Payload: <code>{{"target_state": "DISPATCHED"}}</code> or <code>{{"action": "force_dispatch"}}</code></p>
    </div>
</div>
</body>
</html>'''
            self.wfile.write(html.encode("utf-8"))
        elif parsed.path == "/health":
            self._send_json(200, {"status": "ok", "lab": "A06", "port": PORT})
        elif parsed.path == "/practice/verify":
            # Verification signal: check if high value order (>= 15000) reached DISPATCHED or APPROVED
            high_val_bypassed = any(o["amount"] >= 15000 and o["status"] in ["APPROVED", "DISPATCHED"] for o in orders.values())
            if high_val_bypassed:
                self._send_json(200, {
                    "status": "passed",
                    "message": "Insecure design reproduced: High-value order bypassed manager approval into authorized state!"
                })
            else:
                self._send_json(200, {
                    "status": "failed",
                    "message": "Practice target pending: Manipulate order #1002 or #9999 state machine into APPROVED or DISPATCHED."
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
                self._send_json(404, {"error": "Order not found"})
                return

            target = payload.get("target_state") or payload.get("status") or "APPROVED"
            # INSECURE DESIGN: Directly sets state without evaluating role authorization or manager signature
            orders[order_id]["status"] = target

            resp = {
                "message": f"Order {order_id} transitioned to {target}",
                "order": orders[order_id]
            }
            if order_id == "9999" and target in ["APPROVED", "DISPATCHED"]:
                resp["flag"] = FLAG

            self._send_json(200, resp)
        elif parsed.path == "/api/reset":
            orders = {
                "1001": {"id": "1001", "item": "Office Stationery", "amount": 120, "status": "DRAFT", "owner": "alice"},
                "1002": {"id": "1002", "item": "High-Performance AI Server", "amount": 15000, "status": "PENDING_APPROVAL", "owner": "bob"},
                "9999": {"id": "9999", "item": "Executive Datacenter Cluster", "amount": 50000, "status": "DRAFT", "owner": "learner"}
            }
            self._send_json(200, {"status": "reset", "message": "Procurement orders reset to default."})
        else:
            self._send_json(404, {"error": "Endpoint not recognized"})

if __name__ == "__main__":
    server = HTTPServer(("0.0.0.0", PORT), InsecureDesignHandler)
    print(f"A06 Insecure Design Service running on port {PORT}")
    server.serve_forever()
