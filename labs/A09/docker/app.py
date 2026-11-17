import os
import json
import time
from http.server import HTTPServer, BaseHTTPRequestHandler
import urllib.parse

PORT = 8019
FLAG = "ZITERA{53cur1ty_l0gg1ng_4l3rt1ng_bl1nd5p0t}"

audit_logs = [
    {"time": "2026-11-17 08:00:12", "category": "TELEMATICS", "event": "Fleet Vehicle #TRK-8801 GPS Ping Received (Route: I-80 East)", "severity": "INFO"},
    {"time": "2026-11-17 08:14:45", "category": "DISPATCH", "event": "Cargo Manifest #M-9912 Dispatched to Chicago Depot", "severity": "INFO"},
    {"time": "2026-11-17 08:22:10", "category": "MAINTENANCE", "event": "Vehicle #VAN-104 Tire Pressure Sensor Telemetry Verified", "severity": "INFO"}
]
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

            rows_html = ""
            for log in audit_logs:
                sev_color = "#34d399" if log['severity'] == "INFO" else "#f87171"
                rows_html += f"""
                <tr>
                    <td style="font-family: monospace;">{log['time']}</td>
                    <td><span style="font-weight: 600; color: #60a5fa;">{log['category']}</span></td>
                    <td>{log['event']}</td>
                    <td><span style="color: {sev_color}; font-weight: 600;">{log['severity']}</span></td>
                </tr>
                """

            html = f"""<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>FleetTrack Logistics | Commercial Telematics &amp; Fleet Operations</title>
    <style>
        :root {{
            --bg-primary: #0a0d18;
            --bg-secondary: #111728;
            --bg-tertiary: #1c2438;
            --border: #2d3748;
            --accent: #f59e0b;
            --accent-hover: #d97706;
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
            background: linear-gradient(135deg, #f59e0b, #ea580c);
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
        table {{ width: 100%; border-collapse: collapse; font-size: 0.875rem; }}
        th {{ text-align: left; padding: 0.75rem 1rem; background: var(--bg-tertiary); color: var(--text-muted); font-weight: 600; border-bottom: 1px solid var(--border); }}
        td {{ padding: 0.875rem 1rem; border-bottom: 1px solid var(--border); }}
        tr:hover td {{ background-color: rgba(255, 255, 255, 0.02); }}
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
            <div class="brand-icon">FT</div>
            <span>FleetTrack Logistics</span>
        </a>
        <nav class="nav-links">
            <a href="/" class="nav-link">Telematics Command</a>
            <a href="/api/audit/logs" class="nav-link">Audit Stream API</a>
            <span style="font-size: 0.825rem; color: var(--text-muted);">Operations Center: <strong>Fleet Hub West</strong></span>
        </nav>
    </header>

    <main class="container">
        <div class="breadcrumbs">
            <a href="/">Home</a>
            <span>/</span>
            <a href="/">Telematics Fleet Command</a>
            <span>/</span>
            <span>Security Logging &amp; Audit Telemetry</span>
        </div>

        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 1rem; margin-bottom: 1.5rem;">
            <div class="card" style="margin-bottom: 0;">
                <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Active Commercial Fleet</div>
                <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem; color: var(--success);">340 Units Live</div>
                <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.25rem;">Cellular &amp; Satellite IoT</div>
            </div>
            <div class="card" style="margin-bottom: 0;">
                <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Security Alerting Engine</div>
                <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem; color: var(--warning);">Passive Telemetry</div>
                <div style="font-size: 0.8rem; color: var(--text-muted); margin-top: 0.25rem;">0 Security Incidents Triggered</div>
            </div>
            <div class="card" style="margin-bottom: 0;">
                <div style="font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase;">Dispatcher Gateway</div>
                <div style="font-size: 1.25rem; font-weight: 700; margin-top: 0.25rem;">Port 8019</div>
                <div style="font-size: 0.8rem; color: #60a5fa; margin-top: 0.25rem;">Auth &amp; Dispatcher API</div>
            </div>
        </div>

        <div class="card">
            <div class="card-header">
                <div>
                    <h2 class="card-title">Commercial Telematics &amp; Operations Audit Trail</h2>
                    <p style="font-size: 0.825rem; color: var(--text-muted); margin-top: 0.2rem;">
                        Real-time log events captured by fleet gateway telemetry listeners.
                    </p>
                </div>
                <a href="/api/audit/logs" class="btn btn-outline" style="font-size: 0.8rem;">GET /api/audit/logs &rarr;</a>
            </div>

            <table>
                <thead>
                    <tr>
                        <th>Timestamp (UTC)</th>
                        <th>Subsystem</th>
                        <th>Event Description</th>
                        <th>Severity</th>
                    </tr>
                </thead>
                <tbody>
                    {rows_html}
                </tbody>
            </table>
        </div>
    </main>

    <footer class="footer">
        <p>&copy; 2026 FleetTrack Logistics Inc. All rights reserved. &bull; Enterprise Telematics &amp; Fleet Asset Intelligence</p>
    </footer>
</body>
</html>"""
            self.wfile.write(html.encode("utf-8"))
        elif parsed.path == "/health":
            self._send_json(200, {"status": "ok", "service": "FleetTrack Logistics", "port": PORT})
        elif parsed.path == "/practice/verify":
            if failed_login_count >= 5 and unlogged_escalation_occurred:
                self._send_json(200, {
                    "status": "passed",
                    "message": "Security logging & alerting failure reproduced: 5 brute-force attempts and 1 unlogged role escalation triggered zero SOC alerts!"
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
                self._send_json(200, {"status": "authenticated", "role": "FleetSupervisor"})
            else:
                # Logging failure: Does not emit security telemetry into audit trail
                failed_login_count += 1
                self._send_json(401, {"error": "Invalid dispatcher credentials", "attempt": failed_login_count})
        elif parsed.path == "/api/admin/role_escalate":
            # Logging failure: Silent privilege escalation without audit log entry
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
            audit_logs = [
                {"time": "2026-11-17 08:00:12", "category": "TELEMATICS", "event": "Fleet Vehicle #TRK-8801 GPS Ping Received (Route: I-80 East)", "severity": "INFO"},
                {"time": "2026-11-17 08:14:45", "category": "DISPATCH", "event": "Cargo Manifest #M-9912 Dispatched to Chicago Depot", "severity": "INFO"},
                {"time": "2026-11-17 08:22:10", "category": "MAINTENANCE", "event": "Vehicle #VAN-104 Tire Pressure Sensor Telemetry Verified", "severity": "INFO"}
            ]
            failed_login_count = 0
            unlogged_escalation_occurred = False
            self._send_json(200, {"status": "reset", "message": "Telemetry logs and incident states reset."})
        else:
            self._send_json(404, {"error": "Endpoint not recognized"})

if __name__ == "__main__":
    server = HTTPServer(("0.0.0.0", PORT), LoggingFailureHandler)
    print(f"FleetTrack Logistics service running on port {PORT}")
    server.serve_forever()
