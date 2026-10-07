// A09: Security Logging & Alerting Failures — Native Sandboxed Lab Binary
// Simulates a Security Operations Center (SOC) telemetry and audit logging service:
//   - Brute force authentication attempts on /api/auth/login generate zero SOC alerts
//   - Privilege / role escalations on /api/admin/role_escalate are completely omitted from audit logs
//   - Master flag is revealed once 5+ brute-force attempts and a silent escalation occur
//
// Communicates via stdio JSON protocol (StdioMessage format).

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::process::exit;

const FLAG: &str = "ZITERA{53cur1ty_l0gg1ng_4l3rt1ng_bl1nd5p0t}";
const LAB_ID: &str = "A09";
const LAB_VERSION: &str = "1.0.0";

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct AuditLogEntry {
    time: String,
    category: String,
    event: String,
    severity: String,
}

struct AppState {
    audit_logs: Vec<AuditLogEntry>,
    failed_login_count: u32,
    unlogged_escalation_occurred: bool,
}

impl AppState {
    fn new() -> Self {
        Self {
            audit_logs: Vec::new(),
            failed_login_count: 0,
            unlogged_escalation_occurred: false,
        }
    }

    fn reset(&mut self) {
        self.audit_logs.clear();
        self.failed_login_count = 0;
        self.unlogged_escalation_occurred = false;
    }
}

fn css() -> &'static str {
    r#"
