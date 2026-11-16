use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::process::exit;

const FLAG: &str = "ZITERA{b10k3n_4cc355_c0ntr01_m45t3r}";

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
    reference_code: &'static str,
    title: &'static str,
    amount: f64,
    origin_account: &'static str,
    beneficiary_account: &'static str,
    beneficiary_name: &'static str,
    status: &'static str,
    settlement_date: &'static str,
    confidential_memo: &'static str,
}

struct AppState {
    users: Vec<User>,
    wires: Vec<WireTransaction>,
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
                    full_name: "Alice Vance",
                    role: "Junior Treasury Analyst",
                    department: "Global Liquidity Operations",
                },
                User {
                    id: 2,
                    username: "bob",
                    password: "bobpass",
                    full_name: "Robert Sterling",
                    role: "Treasury Specialist",
                    department: "FX Settlement Desk",
                },
                User {
                    id: 99,
                    username: "admin",
                    password: "adminsecret_unbrute",
                    full_name: "Marcus Reid",
                    role: "VP Corporate Controller & Auditor",
                    department: "Executive Compliance & Treasury Oversight",
                },
            ],
            wires: vec![
                WireTransaction {
                    id: 1,
                    user_id: 1,
                    reference_code: "TRX-APEX-1001",
                    title: "Cloud Infrastructure Cluster - Operational Retainer",
                    amount: 4950.00,
                    origin_account: "ApexFin Master Treasury (US-882194)",
                    beneficiary_account: "AWS-CORP-DIRECT-4412",
                    beneficiary_name: "Amazon Web Services Inc.",
                    status: "SETTLED",
                    settlement_date: "2026-11-15 14:22:01 UTC",
                    confidential_memo: "Standard recurring compute cluster provision for North America Operations.",
                },
                WireTransaction {
                    id: 2,
                    user_id: 2,
                    reference_code: "TRX-APEX-1002",
                    title: "Domain Infrastructure & SSL Cert Renewal",
                    amount: 1450.00,
                    origin_account: "ApexFin Merchant Clearing (US-449102)",
                    beneficiary_account: "DIGICERT-GLOBAL-8901",
                    beneficiary_name: "DigiCert Global Trust Services",
                    status: "SETTLED",
                    settlement_date: "2026-11-14 09:15:30 UTC",
                    confidential_memo: "Annual wildcard certificate renewal for *.apexfin-corp.internal.",
                },
                WireTransaction {
                    id: 42,
                    user_id: 99,
                    reference_code: "TRX-APEX-9901-SOVEREIGN",
                    title: "CLASSIFIED: Sovereign Federal Audit & System License Clearance",
                    amount: 99999.00,
                    origin_account: "ApexFin Executive Escrow (US-001099-CONFIDENTIAL)",
                    beneficiary_account: "FED-AUDIT-TREASURY-009",
                    beneficiary_name: "US Federal Banking Security Oversight",
                    status: "RESTRICTED",
                    settlement_date: "2026-11-16 00:00:00 UTC",
                    confidential_memo: "SOX-404 Sovereign Security Audit Compliance Clearance. FLAG: ZITERA{b10k3n_4cc355_c0ntr01_m45t3r}",
                },
            ],
            sessions: HashMap::new(),
            idor_exploited: false,
        }
    }
}

