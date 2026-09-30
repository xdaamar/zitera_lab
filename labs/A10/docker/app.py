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
            html = f'''<!DOCTYPE html>
<html>
<head>
    <title>Zitera Payment & Security Gate - Fail-Open Evaluation (A10)</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: #181824; color: #e2e8f0; margin: 0; padding: 24px; }}
        .container {{ max-width: 900px; margin: 0 auto; background: #22223b; border: 1px solid #4a4e69; border-radius: 8px; padding: 24px; }}
        h1 {{ color: #f43f5e; font-family: monospace; }}
        .card {{ background: #2a2a40; padding: 16px; border-radius: 6px; margin-top: 16px; }}
        pre {{ background: #12121c; padding: 12px; border-radius: 4px; overflow-x: auto; color: #38bdf8; font-family: monospace; }}
        .badge {{ padding: 4px 8px; border-radius: 4px; font-weight: bold; }}
    </style>
</head>
<body>
<div class="container">
    <h1>[A10:2025] Mishandling of Exceptional Conditions</h1>
    <p>Target Scenario: Transaction Gate Failing Open on Upstream Parser Exceptions.</p>
    <div class="card">
        <h3>Gate Policy & Vulnerability</h3>
        <p>Normal rule: Deny access unless upstream KYC/fraud token validates successfully.</p>
        <p>Vulnerability: When the request induces an unhandled parsing exception (e.g. malformed type, null pointer, upstream 504 timeout simulation), the generic catch block defaults to <code>ALLOW_ACCESS = True</code> to 'avoid blocking legitimate customers'.</p>
    </div>
    <div class="card">
        <h3>Evaluation Endpoint: <code>POST /api/gate/verify</code></h3>
        <p>Normal input: <code>{{"token": "valid_user_token"}}</code> &rarr; Evaluates token.</p>
        <p>Exceptional payload: <code>{{"simulate_exception": "UPSTREAM_TIMEOUT"}}</code> or <code>{{"token": null}}</code> &rarr; Induces exception.</p>
    </div>
</div>
</body>
</html>'''
            self.wfile.write(html.encode("utf-8"))
        elif parsed.path == "/health":
            self._send_json(200, {"status": "ok", "lab": "A10", "port": PORT})
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
                # INSECURE DESIGN / MISHANDLED EXCEPTION: Fail-Open!
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
    print(f"A10 Exceptional Conditions Service running on port {PORT}")
    server.serve_forever()
