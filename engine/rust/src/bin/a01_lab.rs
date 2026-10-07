use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::process::exit;

const FLAG: &str = "ZITERA{b10k3n_4cc355_c0ntr01_m45t3r}";

#[derive(Clone)]
struct User {
    id: u32,
    username: &'static str,
    password: &'static str,
}

#[derive(Clone)]
struct Invoice {
    id: u32,
    user_id: u32,
    title: &'static str,
    amount: f64,
    details: &'static str,
    status: &'static str,
}

struct AppState {
    users: Vec<User>,
    invoices: Vec<Invoice>,
    sessions: HashMap<String, u32>, // session_cookie -> user_id
    idor_exploited: bool,
}

impl AppState {
    fn new() -> Self {
        Self {
            users: vec![
                User {
                    id: 1,
                    username: "alice",
                    password: "password123",
                },
                User {
                    id: 2,
                    username: "bob",
                    password: "bobpass",
                },
                User {
                    id: 99,
                    username: "admin",
                    password: "adminsecret_unbrute",
                },
            ],
            invoices: vec![
                Invoice {
                    id: 1,
                    user_id: 1,
                    title: "Cloud Hosting - March",
                    amount: 49.99,
                    details: "Standard 2 vCPU VPS Instance",
                    status: "PAID",
                },
                Invoice {
                    id: 2,
                    user_id: 2,
                    title: "Domain Registration (bob.io)",
                    amount: 14.50,
                    details: "1 Year Registration",
                    status: "PAID",
                },
                Invoice {
                    id: 42,
                    user_id: 99,
                    title: "CLASSIFIED: Master System License & Audit",
                    amount: 99999.00,
                    details: "FLAG: ZITERA{b10k3n_4cc355_c0ntr01_m45t3r}",
                    status: "CONFIDENTIAL",
                },
            ],
            sessions: HashMap::new(),
            idor_exploited: false,
        }
    }
}

