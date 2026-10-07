//! Standalone Native Sandboxed Lab Binary for OWASP A06: Insecure Design
//!
//! Replaces legacy Python/Docker implementation.
//! Operates purely over stdio machine protocol within Windows AppContainer + Job Object sandbox.
//! Implements the flawed procurement workflow state machine.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::process::exit;

const FLAG: &str = "ZITERA{1n53cur3_d351gn_fl4w3d_w0rkfl0w}";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub item: String,
    pub amount: u64,
    pub status: String,
    pub owner: String,
}

#[derive(Debug, Clone)]
pub struct LabState {
    pub orders: HashMap<String, Order>,
}

impl Default for LabState {
    fn default() -> Self {
        let mut orders = HashMap::new();
        orders.insert(
            "1001".to_string(),
            Order {
                id: "1001".to_string(),
                item: "Office Stationery".to_string(),
                amount: 120,
                status: "DRAFT".to_string(),
                owner: "alice".to_string(),
            },
        );
        orders.insert(
            "1002".to_string(),
            Order {
                id: "1002".to_string(),
                item: "High-Performance AI Server".to_string(),
                amount: 15000,
                status: "PENDING_APPROVAL".to_string(),
                owner: "bob".to_string(),
            },
        );
        orders.insert(
            "9999".to_string(),
            Order {
                id: "9999".to_string(),
                item: "Executive Datacenter Cluster".to_string(),
                amount: 50000,
                status: "DRAFT".to_string(),
                owner: "learner".to_string(),
            },
        );
        Self { orders }
    }
}

