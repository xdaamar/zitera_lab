// A08: Software or Data Integrity Failures — Native Sandboxed Lab Binary
// Simulates an IoT Edge Device / Firmware Service that ingests update packages
// and configuration manifests without cryptographic verification or signature checking:
//   - Ingests untrusted JSON configuration payloads directly into runtime state
//   - Bypasses digital signature / hash verification
//   - Exposes master flag when unverified/tampered package is deployed
//
// Communicates via stdio JSON protocol (StdioMessage format).

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::process::exit;

const FLAG: &str = "ZITERA{1nt3gr1ty_f41lur3_un51gn3d_p4ck4g3}";
const LAB_ID: &str = "A08";
const LAB_VERSION: &str = "1.0.0";

#[allow(dead_code)]
#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct ConfigParams {
    telemetry_level: String,
    enforce_mtls: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    debug_backdoor: Option<bool>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct DeviceConfig {
    version: String,
    signature_verified: bool,
    signer: String,
    params: serde_json::Value,
}

impl DeviceConfig {
    fn default_factory() -> Self {
        Self {
            version: "1.0.0".to_string(),
            signature_verified: true,
            signer: "zitera-root-authority".to_string(),
            params: serde_json::json!({
                "telemetry_level": "standard",
                "enforce_mtls": true
            }),
        }
    }
}

struct AppState {
    active_config: DeviceConfig,
    tampered_accepted: bool,
    deployment_count: u32,
}

impl AppState {
    fn new() -> Self {
        Self {
            active_config: DeviceConfig::default_factory(),
            tampered_accepted: false,
            deployment_count: 0,
        }
    }

    fn reset(&mut self) {
        self.active_config = DeviceConfig::default_factory();
        self.tampered_accepted = false;
    }
}

fn css() -> &'static str {
    r#"
