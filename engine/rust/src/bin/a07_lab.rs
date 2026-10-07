// A07: Authentication Failures — Native Sandboxed Lab Binary
// Simulates an administrative access gateway with intentional authentication flaws:
//   - Zero rate-limiting, account lockout, or CAPTCHA defenses
//   - Short, predictable 4-digit PIN credentials (admin:2026)
//   - Dual web UI and JSON API login endpoints (/login and /api/login)
//   - Protected administrative vault revealing the flag
//
// Communicates via stdio JSON protocol (StdioMessage format).

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::process::exit;

const FLAG: &str = "ZITERA{4uth_f41lur3_brut3_f0rc3_2026}";
const LAB_ID: &str = "A07";
const LAB_VERSION: &str = "1.0.0";
const ADMIN_USER: &str = "admin";
const ADMIN_PIN: &str = "2026";

struct AppState {
    sessions: HashMap<String, String>, // token -> username
    attempt_count: u32,
    brute_force_detected: bool,
    admin_unlocked: bool,
}

impl AppState {
    fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            attempt_count: 0,
            brute_force_detected: false,
            admin_unlocked: false,
        }
    }
}

fn css() -> &'static str {
    r#"
body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #090d16; color: #f1f5f9; margin: 0; padding: 40px; }
.card { background: #131b2e; border: 1px solid #1e293b; border-radius: 10px; padding: 28px; max-width: 600px; margin: 0 auto; box-shadow: 0 10px 25px rgba(0,0,0,0.5); }
.header { border-bottom: 1px solid #1e293b; padding-bottom: 16px; margin-bottom: 20px; display: flex; justify-content: space-between; align-items: center; }
.badge { background: #7c3aed; color: #ede9fe; padding: 4px 10px; border-radius: 4px; font-size: 12px; font-weight: bold; }
.banner { background: #311042; border-left: 4px solid #a855f7; padding: 12px; margin-bottom: 20px; font-size: 13px; color: #f3e8ff; }
.field { margin-bottom: 16px; }
label { display: block; font-size: 13px; color: #94a3b8; margin-bottom: 6px; }
input[type="text"], input[type="password"] { width: 100%; box-sizing: border-box; padding: 10px; background: #090d16; border: 1px solid #334155; color: white; border-radius: 6px; }
button { width: 100%; padding: 12px; background: #7c3aed; color: white; border: none; border-radius: 6px; font-weight: bold; cursor: pointer; }
button:hover { background: #6d28d9; }
.error { color: #f87171; font-size: 13px; margin-bottom: 12px; }
.flag-box { background: #0f172a; border: 2px dashed #4ade80; border-radius: 8px; padding: 16px; margin-top: 16px; color: #4ade80; font-family: monospace; font-size: 15px; font-weight: bold; text-align: center; }
.hint { font-size: 12px; color: #64748b; margin-top: 12px; }
a { color: #a855f7; text-decoration: none; font-size: 13px; }
a:hover { text-decoration: underline; }
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

fn extract_cookie(headers: &HashMap<String, String>, cookie_name: &str) -> Option<String> {
    let cookie_hdr = headers.get("cookie")?;
    for part in cookie_hdr.split(';') {
        let mut kv = part.splitn(2, '=');
        if let Some(name) = kv.next() {
            if name.trim().eq_ignore_ascii_case(cookie_name) {
                return kv.next().map(|v| v.trim().to_string());
            }
        }
    }
    None
}

fn render_login_page(error: Option<&str>) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><title>OmniAuth Control Portal — A07 Lab</title>");
    html.push_str("<style>");
    html.push_str(css());
    html.push_str("</style></head><body><div class=\"card\">");
    html.push_str("<div class=\"header\"><div><h2 style=\"margin: 0;\">OmniAuth // Admin Gateway</h2>");
    html.push_str("<div style=\"color: #94a3b8; font-size: 12px; margin-top: 4px;\">OWASP A07:2025 • Authentication Failures</div></div>");
    html.push_str("<span class=\"badge\">LAB A07 • PORT 8017</span></div>");
    html.push_str("<div class=\"banner\"><strong>SECURITY AUDIT NOTICE:</strong> Notice the complete absence of rate-limiting, CAPTCHA, or account lockout. Brute-force attacks against weak credentials can succeed rapidly.</div>");
    
    if let Some(err) = error {
        html.push_str(&format!("<div class=\"error\">⚠ {}</div>", html_escape(err)));
    }

    html.push_str("<form method=\"POST\" action=\"/login\">");
    html.push_str("<div class=\"field\"><label for=\"username\">Administrative Username</label>");
    html.push_str("<input type=\"text\" id=\"username\" name=\"username\" value=\"admin\" required></div>");
    html.push_str("<div class=\"field\"><label for=\"password\">Security PIN / Password (4-Digit Year)</label>");
    html.push_str("<input type=\"password\" id=\"password\" name=\"password\" placeholder=\"Enter PIN...\" required autofocus></div>");
    html.push_str("<button type=\"submit\">Authenticate</button></form>");
    html.push_str("<div class=\"hint\">Tip: The administrator configured their 4-digit PIN using the current release cycle year (e.g. 2026).</div>");
    html.push_str("</div></body></html>");
    html
}

fn render_admin_page() -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><title>OmniAuth // Admin Vault</title>");
    html.push_str("<style>");
    html.push_str(css());
    html.push_str("</style></head><body><div class=\"card\">");
    html.push_str("<div class=\"header\"><div><h2 style=\"margin: 0;\">OmniAuth // Admin Gateway</h2>");
    html.push_str("<div style=\"color: #94a3b8; font-size: 12px; margin-top: 4px;\">OWASP A07:2025 • Authentication Failures</div></div>");
    html.push_str("<span class=\"badge\">AUTHENTICATED</span></div>");
    html.push_str("<div style=\"color: #4ade80; font-weight: bold; margin-bottom: 10px;\">✓ Authentication Succeeded! Welcome Administrator.</div>");
    html.push_str("<p style=\"color: #94a3b8; font-size: 14px;\">You have successfully bypassed the authentication barrier due to weak credential requirements and zero brute-force protection.</p>");
    html.push_str(&format!("<div class=\"flag-box\">{}</div>", FLAG));
    html.push_str("<div style=\"margin-top: 24px;\"><a href=\"/logout\">← Log Out</a></div>");
    html.push_str("</div></body></html>");
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
            "port": 8017
        })
        .to_string();
        return (200, resp_headers, body);
    }

    let is_auth = extract_cookie(headers, "auth_session")
        .and_then(|tok| state.sessions.get(&tok))
        .is_some();

    if clean_path == "/" {
        if is_auth {
            resp_headers.insert("Location".to_string(), "/admin".to_string());
            return (302, resp_headers, String::new());
        } else {
            resp_headers.insert("Location".to_string(), "/login".to_string());
            return (302, resp_headers, String::new());
        }
    }

    if clean_path == "/login" {
        if method == "POST" {
            state.attempt_count += 1;
            let form = parse_form_body(body);
            let user = form.get("username").map(|s| s.trim()).unwrap_or("");
            let pw = form.get("password").map(|s| s.trim()).unwrap_or("");

            if state.attempt_count >= 3 {
                state.brute_force_detected = true;
            }

            if user == ADMIN_USER && pw == ADMIN_PIN {
                let token = format!("sess_{:x}_{}", state.attempt_count, 1337);
                state.sessions.insert(token.clone(), ADMIN_USER.to_string());
                state.admin_unlocked = true;
                resp_headers.insert("Set-Cookie".to_string(), format!("auth_session={}; Path=/", token));
                resp_headers.insert("Location".to_string(), "/admin".to_string());
                return (302, resp_headers, String::new());
            }

            resp_headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
            let html = render_login_page(Some("Invalid credentials. (Attempt logged, zero rate-limit applied)"));
            return (200, resp_headers, html);
        }

        if is_auth {
            resp_headers.insert("Location".to_string(), "/admin".to_string());
            return (302, resp_headers, String::new());
        }

        resp_headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
        return (200, resp_headers, render_login_page(None));
    }

    if clean_path == "/api/login" && method == "POST" {
        state.attempt_count += 1;
        let (user, pw) = if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(body) {
            let u = json_val.get("username").and_then(|v| v.as_str()).unwrap_or("").trim().to_string();
            let p = json_val.get("password").and_then(|v| v.as_str()).unwrap_or("").trim().to_string();
            (u, p)
        } else {
            let form = parse_form_body(body);
            let u = form.get("username").map(|s| s.trim()).unwrap_or("").to_string();
            let p = form.get("password").map(|s| s.trim()).unwrap_or("").to_string();
            (u, p)
        };

        if state.attempt_count >= 3 {
            state.brute_force_detected = true;
        }

        resp_headers.insert("content-type".to_string(), "application/json".to_string());
        if user == ADMIN_USER && pw == ADMIN_PIN {
            state.admin_unlocked = true;
            let resp = serde_json::json!({
                "status": "passed",
                "message": "Authentication successful!",
                "flag": FLAG
            });
            return (200, resp_headers, resp.to_string());
        } else {
            let resp = serde_json::json!({
                "status": "failed",
                "message": "Invalid credentials"
            });
            return (401, resp_headers, resp.to_string());
        }
    }

    if clean_path == "/admin" {
        if !is_auth {
            resp_headers.insert("Location".to_string(), "/login".to_string());
            return (302, resp_headers, String::new());
        }
        state.admin_unlocked = true;
        resp_headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
        return (200, resp_headers, render_admin_page());
    }

    if clean_path == "/logout" {
        resp_headers.insert("Set-Cookie".to_string(), "auth_session=deleted; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT".to_string());
        resp_headers.insert("Location".to_string(), "/login".to_string());
        return (302, resp_headers, String::new());
    }

    resp_headers.insert("content-type".to_string(), "text/html".to_string());
    (404, resp_headers, "<h1>404 Not Found</h1>".to_string())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--help" || a == "-h" || a == "/?") {
        println!("Zitera Sandboxed Lab A07 (Identification and Authentication Failures) v{}", LAB_VERSION);
        println!("This native lab communicates with the host engine broker via stdin/stdout JSON IPC.");
        println!("Usage: a07-lab.exe");
        return;
    }
    if args.iter().any(|a| a == "--version" || a == "-v" || a == "-V") {
        println!("A07 v{} (native_sandboxed)", LAB_VERSION);
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
                let success = state.admin_unlocked || state.brute_force_detected || state.attempt_count > 0;
                let message = if state.admin_unlocked {
                    "Practice verified: You authenticated to the administrative portal using weak credential brute-force!"
                } else if state.brute_force_detected {
                    "Practice verified: Rapid credential attempts observed without rate limiting or account lockout."
                } else if state.attempt_count > 0 {
                    "Practice partially verified: Login attempt recorded. Test brute-forcing weak 4-digit PINs."
                } else {
                    "Practice incomplete: Submit credentials to /login or /api/login to evaluate rate-limiting."
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
                        "Correct! Authentication failures challenge complete."
                    } else {
                        "Incorrect flag. Hint: Authenticate as admin with the 4-digit year PIN to unlock the admin vault."
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
