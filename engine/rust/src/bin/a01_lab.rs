// A01: Broken Access Control (BOLA / IDOR) — ApexFin Corporate Treasury
// Authentic Clean Soft-UI Commercial FinTech Web Application v1.0.1
// Built with stateful in-memory CRUD (Wire Transfers, Transfer Creation, Status Filtering)
// Target vulnerability: Insecure Direct Object Reference (IDOR) on wire transaction vouchers.

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::process::exit;

const FLAG: &str = "ZITERA{b10k3n_4cc355_c0ntr01_m45t3r}";
const LAB_ID: &str = "A01";
const LAB_VERSION: &str = "1.0.1";

#[derive(Clone)]
struct User {
    id: u32,
    username: &'static str,
    password: &'static str,
    full_name: &'static str,
    role: &'static str,
    department: &'static str,
}

#[derive(Clone)]
struct WireTransaction {
    id: u32,
    user_id: u32,
    reference_code: String,
    title: String,
    amount: f64,
    origin_account: String,
    beneficiary_account: String,
    beneficiary_name: String,
    status: String,
    settlement_date: String,
    confidential_memo: String,
}

struct AppState {
    users: Vec<User>,
    wires: Vec<WireTransaction>,
    sessions: HashMap<String, u32>, // session_cookie -> user_id
    idor_exploited: bool,
    next_wire_id: u32,
}

impl AppState {
    fn new() -> Self {
        Self {
            users: vec![
                User {
                    id: 1,
                    username: "alice",
                    password: "password123",
                    full_name: "Alice Vance",
                    role: "Treasury Operations Analyst",
                    department: "Global Liquidity & Clearing",
                },
                User {
                    id: 2,
                    username: "bob",
                    password: "bobpass",
                    full_name: "Robert Sterling",
                    role: "Senior FX Trader",
                    department: "Cross-Border Settlement Desk",
                },
                User {
                    id: 99,
                    username: "admin",
                    password: "adminsecret_unbrute",
                    full_name: "Marcus Reid",
                    role: "VP Corporate Controller & Executive Auditor",
                    department: "Financial Compliance & Risk Governance",
                },
            ],
            wires: vec![
                WireTransaction {
                    id: 1,
                    user_id: 1,
                    reference_code: "TRX-APEX-1001".to_string(),
                    title: "AWS Cloud Infrastructure - Q4 Dedicated Compute".to_string(),
                    amount: 42500.00,
                    origin_account: "ApexFin Primary Operational Reserve (US-991044)".to_string(),
                    beneficiary_account: "AMZN-PAY-US-7782".to_string(),
                    beneficiary_name: "Amazon Web Services Inc.".to_string(),
                    status: "SETTLED".to_string(),
                    settlement_date: "2026-11-12 14:22:08 UTC".to_string(),
                    confidential_memo: "Monthly enterprise cloud compute and GPU instance reservation for North American clearing node.".to_string(),
                },
                WireTransaction {
                    id: 2,
                    user_id: 2,
                    reference_code: "TRX-APEX-1002".to_string(),
                    title: "Domain Security & Wildcard SSL Certificate Renewal".to_string(),
                    amount: 1450.00,
                    origin_account: "ApexFin Merchant Clearing (US-449102)".to_string(),
                    beneficiary_account: "DIGICERT-GLOBAL-8901".to_string(),
                    beneficiary_name: "DigiCert Trust Solutions".to_string(),
                    status: "SETTLED".to_string(),
                    settlement_date: "2026-11-14 09:15:30 UTC".to_string(),
                    confidential_memo: "Annual EV multi-domain wildcard security certificate for *.apexfin-corp.internal.".to_string(),
                },
                WireTransaction {
                    id: 3,
                    user_id: 1,
                    reference_code: "TRX-APEX-1003".to_string(),
                    title: "Snowflake Enterprise Data Warehouse Ingestion Tier".to_string(),
                    amount: 8200.00,
                    origin_account: "ApexFin Primary Operational Reserve (US-991044)".to_string(),
                    beneficiary_account: "SNOW-BIL-88210".to_string(),
                    beneficiary_name: "Snowflake Inc.".to_string(),
                    status: "PENDING".to_string(),
                    settlement_date: "2026-11-19 11:00:00 UTC".to_string(),
                    confidential_memo: "Automated batch analytics pipeline capacity credit purchase.".to_string(),
                },
                WireTransaction {
                    id: 42,
                    user_id: 99,
                    reference_code: "TRX-APEX-9901-SOVEREIGN".to_string(),
                    title: "Executive Sovereign Audit & Escrow Regulatory Reserve".to_string(),
                    amount: 99999.00,
                    origin_account: "ApexFin Executive Escrow (US-001099-CONFIDENTIAL)".to_string(),
                    beneficiary_account: "FED-AUDIT-TREASURY-009".to_string(),
                    beneficiary_name: "Federal Banking Security & Compliance Oversight".to_string(),
                    status: "RESTRICTED".to_string(),
                    settlement_date: "2026-11-16 00:00:00 UTC".to_string(),
                    confidential_memo: "SOX-404 Sovereign Security Audit Compliance Clearance. Token: ZITERA{b10k3n_4cc355_c0ntr01_m45t3r}".to_string(),
                },
            ],
            sessions: HashMap::new(),
            idor_exploited: false,
            next_wire_id: 1004,
        }
    }
}

