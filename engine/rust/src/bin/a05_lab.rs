// A05: Injection — Native Sandboxed Lab Binary
// Simulates an enterprise hardware logistics catalog with SQL Injection vulnerabilities:
//   - Unsanitized string interpolation in SQL query
//   - Single-quote breakout inducing sqlite3.OperationalError
//   - Boolean-based authentication/filtering bypass (' OR 1=1 --)
//   - UNION-based database extraction from hidden `vault_secrets` table
//
// Communicates via stdio JSON protocol (StdioMessage format).

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::process::exit;

const FLAG: &str = "ZITERA{5q1_1nj3ct10n_m45t3r_2026}";
const LAB_ID: &str = "A05";
const LAB_VERSION: &str = "1.0.0";

struct Product {
    name: &'static str,
    category: &'static str,
    price: f64,
}

const PRODUCTS: &[Product] = &[
    Product { name: "Edge Firewall Gateway", category: "Hardware", price: 1299.00 },
    Product { name: "Rackmount Server 2U", category: "Hardware", price: 3499.50 },
    Product { name: "Managed 48-Port PoE Switch", category: "Networking", price: 850.00 },
    Product { name: "Cat6A Ethernet Bulk Cable (1000ft)", category: "Cables", price: 189.99 },
    Product { name: "Biometric Access Terminal", category: "Security", price: 450.00 },
];

struct AppState {
    syntax_error_seen: bool,
    boolean_injected: bool,
    union_extracted: bool,
}

impl AppState {
    fn new() -> Self {
        Self {
            syntax_error_seen: false,
            boolean_injected: false,
            union_extracted: false,
        }
    }
}

fn css() -> &'static str {
    r#"
