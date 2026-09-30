import os
import json
import time
from http.server import HTTPServer, BaseHTTPRequestHandler
import urllib.parse

PORT = 8019
FLAG = "ZITERA{53cur1ty_l0gg1ng_4l3rt1ng_bl1nd5p0t}"

audit_logs = []
failed_login_count = 0
unlogged_escalation_occurred = False

class LoggingFailureHandler(BaseHTTPRequestHandler):
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
    <title>Zitera Security Operations - Audit & Telemetry Console (A09)</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: #1a1b26; color: #a9b1d6; margin: 0; padding: 24px; }}
        .container {{ max-width: 900px; margin: 0 auto; background: #24283b; border: 1px solid #414868; border-radius: 8px; padding: 24px; }}
        h1 {{ color: #f7768e; font-family: monospace; }}
        .card {{ background: #1f2335; padding: 16px; border-radius: 6px; margin-top: 16px; }}
        pre {{ background: #16161e; padding: 12px; border-radius: 4px; overflow-x: auto; color: #9ece6a; font-family: monospace; }}
        table {{ width: 100%; border-collapse: collapse; margin-top: 12px; }}
        th, td {{ border: 1px solid #414868; padding: 8px; text-align: left; font-size: 13px; }}
        th {{ background: #292e42; color: #7aa2f7; }}
    </style>
</head>
<body>
<div class="container">
    <h1>[A09:2025] Security Logging & Alerting Failures</h1>
    <p>Target Scenario: Silent Failures, Missing Audit Trails & Ineffective Security Telemetry.</p>
    <div class="card">
        <h3>SOC Alerting Status: <span style="color:#9ece6a;">MONITORING PASSIVE (0 ALERTS FIRED)</span></h3>
        <p>Telemetry Failures in this System:</p>
        <ul>
            <li>Brute force credential guessing on <code>POST /api/auth/login</code> produces NO security alert or rate-limit lock.</li>
            <li>Administrative role escalation on <code>POST /api/admin/role_escalate</code> is NOT recorded in audit logs.</li>
        </ul>
    </div>
    <div class="card">
        <h3>System Audit Log Stream (Visible Entries: {len(audit_logs)})</h3>
        <table>
            <tr><th>Timestamp</th><th>Category</th><th>Event</th><th>Severity</th></tr>
            '''
            for log in audit_logs:
                html += f"<tr><td>{log['time']}</td><td>{log['category']}</td><td>{log['event']}</td><td>{log['severity']}</td></tr>"
            html += f'''
        </table>
    </div>
</div>
</body>
</html>'''
            self.wfile.write(html.encode("utf-8"))
        elif parsed.path == "/health":
            self._send_json(200, {"status": "ok", "lab": "A09", "port": PORT})
        elif parsed.path == "/practice/verify":
            if failed_login_count >= 5 and unlogged_escalation_occurred:
                self._send_json(200, {
                    "status": "passed",
                    "message": "Security logging & alerting failure reproduced: 5 brute-force attacks and 1 silent privilege escalation triggered zero SOC alerts!"
                })
            else:
                self._send_json(200, {
                    "status": "failed",
                    "message": f"Practice target pending: Perform >= 5 failed logins (current: {failed_login_count}) and 1 unlogged role escalation."
                })
        elif parsed.path == "/api/audit/logs":
            self._send_json(200, {"logs": audit_logs, "alerts_fired": 0})
        else:
            self._send_json(404, {"error": "Not found"})

    def do_POST(self):
        global audit_logs, failed_login_count, unlogged_escalation_occurred
        parsed = urllib.parse.urlparse(self.path)
        length = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(length).decode("utf-8") if length > 0 else "{}"
        try:
            payload = json.loads(body) if body else {}
        except Exception:
            payload = {}

        if parsed.path == "/api/auth/login":
            user = payload.get("username", "guest")
            pwd = payload.get("password", "")
            if user == "admin" and pwd == "correct_horse_battery":
                self._send_json(200, {"status": "authenticated", "role": "admin"})
            else:
                # VULNERABILITY: Fails to record failed attempt in security audit trail and raises zero alerts
                failed_login_count += 1
                self._send_json(401, {"error": "Invalid credentials", "attempt": failed_login_count})
        elif parsed.path == "/api/admin/role_escalate":
            # VULNERABILITY: Silent escalation without writing to audit_logs!
            target_user = payload.get("user", "learner")
            new_role = payload.get("role", "super_admin")
            unlogged_escalation_occurred = True
            resp = {
                "status": "granted",
                "message": f"User {target_user} escalated to {new_role} silently without audit trail entry.",
                "audit_recorded": False
            }
            if failed_login_count >= 5:
                resp["flag"] = FLAG
            self._send_json(200, resp)
        elif parsed.path == "/api/reset":
            audit_logs = []
            failed_login_count = 0
            unlogged_escalation_occurred = False
            self._send_json(200, {"status": "reset", "message": "Telemetry logs and incident states reset."})
        else:
            self._send_json(404, {"error": "Endpoint not recognized"})

if __name__ == "__main__":
    server = HTTPServer(("0.0.0.0", PORT), LoggingFailureHandler)
    print(f"A09 Logging & Alerting Failures Service running on port {PORT}")
    server.serve_forever()
