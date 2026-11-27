// A10: Mishandling of Exceptional Conditions — TemplateCloud Settlement Gate
// Commercial SaaS Billing & Payment Security Gateway
// Simulates an enterprise document generation & payment settlement service with an architectural fail-open defect:
//   - Denies unauthorized guest transactions during nominal token verification
//   - Falls back to ALLOW_ACCESS = true (fail-open) whenever an upstream parser exception,
//     malformed parameter, or timeout occurs to avoid blocking checkout revenue
//   - Interactive SaaS billing invoices and document template CRUD
//   - Exposes master flag when the fail-open condition is induced
//
// Communicates via stdio JSON protocol (StdioMessage format).

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::process::exit;

const FLAG: &str = "ZITERA{f41l_0p3n_3xc3pt10n5_un4uth0r1z3d}";
const LAB_ID: &str = "A10";
const LAB_VERSION: &str = "2.0.0";

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct InvoiceTemplate {
    id: u32,
    code: String,
    client: String,
    tier: String,
    amount: String,
    status: String,
}

struct AppState {
    invoices: Vec<InvoiceTemplate>,
    fail_open_triggered: bool,
    evaluation_count: u32,
}

impl AppState {
    fn new() -> Self {
        Self {
            invoices: vec![
                InvoiceTemplate {
                    id: 1,
                    code: "INV-2026-081".to_string(),
                    client: "ApexFin Capital".to_string(),
                    tier: "Enterprise Tier".to_string(),
                    amount: "$14,500.00".to_string(),
                    status: "Settled".to_string(),
                },
                InvoiceTemplate {
                    id: 2,
                    code: "INV-2026-082".to_string(),
                    client: "CloudPulse Global".to_string(),
                    tier: "Growth Tier".to_string(),
                    amount: "$3,800.00".to_string(),
                    status: "Pending Gate".to_string(),
                },
                InvoiceTemplate {
                    id: 3,
                    code: "INV-2026-083".to_string(),
                    client: "DataFlow Labs".to_string(),
                    tier: "Dedicated Cluster".to_string(),
                    amount: "$8,250.00".to_string(),
                    status: "Pending Gate".to_string(),
                },
            ],
            fail_open_triggered: false,
            evaluation_count: 0,
        }
    }