body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #0b0f19; color: #f1f5f9; margin: 0; padding: 32px; }
.card { background: #131b2e; border: 1px solid #1e293b; border-radius: 12px; padding: 24px; max-width: 820px; margin: 0 auto; box-shadow: 0 10px 25px rgba(0,0,0,0.5); }
.header { border-bottom: 1px solid #1e293b; padding-bottom: 16px; margin-bottom: 20px; display: flex; justify-content: space-between; align-items: center; }
.badge { background: #ef4444; color: #fef2f2; padding: 4px 10px; border-radius: 4px; font-size: 12px; font-weight: bold; }
.banner { background: #3b0764; border-left: 4px solid #a855f7; padding: 14px; margin-bottom: 20px; font-size: 13px; color: #f3e8ff; line-height: 1.5; border-radius: 4px; }
.status-pill { display: inline-block; padding: 4px 12px; border-radius: 9999px; font-size: 12px; font-weight: bold; margin-bottom: 12px; }
.status-passive { background: #1e293b; color: #94a3b8; border: 1px solid #334155; }
.status-alert { background: #7f1d1d; color: #fecaca; }
.flag-box { background: #064e3b; border: 2px dashed #10b981; border-radius: 8px; padding: 16px; margin-top: 18px; color: #a7f3d0; font-family: monospace; font-size: 15px; font-weight: bold; text-align: center; }
table { width: 100%; border-collapse: collapse; margin-top: 14px; font-size: 13px; }
th, td { border: 1px solid #1e293b; padding: 10px; text-align: left; }
th { background: #090d16; color: #38bdf8; font-weight: 600; }
tr:nth-child(even) { background: #0e1526; }
button { padding: 10px 16px; border-radius: 6px; font-weight: bold; cursor: pointer; border: none; font-size: 13px; }
.btn-primary { background: #ef4444; color: white; }
.btn-primary:hover { background: #dc2626; }
.btn-secondary { background: #6366f1; color: white; }
.btn-secondary:hover { background: #4f46e5; }
.btn-danger { background: #334155; color: white; }
.btn-danger:hover { background: #475569; }
.grid { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; }
@media (max-width: 640px) { .grid { grid-template-columns: 1fr; } }
.metric-box { background: #090d16; border: 1px solid #1e293b; border-radius: 8px; padding: 16px; text-align: center; }
.metric-num { font-size: 28px; font-weight: bold; color: #f87171; margin-top: 4px; font-family: monospace; }
"#
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn parse_form_body(body: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in body.split('&') {
        let mut parts = pair.splitn(2, '=');
        if let Some(key) = parts.next() {
            let val = parts.next().unwrap_or("");
            map.insert(urldecode(key), urldecode(val));
        }
    }
    map
}

fn urldecode(input: &str) -> String {
    let mut result = String::new();
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(val) = u8::from_str_radix(std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or(""), 16) {
                result.push(val as char);
                i += 3;
                continue;
            }
        } else if bytes[i] == b'+' {
            result.push(' ');
            i += 1;
            continue;
        }
        result.push(bytes[i] as char);
        i += 1;
    }
    result
}

fn render_index_page(state: &AppState, banner_msg: Option<&str>) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><title>Zitera Security Operations — Audit & Telemetry Console (A09)</title>");
    html.push_str("<style>");
    html.push_str(css());
    html.push_str("</style></head><body><div class=\"card\">");

    html.push_str("<div class=\"header\"><div><h2 style=\"margin: 0;\">Zitera SOC // Telemetry & Audit Console</h2>");
    html.push_str("<div style=\"color: #94a3b8; font-size: 12px; margin-top: 4px;\">OWASP A09:2025 • Security Logging & Alerting Failures</div></div>");
    html.push_str("<span class=\"badge\">LAB A09 • PORT 8019</span></div>");

    html.push_str("<div class=\"banner\"><strong>LOGGING & ALERTING BLINDSPOTS:</strong> Authentication failures on <code>POST /api/auth/login</code> do not trigger alerts or threshold locks. High-privilege role escalations on <code>POST /api/admin/role_escalate</code> are executed completely unrecorded in the audit stream.</div>");

    if let Some(msg) = banner_msg {
        html.push_str(&format!("<div style=\"background: #1e293b; border-left: 4px solid #38bdf8; padding: 12px; margin-bottom: 20px; font-size: 13px; color: #f1f5f9; border-radius: 4px;\">ℹ {}</div>", html_escape(msg)));
    }

    let is_unlocked = state.failed_login_count >= 5 && state.unlogged_escalation_occurred;
    if is_unlocked {
        html.push_str("<div class=\"flag-box\">");
        html.push_str("<div>TELEMETRY BLINDSPOT REPRODUCED! 5 FAILED LOGINS & SILENT ESCALATION WITHOUT SOC DETECTION:</div>");
        html.push_str(&format!("<div style=\"margin-top: 8px; font-size: 16px;\">{}</div>", FLAG));
        html.push_str("</div><br/>");
    }

    html.push_str("<div class=\"grid\">");
    html.push_str("<div class=\"metric-box\"><div style=\"font-size: 13px; color: #94a3b8;\">Unlogged Failed Logins</div>");
    html.push_str(&format!("<div class=\"metric-num\">{}</div>", state.failed_login_count));
    html.push_str("<div style=\"font-size: 11px; color: #64748b; margin-top: 4px;\">Threshold required: 5+ attempts</div></div>");

    html.push_str("<div class=\"metric-box\"><div style=\"font-size: 13px; color: #94a3b8;\">Silent Role Escalation</div>");
    let esc_label = if state.unlogged_escalation_occurred { "OCCURRED (UNLOGGED)" } else { "NONE" };
    let esc_color = if state.unlogged_escalation_occurred { "#34d399" } else { "#94a3b8" };
    html.push_str(&format!("<div class=\"metric-num\" style=\"color: {}; font-size: 18px; margin-top: 10px;\">{}</div>", esc_color, esc_label));
    html.push_str("<div style=\"font-size: 11px; color: #64748b; margin-top: 4px;\">Zero audit log entries recorded</div></div>");
    html.push_str("</div>");

    html.push_str("<div style=\"margin-top: 20px;\"><h3 style=\"margin-bottom: 10px; font-size: 15px; color: #38bdf8;\">Telemetry Trigger Simulation</h3>");
    html.push_str("<div style=\"display: flex; gap: 10px; flex-wrap: wrap;\">");
    html.push_str("<form method=\"POST\" action=\"/api/auth/login\" style=\"margin: 0;\">");
    html.push_str("<input type=\"hidden\" name=\"username\" value=\"admin\">");
    html.push_str("<input type=\"hidden\" name=\"password\" value=\"wrong_password\">");
    html.push_str("<button type=\"submit\" class=\"btn-primary\">Simulate Failed Login</button></form>");

    html.push_str("<form method=\"POST\" action=\"/api/admin/role_escalate\" style=\"margin: 0;\">");
    html.push_str("<input type=\"hidden\" name=\"user\" value=\"learner\">");
    html.push_str("<input type=\"hidden\" name=\"role\" value=\"super_admin\">");
    html.push_str("<button type=\"submit\" class=\"btn-secondary\">Simulate Silent Escalation</button></form>");

    html.push_str("<form method=\"POST\" action=\"/api/reset\" style=\"margin: 0;\">");
    html.push_str("<button type=\"submit\" class=\"btn-danger\">Reset Incident States</button></form>");
    html.push_str("</div></div>");

    html.push_str("<div style=\"margin-top: 24px;\">");
    html.push_str(&format!("<h3 style=\"margin-bottom: 8px; font-size: 15px; color: #38bdf8;\">SOC Audit Trail (Entries: {})</h3>", state.audit_logs.len()));
    if state.audit_logs.is_empty() {
        html.push_str("<div style=\"background: #090d16; border: 1px solid #1e293b; border-radius: 6px; padding: 20px; text-align: center; color: #64748b; font-size: 13px;\">No audit logs recorded. The system is operating in complete telemetry darkness.</div>");
    } else {
        html.push_str("<table><thead><tr><th>Time</th><th>Category</th><th>Event</th><th>Severity</th></tr></thead><tbody>");
        for log in &state.audit_logs {
            html.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                html_escape(&log.time),
                html_escape(&log.category),
                html_escape(&log.event),
                html_escape(&log.severity)
            ));
        }
        html.push_str("</tbody></table>");
    }
    html.push_str("</div>");

    html.push_str("<div style=\"margin-top: 24px; padding-top: 16px; border-top: 1px solid #1e293b; font-size: 12px; color: #64748b;\">");
    html.push_str("API Endpoints: <code>POST /api/auth/login</code>, <code>POST /api/admin/role_escalate</code>, <code>GET /api/audit/logs</code>, <code>GET /practice/verify</code>, <code>POST /api/reset</code>.");
    html.push_str("</div></div></body></html>");
    html
}

fn handle_request(
    state: &mut AppState,
    method: &str,
    path: &str,
    headers: &HashMap<String, String>,
    body: &str,
) -> (u16, HashMap<String, String>, String) {
    let mut resp_headers = HashMap::new();
    let clean_path = path.split('?').next().unwrap_or("/");

    if clean_path == "/health" {
        resp_headers.insert("content-type".to_string(), "application/json".to_string());
        let body = serde_json::json!({
            "status": "ok",
            "lab": LAB_ID,
            "port": 8019
        })
        .to_string();
        return (200, resp_headers, body);
    }

    if clean_path == "/" || clean_path == "/index.html" {
        resp_headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
        return (200, resp_headers, render_index_page(state, None));
    }

    if clean_path == "/api/audit/logs" && method == "GET" {
        resp_headers.insert("content-type".to_string(), "application/json".to_string());
        let body = serde_json::json!({
            "logs": state.audit_logs,
            "alerts_fired": 0
        })
        .to_string();
        return (200, resp_headers, body);
    }

    if clean_path == "/practice/verify" && method == "GET" {
        resp_headers.insert("content-type".to_string(), "application/json".to_string());
        if state.failed_login_count >= 5 && state.unlogged_escalation_occurred {
            let resp = serde_json::json!({
                "status": "passed",
                "message": "Security logging & alerting failure reproduced: 5 brute-force attacks and 1 silent privilege escalation triggered zero SOC alerts!"
            });
            return (200, resp_headers, resp.to_string());
        } else {
            let resp = serde_json::json!({
                "status": "failed",
                "message": format!(
                    "Practice target pending: Perform >= 5 failed logins (current: {}) and 1 unlogged role escalation.",
                    state.failed_login_count
                )
            });
            return (200, resp_headers, resp.to_string());
        }
    }

    if clean_path == "/api/auth/login" && method == "POST" {
        let is_form = headers.get("content-type")
            .map(|ct| ct.contains("application/x-www-form-urlencoded"))
            .unwrap_or(false);

        let (user, pwd) = if is_form {
            let form = parse_form_body(body);
            let u = form.get("username").cloned().unwrap_or_else(|| "guest".to_string());
            let p = form.get("password").cloned().unwrap_or_default();
            (u, p)
        } else if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(body) {
            let u = json_val.get("username").and_then(|v| v.as_str()).unwrap_or("guest").to_string();
            let p = json_val.get("password").and_then(|v| v.as_str()).unwrap_or("").to_string();
            (u, p)
        } else {
            ("guest".to_string(), String::new())
        };

        if user == "admin" && pwd == "correct_horse_battery" {
            if is_form {
                resp_headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
                return (200, resp_headers, render_index_page(state, Some("Authenticated as admin.")));
            } else {
                resp_headers.insert("content-type".to_string(), "application/json".to_string());
                let resp = serde_json::json!({
                    "status": "authenticated",
                    "role": "admin"
                });
                return (200, resp_headers, resp.to_string());
            }
        } else {
            // VULNERABILITY: Fails to record failed attempt in security audit trail and raises zero alerts
            state.failed_login_count += 1;
            if is_form {
                resp_headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
                return (200, resp_headers, render_index_page(state, Some(&format!("Failed login attempt recorded (Total unlogged: {}). Zero alerts fired.", state.failed_login_count))));
            } else {
                resp_headers.insert("content-type".to_string(), "application/json".to_string());
                let resp = serde_json::json!({
                    "error": "Invalid credentials",
                    "attempt": state.failed_login_count
                });
                return (401, resp_headers, resp.to_string());
            }
        }
    }

    if clean_path == "/api/admin/role_escalate" && method == "POST" {
        let is_form = headers.get("content-type")
            .map(|ct| ct.contains("application/x-www-form-urlencoded"))
            .unwrap_or(false);

        let (target_user, new_role) = if is_form {
            let form = parse_form_body(body);
            let u = form.get("user").cloned().unwrap_or_else(|| "learner".to_string());
            let r = form.get("role").cloned().unwrap_or_else(|| "super_admin".to_string());
            (u, r)
        } else if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(body) {
            let u = json_val.get("user").and_then(|v| v.as_str()).unwrap_or("learner").to_string();
            let r = json_val.get("role").and_then(|v| v.as_str()).unwrap_or("super_admin").to_string();
            (u, r)
        } else {
            ("learner".to_string(), "super_admin".to_string())
        };

        // VULNERABILITY: Silent escalation without writing to audit_logs!
        state.unlogged_escalation_occurred = true;

        if is_form {
            resp_headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
            let msg = format!("User {} escalated to {} silently without audit trail entry.", target_user, new_role);
            return (200, resp_headers, render_index_page(state, Some(&msg)));
        } else {
            resp_headers.insert("content-type".to_string(), "application/json".to_string());
            let mut resp = serde_json::json!({
                "status": "granted",
                "message": format!("User {} escalated to {} silently without audit trail entry.", target_user, new_role),
                "audit_recorded": false
            });
            if state.failed_login_count >= 5 {
                if let Some(map) = resp.as_object_mut() {
                    map.insert("flag".to_string(), serde_json::json!(FLAG));
                }
            }
            return (200, resp_headers, resp.to_string());
        }
    }

    if clean_path == "/api/reset" && method == "POST" {
        state.reset();
        let is_form = headers.get("content-type")
            .map(|ct| ct.contains("application/x-www-form-urlencoded"))
            .unwrap_or(false);

        if is_form {
            resp_headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
            return (200, resp_headers, render_index_page(state, Some("Telemetry logs and incident states reset.")));
        } else {
            resp_headers.insert("content-type".to_string(), "application/json".to_string());
            let resp = serde_json::json!({
                "status": "reset",
                "message": "Telemetry logs and incident states reset."
            });
            return (200, resp_headers, resp.to_string());
        }
    }

    resp_headers.insert("content-type".to_string(), "application/json".to_string());
    (404, resp_headers, serde_json::json!({"error": "Endpoint not recognized"}).to_string())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--help" || a == "-h" || a == "/?") {
        println!("Zitera Sandboxed Lab A09 (Security Logging and Monitoring Failures) v{}", LAB_VERSION);
        println!("This native lab communicates with the host engine broker via stdin/stdout JSON IPC.");
        println!("Usage: a09-lab.exe");
        return;
    }
    if args.iter().any(|a| a == "--version" || a == "-v" || a == "-V") {
        println!("A09 v{} (native_sandboxed)", LAB_VERSION);
        return;
    }

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    let mut state = AppState::new();

    // 1. Emit READY handshake
    let ready = serde_json::json!({
        "type": "READY",
        "lab_id": LAB_ID,
        "version": LAB_VERSION
    });
    let _ = writeln!(stdout, "{}", ready);
    let _ = stdout.flush();

    // 2. Stdio message loop
    let mut lines = stdin.lock().lines();
    while let Some(Ok(line)) = lines.next() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let val: serde_json::Value = match serde_json::from_str(trimmed) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let msg_type = val.get("type").and_then(|v| v.as_str()).unwrap_or("");

        match msg_type {
            "HTTP_REQUEST" => {
                let id = val.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
                let method = val.get("method").and_then(|v| v.as_str()).unwrap_or("GET");
                let path = val.get("path").and_then(|v| v.as_str()).unwrap_or("/");
                let body = val.get("body").and_then(|v| v.as_str()).unwrap_or("");

                let mut headers: HashMap<String, String> = HashMap::new();
                if let Some(h_obj) = val.get("headers").and_then(|v| v.as_object()) {
                    for (k, v) in h_obj {
                        if let Some(s) = v.as_str() {
                            headers.insert(k.to_lowercase(), s.to_string());
                        }
                    }
                }

                let (status, resp_headers, resp_body) =
                    handle_request(&mut state, method, path, &headers, body);

                let resp = serde_json::json!({
                    "type": "HTTP_RESPONSE",
                    "id": id,
                    "status": status,
                    "headers": resp_headers,
                    "body": resp_body
                });
                let _ = writeln!(stdout, "{}", resp);
                let _ = stdout.flush();
            }
            "HEALTH" => {
                let id = val.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
                let resp = serde_json::json!({
                    "type": "STATUS",
                    "id": id,
                    "status": "ok",
                    "lab_id": LAB_ID
                });
                let _ = writeln!(stdout, "{}", resp);
                let _ = stdout.flush();
            }
            "PRACTICE" => {
                let id = val.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
                let success = state.failed_login_count >= 5 && state.unlogged_escalation_occurred;
                let message = if success {
                    "Practice verified: Security logging and alerting failure demonstrated!"
                } else {
                    "Practice incomplete: Perform >= 5 failed logins and 1 unlogged role escalation."
                };
                let resp = serde_json::json!({
                    "type": "PRACTICE_RESULT",
                    "id": id,
                    "success": success,
                    "message": message
                });
                let _ = writeln!(stdout, "{}", resp);
                let _ = stdout.flush();
            }
            "CHALLENGE" => {
                let id = val.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
                let candidate = val
                    .get("flag")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .trim();
                let clean_candidate = candidate.strip_prefix("FLAG: ").unwrap_or(candidate).trim();
                let valid = clean_candidate == FLAG;
                let resp = serde_json::json!({
                    "type": "CHALLENGE_RESULT",
                    "id": id,
                    "valid": valid,
                    "message": if valid {
                        "Correct! Security logging and alerting failures challenge complete."
                    } else {
                        "Incorrect flag. Hint: Trigger 5 failed logins, then silently escalate a role via /api/admin/role_escalate."
                    }
                });
                let _ = writeln!(stdout, "{}", resp);
                let _ = stdout.flush();
            }
            "STOP" => {
                let resp = serde_json::json!({ "type": "CLEAN_EXIT" });
                let _ = writeln!(stdout, "{}", resp);
                let _ = stdout.flush();
                exit(0);
            }
            _ => {}
        }
    }
    exit(0);
}