body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #0f172a; color: #f8fafc; margin: 0; padding: 40px; }
.card { background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 24px; max-width: 750px; margin: 0 auto; box-shadow: 0 4px 12px rgba(0,0,0,0.3); }
.badge { display: inline-block; background: #0284c7; color: white; padding: 4px 8px; border-radius: 4px; font-size: 11px; font-weight: bold; margin-bottom: 12px; }
input[type=text] { width: 75%; padding: 10px; background: #0f172a; border: 1px solid #475569; color: white; border-radius: 4px; box-sizing: border-box; }
button { width: 22%; background: #0284c7; color: white; border: none; padding: 10px; border-radius: 4px; cursor: pointer; font-weight: bold; }
button:hover { background: #0369a1; }
table { width: 100%; border-collapse: collapse; margin-top: 20px; }
th, td { text-align: left; padding: 10px; border-bottom: 1px solid #334155; }
th { background: #0f172a; color: #94a3b8; }
.err { background: #450a0a; border: 1px solid #991b1b; padding: 12px; border-radius: 4px; color: #fca5a5; margin-top: 16px; font-family: monospace; font-size: 12px; }
.info { background: #082f49; border-left: 4px solid #0284c7; padding: 10px 14px; margin-bottom: 16px; font-size: 13px; color: #bae6fd; }
"#
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

struct QueryResult {
    rows: Vec<(String, String, String)>,
    error: Option<String>,
}

fn execute_simulated_query(state: &mut AppState, raw_q: &str) -> QueryResult {
    let q = raw_q.trim();
    if q.is_empty() {
        return QueryResult {
            rows: Vec::new(),
            error: None,
        };
    }

    let upper_q = q.to_uppercase();

    // 1. Check for single-quote syntax break (no comment to close it)
    let has_comment = q.contains("--") || q.contains('#');
    let quote_count = q.chars().filter(|&c| c == '\'').count();

    // If odd number of single quotes and no comment, trigger syntax error
    if (quote_count % 2 == 1) && !has_comment {
        state.syntax_error_seen = true;
        return QueryResult {
            rows: Vec::new(),
            error: Some("sqlite3.OperationalError: unrecognized token: \"'%\"".to_string()),
        };
    }

    // 2. Check for UNION SELECT injection
    if upper_q.contains("UNION") && upper_q.contains("SELECT") {
        state.union_extracted = true;
        let mut rows = Vec::new();

        // If prefix had normal term e.g. "Server' UNION...", filter normal products too
        let prefix = q.split('\'').next().unwrap_or("").trim();
        if !prefix.is_empty() {
            let lower_pre = prefix.to_lowercase();
            for p in PRODUCTS {
                if p.name.to_lowercase().contains(&lower_pre) {
                    rows.push((p.name.to_string(), p.category.to_string(), format!("{:.2}", p.price)));
                }
            }
        }

        // Schema query or vault_secrets query
        if upper_q.contains("SQLITE_MASTER") || upper_q.contains("SQLITE_SCHEMA") {
            rows.push(("products".to_string(), "CREATE TABLE products (id INT, name TEXT, category TEXT, price REAL)".to_string(), "0.00".to_string()));
            rows.push(("vault_secrets".to_string(), "CREATE TABLE vault_secrets (id INT, secret_name TEXT, secret_data TEXT)".to_string(), "0.00".to_string()));
        } else if upper_q.contains("VAULT_SECRETS") || upper_q.contains("FLAG") || upper_q.contains("SECRET") {
            rows.push((
                "PROJECT_ZITERA_CORE_FLAG".to_string(),
                format!("FLAG: {}", FLAG),
                "0.00".to_string(),
            ));
        } else {
            // Generic UNION dummy values e.g. ' UNION SELECT 1, 2, 3 --
            rows.push(("1".to_string(), "2".to_string(), "3.00".to_string()));
        }

        return QueryResult { rows, error: None };
    }

    // 3. Check for Boolean-based bypass (' OR 1=1 --, ' OR 'a'='a, etc.)
    if upper_q.contains("OR 1=1")
        || upper_q.contains("OR '1'='1'")
        || upper_q.contains("OR \"1\"=\"1\"")
        || upper_q.contains("OR ''=''")
        || upper_q.contains("OR TRUE")
    {
        state.boolean_injected = true;
        let mut rows = Vec::new();
        for p in PRODUCTS {
            rows.push((p.name.to_string(), p.category.to_string(), format!("{:.2}", p.price)));
        }
        return QueryResult { rows, error: None };
    }

    // 4. Normal parameterized search
    let lower_q = q.to_lowercase();
    let mut rows = Vec::new();
    for p in PRODUCTS {
        if p.name.to_lowercase().contains(&lower_q) || p.category.to_lowercase().contains(&lower_q) {
            rows.push((p.name.to_string(), p.category.to_string(), format!("{:.2}", p.price)));
        }
    }

    QueryResult { rows, error: None }
}

fn render_page(query: &str, result: &QueryResult) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>Central Logistics Hardware Catalog — A05 Lab</title>");
    html.push_str("<style>");
    html.push_str(css());
    html.push_str("</style></head><body><div class=\"card\">");
    html.push_str("<span class=\"badge\">ZITERA_LAB // A05:2025</span>");
    html.push_str("<h2>Central Logistics Hardware Catalog</h2>");
    html.push_str("<div class=\"info\">Internal search portal connected to SQLite backend storage. Query item records by hardware name or category.</div>");
    
    html.push_str("<form method=\"GET\" action=\"/\">");
    html.push_str(&format!(
        "<input type=\"text\" name=\"q\" value=\"{}\" placeholder=\"Search products (e.g. Server, Switch, Firewall)...\">",
        html_escape(query)
    ));
    html.push_str("<button type=\"submit\">Search</button></form>");

    if let Some(err) = &result.error {
        html.push_str(&format!(
            "<div class=\"err\"><strong>Database Error:</strong><br>{}</div>",
            html_escape(err)
        ));
    } else if !result.rows.is_empty() {
        html.push_str("<table><thead><tr><th>Product Name</th><th>Category</th><th>Price (USD)</th></tr></thead><tbody>");
        for row in &result.rows {
            html.push_str(&format!(
                "<tr><td><strong>{}</strong></td><td>{}</td><td>${}</td></tr>",
                html_escape(&row.0),
                html_escape(&row.1),
                html_escape(&row.2),
            ));
        }
        html.push_str("</tbody></table>");
    } else if !query.is_empty() {
        html.push_str("<p style=\"margin-top: 24px; color: #94a3b8;\">No inventory items found matching your query.</p>");
    }

    html.push_str("</div></body></html>");
    html
}

fn parse_query_param(path: &str, param_name: &str) -> String {
    if let Some(pos) = path.find('?') {
        let query_str = &path[pos + 1..];
        for pair in query_str.split('&') {
            let mut parts = pair.splitn(2, '=');
            if let Some(key) = parts.next() {
                if key == param_name {
                    let val = parts.next().unwrap_or("");
                    // URL decode
                    return urldecode(val);
                }
            }
        }
    }
    String::new()
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

fn handle_request(
    state: &mut AppState,
    _method: &str,
    path: &str,
    _headers: &HashMap<String, String>,
    _body: &str,
) -> (u16, HashMap<String, String>, String) {
    let mut resp_headers = HashMap::new();
    let clean_path = path.split('?').next().unwrap_or("/");

    if clean_path == "/health" {
        resp_headers.insert("content-type".to_string(), "application/json".to_string());
        let body = serde_json::json!({
            "status": "ok",
            "lab": LAB_ID,
            "port": 8015
        })
        .to_string();
        return (200, resp_headers, body);
    }

    if clean_path == "/" {
        let q = parse_query_param(path, "q");
        let result = execute_simulated_query(state, &q);
        let html = render_page(&q, &result);
        resp_headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
        return (200, resp_headers, html);
    }

    if clean_path == "/api/search" {
        let q = parse_query_param(path, "q");
        let result = execute_simulated_query(state, &q);
        resp_headers.insert("content-type".to_string(), "application/json".to_string());
        let items_json: Vec<serde_json::Value> = result
            .rows
            .iter()
            .map(|r| serde_json::json!({ "name": r.0, "category": r.1, "price": r.2 }))
            .collect();
        let body = serde_json::json!({
            "query": q,
            "items": items_json,
            "error": result.error
        })
        .to_string();
        return (200, resp_headers, body);
    }

    resp_headers.insert("content-type".to_string(), "text/html".to_string());
    (404, resp_headers, "<h1>404 Not Found</h1>".to_string())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--help" || a == "-h" || a == "/?") {
        println!("Zitera Sandboxed Lab A05 (Injection) v{}", LAB_VERSION);
        println!("This native lab communicates with the host engine broker via stdin/stdout JSON IPC.");
        println!("Usage: a05-lab.exe");
        return;
    }
    if args.iter().any(|a| a == "--version" || a == "-v" || a == "-V") {
        println!("A05 v{} (native_sandboxed)", LAB_VERSION);
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
                let success = state.union_extracted || state.boolean_injected || state.syntax_error_seen;
                let message = if state.union_extracted {
                    "Practice verified: You successfully performed UNION-based injection to extract records from vault_secrets!"
                } else if state.boolean_injected {
                    "Practice verified: You injected a boolean SQL condition (' OR 1=1 --) and bypassed search filtering."
                } else if state.syntax_error_seen {
                    "Practice partially verified: You triggered an operational database syntax error using a single quote."
                } else {
                    "Practice incomplete: Inject a single quote (') to test for syntax errors, or use ' OR 1=1 -- to dump inventory."
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
                        "Correct! SQL injection extraction challenge complete."
                    } else {
                        "Incorrect flag. Hint: Perform UNION-based injection on /?q= to extract secrets from vault_secrets."
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