fn html_page(content: &str, user_opt: Option<&str>) -> String {
    let user_header = match user_opt {
        Some(u) => format!(
            "<p>Logged in as: <strong>{}</strong> | <a href=\"/logout\">Logout</a></p><p><a href=\"/invoice/1\">View My Invoice (#1)</a></p>",
            u
        ),
        None => "<p style=\"color: #94a3b8;\">Please sign in to access employee accounting portal.</p>".to_string(),
    };

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>ZITERA Finance Portal — A01 Lab</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #0f172a; color: #f8fafc; margin: 0; padding: 40px; }}
        .card {{ background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 24px; max-width: 650px; margin: 0 auto; box-shadow: 0 4px 12px rgba(0,0,0,0.3); }}
        .badge {{ display: inline-block; background: #dc2626; color: white; padding: 4px 8px; border-radius: 4px; font-size: 11px; font-weight: bold; margin-bottom: 12px; }}
        input[type=text], input[type=password] {{ width: 100%; padding: 10px; margin: 8px 0; background: #0f172a; border: 1px solid #475569; color: white; border-radius: 4px; box-sizing: border-box; }}
        button {{ background: #e63946; color: white; border: none; padding: 10px 18px; border-radius: 4px; cursor: pointer; font-weight: bold; }}
        button:hover {{ background: #c52233; }}
        a {{ color: #38bdf8; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
        .invoice-box {{ background: #0f172a; border-left: 4px solid #e63946; padding: 16px; margin-top: 16px; }}
        pre {{ background: #020617; padding: 12px; color: #a7f3d0; border-radius: 4px; overflow-x: auto; }}
    </style>
</head>
<body>
    <div class="card">
        <span class="badge">ZITERA_LAB // A01:2025</span>
        <h2>Internal Financial Accounting Portal</h2>
        {}
        {}
    </div>
</body>
</html>"#,
        user_header, content
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

    // Extract cookie
    let cookie_user = headers.get("cookie").and_then(|c| {
        for part in c.split(';') {
            let kv: Vec<&str> = part.trim().split('=').collect();
            if kv.len() == 2 && kv[0] == "session_user" {
                return Some(kv[1].to_string());
            }
        }
        None
    });

    let current_user_id = cookie_user
        .as_ref()
        .and_then(|u| state.sessions.get(u))
        .copied();
    let current_user_name = current_user_id
        .and_then(|uid| state.users.iter().find(|u| u.id == uid).map(|u| u.username));

    // 1. Health endpoint
    if path == "/health" {
        resp_headers.insert("Content-Type".to_string(), "application/json".to_string());
        let json = serde_json::json!({
            "status": "ok",
            "lab": "A01",
            "runtime": "native_sandboxed",
            "pid": std::process::id()
        });
        return (200, resp_headers, json.to_string());
    }

    // 2. Root / Index
    if path == "/" || path == "/index.html" {
        if let Some(user) = current_user_name {
            let html = html_page(
                "<p>Welcome to the financial accounting portal. Select an invoice above to view billing records.</p>",
                Some(user),
            );
            return (200, resp_headers, html);
        } else {
            let login_form = r#"
            <form method="POST" action="/login">
                <label>Username</label>
                <input type="text" name="username" placeholder="Username (e.g. alice)" required>
                <label>Password</label>
                <input type="password" name="password" placeholder="Password (e.g. password123)" required>
                <button type="submit">Sign In</button>
            </form>"#;
            let html = html_page(login_form, None);
            return (200, resp_headers, html);
        }
    }

    // 3. Login
    if path == "/login" && method == "POST" {
        // Parse form data: username=alice&password=password123
        let mut form = HashMap::new();
        for pair in body.split('&') {
            if let Some((k, v)) = pair.split_once('=') {
                form.insert(k.trim(), v.trim());
            }
        }

        let username = form.get("username").copied().unwrap_or("");
        let password = form.get("password").copied().unwrap_or("");

        if let Some(user) = state
            .users
            .iter()
            .find(|u| u.username == username && u.password == password)
        {
            let session_token = format!("user_sess_{}", user.id);
            state.sessions.insert(session_token.clone(), user.id);
            resp_headers.insert(
                "Set-Cookie".to_string(),
                format!("session_user={}; Path=/; HttpOnly", session_token),
            );
            resp_headers.insert("Location".to_string(), "/invoice/1".to_string());
            return (302, resp_headers, "Redirecting...".to_string());
        } else {
            let err_html = html_page(
                "<p style=\"color: #ef4444;\">Invalid credentials. Try alice / password123</p><p><a href=\"/\">Try again</a></p>",
                None,
            );
            return (401, resp_headers, err_html);
        }
    }

    // 4. Logout
    if path == "/logout" {
        if let Some(token) = cookie_user {
            state.sessions.remove(&token);
        }
        resp_headers.insert(
            "Set-Cookie".to_string(),
            "session_user=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT".to_string(),
        );
        resp_headers.insert("Location".to_string(), "/".to_string());
        return (302, resp_headers, "Redirecting...".to_string());
    }

    // 5. Invoices (INTENTIONALLY VULNERABLE IDOR ENDPOINT)
    if path.starts_with("/invoice/") {
        let id_str = path.trim_start_matches("/invoice/").trim_end_matches('/');
        if let Ok(inv_id) = id_str.parse::<u32>() {
            if let Some(inv) = state.invoices.iter().find(|i| i.id == inv_id) {
                // If invoice 42 is accessed, mark IDOR as exploited
                if inv.id == 42 {
                    state.idor_exploited = true;
                }

                let content = format!(
                    r#"<div class="invoice-box">
                        <h3>Invoice #{}: {}</h3>
                        <p><strong>Owner User ID:</strong> {}</p>
                        <p><strong>Amount:</strong> ${:.2}</p>
                        <p><strong>Status:</strong> {}</p>
                        <p><strong>Details / Line Items:</strong></p>
                        <pre>{}</pre>
                    </div>"#,
                    inv.id, inv.title, inv.user_id, inv.amount, inv.status, inv.details
                );
                let html = html_page(&content, current_user_name);
                return (200, resp_headers, html);
            } else {
                let html = html_page(
                    "<p style=\"color: #f87171;\">Invoice not found.</p>",
                    current_user_name,
                );
                return (404, resp_headers, html);
            }
        }
    }

    let not_found = html_page("<p>404 Not Found</p>", current_user_name);
    (404, resp_headers, not_found)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--help" || a == "-h" || a == "/?") {
        println!("Zitera Sandboxed Lab A01 (Broken Access Control) v1.0.1");
        println!("This native lab communicates with the host engine broker via stdin/stdout JSON IPC.");
        println!("Usage: a01-lab.exe");
        return;
    }
    if args.iter().any(|a| a == "--version" || a == "-v" || a == "-V") {
        println!("A01 v1.0.1 (native_sandboxed)");
        return;
    }

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    let mut state = AppState::new();

    // 1. Initial Handshake: Emit READY message
    let ready_json = serde_json::json!({
        "type": "READY",
        "lab_id": "A01",
        "version": "1.0.1"
    });
    let _ = writeln!(stdout, "{}", ready_json);
    let _ = stdout.flush();

    // 2. Main Protocol Loop over Stdio
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

                let mut headers = HashMap::new();
                if let Some(h_obj) = val.get("headers").and_then(|v| v.as_object()) {
                    for (k, v) in h_obj {
                        if let Some(s) = v.as_str() {
                            headers.insert(k.to_lowercase(), s.to_string());
                        }
                    }
                }

                let (status, resp_headers, resp_body) =
                    handle_request(&mut state, method, path, &headers, body);

                let resp_json = serde_json::json!({
                    "type": "HTTP_RESPONSE",
                    "id": id,
                    "status": status,
                    "headers": resp_headers,
                    "body": resp_body
                });
                let _ = writeln!(stdout, "{}", resp_json);
                let _ = stdout.flush();
            }
            "HEALTH" => {
                let id = val.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
                let resp_json = serde_json::json!({
                    "type": "STATUS",
                    "id": id,
                    "status": "ok",
                    "lab_id": "A01"
                });
                let _ = writeln!(stdout, "{}", resp_json);
                let _ = stdout.flush();
            }
            "PRACTICE" => {
                let id = val.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
                let resp_json = serde_json::json!({
                    "type": "PRACTICE_RESULT",
                    "id": id,
                    "success": state.idor_exploited,
                    "message": if state.idor_exploited {
                        "Practice verified: You successfully accessed unowned invoice #42 via IDOR!"
                    } else {
                        "Practice incomplete: You have not accessed invoice #42 yet."
                    }
                });
                let _ = writeln!(stdout, "{}", resp_json);
                let _ = stdout.flush();
            }
            "CHALLENGE" => {
                let id = val.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
                let candidate = val
                    .get("flag")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .trim();
                let matches = candidate == FLAG;
                let resp_json = serde_json::json!({
                    "type": "CHALLENGE_RESULT",
                    "id": id,
                    "valid": matches,
                    "message": if matches {
                        "Correct! Challenge flag accepted."
                    } else {
                        "Incorrect flag. Try inspecting invoice #42."
                    }
                });
                let _ = writeln!(stdout, "{}", resp_json);
                let _ = stdout.flush();
            }
            "STOP" => {
                let resp_json = serde_json::json!({
                    "type": "CLEAN_EXIT"
                });
                let _ = writeln!(stdout, "{}", resp_json);
                let _ = stdout.flush();
                exit(0);
            }
            _ => {}
        }
    }
    exit(0);
}
