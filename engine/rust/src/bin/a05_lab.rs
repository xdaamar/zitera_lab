// A05: Injection — Native Sandboxed Lab Binary
// Simulates DataFlow CRM — Enterprise Revenue Operations & Deal Flow Management:
//   - Deal pipeline management & client contract inventory (CRUD)
//   - Search query endpoint with dynamic SQL string concatenation
//   - Syntax error disclosure exposing SQLite dialect
//   - UNION-based schema exfiltration revealing Marcus Reid's confidential board vault
//
// Communicates via stdio JSON protocol (StdioMessage format).

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::process::exit;

const FLAG: &str = "ZITERA{5q1_1nj3ct10n_m45t3r_2026}";
const LAB_ID: &str = "A05";
const LAB_VERSION: &str = "1.1.0";

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct Deal {
    id: u32,
    customer: String,
    deal_value: f64,
    stage: String,
    contact_email: String,
    notes: String,
    is_confidential: bool,
}

struct AppState {
    deals: Vec<Deal>,
    next_id: u32,
}

impl AppState {
    fn new() -> Self {
        let initial_deals = vec![
            Deal {
                id: 1,
                customer: "Acme Industrial Logistics".to_string(),
                deal_value: 48500.0,
                stage: "Proposal Sent".to_string(),
                contact_email: "procurement@acme-ind.com".to_string(),
                notes: "Multi-year enterprise gateway license negotiation.".to_string(),
                is_confidential: false,
            },
            Deal {
                id: 2,
                customer: "Globex Cloud Systems".to_string(),
                deal_value: 125000.0,
                stage: "Contract Signed".to_string(),
                contact_email: "vance.a@globex.internal".to_string(),
                notes: "Dedicated cluster provisioning for APAC region.".to_string(),
                is_confidential: false,
            },
            Deal {
                id: 3,
                customer: "Starlight Biotech Ltd".to_string(),
                deal_value: 82000.0,
                stage: "Discovery".to_string(),
                contact_email: "ciso@starlight-bio.org".to_string(),
                notes: "HIPAA compliance audit data pipeline integration.".to_string(),
                is_confidential: false,
            },
            Deal {
                id: 4,
                customer: "FinTech Vault Sovereign".to_string(),
                deal_value: 350000.0,
                stage: "Negotiation".to_string(),
                contact_email: "treasury@vaultsovereign.io".to_string(),
                notes: "High-volume settlement broker integration.".to_string(),
                is_confidential: false,
            },
            Deal {
                id: 5,
                customer: "Rackmount Server 2U Infrastructure".to_string(),
                deal_value: 18500.0,
                stage: "Proposal Sent".to_string(),
                contact_email: "infra@zitera-rack.org".to_string(),
                notes: "Rackmount Server 2U Procurement with Edge Firewall Gateway and Biometric Access Terminal.".to_string(),
                is_confidential: false,
            },
            Deal {
                id: 6,
                customer: "Edge Firewall Gateway Services".to_string(),
                deal_value: 29000.0,
                stage: "Contract Signed".to_string(),
                contact_email: "netsec@zitera-edge.org".to_string(),
                notes: "Edge Firewall Gateway perimeter hardware installation.".to_string(),
                is_confidential: false,
            },
            Deal {
                id: 7,
                customer: "Biometric Access Terminal Fleet".to_string(),
                deal_value: 14500.0,
                stage: "Discovery".to_string(),
                contact_email: "access@zitera-terminal.org".to_string(),
                notes: "Biometric Access Terminal enterprise security upgrade.".to_string(),
                is_confidential: false,
            },
        ];

        Self {
            deals: initial_deals,
            next_id: 5,
        }
    }
}

fn css() -> &'static str {
    r#"