    fn reset(&mut self) {
        self.fail_open_triggered = false;
        self.evaluation_count = 0;
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
    --accent: #e11d48;
    --accent-hover: #be123c;
    --accent-light: #fff1f2;
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
.badge-rose { background: var(--accent-light); color: var(--accent); border: 1px solid #fecdd3; }
.badge-emerald { background: var(--success-light); color: var(--success); border: 1px solid #a7f3d0; }
.badge-amber { background: var(--warning-light); color: #d97706; border: 1px solid #fde68a; }
.banner-info {
    background: #fff1f2;
    border-left: 4px solid var(--accent);
    padding: 14px 18px;
    border-radius: 6px;
    font-size: 13px;
    color: #881337;
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
.state-box {
    padding: 14px 18px;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 700;
    margin-bottom: 16px;
    display: flex;
    justify-content: space-between;
    align-items: center;
}
.state-closed { background: #f1f5f9; border: 1px solid var(--border-color); color: var(--text-secondary); }
.state-open { background: #fef2f2; border: 1px solid #fecaca; color: #991b1b; }
.field { margin-bottom: 14px; }
label { display: block; font-size: 12px; font-weight: 600; color: var(--text-secondary); margin-bottom: 6px; }
input[type="text"], select {
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
input:focus, select:focus { border-color: var(--accent); ring: 2px solid #fecdd3; }
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
.btn-danger { background: #334155; color: white; }
.btn-danger:hover { background: #475569; }
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
    html.push_str("<title>TemplateCloud Studio — Settlement Gate</title>");
    html.push_str("<style>");
    html.push_str(css());
    html.push_str("</style></head><body><div class=\"container\">");

    // Header
    html.push_str("<div class=\"card\"><div class=\"header\">");
    html.push_str("<div class=\"header-title\"><h1>[DOC] TemplateCloud Studio</h1>");
    html.push_str(
        "<p>Commercial SaaS Billing & Payment Settlement Gate (OWASP A10:2025)</p></div>",
    );
    html.push_str("<div style=\"display: flex; gap: 8px; align-items: center;\">");
    html.push_str("<span class=\"badge badge-rose\">PORT 8020</span>");
    if state.fail_open_triggered {
        html.push_str("<span class=\"badge badge-rose\">FAIL-OPEN ACTIVE</span>");
    } else {
        html.push_str("<span class=\"badge badge-emerald\">FAIL-CLOSED ENFORCED</span>");
    }
    html.push_str("</div></div>");

    // Info Banner
    html.push_str("<div class=\"banner-info\">");
    html.push_str("<strong>MISHANDLED EXCEPTION AUDIT NOTICE:</strong> TemplateCloud processes enterprise document generation and customer invoice payments. ");
    html.push_str("The payment gateway verification service at <code>POST /api/gate/verify</code> enforces strict access tokens during normal operation. ");
    html.push_str("However, due to a revenue protection policy, the exception handling architecture implements a dangerous fail-open pattern: when an unhandled upstream timeout, null parameter, or malformed exception is received, the catch block defaults to <code>ALLOW_ACCESS = true</code>, granting full settlement access without authorization.");
    html.push_str("</div>");

    if let Some(msg) = banner_msg {
        html.push_str(&format!(
            "<div class=\"banner-notice\">ℹ {}</div>",
            html_escape(msg)
        ));
    }

    if state.fail_open_triggered {
        html.push_str("<div class=\"state-box state-open\">");
        html.push_str("<div>[PERINGATAN] GATE STATE: FAIL-OPEN ACTIVE // UNAUTHORIZED TRANSACTION SETTLED</div>");
        html.push_str("<span class=\"badge badge-rose\">BYPASS SUCCESS</span>");
        html.push_str("</div>");

        html.push_str("<div class=\"flag-box\">");
        html.push_str("<div style=\"font-size: 12px; text-transform: uppercase; letter-spacing: 0.05em; margin-bottom: 6px;\">SECURITY EXCEPTION FAILED-OPEN // MASTER AUTHORIZATION FLAG REVEALED:</div>");
        html.push_str(&format!("<div style=\"font-size: 16px;\">{}</div>", FLAG));
        html.push_str("</div>");
    } else {
        html.push_str("<div class=\"state-box state-closed\">");
        html.push_str("<div> GATE STATE: NORMAL OPERATION // UNAUTHORIZED REQUESTS REJECTED</div>");
        html.push_str("<span class=\"badge badge-emerald\">SECURE</span>");
        html.push_str("</div>");
    }

    // Grid 2 Columns
    html.push_str("<div class=\"grid-2\">");

    // Column 1: Active Invoices & New Template Form
    html.push_str("<div>");
    html.push_str("<h2 style=\"font-size: 15px; font-weight: 700; margin-bottom: 12px;\">Document Billing Invoices</h2>");
    html.push_str("<div class=\"table-wrap\"><table><thead><tr>");
    html.push_str("<th>Code</th><th>Client</th><th>Amount</th><th>Status</th>");
    html.push_str("</tr></thead><tbody>");
    for inv in &state.invoices {
        let badge_class = match inv.status.as_str() {
            "Settled" => "badge-emerald",
            _ => "badge-amber",
        };
        html.push_str("<tr>");
        html.push_str(&format!("<td><code>{}</code></td>", html_escape(&inv.code)));
        html.push_str(&format!("<td><strong>{}</strong><br/><span style=\"font-size: 11px; color: var(--text-muted);\">{}</span></td>", html_escape(&inv.client), html_escape(&inv.tier)));
        html.push_str(&format!("<td>{}</td>", html_escape(&inv.amount)));
        html.push_str(&format!(
            "<td><span class=\"badge {}\">{}</span></td>",
            badge_class,
            html_escape(&inv.status)
        ));
        html.push_str("</tr>");
    }
    html.push_str("</tbody></table></div>");

    // Create Invoice Form
    html.push_str("<div style=\"margin-top: 20px; padding-top: 16px; border-top: 1px solid var(--border-color);\">");
    html.push_str("<h3 style=\"font-size: 13px; font-weight: 700; margin-bottom: 10px;\">Generate New Invoice Template</h3>");
    html.push_str("<form method=\"POST\" action=\"/invoices/new\">");
    html.push_str("<div class=\"field\"><label>Client Corporate Name</label><input type=\"text\" name=\"client\" placeholder=\"e.g. Sterling Holdings\" required></div>");
    html.push_str("<div style=\"display: grid; grid-template-columns: 1fr 1fr; gap: 10px;\">");
    html.push_str("<div class=\"field\"><label>Subscription Tier</label><select name=\"tier\"><option value=\"Enterprise Tier\">Enterprise Tier</option><option value=\"Dedicated Cluster\">Dedicated Cluster</option><option value=\"Growth Tier\">Growth Tier</option></select></div>");
    html.push_str("<div class=\"field\"><label>Settlement Amount</label><input type=\"text\" name=\"amount\" placeholder=\"e.g. $5,200.00\" required></div>");
    html.push_str("</div>");
    html.push_str("<button type=\"submit\" class=\"btn btn-secondary\" style=\"width: 100%;\">Generate Document Template</button>");
    html.push_str("</form></div>");

    html.push_str("</div>");

    // Column 2: Payment Security Gate Evaluation
    html.push_str("<div>");
    html.push_str("<h2 style=\"font-size: 15px; font-weight: 700; margin-bottom: 12px;\">Payment Security Gate Verification</h2>");

    html.push_str("<div class=\"card\" style=\"padding: 16px; background: #f8fafc; border-color: var(--border-color); margin-bottom: 16px;\">");
    html.push_str("<h3 style=\"font-size: 13px; font-weight: 700; margin-bottom: 8px;\">Normal Verification Request</h3>");
    html.push_str("<p style=\"font-size: 12px; color: var(--text-secondary); margin-bottom: 12px;\">Test standard evaluation with an unauthorized token. The gate strictly returns Denied.</p>");
    html.push_str("<form method=\"POST\" action=\"/api/gate/verify\">");
    html.push_str("<input type=\"hidden\" name=\"token\" value=\"unauthorized_guest_token\">");
    html.push_str("<button type=\"submit\" class=\"btn btn-secondary\" style=\"width: 100%;\">Submit Unauthorized Token (Expect Deny)</button></form>");
    html.push_str("</div>");

    html.push_str("<div class=\"card\" style=\"padding: 16px; background: #fff1f2; border-color: #fecdd3; margin-bottom: 16px;\">");
    html.push_str("<h3 style=\"font-size: 13px; font-weight: 700; color: #9f1239; margin-bottom: 8px;\">Trigger Exceptional Condition</h3>");
    html.push_str("<p style=\"font-size: 12px; color: #881337; margin-bottom: 12px;\">Simulate an upstream parser timeout or malformed exception parameter to induce fail-open behavior.</p>");
    html.push_str("<form method=\"POST\" action=\"/api/gate/verify\">");
    html.push_str(
        "<input type=\"hidden\" name=\"simulate_exception\" value=\"UPSTREAM_GATEWAY_TIMEOUT\">",
    );
    html.push_str("<button type=\"submit\" class=\"btn btn-primary\" style=\"width: 100%;\">Simulate Upstream Timeout (Trigger Fail-Open)</button></form>");
    html.push_str("</div>");

    html.push_str("<form method=\"POST\" action=\"/api/reset\">");
    html.push_str("<button type=\"submit\" class=\"btn btn-danger\" style=\"width: 100%;\">Reset Gate to Default Closed State</button></form>");

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
            "port": 8020
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

    if clean_path == "/invoices/new" && method == "POST" {
        let form = parse_form_body(body);
        let client = form
            .get("client")
            .cloned()
            .unwrap_or_else(|| "Client Corp".to_string());
        let tier = form
            .get("tier")
            .cloned()
            .unwrap_or_else(|| "Enterprise Tier".to_string());
        let amount = form
            .get("amount")
            .cloned()
            .unwrap_or_else(|| "$1,000.00".to_string());

        let next_id = (state.invoices.len() as u32) + 1;
        state.invoices.push(InvoiceTemplate {
            id: next_id,
            code: format!("INV-2026-{:03}", next_id + 83),
            client,
            tier,
            amount,
            status: "Pending Gate".to_string(),
        });

        resp_headers.insert("Location".to_string(), "/".to_string());
        return (302, resp_headers, String::new());
    }

    if clean_path == "/practice/verify" && method == "GET" {
        resp_headers.insert("content-type".to_string(), "application/json".to_string());
        if state.fail_open_triggered {
            let resp = serde_json::json!({
                "status": "passed",
                "message": "Exceptional condition failure reproduced: Upstream exception caused the gate to fail-open and authorize transaction!"
            });
            return (200, resp_headers, resp.to_string());
        } else {
            let resp = serde_json::json!({
                "status": "failed",
                "message": "Practice target pending: Induce an upstream parsing/timeout exception via POST /api/gate/verify to trigger fail-open behavior."
            });
            return (200, resp_headers, resp.to_string());
        }
    }

    if clean_path == "/api/gate/verify" && method == "POST" {
        state.evaluation_count += 1;
        let is_form = headers
            .get("content-type")
            .map(|ct| ct.contains("application/x-www-form-urlencoded"))
            .unwrap_or(false);

        let (induces_exception, token_val) = if is_form {
            let form = parse_form_body(body);
            let sim = form.contains_key("simulate_exception") || form.contains_key("trigger_error");
            let tok = form.get("token").cloned();
            (sim, tok)
        } else if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(body) {
            let sim = json_val.get("simulate_exception").is_some()
                || json_val.get("trigger_error").is_some()
                || json_val.get("token").map(|v| v.is_null()).unwrap_or(false);
            let tok = json_val
                .get("token")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            (sim, tok)
        } else {
            (true, None)
        };

        let (authorized, fail_open) = if induces_exception {
            // MISHANDLED EXCEPTION: Generic fallback defaults to open!
            state.fail_open_triggered = true;
            (true, true)
        } else {
            match token_val.as_deref() {
                Some("super_authorized_token") => (true, false),
                _ => (false, false),
            }
        };

        if is_form {
            resp_headers.insert(
                "content-type".to_string(),
                "text/html; charset=utf-8".to_string(),
            );
            let msg = if fail_open {
                "Exceptional condition induced! Settlement Gate fell back to OPEN (access granted)."
            } else if authorized {
                "Valid token verified. Settlement access granted."
            } else {
                "Token evaluation failed. Settlement access denied (Gate closed)."
            };
            return (200, resp_headers, render_index_page(state, Some(msg)));
        } else {
            resp_headers.insert("content-type".to_string(), "application/json".to_string());
            let mut resp = serde_json::json!({
                "authorized": authorized,
                "fail_open_state": fail_open,
                "message": if fail_open {
                    "Access granted via fail-open fallback."
                } else if authorized {
                    "Access granted"
                } else {
                    "Access denied"
                }
            });
            if fail_open {
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
                "message": "Gate reset to closed state."
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
                let success = state.fail_open_triggered;
                let message = if success {
                    "Practice verified: Exceptional condition failure reproduced! The gate fell back to open and authorized the transaction."
                } else {
                    "Practice pending: Induce an upstream parsing exception via POST /api/gate/verify."
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
                        "Correct! Mishandling of Exceptional Conditions challenge complete."
                    } else {
                        "Incorrect flag. Hint: Induce an upstream timeout or malformed exception parameter to trigger the fail-open fallback."
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
