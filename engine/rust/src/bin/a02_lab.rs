// A02: Security Misconfiguration — Native Sandboxed Lab Binary
// Simulates an internal operations gateway with intentional misconfigurations:
//   - Default credentials (admin:admin)
//   - Unauthenticated debug/diagnostic endpoint (/debug/vars)
//   - Directory listing enabled on /backups/
//   - Downloadable backup config exposing the flag
//
// Communicates via stdio JSON protocol (StdioMessage format).

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::process::exit;

const FLAG: &str = "ZITERA{53cur1ty_m15c0nf1g_b4ckup_134k}";
const LAB_ID: &str = "A02";
const LAB_VERSION: &str = "1.0.1";

struct AppState {
    sessions: HashMap<String, String>, // token -> username
    backup_accessed: bool,
    debug_accessed: bool,
    default_creds_used: bool,
}

impl AppState {
    fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            backup_accessed: false,
            debug_accessed: false,
            default_creds_used: false,
        }
    }
}

fn css() -> &'static str {
    r#"
body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #0f172a; color: #f8fafc; margin: 0; padding: 40px; }
.card { background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 24px; max-width: 700px; margin: 0 auto; box-shadow: 0 4px 12px rgba(0,0,0,0.3); }
.badge { display: inline-block; background: #d97706; color: white; padding: 4px 8px; border-radius: 4px; font-size: 11px; font-weight: bold; margin-bottom: 12px; }
input[type=text], input[type=password] { width: 100%; padding: 10px; margin: 8px 0; background: #0f172a; border: 1px solid #475569; color: white; border-radius: 4px; box-sizing: border-box; }
button { background: #d97706; color: white; border: none; padding: 10px 18px; border-radius: 4px; cursor: pointer; font-weight: bold; }
button:hover { background: #b45309; }
a { color: #38bdf8; text-decoration: none; }
a:hover { text-decoration: underline; }
pre { background: #020617; padding: 12px; color: #a7f3d0; border-radius: 4px; overflow-x: auto; font-size: 13px; }
.warn { background: #431407; border-left: 4px solid #d97706; padding: 12px 16px; margin: 12px 0; border-radius: 4px; }
ul.dir-listing { list-style: none; padding: 0; }
ul.dir-listing li { padding: 6px 0; border-bottom: 1px solid #1e293b; }
ul.dir-listing li a { color: #34d399; }
"#
}

fn html_page(title: &str, body_content: &str, logged_in_as: Option<&str>) -> String {
    let nav = if let Some(user) = logged_in_as {
        format!(
            r#"<p>Logged in as: <strong>{}</strong> | <a href="/logout">Logout</a></p>"#,
            user
        )
    } else {
        r#"<p style="color: #94a3b8;">Not authenticated. <a href="/login">Sign in</a></p>"#
            .to_string()
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>{} — ZITERA Lab A02</title>
    <style>{}</style>
</head>
<body>
    <div class="card">
        <span class="badge">ZITERA_LAB // A02:2025</span>
        <h2>{}</h2>
        {}
        {}
    </div>
</body>
</html>"#,
        title,
        css(),
        title,
        nav,
        body_content
    )
}

fn handle_request(
    state: &mut AppState,
    method: &str,
    path: &str,
    headers: &HashMap<String, String>,
    body: &str,
) -> (u16, HashMap<String, String>, String) {
    let mut resp_headers = HashMap::new();
    resp_headers.insert(
        "Content-Type".to_string(),
        "text/html; charset=utf-8".to_string(),
    );

    // Extract session cookie -> username
    let session_user = headers.get("cookie").and_then(|c| {
        for part in c.split(';') {
            let kv: Vec<&str> = part.trim().split('=').collect();
            if kv.len() == 2 && kv[0].trim() == "ops_session" {
                let token = kv[1].trim().to_string();
                return state.sessions.get(&token).cloned();
            }
        }
        None
    });

    // --- Health ---
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

    // --- Root / ---
    if path == "/" || path == "/index.html" {
        let content = r#"
<p>Welcome to the <strong>OpsGateway Operations Portal</strong>.</p>
<p>Internal system for operations and configuration management.</p>
<ul>
  <li><a href="/login">Login to Portal</a></li>
  <li><a href="/debug/vars">Diagnostics (debug mode)</a></li>
  <li><a href="/backups/">Backup Archive</a></li>
</ul>
<div class="warn">
  <strong>NOTICE:</strong> This system is for internal use only. Unauthorized access is prohibited.
</div>"#;
        return (
            200,
            resp_headers,
            html_page("OpsGateway Portal", content, session_user.as_deref()),
        );
    }

    // --- Login GET ---
    if path == "/login" && method == "GET" {
        let form = r#"
<form method="POST" action="/login">
    <label>Username</label>
    <input type="text" name="username" placeholder="Username" required>
    <label>Password</label>
    <input type="password" name="password" placeholder="Password" required>
    <button type="submit">Sign In</button>
</form>
<p style="color: #64748b; font-size: 13px;">Default credentials apply. Try the most obvious option.</p>"#;
        return (
            200,
            resp_headers,
            html_page("OpsGateway Login", form, None),
        );
    }

    // --- Login POST ---
    if path == "/login" && method == "POST" {
        let mut form: HashMap<&str, &str> = HashMap::new();
        for pair in body.split('&') {
            if let Some((k, v)) = pair.split_once('=') {
                form.insert(k.trim(), v.trim());
            }
        }
        let username = form.get("username").copied().unwrap_or("").to_lowercase();
        let password = form.get("password").copied().unwrap_or("").to_lowercase();

        // Intentional vulnerability: default credentials admin:admin
        let valid = (username == "admin" && password == "admin")
            || (username == "ops" && password == "ops123");

        if valid {
            if username == "admin" && password == "admin" {
                state.default_creds_used = true;
            }
            let token = format!("sess_{}_{}", username, std::process::id());
            state.sessions.insert(token.clone(), username.clone());
            resp_headers.insert(
                "Set-Cookie".to_string(),
                format!("ops_session={}; Path=/; HttpOnly", token),
            );
            resp_headers.insert("Location".to_string(), "/admin".to_string());
            return (302, resp_headers, "Redirecting...".to_string());
        } else {
            let err = r#"<p style="color: #ef4444;">Invalid credentials.</p><p><a href="/login">Try again</a></p>"#;
            return (
                401,
                resp_headers,
                html_page("Login Failed", err, None),
            );
        }
    }

    // --- Admin panel ---
    if path == "/admin" {
        if session_user.is_none() {
            resp_headers.insert("Location".to_string(), "/login".to_string());
            return (302, resp_headers, "Redirecting...".to_string());
        }
        let content = format!(
            r#"
<p>Welcome, <strong>{}</strong>. You have full administrative access.</p>
<h3>Admin Functions</h3>
<ul>
  <li><a href="/debug/vars">System Diagnostics</a></li>
  <li><a href="/backups/">Backup Archive</a></li>
  <li><a href="/admin/config">System Configuration</a></li>
</ul>
<div class="warn">
  You are logged in with default administrator credentials. This is a security misconfiguration.
</div>"#,
            session_user.as_deref().unwrap_or("admin")
        );
        return (
            200,
            resp_headers,
            html_page("Admin Dashboard", &content, session_user.as_deref()),
        );
    }

    // --- Debug/vars endpoint (INTENTIONALLY UNAUTHENTICATED) ---
    if path == "/debug/vars" {
        state.debug_accessed = true;
        resp_headers.insert("Content-Type".to_string(), "application/json".to_string());
        let json = serde_json::json!({
            "app_name": "OpsGateway",
            "version": "2.1.4",
            "debug_mode": true,
            "environment": "production",
            "internal_storage": "/app/backups/",
            "database_host": "internal-db.ops.local",
            "database_port": 5432,
            "admin_email": "ops-admin@corp.internal",
            "session_secret": "default-ops-session-secret-2024",
            "backup_path": "/backups/",
            "runtime_pid": std::process::id(),
            "uptime_seconds": 3600
        });
        return (200, resp_headers, json.to_string());
    }

    // --- Directory listing on /backups/ (INTENTIONALLY VULNERABLE) ---
    if path == "/backups/" || path == "/backups" {
        state.backup_accessed = true;
        let content = r#"
<h3>Index of /backups/</h3>
<ul class="dir-listing">
  <li><a href="/backups/backup_config.json.bak">backup_config.json.bak</a> — 2.1 KB — 2025-03-01</li>
  <li><a href="/backups/ops_keys_old.txt">ops_keys_old.txt</a> — 0.8 KB — 2024-11-15</li>
  <li><a href="/backups/database_schema.sql.bak">database_schema.sql.bak</a> — 18 KB — 2025-01-20</li>
</ul>
<div class="warn">
  Directory listing is enabled. This is a security misconfiguration!
</div>"#;
        return (
            200,
            resp_headers,
            html_page("Backup Archive Directory", content, session_user.as_deref()),
        );
    }

    // --- Backup config file (contains flag) ---
    if path == "/backups/backup_config.json.bak" {
        state.backup_accessed = true;
        resp_headers.insert(
            "Content-Type".to_string(),
            "application/json".to_string(),
        );
        resp_headers.insert(
            "Content-Disposition".to_string(),
            "attachment; filename=\"backup_config.json.bak\"".to_string(),
        );
        let json = serde_json::json!({
            "schema": "opsgateway-backup-v2",
            "generated_at": "2025-03-01T02:00:00Z",
            "app_version": "2.1.4",
            "database": {
                "host": "internal-db.ops.local",
                "port": 5432,
                "name": "opsdb",
                "user": "opsadmin",
                "password_hash": "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3"
            },
            "admin_system_key": FLAG,
            "session_secret": "default-ops-session-secret-2024",
            "backup_storage": "/app/backups/",
            "notes": "Restore key stored in admin_system_key field. Guard carefully."
        });
        return (200, resp_headers, json.to_string());
    }

    // --- Old ops keys (non-flag file, adds realism) ---
    if path == "/backups/ops_keys_old.txt" {
        resp_headers.insert("Content-Type".to_string(), "text/plain".to_string());
        return (
            200,
            resp_headers,
            "# DEPRECATED: Old operations keys — DO NOT USE\n# Rotated: 2024-11-15\nOPS_API_KEY_V1=deprecated-key-12345\nOPS_API_KEY_V2=deprecated-key-67890\n".to_string(),
        );
    }

    // --- Logout ---
    if path == "/logout" {
        if let Some(c) = headers.get("cookie") {
            for part in c.split(';') {
                let kv: Vec<&str> = part.trim().split('=').collect();
                if kv.len() == 2 && kv[0].trim() == "ops_session" {
                    state.sessions.remove(kv[1].trim());
                }
            }
        }
        resp_headers.insert(
            "Set-Cookie".to_string(),
            "ops_session=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT".to_string(),
        );
        resp_headers.insert("Location".to_string(), "/".to_string());
        return (302, resp_headers, "Redirecting...".to_string());
    }

    // --- 404 ---
    let not_found = html_page(
        "Not Found",
        "<p>404 — Page not found.</p><p><a href=\"/\">Return to portal</a></p>",
        session_user.as_deref(),
    );
    (404, resp_headers, not_found)
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
                // Practice verified if: debug accessed AND (default creds used OR backup accessed)
                let success = state.debug_accessed
                    || state.default_creds_used
                    || state.backup_accessed;
                let message = if state.backup_accessed {
                    "Practice verified: You discovered the exposed backup archive and retrieved the configuration file!"
                } else if state.default_creds_used {
                    "Practice partially verified: You used default credentials. Now explore the backup archive."
                } else if state.debug_accessed {
                    "Practice partially verified: You found the debug endpoint. Follow the backup path it reveals."
                } else {
                    "Practice incomplete: Explore the OpsGateway portal. Check diagnostic and backup endpoints."
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
                        "Correct! Security misconfiguration challenge complete."
                    } else {
                        "Incorrect flag. Hint: examine the backup_config.json.bak file from the /backups/ directory."
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