// ---------------------------------------------------------------------------
// HTML Layout & Soft Commercial Styling
// ---------------------------------------------------------------------------

fn render_app_shell(
    page_title: &str,
    content_html: &str,
    current_user: Option<&User>,
    unlocked_flag: Option<&str>,
) -> String {
    let user_pill = match current_user {
        Some(u) => format!(
            r#"<div class="user-pill">
                <div class="user-avatar">{}</div>
                <div class="user-info">
                    <span class="user-name">{}</span>
                    <span class="user-role">{}</span>
                </div>
                <a href="/logout" class="btn-logout" title="Sign Out">Sign Out</a>
            </div>"#,
            &u.full_name[0..1],
            u.full_name,
            u.role
        ),
        None => r#"<a href="/login" class="btn-primary" style="padding: 6px 14px; font-size: 13px;">Employee Sign In</a>"#.to_string(),
    };

    let flag_box = match unlocked_flag {
        Some(f) => format!(
            r#"<div class="flag-callout">
                <div class="flag-callout-icon"></div>
                <div class="flag-callout-body">
                    <div class="flag-callout-title">IDOR Vulnerability Verified</div>
                    <div class="flag-callout-desc">You successfully accessed Marcus Reid's restricted executive audit record (#42). Your bounty flag is ready for submission:</div>
                    <div class="flag-key-box">
                        <code id="flagVal">FLAG: {}</code>
                        <button class="btn-copy" onclick="navigator.clipboard.writeText('{}'); this.innerText=' Copied!'; setTimeout(() => this.innerText='Copy', 2000);">Copy</button>
                    </div>
                </div>
            </div>"#,
            f, f
        ),
        None => String::new(),
    };

    let nav_items = match current_user {
        Some(_) => r#"
            <a href="/" class="nav-link">Dashboard</a>
            <a href="/wires" class="nav-link active">Wire Transfers</a>
            <a href="/wires/new" class="nav-link">New Transfer</a>
            <a href="/audit" class="nav-link">Compliance Audit</a>
        "#,
        None => r#"
            <a href="/" class="nav-link active">Sign In</a>
        "#,
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{} — ApexFin Treasury</title>
    <style>
        :root {{
            --bg-page: #f8fafc;
            --bg-surface: #ffffff;
            --bg-subtle: #f1f5f9;
            --border: #e2e8f0;
            --border-hover: #cbd5e1;
            --text-heading: #0f172a;
            --text-body: #334155;
            --text-muted: #64748b;
            --primary: #0f766e;
            --primary-hover: #115e59;
            --primary-light: #f0fdfa;
            --primary-border: #99f6e4;
            --accent-emerald: #059669;
            --accent-emerald-bg: #ecfdf5;
            --accent-amber: #d97706;
            --accent-amber-bg: #fffbeb;
            --accent-rose: #e11d48;
            --accent-rose-bg: #fff1f2;
            --font-sans: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            --font-mono: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "JetBrains Mono", monospace;
            --shadow-sm: 0 1px 2px 0 rgb(0 0 0 / 0.05);
            --shadow-md: 0 4px 6px -1px rgb(0 0 0 / 0.07), 0 2px 4px -2px rgb(0 0 0 / 0.05);
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            background-color: var(--bg-page);
            color: var(--text-body);
            font-family: var(--font-sans);
            font-size: 14px;
            line-height: 1.5;
            -webkit-font-smoothing: antialiased;
            min-height: 100vh;
            display: flex;
            flex-direction: column;
        }}
        /* Top Navigation Header */
        header.top-header {{
            background: var(--bg-surface);
            border-bottom: 1px solid var(--border);
            height: 64px;
            padding: 0 28px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            position: sticky;
            top: 0;
            z-index: 40;
            box-shadow: var(--shadow-sm);
        }}
        .brand-cluster {{
            display: flex;
            align-items: center;
            gap: 12px;
            text-decoration: none;
            color: inherit;
        }}
        .brand-icon {{
            width: 34px;
            height: 34px;
            background: linear-gradient(135deg, #0f766e, #0d9488);
            border-radius: 8px;
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
            font-weight: 800;
            font-size: 16px;
            box-shadow: 0 2px 4px rgba(15, 118, 110, 0.2);
        }}
        .brand-meta {{ display: flex; flex-direction: column; }}
        .brand-title {{ font-size: 15px; font-weight: 700; color: var(--text-heading); letter-spacing: -0.01em; }}
        .brand-tag {{ font-size: 11px; color: var(--text-muted); }}

        .nav-links {{
            display: flex;
            align-items: center;
            gap: 6px;
        }}
        .nav-link {{
            color: var(--text-muted);
            text-decoration: none;
            font-size: 13px;
            font-weight: 600;
            padding: 8px 14px;
            border-radius: 6px;
            transition: all 0.15s ease;
        }}
        .nav-link:hover {{ color: var(--text-heading); background: var(--bg-subtle); }}
        .nav-link.active {{ color: var(--primary); background: var(--primary-light); font-weight: 700; }}

        /* User Profile Pill */
        .user-pill {{
            display: flex;
            align-items: center;
            gap: 10px;
            background: var(--bg-subtle);
            padding: 5px 12px;
            border-radius: 20px;
            border: 1px solid var(--border);
        }}
        .user-avatar {{
            width: 26px;
            height: 26px;
            background: var(--primary);
            color: white;
            font-weight: 700;
            font-size: 12px;
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
        }}
        .user-info {{ display: flex; flex-direction: column; text-align: left; }}
        .user-name {{ font-size: 12px; font-weight: 700; color: var(--text-heading); line-height: 1.2; }}
        .user-role {{ font-size: 10px; color: var(--text-muted); }}
        .btn-logout {{
            font-size: 11px;
            color: var(--accent-rose);
            text-decoration: none;
            font-weight: 600;
            margin-left: 4px;
            padding-left: 8px;
            border-left: 1px solid var(--border);
        }}
        .btn-logout:hover {{ text-decoration: underline; }}

        /* Flag Callout */
        .flag-callout {{
            background: #f0fdf4;
            border: 1px solid #bbf7d0;
            border-radius: 8px;
            padding: 16px 20px;
            margin-bottom: 24px;
            display: flex;
            gap: 16px;
            align-items: flex-start;
            box-shadow: var(--shadow-sm);
        }}
        .flag-callout-icon {{ font-size: 24px; line-height: 1; }}
        .flag-callout-body {{ flex: 1; }}
        .flag-callout-title {{ font-size: 14px; font-weight: 700; color: #166534; margin-bottom: 4px; }}
        .flag-callout-desc {{ font-size: 13px; color: #15803d; margin-bottom: 12px; }}
        .flag-key-box {{
            display: flex;
            align-items: center;
            gap: 8px;
        }}
        .flag-key-box code {{
            background: white;
            border: 1px solid #86efac;
            padding: 6px 12px;
            border-radius: 6px;
            font-family: var(--font-mono);
            font-size: 13px;
            font-weight: 700;
            color: #14532d;
            box-shadow: var(--shadow-sm);
        }}
        .btn-copy {{
            background: #15803d;
            color: white;
            border: none;
            padding: 6px 12px;
            border-radius: 6px;
            font-size: 12px;
            font-weight: 600;
            cursor: pointer;
            transition: background 0.15s ease;
        }}
        .btn-copy:hover {{ background: #166534; }}

        /* Main Container */
        main.content-area {{
            flex: 1;
            max-width: 1180px;
            width: 100%;
            margin: 0 auto;
            padding: 32px 24px;
        }}

        /* Clean Card */
        .card {{
            background: var(--bg-surface);
            border: 1px solid var(--border);
            border-radius: 10px;
            padding: 24px;
            box-shadow: var(--shadow-sm);
            margin-bottom: 24px;
        }}

        /* Metrics Row */
        .stats-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
            gap: 16px;
            margin-bottom: 24px;
        }}
        .stat-card {{
            background: var(--bg-surface);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 18px 20px;
            box-shadow: var(--shadow-sm);
        }}
        .stat-label {{ font-size: 12px; font-weight: 600; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.04em; }}
        .stat-val {{ font-size: 24px; font-weight: 800; color: var(--text-heading); margin: 6px 0 2px; font-family: var(--font-mono); }}
        .stat-note {{ font-size: 12px; color: var(--accent-emerald); font-weight: 500; }}

        /* Data Tables */
        .table-responsive {{ overflow-x: auto; }}
        table.fin-table {{
            width: 100%;
            border-collapse: collapse;
            font-size: 13px;
            text-align: left;
        }}
        table.fin-table th {{
            background: var(--bg-subtle);
            color: var(--text-muted);
            font-weight: 700;
            padding: 12px 16px;
            border-bottom: 1px solid var(--border);
            font-size: 11px;
            text-transform: uppercase;
            letter-spacing: 0.05em;
        }}
        table.fin-table td {{
            padding: 14px 16px;
            border-bottom: 1px solid var(--border);
            color: var(--text-body);
        }}
        table.fin-table tr:hover td {{ background: #fafafa; }}
        .mono {{ font-family: var(--font-mono); font-weight: 600; }}

        /* Status Badges */
        .badge {{
            display: inline-flex;
            align-items: center;
            padding: 3px 8px;
            border-radius: 12px;
            font-size: 11px;
            font-weight: 700;
            letter-spacing: 0.02em;
        }}
        .badge.settled {{ background: var(--accent-emerald-bg); color: var(--accent-emerald); }}
        .badge.pending {{ background: var(--accent-amber-bg); color: var(--accent-amber); }}
        .badge.restricted {{ background: var(--accent-rose-bg); color: var(--accent-rose); }}

        /* Action Buttons */
        .btn-primary {{
            background: var(--primary);
            color: white;
            text-decoration: none;
            padding: 8px 16px;
            border-radius: 6px;
            font-weight: 600;
            font-size: 13px;
            border: 1px solid var(--primary-hover);
            cursor: pointer;
            display: inline-flex;
            align-items: center;
            gap: 6px;
            transition: background 0.15s ease;
        }}
        .btn-primary:hover {{ background: var(--primary-hover); }}
        .btn-secondary {{
            background: white;
            color: var(--text-body);
            text-decoration: none;
            padding: 8px 14px;
            border-radius: 6px;
            font-weight: 600;
            font-size: 13px;
            border: 1px solid var(--border);
            cursor: pointer;
            display: inline-flex;
            align-items: center;
            gap: 6px;
            transition: all 0.15s ease;
        }}
        .btn-secondary:hover {{ background: var(--bg-subtle); border-color: var(--border-hover); }}

        /* Forms */
        .form-group {{ margin-bottom: 16px; }}
        .form-label {{ display: block; font-size: 13px; font-weight: 600; color: var(--text-heading); margin-bottom: 6px; }}
        .form-control {{
            width: 100%;
            padding: 9px 12px;
            border: 1px solid var(--border);
            border-radius: 6px;
            font-size: 13px;
            font-family: inherit;
            color: var(--text-heading);
            background: white;
            transition: border-color 0.15s ease;
        }}
        .form-control:focus {{ outline: none; border-color: var(--primary); box-shadow: 0 0 0 3px rgba(15, 118, 110, 0.1); }}
        .form-row {{ display: grid; grid-template-columns: 1fr 1fr; gap: 16px; }}

        /* Voucher View */
        .voucher-header {{
            display: flex;
            justify-content: space-between;
            align-items: flex-start;
            padding-bottom: 16px;
            border-bottom: 1px solid var(--border);
            margin-bottom: 20px;
        }}
        .voucher-grid {{
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 20px;
            margin-bottom: 24px;
        }}
        .voucher-field-label {{ font-size: 11px; font-weight: 700; color: var(--text-muted); text-transform: uppercase; margin-bottom: 4px; }}
        .voucher-field-val {{ font-size: 14px; color: var(--text-heading); font-weight: 600; }}
        .voucher-memo-card {{
            background: var(--bg-subtle);
            border: 1px solid var(--border);
            border-radius: 6px;
            padding: 16px;
        }}
        .voucher-memo-card.restricted {{
            background: #fff1f2;
            border-color: #fecdd3;
        }}
        .voucher-memo-text {{
            font-family: var(--font-mono);
            font-size: 13px;
            color: #881337;
            white-space: pre-wrap;
            word-break: break-all;
            margin-top: 6px;
            line-height: 1.6;
        }}

        /* Footer */
        footer.app-footer {{
            border-top: 1px solid var(--border);
            padding: 16px 28px;
            font-size: 12px;
            color: var(--text-muted);
            display: flex;
            justify-content: space-between;
            align-items: center;
            background: white;
        }}
    </style>
</head>
<body>
    <header class="top-header">
        <a href="/" class="brand-cluster">
            <div class="brand-icon">A</div>
            <div class="brand-meta">
                <span class="brand-title">ApexFin Treasury</span>
                <span class="brand-tag">Corporate Clearing & Settlement Platform</span>
            </div>
        </a>
        <nav class="nav-links">
            {}
        </nav>
        <div>
            {}
        </div>
    </header>

    <main class="content-area">
        {}
        {}
    </main>

    <footer class="app-footer">
        <div>ApexFin Corporate Banking Systems &bull; Ledger Node: US-EAST-TREASURY-01 &bull; SOC-2 Type II Certified</div>
        <div>Session Status: Authenticated</div>
    </footer>
</body>
</html>"#,
        page_title,
        nav_items,
        user_pill,
        flag_box,
        content_html
    )
}

// ---------------------------------------------------------------------------
// HTTP Request Routing & Handler
// ---------------------------------------------------------------------------

fn handle_request(
    state: &mut AppState,
    method: &str,
    path: &str,
    headers: &HashMap<String, String>,
    body: &str,
) -> (u16, HashMap<String, String>, String) {
    let mut resp_headers = HashMap::new();
    resp_headers.insert("Content-Type".to_string(), "text/html; charset=utf-8".to_string());
    resp_headers.insert("Cache-Control".to_string(), "no-store".to_string());

    // Determine logged in user from cookie (or default to Alice Vance for realistic lab experience)
    let current_user_id = headers
        .get("cookie")
        .and_then(|c| {
            c.split(';')
                .find(|part| part.trim().starts_with("session_token="))
                .and_then(|part| part.trim().strip_prefix("session_token="))
        })
        .and_then(|tok| state.sessions.get(tok).copied())
        .unwrap_or(1); // Default to Alice Vance (User ID 1)

    let current_user = state.users.iter().find(|u| u.id == current_user_id).cloned();

    // Legacy / Automated Authentication Endpoint
    if path == "/login" && method == "POST" {
        resp_headers.insert("Set-Cookie".to_string(), "session_user=user_sess_1; Path=/".to_string());
        resp_headers.insert("Location".to_string(), "/wires".to_string());
        return (302, resp_headers, "Redirecting to /wires...".to_string());
    }

    if path == "/login" && method == "GET" {
        resp_headers.insert("Location".to_string(), "/".to_string());
        return (302, resp_headers, String::new());
    }

    // Invoice alias for wire vouchers
    if path.starts_with("/invoice/") {
        let inv_id = path.strip_prefix("/invoice/").and_then(|s| s.split('/').next()).and_then(|s| s.parse::<u32>().ok()).unwrap_or(42);
        if inv_id == 42 {
            state.idor_exploited = true;
            return (200, resp_headers, format!("Invoice #42: Marcus Reid Restricted SOX-404 Voucher. FLAG: {}", FLAG));
        }
    }

    // 1. Root / Dashboard Page
    if path == "/" || path == "/index.html" || path == "/wires" {
        let alice_wires: Vec<&WireTransaction> = state
            .wires
            .iter()
            .filter(|w| w.user_id == current_user_id)
            .collect();

        let total_settled: f64 = alice_wires
            .iter()
            .filter(|w| w.status == "SETTLED")
            .map(|w| w.amount)
            .sum();

        let total_pending: f64 = alice_wires
            .iter()
            .filter(|w| w.status == "PENDING")
            .map(|w| w.amount)
            .sum();

        let mut rows = String::new();
        for w in &alice_wires {
            let badge_class = match w.status.as_str() {
                "SETTLED" => "settled",
                "PENDING" => "pending",
                _ => "restricted",
            };
            rows.push_str(&format!(
                r#"<tr>
                    <td class="mono">{}</td>
                    <td><strong>{}</strong><br><span style="color: #64748b; font-size: 11px;">Beneficiary: {}</span></td>
                    <td class="mono">${:.2}</td>
                    <td><span class="badge {}">{}</span></td>
                    <td class="mono" style="font-size: 12px; color: #64748b;">{}</td>
                    <td>
                        <a href="/wires/view?id={}" class="btn-secondary" style="padding: 4px 10px; font-size: 12px;">View Voucher &rarr;</a>
                    </td>
                </tr>"#,
                w.reference_code,
                w.title,
                w.beneficiary_name,
                w.amount,
                badge_class,
                w.status,
                w.settlement_date,
                w.id
            ));
        }

        let dashboard_html = format!(
            r#"
            <div class="stats-grid">
                <div class="stat-card">
                    <div class="stat-label">Settled Disbursements (30D)</div>
                    <div class="stat-val">${:.2}</div>
                    <div class="stat-note">&bull; Cleared via Fedwire ACH</div>
                </div>
                <div class="stat-card">
                    <div class="stat-label">Pending Settlements</div>
                    <div class="stat-val">${:.2}</div>
                    <div class="stat-note" style="color: #d97706;">&bull; Scheduled for next clearing window</div>
                </div>
                <div class="stat-card">
                    <div class="stat-label">Assigned Ledger Node</div>
                    <div class="stat-val" style="font-size: 20px;">TREASURY-US-01</div>
                    <div class="stat-note">&bull; Real-time synchronization active</div>
                </div>
            </div>

            <div class="card">
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
                    <div>
                        <h2 style="font-size: 16px; font-weight: 700; color: #0f172a;">Active Wire Transfer Ledger</h2>
                        <p style="font-size: 12px; color: #64748b; margin-top: 2px;">Review verified outgoing wire transfers and corporate settlement vouchers.</p>
                    </div>
                    <div style="display: flex; gap: 8px;">
                        <a href="/wires/new" class="btn-primary">+ Create New Wire</a>
                    </div>
                </div>

                <div class="table-responsive">
                    <table class="fin-table">
                        <thead>
                            <tr>
                                <th>Reference</th>
                                <th>Description & Beneficiary</th>
                                <th>Amount</th>
                                <th>Status</th>
                                <th>Settlement Date</th>
                                <th>Action</th>
                            </tr>
                        </thead>
                        <tbody>
                            {}
                        </tbody>
                    </table>
                </div>
            </div>
            "#,
            total_settled, total_pending, rows
        );

        let page = render_app_shell("Wire Transfers", &dashboard_html, current_user.as_ref(), None);
        return (200, resp_headers, page);
    }

    // 2. New Wire Transfer Form
    if (path == "/wires/new" || path == "/wire/new") && method == "GET" {
        let form_html = r#"
        <div class="card" style="max-width: 680px; margin: 0 auto;">
            <h2 style="font-size: 18px; font-weight: 700; color: #0f172a; margin-bottom: 4px;">Initiate Corporate Wire Transfer</h2>
            <p style="font-size: 13px; color: #64748b; margin-bottom: 20px;">Draft a new settlement request from your authorized business reserve account.</p>

            <form method="POST" action="/wires/create">
                <div class="form-group">
                    <label class="form-label">Transfer Purpose / Description</label>
                    <input type="text" name="title" class="form-control" placeholder="e.g. Server Hardware Cluster Purchase" required />
                </div>
                <div class="form-row">
                    <div class="form-group">
                        <label class="form-label">Beneficiary Entity Name</label>
                        <input type="text" name="beneficiary_name" class="form-control" placeholder="e.g. Dell Enterprise Systems" required />
                    </div>
                    <div class="form-group">
                        <label class="form-label">Beneficiary Account / Routing</label>
                        <input type="text" name="beneficiary_account" class="form-control" placeholder="e.g. US-DELL-883921" required />
                    </div>
                </div>
                <div class="form-row">
                    <div class="form-group">
                        <label class="form-label">Transfer Amount ($ USD)</label>
                        <input type="number" step="0.01" name="amount" class="form-control" placeholder="5000.00" required />
                    </div>
                    <div class="form-group">
                        <label class="form-label">Originating Ledger Account</label>
                        <input type="text" class="form-control" value="ApexFin Primary Reserve (US-991044)" disabled />
                    </div>
                </div>
                <div class="form-group">
                    <label class="form-label">Internal Compliance Memo</label>
                    <textarea name="memo" class="form-control" rows="3" placeholder="Reference PO or authorization notes..."></textarea>
                </div>
                <div style="display: flex; justify-content: flex-end; gap: 10px; margin-top: 24px;">
                    <a href="/wires" class="btn-secondary">Cancel</a>
                    <button type="submit" class="btn-primary">Submit Wire Voucher</button>
                </div>
            </form>
        </div>
        "#;
        let page = render_app_shell("New Wire Transfer", form_html, current_user.as_ref(), None);
        return (200, resp_headers, page);
    }

    // 3. Create Wire Transfer (POST)
    if (path == "/wires/create" || path == "/wire/create") && method == "POST" {
        let mut form_data = HashMap::new();
        for pair in body.split('&') {
            if let Some((k, v)) = pair.split_once('=') {
                let decoded_val = v.replace('+', " ");
                form_data.insert(k.to_string(), decoded_val);
            }
        }

        let title = form_data.get("title").cloned().unwrap_or_else(|| "Corporate Settlement".to_string());
        let beneficiary_name = form_data.get("beneficiary_name").cloned().unwrap_or_else(|| "Third Party Vendor".to_string());
        let beneficiary_account = form_data.get("beneficiary_account").cloned().unwrap_or_else(|| "ACH-GENERIC-001".to_string());
        let amount_str = form_data.get("amount").cloned().unwrap_or_else(|| "100.00".to_string());
        let amount = amount_str.parse::<f64>().unwrap_or(100.0);
        let memo = form_data.get("memo").cloned().unwrap_or_else(|| "Standard scheduled clearance.".to_string());

        let new_id = state.next_wire_id;
        state.next_wire_id += 1;

        let new_wire = WireTransaction {
            id: new_id,
            user_id: current_user_id,
            reference_code: format!("TRX-APEX-{}", new_id),
            title,
            amount,
            origin_account: "ApexFin Primary Operational Reserve (US-991044)".to_string(),
            beneficiary_account,
            beneficiary_name,
            status: "PENDING".to_string(),
            settlement_date: "2026-11-19 12:00:00 UTC".to_string(),
            confidential_memo: memo,
        };

        state.wires.push(new_wire);

        resp_headers.insert("Location".to_string(), "/wires".to_string());
        return (302, resp_headers, String::new());
    }

    // 4. View Wire Voucher Detail — The Target of IDOR Vulnerability
    // Matches `/wires/view?id=X` or `/wire/X`
    if path.starts_with("/wires/view") || path.starts_with("/wire/") {
        let wire_id: Option<u32> = if path.starts_with("/wires/view") {
            // Extract id from query string e.g. /wires/view?id=42
            path.split_once('?')
                .and_then(|(_, q)| {
                    q.split('&')
                        .find(|part| part.starts_with("id="))
                        .and_then(|part| part.strip_prefix("id="))
                })
                .and_then(|val| val.parse::<u32>().ok())
        } else {
            // Extract id from path e.g. /wire/42
            path.strip_prefix("/wire/")
                .and_then(|seg| seg.split('/').next())
                .and_then(|val| val.parse::<u32>().ok())
        };

        if let Some(target_id) = wire_id {
            // IDOR FLAW: The application queries `wires.iter().find(|w| w.id == target_id)`
            // WITHOUT checking if `w.user_id == current_user_id`!
            let wire_opt = state.wires.iter().find(|w| w.id == target_id).cloned();

            if let Some(w) = wire_opt {
                let is_flag_wire = w.id == 42;
                if is_flag_wire {
                    state.idor_exploited = true;
                }

                let owner = state.users.iter().find(|u| u.id == w.user_id);
                let owner_name = owner.map(|u| u.full_name).unwrap_or("Authorized Signatory");
                let owner_role = owner.map(|u| u.role).unwrap_or("Corporate Executive");

                let badge_class = match w.status.as_str() {
                    "SETTLED" => "settled",
                    "PENDING" => "pending",
                    _ => "restricted",
                };

                let detail_html = format!(
                    r#"
                    <div style="margin-bottom: 16px;">
                        <a href="/wires" class="btn-secondary" style="font-size: 12px; padding: 4px 10px;">&larr; Back to Ledger</a>
                    </div>

                    <div class="card">
                        <div class="voucher-header">
                            <div>
                                <span class="badge {}" style="margin-bottom: 8px;">{}</span>
                                <h1 style="font-size: 20px; font-weight: 800; color: #0f172a;">Official Wire Settlement Voucher</h1>
                                <p style="font-size: 12px; color: #64748b; font-family: var(--font-mono); margin-top: 2px;">
                                    REFERENCE: {} &bull; RECORD ID: #{}
                                </p>
                            </div>
                            <div style="text-align: right;">
                                <div class="voucher-field-label">Disbursement Total</div>
                                <div style="font-size: 24px; font-weight: 800; color: #0f172a; font-family: var(--font-mono);">${:.2}</div>
                            </div>
                        </div>

                        <div class="voucher-grid">
                            <div>
                                <div class="voucher-field-label">Purpose / Description</div>
                                <div class="voucher-field-val">{}</div>
                            </div>
                            <div>
                                <div class="voucher-field-label">Originating Reserve Account</div>
                                <div class="voucher-field-val">{}</div>
                            </div>
                            <div>
                                <div class="voucher-field-label">Beneficiary Payee</div>
                                <div class="voucher-field-val">{} ({})</div>
                            </div>
                            <div>
                                <div class="voucher-field-label">Authorized Signatory / Account Holder</div>
                                <div class="voucher-field-val">{} (User ID: #{}) &bull; {}</div>
                            </div>
                            <div>
                                <div class="voucher-field-label">Execution Timestamp</div>
                                <div class="voucher-field-val" style="font-family: var(--font-mono); font-size: 13px;">{}</div>
                            </div>
                            <div>
                                <div class="voucher-field-label">Clearing Gateway Node</div>
                                <div class="voucher-field-val" style="font-family: var(--font-mono); font-size: 13px;">FEDWIRE-SWIFT-NODE-04</div>
                            </div>
                        </div>

                        <div class="voucher-memo-card {}">
                            <div class="voucher-field-label" style="color: {};">Audit & Regulatory Compliance Memo</div>
                            <div class="voucher-memo-text">{}</div>
                        </div>
                    </div>
                    "#,
                    badge_class,
                    w.status,
                    w.reference_code,
                    w.id,
                    w.amount,
                    w.title,
                    w.origin_account,
                    w.beneficiary_name,
                    w.beneficiary_account,
                    owner_name,
                    w.user_id,
                    owner_role,
                    w.settlement_date,
                    if is_flag_wire { "restricted" } else { "" },
                    if is_flag_wire { "#9f1239" } else { "#475569" },
                    w.confidential_memo
                );

                let page = render_app_shell(
                    &w.reference_code,
                    &detail_html,
                    current_user.as_ref(),
                    if is_flag_wire { Some(FLAG) } else { None },
                );
                return (200, resp_headers, page);
            } else {
                let not_found_html = r#"
                <div class="card" style="text-align: center; padding: 48px 20px;">
                    <h2 style="font-size: 18px; font-weight: 700; color: #0f172a; margin-bottom: 8px;">Voucher Record Not Found</h2>
                    <p style="font-size: 13px; color: #64748b; margin-bottom: 20px;">The specified wire transfer voucher identifier does not exist in the active clearing ledger.</p>
                    <a href="/wires" class="btn-secondary">Return to Ledger</a>
                </div>
                "#;
                let page = render_app_shell("Not Found", not_found_html, current_user.as_ref(), None);
                return (404, resp_headers, page);
            }
        }
    }

    // 5. Audit Tab (Access Control Demonstration)
    if path == "/audit" {
        let is_admin = current_user.as_ref().map(|u| u.id == 99).unwrap_or(false);
        let audit_html = if is_admin {
            r#"
            <div class="card">
                <h2 style="font-size: 18px; font-weight: 700; color: #0f172a; margin-bottom: 8px;">Executive SOX-404 Audit Console</h2>
                <p style="font-size: 13px; color: #64748b; margin-bottom: 16px;">Welcome Marcus Reid (VP Controller). You have administrative authority to review executive escrow vouchers.</p>
                <a href="/wires/view?id=42" class="btn-primary">Inspect Sovereign Escrow Record #42 &rarr;</a>
            </div>
            "#
        } else {
            r#"
            <div class="card" style="text-align: center; padding: 48px 24px;">
                <div style="font-size: 36px; margin-bottom: 12px;">️</div>
                <h2 style="font-size: 18px; font-weight: 700; color: #0f172a; margin-bottom: 8px;">Access Restricted &bull; Executive Privileges Required</h2>
                <p style="font-size: 13px; color: #64748b; max-width: 520px; margin: 0 auto 20px;">
                    The Executive SOX-404 Compliance Console is restricted to Marcus Reid (VP Controller) or Board Audit Committee members. Junior analysts cannot access this view directly through navigation.
                </p>
                <div style="background: #f1f5f9; border: 1px solid #e2e8f0; border-radius: 6px; padding: 12px 16px; display: inline-block; font-size: 12px; color: #475569; text-align: left;">
                    <strong>Pentester Methodology Note:</strong><br/>
                    While the UI restricts the <code>/audit</code> menu, does the underlying wire voucher API endpoint enforce object-level authorization when accessing vouchers directly?
                </div>
                <div style="margin-top: 24px;">
                    <a href="/wires" class="btn-secondary">Return to Treasury Ledger</a>
                </div>
            </div>
            "#
        };
        let page = render_app_shell("Compliance Audit", audit_html, current_user.as_ref(), None);
        return (200, resp_headers, page);
    }

    // Default 404
    let not_found_page = render_app_shell(
        "404 Not Found",
        "<div class='card' style='text-align: center; padding: 40px;'><h2>404 — Endpoint Not Found</h2><p style='margin-top: 8px;'><a href='/' class='btn-secondary'>Return to Dashboard</a></p></div>",
        current_user.as_ref(),
        None,
    );
    (404, resp_headers, not_found_page)
}

// ---------------------------------------------------------------------------
// Main Stdio Protocol Loop
// ---------------------------------------------------------------------------

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--help" || a == "-h" || a == "/?") {
        println!("ApexFin Corporate Treasury Sandboxed Lab A01 (Broken Access Control) v1.0.1");
        println!("Usage: a01-lab.exe");
        return;
    }
    if args.iter().any(|a| a == "--version" || a == "-v" || a == "-V") {
        println!("A01 v1.0.1 (native_sandboxed // ApexFin Treasury)");
        return;
    }

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    let mut state = AppState::new();

    // 1. Initial Handshake: Emit READY message
    let ready_json = serde_json::json!({
        "type": "READY",
        "lab_id": LAB_ID,
        "version": LAB_VERSION
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
                    "type": "HEALTH_RESPONSE",
                    "id": id,
                    "status": "HEALTHY",
                    "lab_id": LAB_ID
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
                        "Practice verified: You accessed Marcus Reid's confidential wire transaction (#42) via IDOR!"
                    } else {
                        "Practice incomplete: Inspect transaction vouchers on the ledger and test object parameter boundaries."
                    }
                });
                let _ = writeln!(stdout, "{}", resp_json);
                let _ = stdout.flush();
            }
            "CHALLENGE" => {
                let id = val.get("id").and_then(|v| v.as_u64()).unwrap_or(0);
                let candidate = val.get("flag").and_then(|v| v.as_str()).unwrap_or("").trim();
                let valid = candidate == FLAG;
                let resp_json = serde_json::json!({
                    "type": "CHALLENGE_RESULT",
                    "id": id,
                    "valid": valid,
                    "message": if valid {
                        "Correct! Broken Access Control challenge complete."
                    } else {
                        "Incorrect flag. Hint: investigate the confidential memo on restricted voucher #42."
                    }
                });
                let _ = writeln!(stdout, "{}", resp_json);
                let _ = stdout.flush();
            }
            "STOP" => {
                let resp_json = serde_json::json!({ "type": "CLEAN_EXIT" });
                let _ = writeln!(stdout, "{}", resp_json);
                let _ = stdout.flush();
                exit(0);
            }
            _ => {}
        }
    }
}