fn html_dashboard(state: &LabState) -> String {
    let mut order_rows = String::new();
    let mut sorted_keys: Vec<String> = state.orders.keys().cloned().collect();
    sorted_keys.sort();

    for key in sorted_keys {
        if let Some(o) = state.orders.get(&key) {
            let badge_class = match o.status.as_str() {
                "DRAFT" => "badge-draft",
                "PENDING_APPROVAL" => "badge-pending",
                "APPROVED" => "badge-approved",
                "DISPATCHED" => "badge-dispatched",
                _ => "badge-draft",
            };
            order_rows.push_str(&format!(
                "<tr><td><strong>#{}</strong></td><td>{}</td><td>${}</td><td>{}</td><td><span class=\"badge {}\">{}</span></td></tr>",
                o.id, o.item, o.amount, o.owner, badge_class, o.status
            ));
        }
    }

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Zitera Corp - Procurement Portal (A06)</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: #0b0f19; color: #f1f5f9; margin: 0; padding: 32px; }}
        .container {{ max-width: 900px; margin: 0 auto; background: #131b2e; border: 1px solid #1e293b; border-radius: 8px; padding: 28px; box-shadow: 0 4px 16px rgba(0,0,0,0.4); }}
        h1 {{ color: #60a5fa; font-family: monospace; margin-top: 0; }}
        table {{ width: 100%; border-collapse: collapse; margin-top: 16px; }}
        th, td {{ border: 1px solid #1e293b; padding: 12px; text-align: left; }}
        th {{ background: #1a243b; color: #94a3b8; font-size: 13px; text-transform: uppercase; }}
        .badge {{ padding: 4px 10px; border-radius: 4px; font-weight: bold; font-size: 12px; }}
        .badge-draft {{ background: #475569; color: #f8fafc; }}
        .badge-pending {{ background: #f59e0b; color: #0f172a; }}
        .badge-approved {{ background: #10b981; color: #0f172a; }}
        .badge-dispatched {{ background: #8b5cf6; color: #f8fafc; }}
        .card {{ background: #1a243b; padding: 20px; border-radius: 6px; margin-top: 24px; border: 1px solid #29354f; }}
        code {{ background: #0f172a; padding: 2px 6px; border-radius: 4px; color: #38bdf8; font-family: monospace; }}
        pre {{ background: #0f172a; padding: 14px; border-radius: 4px; color: #38bdf8; overflow-x: auto; font-family: monospace; }}
        button {{ background: #3b82f6; color: white; border: none; padding: 10px 18px; border-radius: 4px; cursor: pointer; font-weight: bold; }}
        button:hover {{ background: #2563eb; }}
        ul {{ padding-left: 20px; color: #cbd5e1; }}
        li {{ margin-bottom: 8px; }}
    </style>
</head>
<body>
<div class="container">
    <h1>[A06:2025] Zitera Procurement Workflow</h1>
    <p style="color: #94a3b8;">Target Scenario: Business Logic & Workflow State Machine Insecure Design.</p>

    <div class="card">
        <h3 style="margin-top: 0; color: #e2e8f0;">System Business Rules (Intended vs Designed)</h3>
        <ul>
            <li><strong>Rule 1:</strong> Orders &lt; $500 auto-approve upon submission.</li>
            <li><strong>Rule 2:</strong> Orders &gt;= $500 require executive manager signature and verification.</li>
            <li><strong>Design Defect:</strong> The client API endpoint accepts arbitrary state transition payloads (e.g. <code>target_state</code>) or bypasses the approval phase directly to DISPATCHED!</li>
        </ul>
    </div>

    <h3 style="color: #e2e8f0; margin-top: 28px;">Active Procurement Orders</h3>
    <table>
        <tr><th>Order ID</th><th>Procurement Item</th><th>Budget Amount</th><th>Requestor</th><th>Workflow State</th></tr>
        {}
    </table>

    <div class="card">
        <h3 style="margin-top: 0; color: #e2e8f0;">Workflow API Endpoints</h3>
        <p>Transition order state: <code>POST /api/orders/&lt;id&gt;/transition</code></p>
        <p>Payload: <code>{{"target_state": "DISPATCHED"}}</code> or <code>{{"status": "APPROVED"}}</code></p>
        <p>Signal verification: <code>GET /practice/verify</code></p>
    </div>
</div>
</body>
</html>"#,
        order_rows
    )
}

fn handle_request(
    state: &mut LabState,
    method: &str,
    path: &str,
    _headers: &HashMap<String, String>,
    body: &str,
) -> (u16, HashMap<String, String>, String) {
    let mut resp_headers = HashMap::new();
    resp_headers.insert(
        "Content-Type".to_string(),
        "text/html; charset=utf-8".to_string(),
    );
    resp_headers.insert("Access-Control-Allow-Origin".to_string(), "*".to_string());

    if method == "GET" {
        if path == "/" || path == "/index.html" || path.is_empty() {
            let html = html_dashboard(state);
            return (200, resp_headers, html);
        } else if path == "/health" {
            resp_headers.insert("Content-Type".to_string(), "application/json".to_string());
            let json = serde_json::json!({
                "status": "ok",
                "lab": "A06",
                "runtime": "native_sandboxed"
            });
            return (200, resp_headers, json.to_string());
        } else if path == "/practice/verify" {
            resp_headers.insert("Content-Type".to_string(), "application/json".to_string());
            let high_val_bypassed = state
                .orders
                .values()
                .any(|o| o.amount >= 15000 && (o.status == "APPROVED" || o.status == "DISPATCHED"));
            if high_val_bypassed {
                let json = serde_json::json!({
                    "status": "passed",
                    "message": "Insecure design reproduced: High-value order bypassed manager approval into authorized state!"
                });
                return (200, resp_headers, json.to_string());
            } else {
                let json = serde_json::json!({
                    "status": "failed",
                    "message": "Practice target pending: Manipulate order #1002 or #9999 state machine into APPROVED or DISPATCHED."
                });
                return (200, resp_headers, json.to_string());
            }
        } else if path == "/api/orders" {
            resp_headers.insert("Content-Type".to_string(), "application/json".to_string());
            let orders_list: Vec<&Order> = state.orders.values().collect();
            let json = serde_json::json!({ "orders": orders_list });
            return (200, resp_headers, json.to_string());
        } else {
            return (404, resp_headers, "Not Found".to_string());
        }
    }

    if method == "POST" {
        resp_headers.insert("Content-Type".to_string(), "application/json".to_string());

        if path.starts_with("/api/orders/") && path.ends_with("/transition") {
            let parts: Vec<&str> = path.split('/').collect();
            // Path: /api/orders/<id>/transition
            if parts.len() >= 4 {
                let order_id = parts[3];
                if let Some(order) = state.orders.get_mut(order_id) {
                    let parsed_payload: serde_json::Value =
                        serde_json::from_str(body).unwrap_or(serde_json::json!({}));
                    let target_state = parsed_payload
                        .get("target_state")
                        .or_else(|| parsed_payload.get("status"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("APPROVED");

                    // Flawed State Machine: Arbitrary client state assignment without authorization check
                    order.status = target_state.to_string();

                    let mut resp_json = serde_json::json!({
                        "message": format!("Order {} transitioned to {}", order_id, target_state),
                        "order": order
                    });

                    if order_id == "9999"
                        && (target_state == "APPROVED" || target_state == "DISPATCHED")
                    {
                        resp_json["flag"] = serde_json::json!(FLAG);
                    }

                    return (200, resp_headers, resp_json.to_string());
                } else {
                    let err = serde_json::json!({ "error": "Order not found" });
                    return (404, resp_headers, err.to_string());
                }
            }
        } else if path == "/api/reset" {
            *state = LabState::default();
            let json = serde_json::json!({
                "status": "reset",
                "message": "Procurement orders reset to default."
            });
            return (200, resp_headers, json.to_string());
        }
    }

    (404, resp_headers, "Not Found".to_string())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--help" || a == "-h" || a == "/?") {
        println!("Zitera Sandboxed Lab A06 (Insecure Design) v1.0.1");
        println!("This native lab communicates with the host engine broker via stdin/stdout JSON IPC.");
        println!("Usage: a06-lab.exe");
        return;
    }
    if args.iter().any(|a| a == "--version" || a == "-v" || a == "-V") {
        println!("A06 v1.0.1 (native_sandboxed)");
        return;
    }

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut state = LabState::default();

    // 1. Handshake: Announce READY over stdio
    let ready_msg = serde_json::json!({
        "type": "READY",
        "lab_id": "A06",
        "version": "1.0.1"
    });
    let _ = writeln!(stdout, "{}", ready_msg);
    let _ = stdout.flush();

    // 2. Process incoming deterministic stdio requests
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
                if let Some(obj) = val.get("headers").and_then(|v| v.as_object()) {
                    for (k, v) in obj {
                        if let Some(s) = v.as_str() {
                            headers.insert(k.clone(), s.to_string());
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
                    "lab_id": "A06"
                });
                let _ = writeln!(stdout, "{}", resp_json);
                let _ = stdout.flush();
            }
            "PRACTICE" => {
                let id = val.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
                let high_val_bypassed = state.orders.values().any(|o| {
                    o.amount >= 15000 && (o.status == "APPROVED" || o.status == "DISPATCHED")
                });
                let resp_json = serde_json::json!({
                    "type": "PRACTICE_RESULT",
                    "id": id,
                    "passed": high_val_bypassed,
                    "message": if high_val_bypassed {
                        "Practice passed: High-value order bypassed manager approval!"
                    } else {
                        "Practice pending: Transition order #1002 or #9999 to APPROVED or DISPATCHED."
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
                    "passed": matches,
                    "message": if matches {
                        "Authoritative flag verified successfully!"
                    } else {
                        "Flag submission incorrect."
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
}