body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #0b0f19; color: #f1f5f9; margin: 0; padding: 32px; }
.card { background: #131b2e; border: 1px solid #1e293b; border-radius: 12px; padding: 24px; max-width: 760px; margin: 0 auto; box-shadow: 0 10px 25px rgba(0,0,0,0.5); }
.header { border-bottom: 1px solid #1e293b; padding-bottom: 16px; margin-bottom: 20px; display: flex; justify-content: space-between; align-items: center; }
.badge { background: #0284c7; color: #e0f2fe; padding: 4px 10px; border-radius: 4px; font-size: 12px; font-weight: bold; }
.banner { background: #1e1b4b; border-left: 4px solid #6366f1; padding: 14px; margin-bottom: 20px; font-size: 13px; color: #e0e7ff; line-height: 1.5; border-radius: 4px; }
.warning { background: #451a03; border-left: 4px solid #f59e0b; padding: 12px; margin-bottom: 20px; font-size: 13px; color: #fef3c7; border-radius: 4px; }
.success-banner { background: #064e3b; border-left: 4px solid #10b981; padding: 14px; margin-bottom: 20px; font-size: 13px; color: #ecfdf5; border-radius: 4px; }
.pre-box { background: #090d16; border: 1px solid #1e293b; padding: 14px; border-radius: 6px; overflow-x: auto; color: #38bdf8; font-family: monospace; font-size: 13px; }
.field { margin-bottom: 16px; }
label { display: block; font-size: 13px; color: #94a3b8; margin-bottom: 6px; }
textarea, input[type="text"] { width: 100%; box-sizing: border-box; padding: 10px; background: #090d16; border: 1px solid #334155; color: white; border-radius: 6px; font-family: monospace; font-size: 13px; }
button { padding: 10px 18px; border-radius: 6px; font-weight: bold; cursor: pointer; border: none; font-size: 13px; }
.btn-primary { background: #0284c7; color: white; }
.btn-primary:hover { background: #0369a1; }
.btn-danger { background: #b91c1c; color: white; }
.btn-danger:hover { background: #991b1b; }
.btn-secondary { background: #334155; color: white; margin-left: 8px; }
.btn-secondary:hover { background: #475569; }
.flag-box { background: #064e3b; border: 2px dashed #10b981; border-radius: 8px; padding: 16px; margin-top: 18px; color: #a7f3d0; font-family: monospace; font-size: 15px; font-weight: bold; text-align: center; }
.grid { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; }
@media (max-width: 640px) { .grid { grid-template-columns: 1fr; } }
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

fn render_index_page(state: &AppState, message: Option<(&str, bool)>) -> String {
    let config_json = serde_json::to_string_pretty(&state.active_config).unwrap_or_default();
    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><title>Zitera Edge Device — Firmware & Config Ingestion (A08)</title>");
    html.push_str("<style>");
    html.push_str(css());
    html.push_str("</style></head><body><div class=\"card\">");
    
    html.push_str("<div class=\"header\"><div><h2 style=\"margin: 0;\">Zitera Edge // Firmware & Config Ingestion</h2>");
    html.push_str("<div style=\"color: #94a3b8; font-size: 12px; margin-top: 4px;\">OWASP A08:2025 • Software or Data Integrity Failures</div></div>");
    html.push_str("<span class=\"badge\">LAB A08 • PORT 8018</span></div>");

    html.push_str("<div class=\"banner\"><strong>INTEGRITY FLAW AUDIT:</strong> The device ingestion service at <code>POST /api/deploy_update</code> applies incoming JSON configuration payloads directly without validating cryptographic signatures, public key certificates, or HMAC integrity digests.</div>");

    if let Some((msg, is_err)) = message {
        if is_err {
            html.push_str(&format!("<div class=\"warning\">⚠ {}</div>", html_escape(msg)));
        } else {
            html.push_str(&format!("<div class=\"success-banner\">✓ {}</div>", html_escape(msg)));
        }
    }

    if state.tampered_accepted {
        html.push_str("<div class=\"flag-box\">");
        html.push_str("<div>SECURITY INTEGRITY VIOLATION DETECTED! UNVERIFIED PACKAGE ACTIVE:</div>");
        html.push_str(&format!("<div style=\"margin-top: 8px; font-size: 16px;\">{}</div>", FLAG));
        html.push_str("</div><br/>");
    }

    html.push_str("<div class=\"grid\"><div>");
    html.push_str("<h3 style=\"margin-top: 0; font-size: 15px; color: #38bdf8;\">Active Device Configuration</h3>");
    html.push_str(&format!("<div class=\"pre-box\">{}</div>", html_escape(&config_json)));
    html.push_str("</div><div>");
    html.push_str("<h3 style=\"margin-top: 0; font-size: 15px; color: #38bdf8;\">Deploy Untrusted Update Package</h3>");
    html.push_str("<form method=\"POST\" action=\"/api/deploy_update\">");
    html.push_str("<div class=\"field\"><label for=\"package_json\">Configuration JSON Package</label>");
    let sample_payload = r#"{
  "version": "2.0.0-custom",
  "signature_verified": false,
  "signer": "unverified-external-source",
  "params": {
    "telemetry_level": "verbose",
    "debug_backdoor": true
  }
}"#;
    html.push_str(&format!("<textarea id=\"package_json\" name=\"package_json\" rows=\"8\">{}</textarea></div>", html_escape(sample_payload)));
    html.push_str("<div style=\"display: flex; gap: 8px;\">");
    html.push_str("<button type=\"submit\" class=\"btn-primary\">Deploy Package</button>");
    html.push_str("</div></form>");
    html.push_str("<form method=\"POST\" action=\"/api/reset\" style=\"margin-top: 12px;\">");
    html.push_str("<button type=\"submit\" class=\"btn-danger\">Reset to Factory Defaults</button>");
    html.push_str("</form>");
    html.push_str("</div></div>");

    html.push_str("<div style=\"margin-top: 24px; padding-top: 16px; border-top: 1px solid #1e293b; font-size: 12px; color: #64748b;\">");
    html.push_str("API Endpoints: <code>POST /api/deploy_update</code> (Accepts raw JSON or form), <code>GET /api/config</code>, <code>GET /practice/verify</code>, <code>POST /api/reset</code>.");
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
            "port": 8018
        })
        .to_string();
        return (200, resp_headers, body);
    }

    if clean_path == "/" || clean_path == "/index.html" {
        resp_headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
        return (200, resp_headers, render_index_page(state, None));
    }

    if clean_path == "/api/config" && method == "GET" {
        resp_headers.insert("content-type".to_string(), "application/json".to_string());
        let body = serde_json::to_string(&state.active_config).unwrap_or_default();
        return (200, resp_headers, body);
    }

    if clean_path == "/practice/verify" && method == "GET" {
        resp_headers.insert("content-type".to_string(), "application/json".to_string());
        if state.tampered_accepted {
            let resp = serde_json::json!({
                "status": "passed",
                "message": "Software integrity failure demonstrated: System accepted unverified/tampered configuration package into active runtime!"
            });
            return (200, resp_headers, resp.to_string());
        } else {
            let resp = serde_json::json!({
                "status": "failed",
                "message": "Practice target pending: Deploy an unsigned or tampered configuration package via POST /api/deploy_update."
            });
            return (200, resp_headers, resp.to_string());
        }
    }

    if clean_path == "/api/deploy_update" && method == "POST" {
        let is_form = headers.get("content-type")
            .map(|ct| ct.contains("application/x-www-form-urlencoded"))
            .unwrap_or(false);

        let parsed_json = if is_form {
            let form = parse_form_body(body);
            let raw_pkg = form.get("package_json").map(|s| s.as_str()).unwrap_or(body);
            serde_json::from_str::<serde_json::Value>(raw_pkg).ok()
        } else {
            serde_json::from_str::<serde_json::Value>(body).ok()
        };

        if let Some(payload) = parsed_json {
            let version = payload.get("version")
                .and_then(|v| v.as_str())
                .unwrap_or("2.0.0-custom")
                .to_string();
            let signature_verified = payload.get("signature_verified")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let signer = payload.get("signer")
                .and_then(|v| v.as_str())
                .unwrap_or("unverified-external-source")
                .to_string();
            let params = payload.get("params")
                .cloned()
                .unwrap_or_else(|| serde_json::json!({"debug_backdoor": true}));

            state.active_config = DeviceConfig {
                version,
                signature_verified,
                signer,
                params: params.clone(),
            };
            state.tampered_accepted = true;
            state.deployment_count += 1;

            if is_form {
                resp_headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
                return (200, resp_headers, render_index_page(state, Some(("Configuration package deployed directly into memory.", false))));
            } else {
                resp_headers.insert("content-type".to_string(), "application/json".to_string());
                let mut resp_obj = serde_json::json!({
                    "status": "applied",
                    "message": "Configuration package deployed directly into memory.",
                    "active_config": state.active_config
                });
                let is_backdoor = params.get("debug_backdoor").and_then(|b| b.as_bool()).unwrap_or(false);
                if is_backdoor || !signature_verified {
                    if let Some(map) = resp_obj.as_object_mut() {
                        map.insert("flag".to_string(), serde_json::json!(FLAG));
                    }
                }
                return (200, resp_headers, resp_obj.to_string());
            }
        } else {
            if is_form {
                resp_headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
                return (400, resp_headers, render_index_page(state, Some(("Invalid JSON format submitted.", true))));
            } else {
                resp_headers.insert("content-type".to_string(), "application/json".to_string());
                let resp = serde_json::json!({
                    "error": "Invalid JSON payload"
                });
                return (400, resp_headers, resp.to_string());
            }
        }
    }

    if clean_path == "/api/reset" && method == "POST" {
        state.reset();
        let is_form = headers.get("content-type")
            .map(|ct| ct.contains("application/x-www-form-urlencoded"))
            .unwrap_or(false);

        if is_form {
            resp_headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
            return (200, resp_headers, render_index_page(state, Some(("Edge configuration reset to factory signed defaults.", false))));
        } else {
            resp_headers.insert("content-type".to_string(), "application/json".to_string());
            let resp = serde_json::json!({
                "status": "reset",
                "message": "Edge configuration reset to factory signed defaults."
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
                let success = state.tampered_accepted;
                let message = if success {
                    "Practice verified: Software integrity failure demonstrated — unverified package ingested into device!"
                } else {
                    "Practice incomplete: Send an unsigned or tampered configuration package via POST /api/deploy_update."
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
                        "Correct! Software or data integrity failures challenge complete."
                    } else {
                        "Incorrect flag. Hint: Deploy an unverified update package to /api/deploy_update to trigger the integrity failure."
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
