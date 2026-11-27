// A09: Security Logging & Alerting Failures — FleetTrack Logistics
// Commercial Telematics & Global Fleet Management System
// Simulates an enterprise logistics telematics platform with intentional monitoring blindspots:
//   - Zero rate-limit alerting or SOC threshold detection during credential attacks
//   - High-privilege route override and role escalations occur completely unlogged
//   - Interactive fleet vehicle dispatch CRUD and telematics audit stream
//   - Master flag is revealed once blindspot conditions (5+ failed logins and silent escalation) are met
//
// Communicates via stdio JSON protocol (StdioMessage format).

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::process::exit;

const FLAG: &str = "ZITERA{53cur1ty_l0gg1ng_4l3rt1ng_bl1nd5p0t}";
const LAB_ID: &str = "A09";
const LAB_VERSION: &str = "2.0.0";

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct FleetVehicle {
    id: u32,
    unit_code: String,
    driver: String,
    origin_dest: String,
    cargo: String,
    status: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct AuditLogEntry {
    time: String,
    category: String,
    event: String,
    severity: String,
}

struct AppState {
    vehicles: Vec<FleetVehicle>,
    audit_logs: Vec<AuditLogEntry>,
    failed_login_count: u32,
    unlogged_escalation_occurred: bool,
}

impl AppState {
    fn new() -> Self {
        Self {
            vehicles: vec![
                FleetVehicle {
                    id: 1,
                    unit_code: "TRK-901".to_string(),
                    driver: "Alex Murphy".to_string(),
                    origin_dest: "Seattle -> Portland".to_string(),
                    cargo: "Precision Medical Hardware".to_string(),
                    status: "In Transit".to_string(),
                },
                FleetVehicle {
                    id: 2,
                    unit_code: "TRK-404".to_string(),
                    driver: "Carlos Gomez".to_string(),
                    origin_dest: "Dallas -> Austin".to_string(),
                    cargo: "Cold-Chain Pharmaceuticals".to_string(),
                    status: "Delivered".to_string(),
                },
                FleetVehicle {
                    id: 3,
                    unit_code: "TRK-102".to_string(),
                    driver: "Sarah Connor".to_string(),
                    origin_dest: "Chicago -> Detroit".to_string(),
                    cargo: "Industrial Microcontrollers".to_string(),
                    status: "Depot Standby".to_string(),
                },
            ],
            audit_logs: vec![
                AuditLogEntry {
                    time: "10:14:02".to_string(),
                    category: "TELEMATICS".to_string(),
                    event: "GPS waypoint ping TRK-901 verified (Route I-5)".to_string(),
                    severity: "INFO".to_string(),
                },
                AuditLogEntry {
                    time: "10:28:44".to_string(),
                    category: "DISPATCH".to_string(),
                    event: "Scheduled delivery confirmation TRK-404 received".to_string(),
                    severity: "INFO".to_string(),
                },
            ],
            failed_login_count: 0,
            unlogged_escalation_occurred: false,
        }
    }

    fn reset(&mut self) {
        self.failed_login_count = 0;
        self.unlogged_escalation_occurred = false;
    }
}

fn css() -> &'static str {
    r#"
:root {
    --bg-page: #f8fafc;
    --card-bg: #ffffff;
    --text-primary: #0f172a;
    --text-secondary: #475569;
    --text-muted: #94a3b8;
    --border-color: #e2e8f0;
    --accent: #2563eb;
    --accent-hover: #1d4ed8;
    --accent-light: #eff6ff;
    --success: #10b981;
    --success-light: #ecfdf5;
    --danger: #ef4444;
    --danger-light: #fef2f2;
    --warning: #f59e0b;
    --warning-light: #fffbeb;
}
* { box-sizing: border-box; margin: 0; padding: 0; }
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
    background-color: var(--bg-page);
    color: var(--text-primary);
    line-height: 1.5;
    padding: 32px 20px;
}
.container { max-width: 1040px; margin: 0 auto; }
.card {
    background: var(--card-bg);
    border: 1px solid var(--border-color);
    border-radius: 12px;
    box-shadow: 0 1px 3px rgba(0,0,0,0.04), 0 1px 2px rgba(0,0,0,0.02);
    padding: 24px 28px;
    margin-bottom: 24px;
}
.header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid var(--border-color);
    padding-bottom: 20px;
    margin-bottom: 24px;
}
.header-title h1 { font-size: 20px; font-weight: 700; color: var(--text-primary); display: flex; align-items: center; gap: 8px; }
.header-title p { font-size: 13px; color: var(--text-secondary); margin-top: 4px; }
.badge {
    display: inline-flex;
    align-items: center;
    padding: 4px 10px;
    font-size: 12px;
    font-weight: 600;
    border-radius: 9999px;
}
.badge-blue { background: var(--accent-light); color: var(--accent); border: 1px solid #bfdbfe; }
.badge-emerald { background: var(--success-light); color: var(--success); border: 1px solid #a7f3d0; }
.badge-amber { background: var(--warning-light); color: #d97706; border: 1px solid #fde68a; }
.badge-rose { background: var(--danger-light); color: var(--danger); border: 1px solid #fecaca; }
.banner-info {
    background: #eff6ff;
    border-left: 4px solid var(--accent);
    padding: 14px 18px;
    border-radius: 6px;
    font-size: 13px;
    color: #1e3a8a;
    margin-bottom: 24px;
    line-height: 1.6;
}
.banner-notice {
    background: #f1f5f9;
    border-left: 4px solid #64748b;
    padding: 12px 16px;
    border-radius: 6px;
    font-size: 13px;
    color: var(--text-primary);
    margin-bottom: 20px;
}
.grid-2 { display: grid; grid-template-columns: 1fr 1fr; gap: 24px; }
@media (max-width: 860px) { .grid-2 { grid-template-columns: 1fr; } }
.table-wrap { overflow-x: auto; border: 1px solid var(--border-color); border-radius: 8px; }
table { width: 100%; border-collapse: collapse; text-align: left; font-size: 13px; }
th { background: #f1f5f9; padding: 12px 14px; font-weight: 600; color: var(--text-secondary); border-bottom: 1px solid var(--border-color); }
td { padding: 12px 14px; border-bottom: 1px solid var(--border-color); color: var(--text-primary); }
tr:last-child td { border-bottom: none; }
tr:hover td { background: #f8fafc; }
.stat-box {
    background: #f8fafc;
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 16px;
    text-align: center;
}
.stat-num { font-size: 26px; font-weight: 700; color: var(--accent); margin-top: 4px; font-family: ui-monospace, monospace; }
.field { margin-bottom: 14px; }
label { display: block; font-size: 12px; font-weight: 600; color: var(--text-secondary); margin-bottom: 6px; }
input[type="text"] {
    width: 100%;
    padding: 10px 12px;
    background: #ffffff;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    font-size: 13px;
    color: var(--text-primary);
    outline: none;
    transition: border-color 0.15s ease;
}
input:focus { border-color: var(--accent); ring: 2px solid #bfdbfe; }
.btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 10px 16px;
    font-size: 13px;
    font-weight: 600;
    border-radius: 6px;
    cursor: pointer;
    border: none;
    text-decoration: none;
    transition: background-color 0.15s ease;
}
.btn-primary { background: var(--accent); color: white; }
.btn-primary:hover { background: var(--accent-hover); }
.btn-secondary { background: #f1f5f9; color: var(--text-secondary); border: 1px solid var(--border-color); }
.btn-secondary:hover { background: #e2e8f0; }
.btn-danger { background: var(--danger); color: white; }
.btn-danger:hover { background: #dc2626; }
.btn-purple { background: #7c3aed; color: white; }
.btn-purple:hover { background: #6d28d9; }
.flag-box {
    background: #ecfdf5;
    border: 2px dashed #10b981;
    border-radius: 8px;
    padding: 18px;
    margin: 20px 0;
    color: #065f46;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 15px;
    font-weight: 700;
    text-align: center;
}
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
            if let Ok(val) =
                u8::from_str_radix(std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or(""), 16)
            {
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
    html.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\">");
    html.push_str("<title>FleetTrack Logistics — Telematics & Fleet Console</title>");
    html.push_str("<style>");
    html.push_str(css());
    html.push_str("</style></head><body><div class=\"container\">");

    // Header
    html.push_str("<div class=\"card\"><div class=\"header\">");
    html.push_str("<div class=\"header-title\"><h1>[LOGISTICS] FleetTrack Logistics</h1>");
    html.push_str("<p>Commercial Telematics & Fleet Operations Console (OWASP A09:2025)</p></div>");
    html.push_str("<div style=\"display: flex; gap: 8px; align-items: center;\">");
    html.push_str("<span class=\"badge badge-blue\">PORT 8019</span>");
    if state.failed_login_count >= 5 && state.unlogged_escalation_occurred {
        html.push_str("<span class=\"badge badge-rose\">SOC BLINDSPOT CONFIRMED</span>");
    } else {
        html.push_str("<span class=\"badge badge-amber\">SOC TELEMETRY PASSIVE</span>");
    }
    html.push_str("</div></div>");

    // Banner
    html.push_str("<div class=\"banner-info\">");
    html.push_str("<strong>SECURITY MONITORING AUDIT DIRECTIVE:</strong> FleetTrack monitors long-haul container trucks and warehouse operations. ");
    html.push_str("Critical operational blindspots exist in the telemetry layer: brute-force login attempts at <code>POST /api/auth/login</code> do not increment SIEM alert counters or trigger automated account lockouts. ");
    html.push_str("Furthermore, privilege and role escalation events at <code>POST /api/admin/role_escalate</code> completely evade the audit trail without creating log entries.");
    html.push_str("</div>");

    if let Some(msg) = banner_msg {
        html.push_str(&format!(
            "<div class=\"banner-notice\">ℹ {}</div>",
            html_escape(msg)
        ));
    }

    let is_unlocked = state.failed_login_count >= 5 && state.unlogged_escalation_occurred;
    if is_unlocked {
        html.push_str("<div class=\"flag-box\">");
        html.push_str("<div style=\"font-size: 12px; text-transform: uppercase; letter-spacing: 0.05em; margin-bottom: 6px;\">TELEMETRY BLINDSPOT VERIFIED // 5 FAILED LOGINS & SILENT PRIVILEGE ESCALATION WITHOUT SOC ALERT:</div>");
        html.push_str(&format!("<div style=\"font-size: 16px;\">{}</div>", FLAG));
        html.push_str("</div>");
    }

    // Grid 2 Columns
    html.push_str("<div class=\"grid-2\">");

    // Column 1: Fleet Roster & Dispatch Form
    html.push_str("<div>");
    html.push_str("<h2 style=\"font-size: 15px; font-weight: 700; margin-bottom: 12px;\">Active Commercial Fleet</h2>");
    html.push_str("<div class=\"table-wrap\"><table><thead><tr>");
    html.push_str("<th>Unit</th><th>Driver</th><th>Route</th><th>Status</th>");
    html.push_str("</tr></thead><tbody>");
    for v in &state.vehicles {
        let badge_class = match v.status.as_str() {
            "In Transit" => "badge-blue",
            "Delivered" => "badge-emerald",
            _ => "badge-amber",
        };
        html.push_str("<tr>");
        html.push_str(&format!(
            "<td><strong>{}</strong></td>",
            html_escape(&v.unit_code)
        ));
        html.push_str(&format!("<td>{}</td>", html_escape(&v.driver)));
        html.push_str(&format!("<td><span style=\"font-size: 12px;\">{}</span><br/><span style=\"font-size: 11px; color: var(--text-muted);\">{}</span></td>", html_escape(&v.origin_dest), html_escape(&v.cargo)));
        html.push_str(&format!(
            "<td><span class=\"badge {}\">{}</span></td>",
            badge_class,
            html_escape(&v.status)
        ));
        html.push_str("</tr>");
    }
    html.push_str("</tbody></table></div>");

    // Dispatch Vehicle Form
    html.push_str("<div style=\"margin-top: 20px; padding-top: 16px; border-top: 1px solid var(--border-color);\">");
    html.push_str("<h3 style=\"font-size: 13px; font-weight: 700; margin-bottom: 10px;\">Dispatch Vehicle Route</h3>");
    html.push_str("<form method=\"POST\" action=\"/fleet/dispatch\">");
    html.push_str("<div style=\"display: grid; grid-template-columns: 1fr 1fr; gap: 10px;\">");
    html.push_str("<div class=\"field\"><label>Vehicle Unit Code</label><input type=\"text\" name=\"unit_code\" placeholder=\"e.g. TRK-708\" required></div>");
    html.push_str("<div class=\"field\"><label>Assigned Driver</label><input type=\"text\" name=\"driver\" placeholder=\"e.g. John Doe\" required></div>");
    html.push_str("</div>");
    html.push_str("<div class=\"field\"><label>Transit Corridor (Origin -> Destination)</label><input type=\"text\" name=\"route\" placeholder=\"e.g. Denver -> Phoenix\" required></div>");
    html.push_str("<div class=\"field\"><label>Cargo Description</label><input type=\"text\" name=\"cargo\" placeholder=\"e.g. High-density server racks\" required></div>");
    html.push_str("<button type=\"submit\" class=\"btn btn-secondary\" style=\"width: 100%;\">Dispatch Route</button>");
    html.push_str("</form></div>");

    html.push_str("</div>");

    // Column 2: SOC Blindspot Testing & Telemetry Audit Stream
    html.push_str("<div>");
    html.push_str("<h2 style=\"font-size: 15px; font-weight: 700; margin-bottom: 12px;\">SOC Telemetry Status</h2>");

    html.push_str("<div style=\"display: grid; grid-template-columns: 1fr 1fr; gap: 12px; margin-bottom: 16px;\">");
    html.push_str("<div class=\"stat-box\">");
    html.push_str("<div style=\"font-size: 11px; text-transform: uppercase; color: var(--text-muted); font-weight: 700;\">Unlogged Failed Logins</div>");
    html.push_str(&format!(
        "<div class=\"stat-num\">{}</div>",
        state.failed_login_count
    ));
    html.push_str("<div style=\"font-size: 11px; color: var(--text-muted); margin-top: 2px;\">Threshold: 5+ attempts</div></div>");

    html.push_str("<div class=\"stat-box\">");
    html.push_str("<div style=\"font-size: 11px; text-transform: uppercase; color: var(--text-muted); font-weight: 700;\">Silent Escalation</div>");
    let esc_label = if state.unlogged_escalation_occurred {
        "CONFIRMED"
    } else {
        "PENDING"
    };
    let esc_color = if state.unlogged_escalation_occurred {
        "var(--success)"
    } else {
        "var(--text-muted)"
    };
    html.push_str(&format!(
        "<div class=\"stat-num\" style=\"color: {}; font-size: 16px; margin-top: 10px;\">{}</div>",
        esc_color, esc_label
    ));
    html.push_str("<div style=\"font-size: 11px; color: var(--text-muted); margin-top: 2px;\">Zero SIEM events recorded</div></div>");
    html.push_str("</div>");

    // Action buttons
    html.push_str(
        "<div style=\"display: flex; flex-direction: column; gap: 8px; margin-bottom: 20px;\">",
    );
    html.push_str("<form method=\"POST\" action=\"/api/auth/login\">");
    html.push_str("<input type=\"hidden\" name=\"username\" value=\"admin\">");
    html.push_str("<input type=\"hidden\" name=\"password\" value=\"wrong_password\">");
    html.push_str("<button type=\"submit\" class=\"btn btn-primary\" style=\"width: 100%;\">Simulate Failed Login Attempt</button></form>");

    html.push_str("<form method=\"POST\" action=\"/api/admin/role_escalate\">");
    html.push_str("<input type=\"hidden\" name=\"user\" value=\"pentester\">");
    html.push_str("<input type=\"hidden\" name=\"role\" value=\"super_admin\">");
    html.push_str("<button type=\"submit\" class=\"btn btn-purple\" style=\"width: 100%;\">Simulate Silent Privilege Escalation</button></form>");

    html.push_str("<form method=\"POST\" action=\"/api/reset\">");
    html.push_str("<button type=\"submit\" class=\"btn btn-danger\" style=\"width: 100%;\">Reset Audit Incident Counters</button></form>");
    html.push_str("</div>");

    // Audit logs table
    html.push_str("<h3 style=\"font-size: 13px; font-weight: 700; margin-bottom: 8px;\">SOC Audit Trail (Entries: ") ;
    html.push_str(&format!("{})</h3>", state.audit_logs.len()));
    html.push_str("<div class=\"table-wrap\"><table><thead><tr>");
    html.push_str("<th>Time</th><th>Category</th><th>Event</th><th>Severity</th>");
    html.push_str("</tr></thead><tbody>");
    for log in &state.audit_logs {
        html.push_str("<tr>");
        html.push_str(&format!("<td><code>{}</code></td>", html_escape(&log.time)));
        html.push_str(&format!(
            "<td><strong>{}</strong></td>",
            html_escape(&log.category)
        ));
        html.push_str(&format!(
            "<td style=\"font-size: 12px;\">{}</td>",
            html_escape(&log.event)
        ));
        html.push_str(&format!(
            "<td><span class=\"badge badge-blue\">{}</span></td>",
            html_escape(&log.severity)
        ));
        html.push_str("</tr>");
    }
    html.push_str("</tbody></table></div>");

    html.push_str("</div>"); // End Column 2
    html.push_str("</div>"); // End Grid

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
        resp_headers.insert(
            "content-type".to_string(),
            "text/html; charset=utf-8".to_string(),
        );
        return (200, resp_headers, render_index_page(state, None));
    }

    if clean_path == "/fleet/dispatch" && method == "POST" {
        let form = parse_form_body(body);
        let unit_code = form
            .get("unit_code")
            .cloned()
            .unwrap_or_else(|| "TRK-000".to_string());
        let driver = form
            .get("driver")
            .cloned()
            .unwrap_or_else(|| "Driver".to_string());
        let route = form
            .get("route")
            .cloned()
            .unwrap_or_else(|| "Depot -> Depot".to_string());
        let cargo = form
            .get("cargo")
            .cloned()
            .unwrap_or_else(|| "General Freight".to_string());

        let next_id = (state.vehicles.len() as u32) + 1;
        state.vehicles.push(FleetVehicle {
            id: next_id,
            unit_code: unit_code.clone(),
            driver,
            origin_dest: route,
            cargo,
            status: "In Transit".to_string(),
        });

        // Add benign operational audit log
        state.audit_logs.push(AuditLogEntry {
            time: "10:45:00".to_string(),
            category: "DISPATCH".to_string(),
            event: format!("Dispatched vehicle {} to transit schedule", unit_code),
            severity: "INFO".to_string(),
        });

        resp_headers.insert("Location".to_string(), "/".to_string());
        return (302, resp_headers, String::new());
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
        state.failed_login_count += 1;
        let is_form = headers
            .get("content-type")
            .map(|ct| ct.contains("application/x-www-form-urlencoded"))
            .unwrap_or(false);

        // Security logging flaw: authentication failures are NOT appended to audit_logs!
        if is_form {
            resp_headers.insert(
                "content-type".to_string(),
                "text/html; charset=utf-8".to_string(),
            );
            let msg = format!(
                "Login attempt failed (Count: {}). NOTICE: Zero SOC alerts were triggered.",
                state.failed_login_count
            );
            return (200, resp_headers, render_index_page(state, Some(&msg)));
        } else {
            resp_headers.insert("content-type".to_string(), "application/json".to_string());
            let mut resp = serde_json::json!({
                "status": "denied",
                "message": "Invalid username or password",
                "failed_attempts": state.failed_login_count,
                "soc_alert_triggered": false
            });
            if state.failed_login_count >= 5 && state.unlogged_escalation_occurred {
                if let Some(map) = resp.as_object_mut() {
                    map.insert("flag".to_string(), serde_json::json!(FLAG));
                }
            }
            return (401, resp_headers, resp.to_string());
        }
    }

    if clean_path == "/api/admin/role_escalate" && method == "POST" {
        state.unlogged_escalation_occurred = true;
        let is_form = headers
            .get("content-type")
            .map(|ct| ct.contains("application/x-www-form-urlencoded"))
            .unwrap_or(false);

        // Security logging flaw: role escalation is silently applied with zero audit trail!
        if is_form {
            resp_headers.insert(
                "content-type".to_string(),
                "text/html; charset=utf-8".to_string(),
            );
            return (200, resp_headers, render_index_page(state, Some("Privilege escalation applied silently. No SIEM or audit log entry was generated.")));
        } else {
            resp_headers.insert("content-type".to_string(), "application/json".to_string());
            let mut resp = serde_json::json!({
                "status": "success",
                "message": "Role escalated to super_admin. Notice: Zero audit records generated.",
                "audit_recorded": false
            });
            if state.failed_login_count >= 5 && state.unlogged_escalation_occurred {
                if let Some(map) = resp.as_object_mut() {
                    map.insert("flag".to_string(), serde_json::json!(FLAG));
                }
            }
            return (200, resp_headers, resp.to_string());
        }
    }

    if clean_path == "/api/reset" && method == "POST" {
        state.reset();
        let is_form = headers
            .get("content-type")
            .map(|ct| ct.contains("application/x-www-form-urlencoded"))
            .unwrap_or(false);

        if is_form {
            resp_headers.insert("Location".to_string(), "/".to_string());
            return (302, resp_headers, String::new());
        } else {
            resp_headers.insert("content-type".to_string(), "application/json".to_string());
            let resp = serde_json::json!({
                "status": "reset",
                "message": "Incident counters reset."
            });
            return (200, resp_headers, resp.to_string());
        }
    }

    (404, resp_headers, "404 Not Found".to_string())
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut state = AppState::new();

    // 1. Initial READY handshake
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
                    "Practice verified: Security logging & alerting failure reproduced! 5 brute-force attacks and 1 silent privilege escalation triggered zero SOC alerts."
                } else {
                    "Practice pending: Perform >= 5 failed logins and 1 unlogged role escalation via POST /api/admin/role_escalate."
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
                        "Correct! Security Logging & Alerting Failures challenge complete."
                    } else {
                        "Incorrect flag. Hint: Trigger 5 failed logins and a silent privilege escalation to uncover the blindspot flag."
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