fn render_enterprise_shell(
    page_title: &str,
    content_html: &str,
    current_user: Option<&User>,
    unlocked_flag: Option<&str>,
) -> String {
    let user_profile_badge = match current_user {
        Some(u) => format!(
            r#"<div class="user-pill">
                <span class="avatar-dot"></span>
                <div class="user-meta">
                    <span class="user-name">{}</span>
                    <span class="user-role">{}</span>
                </div>
                <a href="/logout" class="btn-logout" title="Sign Out">Sign Out</a>
            </div>"#,
            u.full_name, u.role
        ),
        None => r#"<span class="auth-required-tag">UNAUTHENTICATED</span>"#.to_string(),
    };

    let flag_telemetry_banner = match unlocked_flag {
        Some(f) => format!(
            r#"<div class="flag-telemetry-ribbon active animate-pulse">
                <div class="ribbon-content">
                    <span class="ribbon-badge">CLASSIFIED PAYLOAD DETECTED</span>
                    <span class="ribbon-desc">Target Wire #42 accessed via IDOR parameter tampering!</span>
                </div>
                <div class="ribbon-actions">
                    <code class="flag-code" id="targetFlag">{}</code>
                    <button class="btn-copy-flag" onclick="copyFlag()">📋 COPY FLAG</button>
                </div>
            </div>"#,
            f
        ),
        None => r#"<div class="flag-telemetry-ribbon standby">
                <div class="ribbon-content">
                    <span class="ribbon-badge standby">ENVIRONMENT: STAGING SANDBOX</span>
                    <span class="ribbon-desc">Tenant ID: #APEX-9021-TREASURY // Fedwire Settlement Node: ACTIVE</span>
                </div>
                <div class="ribbon-actions">
                    <span class="system-status-pill">SYS_OK (200)</span>
                </div>
            </div>"#.to_string(),
    };

    let nav_links = match current_user {
        Some(_) => r#"
            <a href="/" class="nav-item">📊 Dashboard</a>
            <a href="/wires" class="nav-item active">💳 Wire Transfers</a>
            <a href="/audit" class="nav-item">🛡️ Compliance Audit</a>
            <a href="/admin" class="nav-item">⚙️ Executive Controls</a>
        "#,
        None => r#"
            <a href="/" class="nav-item active">🔐 Employee Portal Login</a>
        "#,
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{} — ApexFin Corporate Treasury</title>
    <style>
        :root {{
            --bg-canvas: #090d16;
            --bg-surface: #101726;
            --bg-surface-elevated: #162035;
            --bg-subtle: #1e2c47;
            --border-default: #25334d;
            --border-active: #3b82f6;
            --text-primary: #f8fafc;
            --text-secondary: #94a3b8;
            --text-muted: #64748b;
            --accent-primary: #0284c7;
            --accent-hover: #0369a1;
            --accent-emerald: #10b981;
            --accent-amber: #f59e0b;
            --accent-rose: #f43f5e;
            --font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            --font-mono: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "JetBrains Mono", monospace;
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            background-color: var(--bg-canvas);
            color: var(--text-primary);
            font-family: var(--font-family);
            line-height: 1.5;
            -webkit-font-smoothing: antialiased;
            min-height: 100vh;
            display: flex;
            flex-direction: column;
        }}
        /* Ambient Top Ribbon */
        .flag-telemetry-ribbon {{
            background: #0c1322;
            border-bottom: 1px solid var(--border-default);
            padding: 8px 24px;
            display: flex;
            align-items: center;
            justify-content: space-between;
            font-size: 12px;
            font-family: var(--font-mono);
        }}
        .flag-telemetry-ribbon.active {{
            background: linear-gradient(90deg, #1e1b4b 0%, #064e3b 100%);
            border-bottom: 1px solid #10b981;
        }}
        .ribbon-content {{ display: flex; align-items: center; gap: 12px; }}
        .ribbon-badge {{
            padding: 2px 8px;
            border-radius: 4px;
            font-weight: 700;
            font-size: 11px;
            background: #10b981;
            color: #022c22;
        }}
        .ribbon-badge.standby {{
            background: #1e293b;
            color: #94a3b8;
            border: 1px solid #334155;
        }}
        .ribbon-desc {{ color: var(--text-secondary); }}
        .ribbon-actions {{ display: flex; align-items: center; gap: 10px; }}
        .flag-code {{
            background: #022c22;
            color: #34d399;
            padding: 4px 10px;
            border-radius: 4px;
            border: 1px solid #059669;
            font-weight: bold;
        }}
        .btn-copy-flag {{
            background: #10b981;
            color: #064e3b;
            border: none;
            padding: 4px 12px;
            border-radius: 4px;
            font-weight: bold;
            font-family: var(--font-family);
            cursor: pointer;
            transition: all 0.2s;
        }}
        .btn-copy-flag:hover {{ background: #34d399; }}
        .system-status-pill {{
            color: #10b981;
            font-weight: bold;
            padding: 2px 6px;
            background: rgba(16, 185, 129, 0.1);
            border-radius: 4px;
        }}
        /* Corporate Header */
        header.enterprise-header {{
            background: var(--bg-surface);
            border-bottom: 1px solid var(--border-default);
            padding: 0 24px;
            height: 64px;
            display: flex;
            align-items: center;
            justify-content: space-between;
        }}
        .brand-container {{ display: flex; align-items: center; gap: 14px; text-decoration: none; color: inherit; }}
        .brand-logo-icon {{
            width: 34px;
            height: 34px;
            background: linear-gradient(135deg, #0284c7 0%, #0369a1 100%);
            border-radius: 8px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: 900;
            color: white;
            font-size: 16px;
            box-shadow: 0 2px 8px rgba(2, 132, 199, 0.4);
        }}
        .brand-text h1 {{ font-size: 16px; font-weight: 800; letter-spacing: -0.02em; color: #f8fafc; }}
        .brand-text p {{ font-size: 11px; color: var(--text-muted); font-family: var(--font-mono); }}
        .main-nav {{ display: flex; align-items: center; gap: 4px; }}
        .nav-item {{
            color: var(--text-secondary);
            text-decoration: none;
            font-size: 13px;
            font-weight: 600;
            padding: 8px 14px;
            border-radius: 6px;
            transition: all 0.15s;
        }}
        .nav-item:hover {{ color: var(--text-primary); background: var(--bg-surface-elevated); }}
        .nav-item.active {{ color: #38bdf8; background: var(--bg-surface-elevated); border: 1px solid var(--border-default); }}
        .user-pill {{
            display: flex;
            align-items: center;
            gap: 10px;
            background: var(--bg-surface-elevated);
            padding: 6px 12px;
            border-radius: 20px;
            border: 1px solid var(--border-default);
        }}
        .avatar-dot {{ width: 8px; height: 8px; background: #10b981; border-radius: 50%; box-shadow: 0 0 6px #10b981; }}
        .user-meta {{ display: flex; flex-direction: column; text-align: left; }}
        .user-name {{ font-size: 12px; font-weight: 700; color: var(--text-primary); }}
        .user-role {{ font-size: 10px; color: var(--text-muted); }}
        .btn-logout {{
            color: #f43f5e;
            text-decoration: none;
            font-size: 11px;
            font-weight: bold;
            margin-left: 6px;
            padding-left: 8px;
            border-left: 1px solid var(--border-default);
        }}
        .btn-logout:hover {{ text-decoration: underline; }}
        /* Main Layout */
        main.app-viewport {{
            flex: 1;
            max-width: 1200px;
            width: 100%;
            margin: 0 auto;
            padding: 32px 24px;
        }}
        .card {{
            background: var(--bg-surface);
            border: 1px solid var(--border-default);
            border-radius: 10px;
            padding: 24px;
            box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
            margin-bottom: 24px;
        }}
        /* Metric Cards */
        .metrics-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
            gap: 16px;
            margin-bottom: 24px;
        }}
        .metric-card {{
            background: var(--bg-surface);
            border: 1px solid var(--border-default);
            border-radius: 8px;
            padding: 18px;
        }}
        .metric-title {{ font-size: 12px; font-weight: 600; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.05em; }}
        .metric-value {{ font-size: 24px; font-weight: 800; color: var(--text-primary); margin: 6px 0 2px; font-family: var(--font-mono); }}
        .metric-sub {{ font-size: 11px; color: #10b981; }}
        /* Tables */
        table.enterprise-table {{
            width: 100%;
            border-collapse: collapse;
            font-size: 13px;
            text-align: left;
        }}
        table.enterprise-table th {{
            background: var(--bg-surface-elevated);
            color: var(--text-secondary);
            font-weight: 700;
            padding: 12px 16px;
            border-bottom: 1px solid var(--border-default);
            font-family: var(--font-mono);
            font-size: 11px;
            text-transform: uppercase;
        }}
        table.enterprise-table td {{
            padding: 14px 16px;
            border-bottom: 1px solid var(--border-default);
            color: var(--text-primary);
        }}
        table.enterprise-table tr:hover td {{
            background: var(--bg-surface-elevated);
        }}
        /* Badges */
        .badge {{
            display: inline-block;
            padding: 3px 8px;
            border-radius: 4px;
            font-size: 11px;
            font-weight: 700;
            font-family: var(--font-mono);
        }}
        .badge-settled {{ background: rgba(16, 185, 129, 0.15); color: #34d399; border: 1px solid rgba(16, 185, 129, 0.3); }}
        .badge-restricted {{ background: rgba(244, 63, 94, 0.15); color: #fb7185; border: 1px solid rgba(244, 63, 94, 0.4); }}
        .badge-pending {{ background: rgba(245, 158, 11, 0.15); color: #fbbf24; border: 1px solid rgba(245, 158, 11, 0.3); }}
        /* Forms */
        .form-group {{ margin-bottom: 16px; }}
        label {{ display: block; font-size: 12px; font-weight: 700; color: var(--text-secondary); margin-bottom: 6px; text-transform: uppercase; font-family: var(--font-mono); }}
        input[type=text], input[type=password] {{
            width: 100%;
            background: var(--bg-canvas);
            border: 1px solid var(--border-default);
            padding: 12px 14px;
            border-radius: 6px;
            color: white;
            font-family: var(--font-family);
            font-size: 14px;
            outline: none;
            transition: border-color 0.15s;
        }}
        input[type=text]:focus, input[type=password]:focus {{
            border-color: var(--border-active);
            box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2);
        }}
        .btn-primary {{
            background: linear-gradient(135deg, #0284c7 0%, #0369a1 100%);
            color: white;
            border: none;
            padding: 12px 20px;
            border-radius: 6px;
            font-weight: 700;
            font-size: 13px;
            cursor: pointer;
            width: 100%;
            transition: opacity 0.15s;
        }}
        .btn-primary:hover {{ opacity: 0.92; }}
        .btn-secondary {{
            background: var(--bg-surface-elevated);
            color: #38bdf8;
            border: 1px solid var(--border-default);
            padding: 6px 12px;
            border-radius: 4px;
            text-decoration: none;
            font-size: 12px;
            font-weight: 600;
            display: inline-block;
        }}
        .btn-secondary:hover {{ border-color: #38bdf8; background: var(--bg-subtle); }}
        /* Wire Document Presentation */
        .wire-doc-header {{
            display: flex;
            justify-content: space-between;
            align-items: flex-start;
            padding-bottom: 20px;
            border-bottom: 1px solid var(--border-default);
            margin-bottom: 20px;
        }}
        .wire-doc-grid {{
            display: grid;
            grid-template-columns: repeat(2, 1fr);
            gap: 20px;
            margin-bottom: 24px;
        }}
        .doc-field-label {{ font-size: 11px; font-weight: bold; color: var(--text-muted); text-transform: uppercase; font-family: var(--font-mono); }}
        .doc-field-value {{ font-size: 14px; font-weight: 600; color: var(--text-primary); margin-top: 4px; }}
        .doc-field-value.amount {{ font-size: 22px; font-weight: 800; color: #38bdf8; font-family: var(--font-mono); }}
        .confidential-memo-box {{
            background: #090d16;
            border: 1px solid #334155;
            border-left: 4px solid #0284c7;
            padding: 16px;
            border-radius: 4px;
            margin-top: 20px;
        }}
        .confidential-memo-box.restricted {{
            border-left-color: #f43f5e;
            background: rgba(244, 63, 94, 0.05);
        }}
        pre.memo-text {{
            font-family: var(--font-mono);
            font-size: 12.5px;
            color: #cbd5e1;
            white-space: pre-wrap;
            word-break: break-all;
            margin-top: 6px;
        }}
        footer.enterprise-footer {{
            border-top: 1px solid var(--border-default);
            padding: 16px 24px;
            text-align: center;
            font-size: 11px;
            color: var(--text-muted);
            font-family: var(--font-mono);
            background: var(--bg-surface);
        }}
    </style>
</head>
<body>
    {}
    <header class="enterprise-header">
        <a href="/" class="brand-container">
            <div class="brand-logo-icon">AF</div>
            <div class="brand-text">
                <h1>ApexFin Corporate Treasury</h1>
                <p>Internal Financial Accounting Portal // Global Liquidity System</p>
            </div>
        </a>
        <nav class="main-nav">
            {}
        </nav>
        <div class="header-user-section">
            {}
        </div>
    </header>

    <main class="app-viewport">
        {}
    </main>

    <footer class="enterprise-footer">
        CONFIDENTIAL & PROPRIETARY — APEXFINANCIAL GLOBAL TREASURY GATEWAY // REGULATED BY FEDWIRE SOVEREIGN CLEARINGHOUSE
    </footer>

    <script>
        function copyFlag() {{
            const flagEl = document.getElementById('targetFlag');
            if (flagEl) {{
                navigator.clipboard.writeText(flagEl.innerText.trim()).then(() => {{
                    const btn = document.querySelector('.btn-copy-flag');
                    if (btn) {{
                        btn.innerText = '✅ COPIED!';
                        setTimeout(() => btn.innerText = '📋 COPY FLAG', 2500);
                    }}
                }});
            }}
        }}
    </script>
</body>
</html>"#,
        page_title,
        flag_telemetry_banner,
        nav_links,
        user_profile_badge,
        content_html
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
    let current_user = current_user_id.and_then(|uid| state.users.iter().find(|u| u.id == uid));

    // 1. Health endpoint
    if path == "/health" {
        resp_headers.insert("Content-Type".to_string(), "application/json".to_string());
        let json = serde_json::json!({
            "status": "ok",
            "lab": "A01",
            "runtime": "native_sandboxed",
            "archetype": "ApexFin Corporate Treasury",
            "pid": std::process::id()
        });
        return (200, resp_headers, json.to_string());
    }

    // 2. Root / Dashboard
    if path == "/" || path == "/index.html" {
        if let Some(user) = current_user {
            let user_wires: Vec<&WireTransaction> = state
                .wires
                .iter()
                .filter(|w| w.user_id == user.id)
                .collect();

            let mut table_rows = String::new();
            for w in &user_wires {
                let badge_class = if w.status == "SETTLED" {
                    "badge-settled"
                } else {
                    "badge-pending"
                };
                table_rows.push_str(&format!(
                    r#"<tr>
                        <td><strong>{}</strong></td>
                        <td>{}</td>
                        <td>{}</td>
                        <td><strong>${:.2}</strong></td>
                        <td><span class="badge {}">{}</span></td>
                        <td>{}</td>
                        <td><a href="/wire/{}" class="btn-secondary">View Wire Slip</a></td>
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
                <div class="metrics-grid">
                    <div class="metric-card">
                        <div class="metric-title">Operating Liquidity</div>
                        <div class="metric-value">$42,500,210.00</div>
                        <div class="metric-sub">↑ 4.2% from prior settlement</div>
                    </div>
                    <div class="metric-card">
                        <div class="metric-title">Authorized Transactions</div>
                        <div class="metric-value">{} Active</div>
                        <div class="metric-sub">Role: {}</div>
                    </div>
                    <div class="metric-card">
                        <div class="metric-title">Compliance Status</div>
                        <div class="metric-value">AUDITED</div>
                        <div class="metric-sub">SOX-404 Node Live</div>
                    </div>
                </div>

                <div class="card">
                    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;">
                        <div>
                            <h2 style="font-size: 18px; font-weight: 800;">Corporate Wire Transfers Ledger</h2>
                            <p style="font-size: 12px; color: var(--text-muted);">Displaying verified transactions for operative <strong>{}</strong> (User ID: #{}).</p>
                        </div>
                    </div>
                    <table class="enterprise-table">
                        <thead>
                            <tr>
                                <th>Ref Code</th>
                                <th>Description / Purpose</th>
                                <th>Beneficiary</th>
                                <th>Amount (USD)</th>
                                <th>Settlement</th>
                                <th>Timestamp</th>
                                <th>Action</th>
                            </tr>
                        </thead>
                        <tbody>
                            {}
                        </tbody>
                    </table>
                </div>
                "#,
                user_wires.len(),
                user.role,
                user.full_name,
                user.id,
                table_rows
            );

            let page = render_enterprise_shell("Operations Dashboard", &dashboard_html, Some(user), None);
            return (200, resp_headers, page);
        } else {
            // Login view
            let login_box = r#"
            <div style="max-width: 440px; margin: 40px auto;">
                <div class="card">
                    <div style="text-align: center; margin-bottom: 24px;">
                        <div style="width: 48px; height: 48px; background: #0284c7; border-radius: 12px; margin: 0 auto 12px; display: flex; align-items: center; justify-content: center; font-size: 24px;">🏛️</div>
                        <h2 style="font-size: 20px; font-weight: 800;">Treasury Operator Access</h2>
                        <p style="font-size: 12px; color: var(--text-muted); margin-top: 4px;">Sign in to access corporate wire settlement ledger.</p>
                    </div>

                    <form method="POST" action="/login">
                        <div class="form-group">
                            <label>Employee Username</label>
                            <input type="text" name="username" placeholder="e.g. alice" value="alice" required>
                        </div>
                        <div class="form-group">
                            <label>Security Password</label>
                            <input type="password" name="password" placeholder="Password" value="password123" required>
                        </div>
                        <button type="submit" class="btn-primary">Sign In to Treasury Node</button>
                    </form>

                    <div style="margin-top: 20px; padding: 12px; background: var(--bg-canvas); border-radius: 6px; border: 1px solid var(--border-default); font-size: 11.5px; color: var(--text-secondary);">
                        <strong style="color: #38bdf8;">Standard Demo Credentials:</strong><br>
                        • Junior Analyst: <code>alice</code> / <code>password123</code><br>
                        • Controller (Locked): <code>admin</code> / <code>adminsecret_unbrute</code>
                    </div>
                </div>
            </div>
            "#;
            let page = render_enterprise_shell("Employee Access", login_box, None, None);
            return (200, resp_headers, page);
        }
    }

    // 3. Login
    if path == "/login" && method == "POST" {
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
            resp_headers.insert("Location".to_string(), "/wire/1".to_string());
            return (302, resp_headers, "Redirecting...".to_string());
        } else {
            let err_content = r#"
            <div style="max-width: 440px; margin: 40px auto;" class="card">
                <p style="color: #f43f5e; font-weight: bold; margin-bottom: 12px;">Authentication Failed: Invalid credentials.</p>
                <a href="/" class="btn-secondary">Return to Sign In</a>
            </div>
            "#;
            let page = render_enterprise_shell("Auth Error", err_content, None, None);
            return (401, resp_headers, page);
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

    // 5. Wires List Redirect
    if path == "/wires" {
        resp_headers.insert("Location".to_string(), "/".to_string());
        return (302, resp_headers, "Redirecting...".to_string());
    }

    // 6. Wire Detail (INTENTIONALLY VULNERABLE IDOR ENDPOINT)
    // Supports both /wire/<id> and legacy /invoice/<id>
    if path.starts_with("/wire/") || path.starts_with("/invoice/") {
        let prefix = if path.starts_with("/wire/") {
            "/wire/"
        } else {
            "/invoice/"
        };
        let id_str = path.trim_start_matches(prefix).trim_end_matches('/');
        if let Ok(wire_id) = id_str.parse::<u32>() {
            if let Some(w) = state.wires.iter().find(|i| i.id == wire_id) {
                // If wire 42 is accessed, mark IDOR as exploited
                let is_flag_wire = w.id == 42;
                if is_flag_wire {
                    state.idor_exploited = true;
                }

                let badge_class = if is_flag_wire {
                    "badge-restricted"
                } else {
                    "badge-settled"
                };

                let wire_owner = state.users.iter().find(|u| u.id == w.user_id);
                let owner_name = wire_owner.map(|u| u.full_name).unwrap_or("Unknown");
                let owner_role = wire_owner.map(|u| u.role).unwrap_or("Unknown");

                let wire_detail_html = format!(
                    r#"
                    <div style="margin-bottom: 16px;">
                        <a href="/" class="btn-secondary">← Back to Operations Ledger</a>
                    </div>

                    <div class="card">
                        <div class="wire-doc-header">
                            <div>
                                <span class="badge {}" style="margin-bottom: 8px;">{}</span>
                                <h2 style="font-size: 20px; font-weight: 800;">Official Wire Settlement Voucher</h2>
                                <p style="font-size: 12px; color: var(--text-muted); font-family: var(--font-mono);">TRANSACTION REFERENCE: {}</p>
                            </div>
                            <div style="text-align: right;">
                                <div class="doc-field-label">Settlement Amount</div>
                                <div class="doc-field-value amount">${:.2}</div>
                            </div>
                        </div>

                        <div class="wire-doc-grid">
                            <div>
                                <div class="doc-field-label">Originating Account (Debtor)</div>
                                <div class="doc-field-value">{}</div>
                            </div>
                            <div>
                                <div class="doc-field-label">Beneficiary Account (Creditor)</div>
                                <div class="doc-field-value">{} ({})</div>
                            </div>
                            <div>
                                <div class="doc-field-label">Authorized Signatory / Owner</div>
                                <div class="doc-field-value">{} (User ID: #{}) — {}</div>
                            </div>
                            <div>
                                <div class="doc-field-label">Settlement Timestamp</div>
                                <div class="doc-field-value" style="font-family: var(--font-mono);">{}</div>
                            </div>
                        </div>

                        <div class="confidential-memo-box {}">
                            <div class="doc-field-label" style="color: #38bdf8;">Compliance & Regulatory Clearance Memo</div>
                            <pre class="memo-text">{}</pre>
                        </div>
                    </div>
                    "#,
                    badge_class,
                    w.status,
                    w.reference_code,
                    w.amount,
                    w.origin_account,
                    w.beneficiary_account,
                    w.beneficiary_name,
                    owner_name,
                    w.user_id,
                    owner_role,
                    w.settlement_date,
                    if is_flag_wire { "restricted" } else { "" },
                    w.confidential_memo
                );

                let page = render_enterprise_shell(
                    w.reference_code,
                    &wire_detail_html,
                    current_user,
                    if is_flag_wire { Some(FLAG) } else { None },
                );
                return (200, resp_headers, page);
            } else {
                let not_found_html = r#"
                <div class="card" style="text-align: center; padding: 40px 20px;">
                    <div style="font-size: 32px; margin-bottom: 8px;">🔍</div>
                    <h2 style="font-size: 18px; font-weight: 800; color: #f43f5e;">Transaction Record Not Found</h2>
                    <p style="font-size: 13px; color: var(--text-muted); margin: 8px 0 16px;">The requested wire voucher identifier does not exist in the active settlement ledger.</p>
                    <a href="/" class="btn-secondary">Return to Ledger</a>
                </div>
                "#;
                let page = render_enterprise_shell("Record Not Found", not_found_html, current_user, None);
                return (404, resp_headers, page);
            }
        }
    }

    // 7. Audit & Admin Tabs
    if path == "/audit" || path == "/admin" {
        let is_admin = current_user.map(|u| u.username == "admin").unwrap_or(false);
        let content = if is_admin {
            r#"
            <div class="card">
                <h2 style="font-size: 18px; font-weight: 800; color: #10b981;">🛡️ SOX-404 Executive Audit Console</h2>
                <p style="font-size: 13px; color: var(--text-muted); margin-top: 6px;">You are authenticated as Marcus Reid (VP Controller). Direct inspection of all treasury vouchers is authorized.</p>
                <div style="margin-top: 16px;">
                    <a href="/wire/42" class="btn-secondary">Inspect Sovereign Audit Record #42</a>
                </div>
            </div>
            "#
        } else {
            r#"
            <div class="card" style="text-align: center; padding: 40px 20px;">
                <div style="font-size: 32px; margin-bottom: 8px;">🚫</div>
                <h2 style="font-size: 18px; font-weight: 800; color: #f43f5e;">Access Prohibited — Role Insufficient</h2>
                <p style="font-size: 13px; color: var(--text-muted); margin: 8px 0 16px;">The Executive Compliance Console requires Marcus Reid (VP Controller) or Board Audit privileges. Standard analysts are restricted from administrative views.</p>
                <p style="font-size: 12px; color: #38bdf8;">Hint: Can you access confidential treasury records directly without accessing this admin page?</p>
                <div style="margin-top: 16px;">
                    <a href="/" class="btn-secondary">Return to Standard Dashboard</a>
                </div>
            </div>
            "#
        };
        let page = render_enterprise_shell("Executive Controls", content, current_user, None);
        return (200, resp_headers, page);
    }

    let not_found = render_enterprise_shell(
        "404 Not Found",
        "<div class='card'><h2>404 — Endpoint Not Found</h2><p style='margin-top: 8px;'><a href='/' class='btn-secondary'>Return Home</a></p></div>",
        current_user,
        None,
    );
    (404, resp_headers, not_found)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--help" || a == "-h" || a == "/?") {
        println!("ApexFin Corporate Treasury Sandboxed Lab A01 (Broken Access Control) v1.0.1");
        println!("This native lab communicates with the host engine broker via stdin/stdout JSON IPC.");
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
                        "Practice verified: You successfully accessed unowned sovereign wire voucher #42 via IDOR parameter tampering!"
                    } else {
                        "Practice incomplete: You have not accessed wire voucher #42 yet."
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
                        "Correct! Sovereign audit clearance flag accepted."
                    } else {
                        "Incorrect flag. Try inspecting unowned wire voucher #42."
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
