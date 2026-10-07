// A03: Software Supply Chain Failures — Native Sandboxed Lab Binary
// Simulates an internal corporate package registry and dependency audit system:
//   - Dependency audit inspector (/packages/audit/package_lock_audit.json)
//   - Unpinned untrusted internal telemetry dependency
//   - Compromised lockfile exfiltrating pipeline credentials
//   - Token verification endpoint
//
// Communicates via stdio JSON protocol (StdioMessage format).

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::process::exit;

const FLAG: &str = "ZITERA{5upply_ch41n_p0150n1ng_d3p_2026}";
const LAB_ID: &str = "A03";
const LAB_VERSION: &str = "1.0.1";

struct AppState {
    audit_accessed: bool,
    token_verified: bool,
}

impl AppState {
    fn new() -> Self {
        Self {
            audit_accessed: false,
            token_verified: false,
        }
    }
}

fn css() -> &'static str {
    r#"
body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #0f172a; color: #f8fafc; margin: 0; padding: 24px; }
.container { max-width: 900px; margin: 0 auto; }
.header { border-bottom: 2px solid #334155; padding-bottom: 16px; margin-bottom: 24px; display: flex; justify-content: space-between; align-items: center; }
.badge { background: #0369a1; color: #e0f2fe; padding: 4px 10px; border-radius: 4px; font-size: 12px; font-weight: bold; }
.card { background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 20px; margin-bottom: 20px; box-shadow: 0 4px 12px rgba(0,0,0,0.3); }
.banner { background: #1e1b4b; border-left: 4px solid #6366f1; padding: 12px; margin-bottom: 16px; font-size: 13px; color: #e0e7ff; }
table { width: 100%; border-collapse: collapse; margin-top: 12px; }
th, td { text-align: left; padding: 10px; border-bottom: 1px solid #334155; font-size: 13px; }
th { background: #0f172a; color: #94a3b8; }
.code { font-family: monospace; background: #0f172a; padding: 2px 6px; border-radius: 4px; color: #38bdf8; }
a { color: #38bdf8; text-decoration: none; }
a:hover { text-decoration: underline; }
.tag-warn { color: #f59e0b; font-weight: bold; }
input[type=text] { width: 100%; padding: 10px; background: #0f172a; border: 1px solid #334155; color: white; border-radius: 4px; box-sizing: border-box; }
button { padding: 10px 18px; background: #6366f1; color: white; border: none; border-radius: 4px; cursor: pointer; font-weight: bold; }
button:hover { background: #4f46e5; }
"#
}

fn handle_request(
    state: &mut AppState,
    _method: &str,
    path: &str,
    _headers: &HashMap<String, String>,
    body: &str,
) -> (u16, HashMap<String, String>, String) {
    let mut resp_headers = HashMap::new();
    resp_headers.insert(
        "Content-Type".to_string(),
        "text/html; charset=utf-8".to_string(),
    );

    // 1. Health
    if path == "/health" {
        resp_headers.insert("Content-Type".to_string(), "application/json".to_string());
        let json = serde_json::json!({
            "status": "ok",
            "lab": LAB_ID,
            "version": LAB_VERSION,
            "runtime": "native_sandboxed",
            "pid": std::process::id()
        });
        return (200, resp_headers, json.to_string());
    }

    // 2. Index / Root
    if path == "/" || path == "/index.html" {
        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>ApexCorp — Supply Chain & Package Registry</title>
    <style>{}</style>
</head>
<body>
<div class="container">
    <div class="header">
        <div>
            <h2>ApexCorp // Internal Package Registry</h2>
            <div style="color: #94a3b8; font-size: 13px;">Software Supply Chain Dependency Inspector • v2.4.1</div>
        </div>
        <span class="badge">LAB A03 • OWASP:2025</span>
    </div>

    <div class="banner">
        <strong>SECURITY NOTICE:</strong> Modern applications incorporate hundreds of external packages. Always audit lockfiles and verify signature provenance before allowing packages into automated production builds.
    </div>

    <div class="card">
        <h3>Active Registry Dependencies</h3>
        <p style="color: #94a3b8; font-size: 13px;">
            The CI/CD pipeline recently completed an automated dependency audit. Download and inspect the full lockfile audit manifest:
            <br>
            <a href="/packages/audit/package_lock_audit.json" class="code">GET /packages/audit/package_lock_audit.json</a>
        </p>

        <table>
            <thead>
                <tr>
                    <th>Package Name</th>
                    <th>Resolved Version</th>
                    <th>Registry Source</th>
                    <th>Status</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td><code>express</code></td>
                    <td>4.19.2</td>
                    <td>https://registry.npmjs.org/</td>
                    <td style="color: #4ade80;">Verified</td>
                </tr>
                <tr>
                    <td><code>lodash</code></td>
                    <td>4.17.21</td>
                    <td>https://registry.npmjs.org/</td>
                    <td style="color: #4ade80;">Verified</td>
                </tr>
                <tr>
                    <td><code>apex-internal-telemetry</code></td>
                    <td>0.1.0-alpha.build.88</td>
                    <td>http://127.0.0.1:8013/packages/download/</td>
                    <td class="tag-warn">UNTRUSTED / UNPINNED</td>
                </tr>
            </tbody>
        </table>
    </div>

    <div class="card">
        <h3>Pipeline Verification Terminal</h3>
        <p style="color: #94a3b8; font-size: 13px;">
            Verify an exfiltrated pipeline token recovered from the compromised dependency manifest:
        </p>
        <form method="POST" action="/api/supply-chain/verify" style="display: flex; gap: 8px;">
            <input type="text" name="token" placeholder="ZITERA{{...}}" required>
            <button type="submit">Verify Token</button>
        </form>
    </div>
</div>
</body>
</html>"#,
            css()
        );
        return (200, resp_headers, html);
    }

    // 3. Package Lock Audit Manifest
    if path == "/packages/audit/package_lock_audit.json" {
        state.audit_accessed = true;
        resp_headers.insert("Content-Type".to_string(), "application/json".to_string());
        let candidates = [
            "packages/package_lock_audit.json",
            "docker/packages/package_lock_audit.json",
            "../../labs/A03/packages/package_lock_audit.json",
        ];
        for candidate in &candidates {
            if let Ok(content) = std::fs::read_to_string(candidate) {
                return (200, resp_headers, content);
            }
        }
        let fallback = serde_json::json!({
            "name": "apexcorp-dependency-manager",
            "version": "2.4.1",
            "lockfileVersion": 3,
            "dependencies": {
                "express": { "version": "4.19.2" },
                "lodash": { "version": "4.17.21" },
                "apex-internal-telemetry": {
                    "version": "0.1.0-alpha.build.88",
                    "exfiltrated_pipeline_token": FLAG
                }
            }
        });
        return (200, resp_headers, fallback.to_string());
    }

    // 4. API Packages list
    if path == "/api/packages" {
        resp_headers.insert("Content-Type".to_string(), "application/json".to_string());
        let json = serde_json::json!({
            "packages": [
                {"name": "express", "version": "4.19.2", "verified": true},
                {"name": "lodash", "version": "4.17.21", "verified": true},
                {"name": "apex-internal-telemetry", "version": "0.1.0-alpha.build.88", "verified": false}
            ]
        });
        return (200, resp_headers, json.to_string());
    }

    // 5. Verify Token API / Form POST
    if path == "/api/supply-chain/verify" {
        resp_headers.insert("Content-Type".to_string(), "application/json".to_string());

        // Extract token from form body or JSON body
        let mut token = String::new();
        if body.contains("token=") {
            for pair in body.split('&') {
                if let Some((k, v)) = pair.split_once('=') {
                    if k.trim() == "token" {
                        token = urlencoding_decode(v.trim());
                    }
                }
            }
        } else if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body) {
            if let Some(t) = parsed.get("token").and_then(|v| v.as_str()) {
                token = t.trim().to_string();
            }
        }

        if token == FLAG {
            state.token_verified = true;
            let json = serde_json::json!({
                "status": "passed",
                "message": "Flag verified! Supply chain dependency successfully audited.",
                "flag": FLAG
            });
            return (200, resp_headers, json.to_string());
        } else {
            let json = serde_json::json!({
                "status": "failed",
                "message": "Invalid deployment token. Review /packages/audit/package_lock_audit.json for compromised package metadata."
            });
            return (400, resp_headers, json.to_string());
        }
    }

    // 404
    (
        404,
        resp_headers,
        "{\"error\": \"Endpoint not found\"}".to_string(),
    )
}

fn urlencoding_decode(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars();
    while let Some(ch) = chars.next() {
        if ch == '%' {
            let h1 = chars.next();
            let h2 = chars.next();
            if let (Some(c1), Some(c2)) = (h1, h2) {
                let hex_str = format!("{}{}", c1, c2);
                if let Ok(byte) = u8::from_str_radix(&hex_str, 16) {
                    result.push(byte as char);
                    continue;
                }
            }
        } else if ch == '+' {
            result.push(' ');
        } else {
            result.push(ch);
        }
    }
    result
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--help" || a == "-h" || a == "/?") {
        println!("Zitera Sandboxed Lab A03 (Software Supply Chain Failures) v{}", LAB_VERSION);
        println!("This native lab communicates with the host engine broker via stdin/stdout JSON IPC.");
        println!("Usage: a03-lab.exe");
        return;
    }
    if args.iter().any(|a| a == "--version" || a == "-v" || a == "-V") {
        println!("A03 v{} (native_sandboxed)", LAB_VERSION);
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
                let success = state.audit_accessed || state.token_verified;
                let message = if state.token_verified {
                    "Practice verified: You successfully audited the supply chain and verified the exfiltrated dependency token!"
                } else if state.audit_accessed {
                    "Practice partially verified: You located the lockfile audit manifest. Now identify the malicious dependency token."
                } else {
                    "Practice incomplete: Inspect the package registry and download the dependency audit lockfile."
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
                let valid = candidate == FLAG;
                let resp = serde_json::json!({
                    "type": "CHALLENGE_RESULT",
                    "id": id,
                    "valid": valid,
                    "message": if valid {
                        "Correct! Software supply chain failure challenge complete."
                    } else {
                        "Incorrect flag. Hint: Inspect the exfiltrated_pipeline_token in package_lock_audit.json."
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
