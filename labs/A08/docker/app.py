import os
import json
import hashlib
from http.server import HTTPServer, BaseHTTPRequestHandler
import urllib.parse

PORT = 8018
FLAG = "ZITERA{1nt3gr1ty_f41lur3_un51gn3d_p4ck4g3}"

DEFAULT_CONFIG = {
    "version": "1.0.0",
    "signature_verified": True,
    "signer": "zitera-root-authority",
    "params": {"telemetry_level": "standard", "enforce_mtls": True}
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
            html = f'''<!DOCTYPE html>
<html>
<head>
    <title>Zitera Edge Device - Firmware & Config Ingestion (A08)</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: #0d1117; color: #c9d1d9; margin: 0; padding: 24px; }}
        .container {{ max-width: 900px; margin: 0 auto; background: #161b22; border: 1px solid #30363d; border-radius: 8px; padding: 24px; }}
        h1 {{ color: #58a6ff; font-family: monospace; }}
        .card {{ background: #21262d; padding: 16px; border-radius: 6px; margin-top: 16px; }}
        pre {{ background: #0d1117; padding: 12px; border-radius: 4px; overflow-x: auto; color: #7ee787; font-family: monospace; }}
        button {{ background: #238636; color: #fff; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; font-weight: bold; }}
    </style>
</head>
<body>
<div class="container">
    <h1>[A08:2025] Software or Data Integrity Failures</h1>
    <p>Target Scenario: Edge Device Ingesting Unsigned / Modified Configuration Packages.</p>
    <div class="card">
        <h3>Active System Configuration</h3>
        <pre>{json.dumps(active_config, indent=2)}</pre>
    </div>
    <div class="card">
        <h3>Integrity Vulnerability</h3>
        <p>The endpoint <code>POST /api/deploy_update</code> ingests incoming JSON configurations without verifying a valid cryptographic signature or trusted hash.</p>
        <p>Send an update payload with <code>"signature_verified": false</code> or custom malicious parameters to test integrity enforcement.</p>
    </div>
</div>
</body>
</html>'''
            self.wfile.write(html.encode("utf-8"))
        elif parsed.path == "/health":
            self._send_json(200, {"status": "ok", "lab": "A08", "port": PORT})
        elif parsed.path == "/practice/verify":
            if tampered_accepted:
                self._send_json(200, {
                    "status": "passed",
                    "message": "Software integrity failure demonstrated: System accepted unverified/tampered configuration package into active runtime!"
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
            # VULNERABILITY: Blindly trusts client-supplied package without validating cryptographic signature
            active_config = {
                "version": payload.get("version", "2.0.0-custom"),
                "signature_verified": payload.get("signature_verified", False),
                "signer": payload.get("signer", "unverified-external-source"),
                "params": payload.get("params", {"debug_backdoor": True})
            }
            tampered_accepted = True
            resp = {
                "status": "applied",
                "message": "Configuration package deployed directly into memory.",
                "active_config": active_config
            }
            if active_config.get("params", {}).get("debug_backdoor") is True or not active_config["signature_verified"]:
                resp["flag"] = FLAG
            self._send_json(200, resp)
        elif parsed.path == "/api/reset":
            active_config = dict(DEFAULT_CONFIG)
            tampered_accepted = False
            self._send_json(200, {"status": "reset", "message": "Edge configuration reset to factory signed defaults."})
        else:
            self._send_json(404, {"error": "Endpoint not recognized"})

if __name__ == "__main__":
    server = HTTPServer(("0.0.0.0", PORT), IntegrityFailureHandler)
    print(f"A08 Integrity Failures Service running on port {PORT}")
    server.serve_forever()