:root {
    --bg-page: #f8fafc;
    --bg-card: #ffffff;
    --bg-subtle: #f1f5f9;
    --border: #e2e8f0;
    --text-heading: #0f172a;
    --text-body: #334155;
    --text-muted: #64748b;
    --primary: #4f46e5;
    --primary-light: #eef2ff;
    --accent-emerald: #059669;
    --accent-emerald-light: #ecfdf5;
    --accent-rose: #e11d48;
    --shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
}
* { box-sizing: border-box; margin: 0; padding: 0; }
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
    background: var(--bg-page);
    color: var(--text-body);
    line-height: 1.5;
    -webkit-font-smoothing: antialiased;
}
.app-container {
    max-width: 1120px;
    margin: 0 auto;
    padding: 28px 24px 60px;
}
.navbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 14px 24px;
    margin-bottom: 24px;
    box-shadow: var(--shadow-sm);
}
.brand {
    display: flex;
    align-items: center;
    gap: 12px;
}
.brand-icon {
    width: 36px;
    height: 36px;
    background: linear-gradient(135deg, #4f46e5, #6366f1);
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    font-size: 18px;
    font-weight: 800;
}
.brand-title {
    font-size: 16px;
    font-weight: 800;
    color: var(--text-heading);
    letter-spacing: -0.01em;
}
.brand-sub {
    font-size: 11px;
    color: var(--text-muted);
}
.nav-links {
    display: flex;
    align-items: center;
    gap: 8px;
}
.nav-link {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-muted);
    text-decoration: none;
    padding: 6px 14px;
    border-radius: 6px;
    transition: all 0.15s ease;
}
.nav-link:hover {
    color: var(--text-heading);
    background: var(--bg-subtle);
}
.nav-link.active {
    color: var(--primary);
    background: var(--primary-light);
}
.card {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 24px;
    margin-bottom: 24px;
    box-shadow: var(--shadow-sm);
}
.stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 16px;
    margin-bottom: 24px;
}
.stat-card {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 10px;
    padding: 16px 20px;
    box-shadow: var(--shadow-sm);
}
.stat-label {
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
    margin-bottom: 6px;
}
.stat-val {
    font-size: 22px;
    font-weight: 800;
    color: var(--text-heading);
}
.badge {
    display: inline-flex;
    align-items: center;
    padding: 3px 10px;
    border-radius: 20px;
    font-size: 11px;
    font-weight: 700;
    line-height: 1.2;
}
.badge-active { background: #dcfce7; color: #15803d; border: 1px solid #bbf7d0; }
.badge-indigo { background: #e0e7ff; color: #4338ca; border: 1px solid #c7d2fe; }
.badge-vault { background: #fef2f2; color: #991b1b; border: 1px solid #fecaca; }
table {
    width: 100%;
    border-collapse: separate;
    border-spacing: 0;
    margin-top: 14px;
}
th {
    background: var(--bg-subtle);
    padding: 10px 16px;
    font-size: 11px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--text-muted);
    text-align: left;
    border-bottom: 1px solid var(--border);
}
th:first-child { border-top-left-radius: 8px; }
th:last-child { border-top-right-radius: 8px; }
td {
    padding: 14px 16px;
    font-size: 13px;
    color: var(--text-body);
    border-bottom: 1px solid var(--border);
    vertical-align: middle;
}
tr:last-child td { border-bottom: none; }
tr:hover td { background: #fafafa; }
code, .code-pill {
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 12px;
    background: var(--bg-subtle);
    padding: 2px 7px;
    border-radius: 4px;
    color: var(--text-heading);
    border: 1px solid var(--border);
}
.btn-primary {
    background: var(--primary);
    color: white;
    font-size: 13px;
    font-weight: 700;
    padding: 9px 18px;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    text-decoration: none;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    transition: background 0.15s ease;
}
.btn-primary:hover { background: #4338ca; }
.btn-secondary {
    background: white;
    color: var(--text-heading);
    font-size: 13px;
    font-weight: 700;
    padding: 9px 18px;
    border: 1px solid var(--border);
    border-radius: 8px;
    cursor: pointer;
    text-decoration: none;
    display: inline-flex;
    align-items: center;
    gap: 8px;
    transition: background 0.15s ease;
}
.btn-secondary:hover { background: var(--bg-subtle); }
input[type=text], input[type=number], textarea, select {
    width: 100%;
    padding: 10px 14px;
    font-size: 13px;
    border: 1px solid var(--border);
    border-radius: 8px;
    background: white;
    color: var(--text-heading);
    outline: none;
}
input[type=text]:focus, textarea:focus { border-color: var(--primary); }
.form-group { margin-bottom: 16px; }
.form-label { display: block; font-size: 12px; font-weight: 700; color: var(--text-heading); margin-bottom: 6px; }
.notice-box {
    background: var(--primary-light);
    border: 1px solid #c7d2fe;
    border-radius: 8px;
    padding: 14px 18px;
    font-size: 13px;
    color: #3730a3;
    margin-bottom: 20px;
}
.err-box {
    background: #fef2f2;
    border: 1px solid #fecaca;
    border-radius: 8px;
    padding: 14px 18px;
    font-family: monospace;
    font-size: 12px;
    color: #991b1b;
    margin-bottom: 20px;
    line-height: 1.5;
}
.flag-banner {
    background: #ecfdf5;
    border: 1.5px solid #6ee7b7;
    border-radius: 10px;
    padding: 20px 24px;
    margin-bottom: 24px;
    display: flex;
    align-items: flex-start;
    gap: 16px;
}
.flag-val {
    font-family: monospace;
    font-size: 14px;
    font-weight: 800;
    color: #047857;
    background: #d1fae5;
    padding: 6px 14px;
    border-radius: 6px;
    margin-top: 8px;
    display: inline-block;
}
"#
}

fn render_navbar(active_tab: &str) -> String {
    format!(
        r#"<header class="navbar">
    <div class="brand">
        <div class="brand-icon"></div>
        <div>
            <div class="brand-title">DataFlow // Revenue Operations CRM</div>
            <div class="brand-sub">Enterprise Deal Flow & Customer Ledger &bull; Port 8015</div>
        </div>
    </div>
    <nav class="nav-links">
        <a href="/" class="nav-link {pipeline_active}">Deals Pipeline</a>
        <a href="/deals/new" class="nav-link {new_active}">+ New Enterprise Deal</a>
        <a href="/api/v1/deals" class="nav-link" target="_blank">API JSON</a>
    </nav>
</header>"#,
        pipeline_active = if active_tab == "pipeline" { "active" } else { "" },
        new_active = if active_tab == "new" { "active" } else { "" }
    )
}

fn handle_request(
    state: &mut AppState,
    method: &str,
    path: &str,
    _headers: &HashMap<String, String>,
    body: &str,
) -> (u16, HashMap<String, String>, String) {
    let mut resp_headers = HashMap::new();
    resp_headers.insert("Content-Type".to_string(), "text/html; charset=utf-8".to_string());

    // 1. Health Probe
    if path == "/health" {
        resp_headers.insert("Content-Type".to_string(), "application/json".to_string());
        let json = serde_json::json!({
            "status": "ok",
            "lab": LAB_ID,
            "version": LAB_VERSION,
            "target": "DataFlow CRM",
            "runtime": "native_sandboxed",
            "pid": std::process::id()
        });
        return (200, resp_headers, json.to_string());
    }

    // 2. Deals Pipeline / Search (Index)
    let clean_path = path.split('?').next().unwrap_or(path);
    if clean_path == "/" || clean_path == "/index.html" || clean_path.starts_with("/deals/search") {
        let mut query = String::new();
        if let Some(q_str) = path.split('?').nth(1) {
            for pair in q_str.split('&') {
                if let Some((k, v)) = pair.split_once('=') {
                    if k == "q" {
                        query = urlencoding_decode(v);
                        break;
                    }
                }
            }
        }

        let mut error_msg: Option<String> = None;
        let mut rendered_deals: Vec<Deal> = Vec::new();
        let mut flag_discovered = false;

        // SQLi Evaluation Logic
        let q_lower = query.to_lowercase();
        if query.contains('\'') && !q_lower.contains("--") && !q_lower.contains("#") {
            // Unbalanced single quote -> trigger syntax error
            error_msg = Some(format!(
                "sqlite3.OperationalError: near \"{}\": syntax error in SELECT id, customer, value, stage, contact, notes FROM deals WHERE (customer LIKE '%{}%' OR notes LIKE '%{}%')",
                query, query, query
            ));
        } else if q_lower.contains("union") && (q_lower.contains("vault") || q_lower.contains("flag") || q_lower.contains("select")) {
            // UNION Injection match -> extract hidden vault table
            flag_discovered = true;
            rendered_deals = state.deals.clone();
            rendered_deals.push(Deal {
                id: 99,
                customer: "PROJECT_ZITERA_CORE_FLAG Marcus Reid (VP Controller) Sovereign Reserve".to_string(),
                deal_value: 5000000.0,
                stage: "BOARD_RESERVE_ESCROW".to_string(),
                contact_email: "m.reid@apexcorp.internal".to_string(),
                notes: format!("PROJECT_ZITERA_CORE_FLAG SOX-404 Compliance Vault Key: {}", FLAG),
                is_confidential: true,
            });
        } else if q_lower.contains("or 1=1") || q_lower.contains("or '1'='1") || q_lower.contains("or 2>1") {
            // Boolean-based SQLi -> returns all records
            rendered_deals = state.deals.clone();
        } else if !query.is_empty() {
            // Standard substring search
            rendered_deals = state
                .deals
                .iter()
                .filter(|d| {
                    d.customer.to_lowercase().contains(&q_lower)
                        || d.notes.to_lowercase().contains(&q_lower)
                        || d.contact_email.to_lowercase().contains(&q_lower)
                })
                .cloned()
                .collect();
        } else {
            rendered_deals = state.deals.clone();
        }

        let mut rows = String::new();
        for d in &rendered_deals {
            let stage_badge = if d.is_confidential {
                "<span class=\"badge badge-vault\">[ALERT] CONFIDENTIAL VAULT</span>"
            } else if d.stage == "Contract Signed" {
                "<span class=\"badge badge-active\"> Contract Signed</span>"
            } else {
                "<span class=\"badge badge-indigo\">In Negotiation</span>"
            };

            let formatted_value = format!("${:.2}", d.deal_value);

            rows.push_str(&format!(
                r#"<tr>
                    <td>
                        <div style="font-weight: 700; color: #0f172a;">{customer}</div>
                        <div style="font-size: 11px; color: #64748b; margin-top: 2px;">ID #{id} &bull; {contact}</div>
                    </td>
                    <td style="font-weight: 700; color: #0f172a;">{val}</td>
                    <td>{stage_badge}</td>
                    <td style="max-width: 320px; font-size: 12px; color: #475569;">{notes}</td>
                </tr>"#,
                id = d.id,
                customer = d.customer,
                contact = d.contact_email,
                val = formatted_value,
                stage_badge = stage_badge,
                notes = d.notes
            ));
        }

        let total_val: f64 = state.deals.iter().map(|d| d.deal_value).sum();
        let total_formatted = format!("${:.2}", total_val);

        let flag_banner = if flag_discovered {
            format!(
                r#"<div class="flag-banner">
                    <div style="font-size: 32px;"></div>
                    <div>
                        <div style="font-size: 16px; font-weight: 800; color: #065f46;">UNION-BASED SQL INJECTION SUCCESSFUL!</div>
                        <p style="font-size: 13px; color: #047857; margin-top: 4px;">
                            You have successfully bypassed query isolation and exfiltrated Marcus Reid's confidential SOX-404 compliance record:
                        </p>
                        <div class="flag-val">{flag}</div>
                    </div>
                </div>"#,
                flag = FLAG
            )
        } else {
            "".to_string()
        };

        let err_box = if let Some(ref e) = error_msg {
            format!(
                r#"<div class="err-box">
                    <strong>INTERNAL DATABASE EXCEPTION:</strong><br/>
                    {err}
                </div>"#,
                err = e
            )
        } else {
            "".to_string()
        };

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Deals Pipeline // DataFlow CRM</title>
    <style>{css}</style>
</head>
<body>
<div class="app-container">
    {navbar}

    {flag_banner}
    {err_box}

    <div class="stats-grid">
        <div class="stat-card">
            <div class="stat-label">Active Deals in Pipeline</div>
            <div class="stat-val">{deals_count}</div>
        </div>
        <div class="stat-card">
            <div class="stat-label">Total Pipeline Revenue</div>
            <div class="stat-val" style="color: #4f46e5;">{total_rev}</div>
        </div>
        <div class="stat-card">
            <div class="stat-label">Database Engine</div>
            <div class="stat-val" style="font-size: 14px; margin-top: 4px; color: #0f766e;">SQLite 3.42 (Raw Concatenation)</div>
        </div>
        <div class="stat-card">
            <div class="stat-label">Active Filter</div>
            <div class="stat-val" style="font-size: 13px; margin-top: 4px; color: #64748b;">{active_q}</div>
        </div>
    </div>

    <div class="card">
        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; gap: 16px;">
            <form method="GET" action="/deals/search" style="display: flex; gap: 10px; flex: 1;">
                <input type="text" name="q" placeholder="Filter customer name or notes (e.g. Acme, Logistics)..." value="{q_val}">
                <button type="submit" class="btn-primary">Search</button>
                <a href="/" class="btn-secondary">Reset</a>
            </form>
            <a href="/deals/new" class="btn-primary">+ Create Deal</a>
        </div>

        <table>
            <thead>
                <tr>
                    <th>Customer & Contact</th>
                    <th>Deal Value (USD)</th>
                    <th>Sales Stage</th>
                    <th>Internal Notes & Telemetry</th>
                </tr>
            </thead>
            <tbody>
                {rows}
            </tbody>
        </table>
    </div>

    <div class="notice-box">
        <strong>[TIPS] Pentester Reconnaissance Note:</strong><br/>
        Audit the search parameter <code>/deals/search?q=...</code>. What happens if you submit a single quote <code>'</code>? Does the application disclose the internal database query structure? Try constructing a <code>' UNION SELECT ...</code> payload to extract the hidden <code>vault_secrets</code> table!
    </div>
</div>
</body>
</html>"#,
            css = css(),
            navbar = render_navbar("pipeline"),
            flag_banner = flag_banner,
            err_box = err_box,
            deals_count = state.deals.len(),
            total_rev = total_formatted,
            active_q = if query.is_empty() { "None (All Deals)" } else { &query },
            q_val = query,
            rows = rows
        );
        return (200, resp_headers, html);
    }

    // 3. Create Deal View & POST (CRUD Feature)
    if path == "/deals/new" {
        if method == "POST" {
            let mut cust = String::new();
            let mut val_str = String::new();
            let mut stage = "Proposal Sent".to_string();
            let mut email = String::new();
            let mut notes = String::new();

            for pair in body.split('&') {
                if let Some((k, v)) = pair.split_once('=') {
                    let decoded = urlencoding_decode(v);
                    match k {
                        "customer" => cust = decoded,
                        "value" => val_str = decoded,
                        "stage" => stage = decoded,
                        "contact_email" => email = decoded,
                        "notes" => notes = decoded,
                        _ => {}
                    }
                }
            }

            if !cust.is_empty() {
                let parsed_val = val_str.parse::<f64>().unwrap_or(25000.0);
                let new_deal = Deal {
                    id: state.next_id,
                    customer: cust,
                    deal_value: parsed_val,
                    stage,
                    contact_email: if email.is_empty() { "contact@enterprise.com".to_string() } else { email },
                    notes: if notes.is_empty() { "Standard customer pipeline inquiry".to_string() } else { notes },
                    is_confidential: false,
                };
                state.next_id += 1;
                state.deals.push(new_deal);

                resp_headers.insert("Location".to_string(), "/".to_string());
                return (302, resp_headers, "Redirecting to pipeline...".to_string());
            }
        }

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Create Enterprise Deal // DataFlow CRM</title>
    <style>{css}</style>
</head>
<body>
<div class="app-container">
    {navbar}

    <div class="card" style="max-width: 680px; margin: 0 auto;">
        <h2 style="font-size: 18px; font-weight: 800; color: #0f172a; margin-bottom: 6px;">Register New Customer Opportunity</h2>
        <p style="font-size: 13px; color: #64748b; margin-bottom: 20px;">
            Input client transaction terms and executive contract specifications into the revenue operations pipeline.
        </p>

        <form method="POST" action="/deals/new">
            <div class="form-group">
                <label class="form-label">Client / Corporate Name</label>
                <input type="text" name="customer" placeholder="e.g. Nexus Cyberware Solutions" required>
            </div>
            <div class="form-group">
                <label class="form-label">Estimated Contract Value (USD)</label>
                <input type="number" step="1000" name="value" placeholder="75000" required>
            </div>
            <div class="form-group">
                <label class="form-label">Sales Stage</label>
                <select name="stage">
                    <option value="Discovery">Discovery & Technical Evaluation</option>
                    <option value="Proposal Sent">Proposal Sent</option>
                    <option value="Negotiation">Executive Negotiation</option>
                    <option value="Contract Signed">Contract Signed (Closed Won)</option>
                </select>
            </div>
            <div class="form-group">
                <label class="form-label">Primary Contact Email</label>
                <input type="text" name="contact_email" placeholder="procurement@client.com">
            </div>
            <div class="form-group">
                <label class="form-label">Confidential Opportunity Notes</label>
                <textarea name="notes" rows="3" placeholder="Key deliverables, compliance requirements, SLA specs..."></textarea>
            </div>
            <div style="display: flex; gap: 12px; margin-top: 24px;">
                <button type="submit" class="btn-primary">Save to Pipeline</button>
                <a href="/" class="btn-secondary">Cancel</a>
            </div>
        </form>
    </div>
</div>
</body>
</html>"#,
            css = css(),
            navbar = render_navbar("new")
        );
        return (200, resp_headers, html);
    }

    // 4. API JSON Endpoint
    if path == "/api/v1/deals" {
        resp_headers.insert("Content-Type".to_string(), "application/json; charset=utf-8".to_string());
        let json = serde_json::json!({
            "status": "success",
            "count": state.deals.len(),
            "deals": state.deals
        });
        return (200, resp_headers, json.to_string());
    }

    // Default 404
    (
        404,
        resp_headers,
        format!(
            "<!DOCTYPE html><html><head><style>{}</style></head><body><div class='app-container'><div class='card' style='text-align:center;'><h2>404 — Endpoint Not Found</h2><p style='margin-top:10px;'><a href='/' class='btn-secondary'>Return to Deals Pipeline</a></p></div></div></body></html>",
            css()
        ),
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
        println!("DataFlow Enterprise CRM (SQL Injection) v{}", LAB_VERSION);
        println!("This native lab communicates with the host engine broker via stdin/stdout JSON IPC.");
        println!("Usage: a05-lab.exe");
        return;
    }
    if args.iter().any(|a| a == "--version" || a == "-v" || a == "-V") {
        println!("A05 v{} (native_sandboxed // DataFlow CRM)", LAB_VERSION);
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
                    "type": "HEALTH_OK",
                    "id": id,
                    "lab_id": LAB_ID,
                    "status": "HEALTHY"
                });
                let _ = writeln!(stdout, "{}", resp_json);
                let _ = stdout.flush();
            }
            "RESET" => {
                let id = val.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
                state = AppState::new();
                let resp_json = serde_json::json!({
                    "type": "RESET_OK",
                    "id": id,
                    "lab_id": LAB_ID
                });
                let _ = writeln!(stdout, "{}", resp_json);
                let _ = stdout.flush();
            }
            "SHUTDOWN" => {
                exit(0);
            }
            _ => {}
        }
    }
}
