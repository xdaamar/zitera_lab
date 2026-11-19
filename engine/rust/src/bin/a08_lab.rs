// A08: Software or Data Integrity Failures — DataFlow Analytics
// Enterprise BI & Data Processing Engine
// Simulates an enterprise analytics platform with intentional software/data integrity flaws:
//   - Ingests untrusted pipeline configurations and models without cryptographic signature verification
//   - Bypasses HMAC integrity digest checking
//   - Interactive BI model catalog and pipeline execution CRUD
//   - Exposes master flag when unverified/tampered pipeline package is deployed
//
// Communicates via stdio JSON protocol (StdioMessage format).

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::process::exit;

const FLAG: &str = "ZITERA{1nt3gr1ty_f41lur3_un51gn3d_p4ck4g3}";
const LAB_ID: &str = "A08";
const LAB_VERSION: &str = "2.0.0";

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct PipelineModel {
    id: u32,
    name: String,
    framework: String,
    ingestion_rate: String,
    status: String,
    last_verified: String,
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
                "enforce_mtls": true,
                "sandbox_isolation": "strict"
            }),
        }
    }
}

struct AppState {
    models: Vec<PipelineModel>,
    active_config: DeviceConfig,
    tampered_accepted: bool,
    deployment_count: u32,
}

impl AppState {
    fn new() -> Self {
        Self {
            models: vec![
                PipelineModel {
                    id: 1,
                    name: "Revenue_Forecasting_Q4".to_string(),
                    framework: "PyTorch 2.3".to_string(),
                    ingestion_rate: "12,400 ev/s".to_string(),
                    status: "Active".to_string(),
                    last_verified: "Signed (Root CA)".to_string(),
                },
                PipelineModel {
                    id: 2,
                    name: "Customer_Churn_Classifier".to_string(),
                    framework: "Scikit-Learn".to_string(),
                    ingestion_rate: "4,200 ev/s".to_string(),
                    status: "Active".to_string(),
                    last_verified: "Signed (Root CA)".to_string(),
                },
                PipelineModel {
                    id: 3,
                    name: "Executive_Audit_Stream".to_string(),
                    framework: "Apache Arrow".to_string(),
                    ingestion_rate: "1,850 ev/s".to_string(),
                    status: "Idle".to_string(),
                    last_verified: "Signed (Root CA)".to_string(),
                },
            ],
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
:root {
    --bg-page: #f8fafc;
    --card-bg: #ffffff;
    --text-primary: #0f172a;
    --text-secondary: #475569;
    --text-muted: #94a3b8;
    --border-color: #e2e8f0;
    --accent: #4f46e5;
    --accent-hover: #4338ca;
    --accent-light: #eef2ff;
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
.badge-indigo { background: var(--accent-light); color: var(--accent); border: 1px solid #c7d2fe; }
.badge-emerald { background: var(--success-light); color: var(--success); border: 1px solid #a7f3d0; }
.badge-amber { background: var(--warning-light); color: #d97706; border: 1px solid #fde68a; }
.badge-rose { background: var(--danger-light); color: var(--danger); border: 1px solid #fecaca; }
.banner-info {
    background: #eef2ff;
    border-left: 4px solid var(--accent);
    padding: 14px 18px;
    border-radius: 6px;
    font-size: 13px;
    color: #312e81;
    margin-bottom: 24px;
    line-height: 1.6;
}
.banner-success {
    background: var(--success-light);
    border-left: 4px solid var(--success);
    padding: 14px 18px;
    border-radius: 6px;
    font-size: 13px;
    color: #065f46;
    margin-bottom: 24px;
}
.banner-warning {
    background: var(--warning-light);
    border-left: 4px solid var(--warning);
    padding: 14px 18px;
    border-radius: 6px;
    font-size: 13px;
    color: #92400e;
    margin-bottom: 24px;
}
.grid-2 { display: grid; grid-template-columns: 1fr 1fr; gap: 24px; }
@media (max-width: 860px) { .grid-2 { grid-template-columns: 1fr; } }
.table-wrap { overflow-x: auto; border: 1px solid var(--border-color); border-radius: 8px; }
table { width: 100%; border-collapse: collapse; text-align: left; font-size: 13px; }
th { background: #f1f5f9; padding: 12px 14px; font-weight: 600; color: var(--text-secondary); border-bottom: 1px solid var(--border-color); }
td { padding: 12px 14px; border-bottom: 1px solid var(--border-color); color: var(--text-primary); }
tr:last-child td { border-bottom: none; }
tr:hover td { background: #f8fafc; }
.pre-box {
    background: #0f172a;
    border: 1px solid #334155;
    border-radius: 8px;
    padding: 14px;
    color: #38bdf8;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 12px;
    line-height: 1.5;
    overflow-x: auto;
    white-space: pre-wrap;
}
.field { margin-bottom: 14px; }
label { display: block; font-size: 12px; font-weight: 600; color: var(--text-secondary); margin-bottom: 6px; }
input[type="text"], textarea {
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
textarea { font-family: ui-monospace, monospace; font-size: 12px; line-height: 1.4; }
input:focus, textarea:focus { border-color: var(--accent); ring: 2px solid #c7d2fe; }
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
    html.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\">");
    html.push_str("<title>DataFlow Analytics — Enterprise BI & Ingestion Hub</title>");
    html.push_str("<style>");
    html.push_str(css());
    html.push_str("</style></head><body><div class=\"container\">");

    // Header
    html.push_str("<div class=\"card\"><div class=\"header\">");
    html.push_str("<div class=\"header-title\"><h1>📊 DataFlow Analytics</h1>");
    html.push_str("<p>Enterprise BI Engine & Pipeline Configuration Ingestion (OWASP A08:2025)</p></div>");
    html.push_str("<div style=\"display: flex; gap: 8px; align-items: center;\">");
    html.push_str("<span class=\"badge badge-indigo\">PORT 8018</span>");
    if state.tampered_accepted {
        html.push_str("<span class=\"badge badge-rose\">INTEGRITY COMPROMISED</span>");
    } else {
        html.push_str("<span class=\"badge badge-emerald\">SIGNATURES ENFORCED</span>");
    }
    html.push_str("</div></div>");

    // Info Banner
    html.push_str("<div class=\"banner-info\">");
    html.push_str("<strong>DATA INTEGRITY AUDIT NOTICE:</strong> DataFlow processes automated BI analytics pipelines. ");
    html.push_str("The backend configuration loader at <code>POST /api/deploy_update</code> receives pipeline definitions. ");
    html.push_str("Although enterprise guidelines mandate cryptographic digital signatures, the parser blindly accepts deserialized JSON packages where <code>signature_verified</code> is falsified or <code>debug_backdoor: true</code> is injected, bypassing integrity validation entirely.");
    html.push_str("</div>");

    if let Some((msg, is_err)) = message {
        if is_err {
            html.push_str(&format!("<div class=\"banner-warning\">⚠ {}</div>", html_escape(msg)));
        } else {
            html.push_str(&format!("<div class=\"banner-success\">✓ {}</div>", html_escape(msg)));
        }
    }

    if state.tampered_accepted {
        html.push_str("<div class=\"flag-box\">");
        html.push_str("<div style=\"font-size: 12px; text-transform: uppercase; letter-spacing: 0.05em; margin-bottom: 6px;\">SECURITY INTEGRITY VIOLATION DETECTED // MASTER ANALYTICS KERNEL UNLOCKED:</div>");
        html.push_str(&format!("<div style=\"font-size: 16px;\">{}</div>", FLAG));
        html.push_str("</div>");
    }

    // Two Columns
    html.push_str("<div class=\"grid-2\">");

    // Column 1: Active BI Models
    html.push_str("<div>");
    html.push_str("<h2 style=\"font-size: 15px; font-weight: 700; margin-bottom: 12px;\">Active Analytics Pipelines</h2>");
    html.push_str("<div class=\"table-wrap\"><table><thead><tr>");
    html.push_str("<th>Model Name</th><th>Framework</th><th>Throughput</th><th>Integrity</th>");
    html.push_str("</tr></thead><tbody>");
    for m in &state.models {
        html.push_str("<tr>");
        html.push_str(&format!("<td><strong>{}</strong></td>", html_escape(&m.name)));
        html.push_str(&format!("<td><code>{}</code></td>", html_escape(&m.framework)));
        html.push_str(&format!("<td>{}</td>", html_escape(&m.ingestion_rate)));
        html.push_str(&format!("<td><span class=\"badge badge-emerald\">{}</span></td>", html_escape(&m.last_verified)));
        html.push_str("</tr>");
    }
    html.push_str("</tbody></table></div>");

    // Add Model Form
    html.push_str("<div style=\"margin-top: 20px; padding-top: 16px; border-top: 1px solid var(--border-color);\">");
    html.push_str("<h3 style=\"font-size: 13px; font-weight: 700; margin-bottom: 10px;\">Register New Analytics Model</h3>");
    html.push_str("<form method=\"POST\" action=\"/models/new\">");
    html.push_str("<div class=\"field\"><label>Model Identifier</label><input type=\"text\" name=\"name\" placeholder=\"e.g. Anomaly_Detection_v1\" required></div>");
    html.push_str("<div class=\"field\"><label>Execution Framework</label><input type=\"text\" name=\"framework\" placeholder=\"e.g. ONNX Runtime / PyTorch\" required></div>");
    html.push_str("<div class=\"field\"><label>Target Ingestion Throughput</label><input type=\"text\" name=\"throughput\" placeholder=\"e.g. 8,500 ev/s\" required></div>");
    html.push_str("<button type=\"submit\" class=\"btn btn-secondary\" style=\"width: 100%;\">Register Model</button>");
    html.push_str("</form></div>");

    html.push_str("</div>");

    // Column 2: Ingestion Manifest & Config
    html.push_str("<div>");
    html.push_str("<h2 style=\"font-size: 15px; font-weight: 700; margin-bottom: 12px;\">Active Kernel Runtime Manifest</h2>");
    html.push_str(&format!("<div class=\"pre-box\">{}</div>", html_escape(&config_json)));

    html.push_str("<div style=\"margin-top: 20px;\">");
    html.push_str("<h3 style=\"font-size: 13px; font-weight: 700; margin-bottom: 8px;\">Deploy Untrusted Configuration Package</h3>");
    html.push_str("<form method=\"POST\" action=\"/api/deploy_update\">");
    html.push_str("<div class=\"field\"><label>Configuration JSON Package (Untrusted Payload)</label>");
    let sample_payload = r#"{
  "version": "2.4.0-enterprise",
  "signature_verified": false,
  "signer": "unverified-external-source",
  "params": {
    "telemetry_level": "verbose",
    "debug_backdoor": true,
    "sandbox_isolation": "bypassed"
  }
}"#;
    html.push_str(&format!("<textarea name=\"package_json\" rows=\"8\">{}</textarea></div>", html_escape(sample_payload)));
    html.push_str("<div style=\"display: flex; gap: 8px;\">");
    html.push_str("<button type=\"submit\" class=\"btn btn-primary\" style=\"flex: 1;\">Deploy Ingestion Package</button>");
    html.push_str("</div></form>");

    html.push_str("<form method=\"POST\" action=\"/api/reset\" style=\"margin-top: 10px;\">");
    html.push_str("<button type=\"submit\" class=\"btn btn-danger\" style=\"width: 100%;\">Reset to Verified Baseline</button>");
    html.push_str("</form>");
    html.push_str("</div>");

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
            "port": 8018
        })
        .to_string();
        return (200, resp_headers, body);
    }

    if clean_path == "/" || clean_path == "/index.html" {
        resp_headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
        return (200, resp_headers, render_index_page(state, None));
    }

    if clean_path == "/models/new" && method == "POST" {
        let form = parse_form_body(body);
        let name = form.get("name").cloned().unwrap_or_else(|| "Custom_Model".to_string());
        let framework = form.get("framework").cloned().unwrap_or_else(|| "ONNX".to_string());
        let throughput = form.get("throughput").cloned().unwrap_or_else(|| "5,000 ev/s".to_string());

        let next_id = (state.models.len() as u32) + 1;
        state.models.push(PipelineModel {
            id: next_id,
            name,
            framework,
            ingestion_rate: throughput,
            status: "Active".to_string(),
            last_verified: "Self-Reported".to_string(),
        });

        resp_headers.insert("Location".to_string(), "/".to_string());
        return (302, resp_headers, String::new());
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
                .unwrap_or("2.4.0-enterprise")
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
            resp_headers.insert("Location".to_string(), "/".to_string());
            return (302, resp_headers, String::new());
        } else {
            resp_headers.insert("content-type".to_string(), "application/json".to_string());
            let resp = serde_json::json!({
                "status": "reset",
                "message": "Configuration restored to verified baseline."
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
                let success = state.tampered_accepted;
                let message = if success {
                    "Practice verified: Software integrity failure demonstrated! The system accepted an untrusted package into active runtime."
                } else {
                    "Practice pending: Submit an unverified configuration package to /api/deploy_update."
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
                        "Correct! Software and Data Integrity Failures challenge complete."
                    } else {
                        "Incorrect flag. Hint: Deploy an untrusted configuration package with debug_backdoor enabled to compromise integrity."
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
