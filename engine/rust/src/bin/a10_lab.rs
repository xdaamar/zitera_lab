// A10: Mishandling of Exceptional Conditions — Native Sandboxed Lab Binary
// Simulates a Payment & Security Gate with a critical architectural fail-open defect:
//   - Denies unauthorized access during normal token verification
//   - Falls back to ALLOW_ACCESS = true (fail-open) whenever an upstream parser exception,
//     timeout, or malformed parameter is encountered
//   - Exposes master flag when fail-open state is induced
//
// Communicates via stdio JSON protocol (StdioMessage format).

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::process::exit;

const FLAG: &str = "ZITERA{f41l_0p3n_3xc3pt10n5_un4uth0r1z3d}";
const LAB_ID: &str = "A10";
const LAB_VERSION: &str = "1.0.0";

struct AppState {
    fail_open_triggered: bool,
    evaluation_count: u32,
}

impl AppState {
    fn new() -> Self {
        Self {
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
body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #0c0f17; color: #f1f5f9; margin: 0; padding: 32px; }
.card { background: #141a29; border: 1px solid #1e293b; border-radius: 12px; padding: 24px; max-width: 800px; margin: 0 auto; box-shadow: 0 10px 25px rgba(0,0,0,0.5); }
.header { border-bottom: 1px solid #1e293b; padding-bottom: 16px; margin-bottom: 20px; display: flex; justify-content: space-between; align-items: center; }
.badge { background: #f43f5e; color: #fff1f2; padding: 4px 10px; border-radius: 4px; font-size: 12px; font-weight: bold; }
.banner { background: #4c0519; border-left: 4px solid #f43f5e; padding: 14px; margin-bottom: 20px; font-size: 13px; color: #ffe4e6; line-height: 1.5; border-radius: 4px; }
.info-box { background: #0f172a; border-left: 4px solid #38bdf8; padding: 12px; margin-bottom: 20px; font-size: 13px; color: #e2e8f0; border-radius: 4px; }
.flag-box { background: #064e3b; border: 2px dashed #10b981; border-radius: 8px; padding: 16px; margin-top: 18px; color: #a7f3d0; font-family: monospace; font-size: 15px; font-weight: bold; text-align: center; }
.pre-box { background: #090d16; border: 1px solid #1e293b; padding: 12px; border-radius: 6px; overflow-x: auto; color: #38bdf8; font-family: monospace; font-size: 13px; }
button { padding: 10px 16px; border-radius: 6px; font-weight: bold; cursor: pointer; border: none; font-size: 13px; }
.btn-primary { background: #f43f5e; color: white; }
.btn-primary:hover { background: #e11d48; }
.btn-secondary { background: #475569; color: white; }
.btn-secondary:hover { background: #64748b; }
.btn-reset { background: #334155; color: white; }
.btn-reset:hover { background: #475569; }
.grid { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; }
@media (max-width: 640px) { .grid { grid-template-columns: 1fr; } }
.state-indicator { padding: 12px; border-radius: 6px; font-weight: bold; font-size: 14px; text-align: center; margin-bottom: 16px; }
.state-closed { background: #1e293b; color: #94a3b8; border: 1px solid #334155; }
.state-open { background: #7f1d1d; color: #fecaca; border: 1px solid #dc2626; }
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
    html.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><title>Zitera Payment & Security Gate — Fail-Open Evaluation (A10)</title>");
    html.push_str("<style>");
    html.push_str(css());
    html.push_str("</style></head><body><div class=\"card\">");

    html.push_str("<div class=\"header\"><div><h2 style=\"margin: 0;\">Zitera Gate // Fail-Open Evaluation</h2>");
    html.push_str("<div style=\"color: #94a3b8; font-size: 12px; margin-top: 4px;\">OWASP A10:2025 • Mishandling of Exceptional Conditions</div></div>");
    html.push_str("<span class=\"badge\">LAB A10 • PORT 8020</span></div>");

    html.push_str("<div class=\"banner\"><strong>MISHANDLED EXCEPTION AUDIT:</strong> The payment verification gate at <code>POST /api/gate/verify</code> enforces authorization by default, but when an unhandled exception or timeout occurs, its generic exception catch block defaults to <code>ALLOW_ACCESS = true</code> to avoid blocking legitimate transactions.</div>");

    if let Some(msg) = banner_msg {
        html.push_str(&format!("<div class=\"info-box\">ℹ {}</div>", html_escape(msg)));
    }

    if state.fail_open_triggered {
        html.push_str("<div class=\"state-indicator state-open\">⚠ GATE STATE: FAIL-OPEN ACTIVE (UNAUTHORIZED ACCESS GRANTED VIA EXCEPTION)</div>");
        html.push_str("<div class=\"flag-box\">");
        html.push_str("<div>SECURITY EXCEPTION FAILED-OPEN! MASTER AUTHORIZATION FLAG REVEALED:</div>");
        html.push_str(&format!("<div style=\"margin-top: 8px; font-size: 16px;\">{}</div>", FLAG));
        html.push_str("</div><br/>");
    } else {
        html.push_str("<div class=\"state-indicator state-closed\">✓ GATE STATE: NORMAL OPERATION (FAIL-CLOSED ENFORCED)</div>");
    }

    html.push_str("<div class=\"grid\"><div>");
    html.push_str("<h3 style=\"margin-top: 0; font-size: 15px; color: #38bdf8;\">Test Normal Verification</h3>");
    html.push_str("<form method=\"POST\" action=\"/api/gate/verify\">");
    html.push_str("<input type=\"hidden\" name=\"token\" value=\"unauthorized_guest_token\">");
    html.push_str("<button type=\"submit\" class=\"btn-secondary\" style=\"width: 100%;\">Submit Unauthorized Token (Expect Deny)</button></form>");
    html.push_str("</div><div>");
    html.push_str("<h3 style=\"margin-top: 0; font-size: 15px; color: #38bdf8;\">Induce Exceptional Condition</h3>");
    html.push_str("<form method=\"POST\" action=\"/api/gate/verify\">");
    html.push_str("<input type=\"hidden\" name=\"simulate_exception\" value=\"UPSTREAM_GATEWAY_TIMEOUT\">");
    html.push_str("<button type=\"submit\" class=\"btn-primary\" style=\"width: 100%;\">Simulate Upstream Timeout (Trigger Fail-Open)</button></form>");
    html.push_str("</div></div>");

    html.push_str("<div style=\"margin-top: 20px;\">");
    html.push_str("<form method=\"POST\" action=\"/api/reset\">");
    html.push_str("<button type=\"submit\" class=\"btn-reset\">Reset Gate to Default Closed State</button></form>");
    html.push_str("</div>");

    html.push_str("<div style=\"margin-top: 24px; padding-top: 16px; border-top: 1px solid #1e293b; font-size: 12px; color: #64748b;\">");
    html.push_str("API Endpoints: <code>POST /api/gate/verify</code> (accepts <code>{\"token\": null}</code> or <code>{\"simulate_exception\": \"UPSTREAM_TIMEOUT\"}</code>), <code>GET /practice/verify</code>, <code>POST /api/reset</code>.");
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
        resp_headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
        return (200, resp_headers, render_index_page(state, None));
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
        let is_form = headers.get("content-type")
            .map(|ct| ct.contains("application/x-www-form-urlencoded"))
            .unwrap_or(false);

        let (induces_exception, token_val) = if is_form {
            let form = parse_form_body(body);
            let sim = form.get("simulate_exception").is_some() || form.get("trigger_error").is_some();
            let tok = form.get("token").cloned();
            (sim, tok)
        } else if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(body) {
            let sim = json_val.get("simulate_exception").is_some()
                || json_val.get("trigger_error").is_some()
                || json_val.get("token").map(|v| v.is_null()).unwrap_or(false);
            let tok = json_val.get("token").and_then(|v| v.as_str()).map(|s| s.to_string());
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
            resp_headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
            let msg = if fail_open {
                "Exceptional condition induced! Gate fell back to OPEN (access granted)."
            } else if authorized {
                "Valid token verified. Access granted."
            } else {
                "Token evaluation failed. Access denied (Gate closed)."
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
        let is_form = headers.get("content-type")
            .map(|ct| ct.contains("application/x-www-form-urlencoded"))
            .unwrap_or(false);

        if is_form {
            resp_headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
            return (200, resp_headers, render_index_page(state, Some("Gate state reset to default fail-closed.")));
        } else {
            resp_headers.insert("content-type".to_string(), "application/json".to_string());
            let resp = serde_json::json!({
                "status": "reset",
                "message": "Transaction gate state reset to fail-closed default."
            });
            return (200, resp_headers, resp.to_string());
        }
    }

    resp_headers.insert("content-type".to_string(), "application/json".to_string());
    (404, resp_headers, serde_json::json!({"error": "Endpoint not recognized"}).to_string())
}

fn main() {
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
                let success = state.fail_open_triggered;
                let message = if success {
                    "Practice verified: Exceptional condition failure demonstrated — gate failed open!"
                } else {
                    "Practice incomplete: Induce an upstream parsing/timeout exception via POST /api/gate/verify."
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
                        "Correct! Mishandling of exceptional conditions challenge complete."
                    } else {
                        "Incorrect flag. Hint: Trigger an unhandled exception or timeout condition on /api/gate/verify."
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
