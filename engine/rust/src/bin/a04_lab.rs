// A04: Cryptographic Failures — Native Sandboxed Lab Binary
// Simulates an enterprise cryptographic key management vault with intentional weaknesses:
//   - Unsalted MD5 password hashes exposed in /api/audit/hashes
//   - Admin authentication bypass via reversed MD5 rainbow lookup
//   - Protected /vault endpoint revealing activation key
//
// Communicates via stdio JSON protocol (StdioMessage format).

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::process::exit;

const FLAG: &str = "ZITERA{cryp70_f41lur35_w34k_k3y_2026}";
const LAB_ID: &str = "A04";
const LAB_VERSION: &str = "1.0.0";

struct AppState {
    sessions: HashMap<String, String>, // token -> username
    hashes_accessed: bool,
    vault_accessed: bool,
}

impl AppState {
    fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            hashes_accessed: false,
            vault_accessed: false,
        }
    }
}

fn css() -> &'static str {
    r#"
body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #0f172a; color: #f8fafc; margin: 0; padding: 40px; }
.card { background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 24px; max-width: 650px; margin: 0 auto; box-shadow: 0 4px 12px rgba(0,0,0,0.3); }
.badge { display: inline-block; background: #8b5cf6; color: white; padding: 4px 8px; border-radius: 4px; font-size: 11px; font-weight: bold; margin-bottom: 12px; }
.banner { background: #2e1065; border-left: 4px solid #8b5cf6; padding: 12px; margin-bottom: 16px; font-size: 13px; color: #e9d5ff; }
input[type=text], input[type=password] { width: 100%; padding: 10px; margin: 8px 0; background: #0f172a; border: 1px solid #475569; color: white; border-radius: 4px; box-sizing: border-box; }
button { background: #8b5cf6; color: white; border: none; padding: 10px 18px; border-radius: 4px; cursor: pointer; font-weight: bold; }
button:hover { background: #7c3aed; }
a { color: #38bdf8; text-decoration: none; }
a:hover { text-decoration: underline; }
table { width: 100%; border-collapse: collapse; margin-top: 14px; font-size: 12px; }
th, td { border: 1px solid #334155; padding: 8px 12px; text-align: left; }
th { background: #0f172a; color: #8b5cf6; }
code { background: #334155; padding: 2px 6px; border-radius: 4px; font-family: monospace; font-size: 12px; color: #38bdf8; }
"#
}

fn get_user_md5(username: &str) -> &'static str {
    match username {
        "alice" => "482c811da5d5b4bc6d497ffa98491e38",
        "bob" => "5ebe2294ecd0e0f08eab7690d2a6ee69",
        "admin" => "21232f297a57a5a743894a0e4a801fc3",
        _ => "",
    }
}

fn verify_password(username: &str, password: &str) -> bool {
    match username {
        "alice" => password == "password123",
        "bob" => password == "secret",
        "admin" => password == "admin",
        _ => false,
    }
}

fn handle_request(
    state: &mut AppState,
    _method: &str,
    path: &str,
    headers: &HashMap<String, String>,
    body: &str,
) -> (u16, HashMap<String, String>, String) {
    let mut resp_headers = HashMap::new();
    resp_headers.insert(
        "Content-Type".to_string(),
        "text/html; charset=utf-8".to_string(),
    );

    // Session cookie extraction
    let session_user = headers.get("cookie").and_then(|c| {
        for part in c.split(';') {
            let kv: Vec<&str> = part.trim().split('=').collect();
            if kv.len() == 2 && kv[0].trim() == "vault_session" {
                let token = kv[1].trim();
                return state.sessions.get(token).cloned();
            }
        }
        None
    });

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

    // 2. Root / Index
    if path == "/" || path == "/index.html" {
        let auth_section = if let Some(ref u) = session_user {
            format!(
                r#"<p>Authenticated Session: <strong>{}</strong></p>
                <p><a href="/vault">Access Master Crypto Vault</a> | <a href="/logout">Logout</a></p>"#,
                u
            )
        } else {
            r#"<p>Please authenticate to access cryptographic vault storage:</p>
            <p><a href="/login"><button>Sign In to Vault</button></a></p>"#.to_string()
        };

        let html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>ZITERA CryptoVault — A04 Cryptographic Failures</title>
    <style>{}</style>
</head>
<body>
    <div class="card">
        <span class="badge">OWASP A04:2025 LAB</span>
        <h2>ZITERA CryptoVault Key Management System</h2>
        <div class="banner">
            Notice: This lab demonstrates cryptographic weaknesses: legacy broken hash algorithms (unsalted MD5) and sensitive data exposure.
        </div>
        <p>Welcome to CryptoVault, the enterprise token and credential validation service.</p>
        {}
        <div style="margin-top: 20px; padding: 12px; background: #0f172a; border-radius: 6px;">
            <p style="margin: 0; font-size: 12px; font-weight: bold; color: #8b5cf6;">Public Cryptographic Inspection Endpoints:</p>
            <ul style="font-size: 12px; color: #cbd5e1; margin-top: 6px;">
                <li><a href="/health">/health</a> — Service health probe</li>
                <li><a href="/api/audit/hashes">/api/audit/hashes</a> — Legacy audit log exposing unsalted MD5 password hashes</li>
                <li><a href="/vault">/vault</a> — Protected Vault storage (Requires Administrator login)</li>
            </ul>
        </div>
    </div>
</body>
</html>"#,
            css(),
            auth_section
        );
        return (200, resp_headers, html);
    }

    // 3. Hashes audit endpoint (INTENTIONALLY VULNERABLE)
    if path == "/api/audit/hashes" {
        state.hashes_accessed = true;
        resp_headers.insert("Content-Type".to_string(), "application/json".to_string());
        let json = serde_json::json!({
            "status": "success",
            "description": "Legacy authentication audit table. Warning: MD5 is cryptographically broken and vulnerable to lookup tables / rainbow tables.",
            "records": [
                {"username": "alice", "role": "Analyst", "hash_type": "MD5 (Unsalted)", "hash": get_user_md5("alice")},
                {"username": "bob", "role": "Auditor", "hash_type": "MD5 (Unsalted)", "hash": get_user_md5("bob")},
                {"username": "admin", "role": "SecurityOfficer", "hash_type": "MD5 (Unsalted)", "hash": get_user_md5("admin")}
            ]
        });
        return (200, resp_headers, json.to_string());
    }

    // 4. Login GET
    if path == "/login" && _method == "GET" {
        let html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>CryptoVault Sign In — A04</title>
    <style>{}</style>
</head>
<body>
    <div class="card">
        <span class="badge">OWASP A04:2025 LAB</span>
        <h2>CryptoVault Sign In</h2>
        <form method="POST" action="/login">
            <label style="font-size: 13px;">Username</label>
            <input type="text" name="username" placeholder="e.g. admin" required>
            <label style="font-size: 13px;">Password</label>
            <input type="password" name="password" placeholder="Password" required>
            <button type="submit" style="margin-top: 10px;">Authenticate</button>
        </form>
        <p style="font-size: 12px; color: #94a3b8; margin-top: 14px;">
            Hint: Have you checked <code>/api/audit/hashes</code> for password hashes? Modern rainbow tables or hash lookups reverse unsalted MD5 instantly.
        </p>
        <p><a href="/">Return to Portal</a></p>
    </div>
</body>
</html>"#,
            css()
        );
        return (200, resp_headers, html);
    }

    // 5. Login POST
    if path == "/login" && _method == "POST" {
        let mut form: HashMap<String, String> = HashMap::new();
        for pair in body.split('&') {
            if let Some((k, v)) = pair.split_once('=') {
                form.insert(k.trim().to_string(), v.trim().to_string());
            }
        }

        let username = form.get("username").cloned().unwrap_or_default();
        let password = form.get("password").cloned().unwrap_or_default();

        let valid = verify_password(&username, &password);

        if valid {
            let token = format!("sess_{}_{}", username, std::process::id());
            state.sessions.insert(token.clone(), username.clone());
            resp_headers.insert(
                "Set-Cookie".to_string(),
                format!("vault_session={}; Path=/; HttpOnly", token),
            );
            let target = if username == "admin" { "/vault" } else { "/" };
            resp_headers.insert("Location".to_string(), target.to_string());
            return (302, resp_headers, "Redirecting...".to_string());
        } else {
            let html = format!(
                r#"<!DOCTYPE html>
<html>
<head><style>{}</style></head>
<body>
    <div class="card">
        <p style="color: #ef4444;">Authentication failed: Password hash mismatch.</p>
        <p><a href="/login">Try again</a></p>
    </div>
</body>
</html>"#,
                css()
            );
            return (401, resp_headers, html);
        }
    }

    // 6. Vault (Protected)
    if path == "/vault" {
        if session_user.as_deref() != Some("admin") {
            resp_headers.insert("Location".to_string(), "/login".to_string());
            return (302, resp_headers, "Redirecting to login...".to_string());
        }

        state.vault_accessed = true;
        let html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Master Cryptographic Vault — A04</title>
    <style>{}</style>
</head>
<body>
    <div class="card">
        <span class="badge">A04:2025 VAULT</span>
        <h2 style="color: #4ade80;">Master Cryptographic Vault</h2>
        <p>Authentication Succeeded: Logged in as SecurityOfficer (admin).</p>
        <div style="background: #0f172a; padding: 18px; border-radius: 6px; border-left: 4px solid #8b5cf6;">
            <p style="margin: 0; font-size: 13px; font-weight: bold; color: #a78bfa;">CLASSIFIED VAULT SECRET ACTIVATION KEY:</p>
            <p style="font-family: monospace; font-size: 16px; color: #38bdf8; font-weight: bold; margin: 10px 0;">
                {}
            </p>
            <p style="margin: 0; font-size: 11px; color: #94a3b8;">
                Root Cause: The administrator's password ('admin') was stored as an unsalted MD5 hash (<code>21232f297a57a5a743894a0e4a801fc3</code>).
            </p>
        </div>
        <p style="margin-top: 18px;"><a href="/">Return to Portal</a> | <a href="/logout">Logout</a></p>
    </div>
</body>
</html>"#,
            css(),
            FLAG
        );
        return (200, resp_headers, html);
    }

    // 7. Logout
    if path == "/logout" {
        if let Some(c) = headers.get("cookie") {
            for part in c.split(';') {
                let kv: Vec<&str> = part.trim().split('=').collect();
                if kv.len() == 2 && kv[0].trim() == "vault_session" {
                    state.sessions.remove(kv[1].trim());
                }
            }
        }
        resp_headers.insert(
            "Set-Cookie".to_string(),
            "vault_session=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT".to_string(),
        );
        resp_headers.insert("Location".to_string(), "/".to_string());
        return (302, resp_headers, "Redirecting...".to_string());
    }

    (404, resp_headers, "<h1>404 Not Found</h1>".to_string())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--help" || a == "-h" || a == "/?") {
        println!("Zitera Sandboxed Lab A04 (Cryptographic Failures) v{}", LAB_VERSION);
        println!("This native lab communicates with the host engine broker via stdin/stdout JSON IPC.");
        println!("Usage: a04-lab.exe");
        return;
    }
    if args.iter().any(|a| a == "--version" || a == "-v" || a == "-V") {
        println!("A04 v{} (native_sandboxed)", LAB_VERSION);
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
                let success = state.hashes_accessed || state.vault_accessed;
                let message = if state.vault_accessed {
                    "Practice verified: You reversed the MD5 hash and unlocked the administrator vault!"
                } else if state.hashes_accessed {
                    "Practice partially verified: You discovered the unsalted MD5 hashes table at /api/audit/hashes."
                } else {
                    "Practice incomplete: Inspect the crypto portal and find the exposed password hashes."
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
                        "Correct! Cryptographic failures challenge complete."
                    } else {
                        "Incorrect flag. Hint: Reverse the admin's unsalted MD5 hash from /api/audit/hashes and open /vault."
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
