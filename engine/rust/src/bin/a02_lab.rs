// A02: Security Misconfiguration — Native Sandboxed Lab Binary v1.1.0
// Simulates an internal enterprise operations gateway with intentional misconfigurations:
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
const LAB_VERSION: &str = "1.1.0";

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
:root {
    --bg-main: #f8fafc;
    --bg-surface: #ffffff;
    --bg-card: #f1f5f9;
    --border: #e2e8f0;
    --border-accent: #0284c7;
    --text-primary: #0f172a;
    --text-secondary: #334155;
    --text-muted: #64748b;
    --accent: #0284c7;
    --accent-glow: rgba(2, 132, 199, 0.08);
    --warning: #d97706;
    --danger: #dc2626;
    --success: #059669;
}
* { box-sizing: border-box; margin: 0; padding: 0; }
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
    background-color: var(--bg-main);
    color: var(--text-primary);
    line-height: 1.5;
    min-height: 100vh;
}
a { color: var(--accent); text-decoration: none; transition: color 0.15s ease; }
a:hover { color: #0369a1; text-decoration: underline; }

/* Top Enterprise Header */
.top-nav {
    background-color: var(--bg-surface);
    border-bottom: 1px solid var(--border);
    padding: 12px 24px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    position: sticky;
    top: 0;
    z-index: 50;
}
.brand-group {
    display: flex;
    align-items: center;
    gap: 12px;
}
.brand-logo {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    background: linear-space(135deg, #0284c7, #0369a1);
    background-color: #0284c7;
    color: white;
    font-weight: 800;
    border-radius: 6px;
    font-size: 16px;
}
.brand-title {
    font-size: 15px;
    font-weight: 700;
    letter-spacing: -0.01em;
    color: var(--text-primary);
}
.brand-badge {
    background-color: #f1f5f9;
    color: #0284c7;
    border: 1px solid #e2e8f0;
    font-size: 11px;
    padding: 2px 8px;
    border-radius: 9999px;
    font-weight: 600;
}
.nav-links {
    display: flex;
    align-items: center;
    gap: 20px;
    font-size: 13px;
    font-weight: 500;
}
.nav-item {
    color: var(--text-muted);
    display: flex;
    align-items: center;
    gap: 6px;
}
.nav-item.active {
    color: var(--accent);
    font-weight: 600;
}
.user-panel {
    display: flex;
    align-items: center;
    gap: 12px;
    font-size: 13px;
}
.status-pill {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 10px;
    border-radius: 9999px;
    font-size: 12px;
    font-weight: 500;
    background-color: #ecfdf5;
    color: #059669;
    border: 1px solid #a7f3d0;
}
.status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background-color: #10b981;
}

/* Page Layout */
.container {
    max-width: 1280px;
    margin: 0 auto;
    padding: 24px;
}
.header-banner {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 24px;
    border-bottom: 1px solid var(--border);
    padding-bottom: 16px;
}
.header-banner h1 {
    font-size: 22px;
    font-weight: 700;
    letter-spacing: -0.02em;
    color: var(--text-primary);
}
.header-banner p {
    font-size: 13px;
    color: var(--text-secondary);
    margin-top: 4px;
}

/* KPI Telemetry Cards */
.metrics-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 16px;
    margin-bottom: 24px;
}
.metric-card {
    background-color: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 16px;
    display: flex;
    flex-direction: column;
    position: relative;
    overflow: hidden;
}
.metric-card.warning-card {
    border-color: rgba(245, 158, 11, 0.4);
}
.metric-card.warning-card::before {
    content: "";
    position: absolute;
    top: 0; left: 0; right: 0;
    height: 3px;
    background: var(--warning);
}
.metric-title {
    font-size: 12px;
    color: var(--text-muted);
    text-transform: uppercase;
    font-weight: 600;
    letter-spacing: 0.05em;
}
.metric-value {
    font-size: 20px;
    font-weight: 700;
    margin: 8px 0 4px 0;
    color: var(--text-primary);
}
.metric-subtitle {
    font-size: 12px;
    color: var(--text-secondary);
}

/* Action Cards */
.section-title {
    font-size: 15px;
    font-weight: 700;
    margin-bottom: 12px;
    color: var(--text-primary);
    display: flex;
    align-items: center;
    gap: 8px;
}
.action-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
    gap: 16px;
    margin-bottom: 24px;
}
.action-card {
    background-color: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 20px;
    transition: transform 0.15s ease, border-color 0.15s ease, box-shadow 0.15s ease;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
}
.action-card:hover {
    transform: translateY(-2px);
    border-color: var(--accent);
    box-shadow: 0 4px 16px var(--accent-glow);
}
.action-card-header {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 10px;
}
.action-icon {
    font-size: 22px;
}
.action-card h3 {
    font-size: 15px;
    font-weight: 600;
    color: var(--text-primary);
}
.action-card p {
    font-size: 13px;
    color: var(--text-secondary);
    margin-bottom: 16px;
}
.action-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    background-color: #0284c7;
    color: white;
    padding: 8px 14px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 600;
    border: none;
    cursor: pointer;
    text-decoration: none;
}
.action-btn:hover {
    background-color: #0369a1;
    color: white;
    text-decoration: none;
}
.action-btn.secondary {
    background-color: #ffffff;
    border: 1px solid var(--border);
    color: var(--text-primary);
}
.action-btn.secondary:hover {
    background-color: #f1f5f9;
}

/* Warnings and Notices */
.alert-banner {
    background-color: #fffbeb;
    border: 1px solid #fde68a;
    border-left: 4px solid var(--warning);
    border-radius: 6px;
    padding: 14px 18px;
    margin-bottom: 24px;
    display: flex;
    align-items: flex-start;
    gap: 12px;
}
.alert-banner.critical {
    background-color: #fff1f2;
    border-color: #fecdd3;
    border-left-color: var(--danger);
}
.alert-content h4 {
    font-size: 13px;
    font-weight: 700;
    color: #b45309;
    margin-bottom: 2px;
}
.alert-banner.critical .alert-content h4 {
    color: #b91c1c;
}
.alert-content p {
    font-size: 12px;
    color: #475569;
}

/* Data Table */
.data-table-wrapper {
    background-color: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 8px;
    overflow: hidden;
    margin-bottom: 24px;
    box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.04);
}
table.enterprise-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 13px;
    text-align: left;
}
table.enterprise-table th {
    background-color: #f8fafc;
    color: var(--text-muted);
    font-weight: 600;
    text-transform: uppercase;
    font-size: 11px;
    letter-spacing: 0.05em;
    padding: 10px 16px;
    border-bottom: 1px solid var(--border);
}
table.enterprise-table td {
    padding: 12px 16px;
    border-bottom: 1px solid var(--border);
    color: var(--text-secondary);
}
table.enterprise-table tr:last-child td {
    border-bottom: none;
}
table.enterprise-table tr:hover td {
    background-color: #f8fafc;
}
.tag {
    display: inline-block;
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 11px;
    font-weight: 600;
    font-family: monospace;
}
.tag.danger { background: #fee2e2; color: #dc2626; border: 1px solid #fca5a5; }
.tag.warning { background: #fef3c7; color: #d97706; border: 1px solid #fde68a; }
.tag.info { background: #e0f2fe; color: #0284c7; border: 1px solid #bae6fd; }
.tag.success { background: #ecfdf5; color: #059669; border: 1px solid #a7f3d0; }

/* Form Styles */
.form-card {
    background-color: var(--bg-surface);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 32px;
    max-width: 440px;
    margin: 40px auto;
    box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.05);
}
.form-card h2 {
    font-size: 18px;
    font-weight: 700;
    margin-bottom: 6px;
    color: var(--text-primary);
}
.form-card p.subtitle {
    font-size: 13px;
    color: var(--text-muted);
    margin-bottom: 24px;
}
.form-group {
    margin-bottom: 16px;
}
.form-group label {
    display: block;
    font-size: 12px;
    font-weight: 600;
    color: var(--text-secondary);
    margin-bottom: 6px;
    text-transform: uppercase;
}
.form-input {
    width: 100%;
    padding: 10px 12px;
    background-color: #ffffff;
    border: 1px solid var(--border);
    border-radius: 6px;
    color: var(--text-primary);
    font-size: 14px;
    outline: none;
    transition: border-color 0.15s ease;
}
.form-input:focus {
    border-color: var(--accent);
    box-shadow: 0 0 0 2px var(--accent-glow);
}
.submit-btn {
    width: 100%;
    padding: 12px;
    background-color: #0284c7;
    color: white;
    border: none;
    border-radius: 6px;
    font-weight: 600;
    font-size: 14px;
    cursor: pointer;
    transition: background 0.15s ease;
    margin-top: 8px;
}
.submit-btn:hover {
    background-color: #0369a1;
}

/* Flag Hero Box */
.flag-banner {
    background: #f0fdf4;
    border: 2px solid #86efac;
    border-radius: 8px;
    padding: 20px;
    margin-bottom: 24px;
    text-align: center;
    color: #14532d;
}
.flag-banner h2 {
    color: #15803d;
    font-size: 18px;
    font-weight: 800;
    margin-bottom: 8px;
}
.flag-box {
    display: inline-block;
    background-color: #ffffff;
    color: #14532d;
    font-family: "JetBrains Mono", Consolas, monospace;
    font-size: 15px;
    font-weight: bold;
    padding: 8px 16px;
    border-radius: 6px;
    border: 1px dashed #22c55e;
    margin: 8px 0;
    user-select: all;
}

/* Directory Listing view */
.dir-header {
    font-size: 20px;
    font-weight: 700;
    margin-bottom: 8px;
    font-family: monospace;
}
.dir-table {
    width: 100%;
    border-collapse: collapse;
    font-family: monospace;
    font-size: 13px;
    margin-top: 16px;
}
.dir-table th {
    text-align: left;
    border-bottom: 1px solid var(--border);
    padding: 8px;
    color: var(--text-muted);
}
.dir-table td {
    padding: 8px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}
.dir-footer {
    margin-top: 24px;
    font-size: 12px;
    color: var(--text-muted);
    font-family: monospace;
    border-top: 1px solid var(--border);
    padding-top: 8px;
}

/* Footer */
footer {
    border-top: 1px solid var(--border);
    padding: 20px;
    text-align: center;
    font-size: 12px;
    color: var(--text-muted);
    margin-top: 40px;
    background: #ffffff;
}
"#
}

fn html_page(title: &str, body_content: &str, session_user: Option<&str>) -> String {
    let user_html = if let Some(u) = session_user {
        format!(
            r#"<span class="status-pill"><span class="status-dot"></span> Auth: <strong>{}</strong></span>
               <a href="/logout" style="font-size: 12px; color: #ef4444; font-weight: 600;">Sign Out</a>"#,
            u
        )
    } else {
        r#"<span style="font-size: 12px; color: var(--text-muted);">Guest Session</span>
           <a href="/login" class="action-btn secondary" style="padding: 4px 10px; font-size: 11px;">Sign In</a>"#
            .to_string()
    };

    let active_overview = if title.contains("Portal") || title.contains("Overview") { "active" } else { "" };
    let active_login = if title.contains("Login") { "active" } else { "" };
    let active_admin = if title.contains("Admin") { "active" } else { "" };
    let active_backups = if title.contains("Backup") { "active" } else { "" };

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} — OpsGateway Enterprise Platform</title>
    <style>{}</style>
</head>
<body>
    <header class="top-nav">
        <div class="brand-group">
            <div class="brand-logo">OG</div>
            <a href="/" style="text-decoration: none;">
                <span class="brand-title">OpsGateway Cloud</span>
            </a>
            <span class="brand-badge">PROD-CORE-01</span>
        </div>
        <nav class="nav-links">
            <a href="/" class="nav-item {}">Dashboard</a>
            <a href="/backups/" class="nav-item {}">Storage (/backups/)</a>
            <a href="/debug/vars" class="nav-item" target="_blank">Telemetry (/debug/vars)</a>
            <a href="/login" class="nav-item {}">Admin Console</a>
            {}
        </nav>
        <div class="user-panel">
            {}
        </div>
    </header>
    <main class="container">
        {}
    </main>
    <footer>
        OpsGateway Enterprise Infrastructure Manager v{} &bull; Environment: Production &bull; Cluster: US-East-1
    </footer>
</body>
</html>"#,
        title,
        css(),
        active_overview,
        active_backups,
        active_login,
        if session_user.is_some() {
            format!(r#"<a href="/admin" class="nav-item {}">Root Access</a>"#, active_admin)
        } else {
            "".to_string()
        },
        user_html,
        body_content,
        LAB_VERSION
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
    resp_headers.insert("Server".to_string(), "OpsGateway/2.1.4 (Ubuntu)".to_string());
    resp_headers.insert("X-Content-Type-Options".to_string(), "nosniff".to_string());

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
    if path == "/health" || path == "/health/" {
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

    // --- Root / or /index.html ---
    if path == "/" || path == "/index.html" || path.is_empty() {
        let content = r#"
        <div class="header-banner">
            <div>
                <h1>Infrastructure & Operations Control Panel</h1>
                <p>Enterprise orchestration, backup archival, and diagnostic telemetry dashboard.</p>
            </div>
            <div style="text-align: right;">
                <span class="status-pill"><span class="status-dot"></span> All Clusters Online</span>
                <p style="font-size: 11px; color: var(--text-muted); margin-top: 4px;">Host: 127.0.0.1:8012</p>
            </div>
        </div>

        <div class="alert-banner critical">
            <div style="font-size: 20px;">⚠️</div>
            <div class="alert-content">
                <h4>Security Baseline Non-Compliance Detected (OWASP A02 Alert)</h4>
                <p>Automated security scanning detected exposed runtime diagnostics at <code>/debug/vars</code> and directory autoindexing on the archive partition at <code>/backups/</code>. Furthermore, default system supervisor credentials remain active.</p>
            </div>
        </div>

        <div class="metrics-grid">
            <div class="metric-card">
                <span class="metric-title">Active Compute Nodes</span>
                <span class="metric-value">16 / 16 Healthy</span>
                <span class="metric-subtitle">99.98% cluster uptime (30d)</span>
            </div>
            <div class="metric-card warning-card">
                <span class="metric-title">Archival Storage Pool</span>
                <span class="metric-value" style="color: var(--warning);">/backups/ Exposed</span>
                <span class="metric-subtitle"><span class="tag warning">Autoindex: ON</span> Directory listing active</span>
            </div>
            <div class="metric-card warning-card">
                <span class="metric-title">Debug Telemetry</span>
                <span class="metric-value" style="color: #38bdf8;">/debug/vars Live</span>
                <span class="metric-subtitle"><span class="tag info">debug_mode: true</span> Unauthenticated</span>
            </div>
            <div class="metric-card">
                <span class="metric-title">Database Cluster</span>
                <span class="metric-value" style="color: #34d399;">PostgreSQL 15</span>
                <span class="metric-subtitle">Host: internal-db.ops.local:5432</span>
            </div>
        </div>

        <div class="section-title">
            <span>Operational Navigation & Target Interfaces</span>
        </div>
        <div class="action-grid">
            <div class="action-card">
                <div>
                    <div class="action-card-header">
                        <span class="action-icon">📁</span>
                        <h3>Storage Partition (/backups/)</h3>
                    </div>
                    <p>Review active snapshot archives and filesystem dumps stored on the public archival partition. Directory browsing is currently accessible without credentials.</p>
                </div>
                <div>
                    <a href="/backups/" class="action-btn">Inspect Archive Directory &rarr;</a>
                </div>
            </div>
            <div class="action-card">
                <div>
                    <div class="action-card-header">
                        <span class="action-icon">🔍</span>
                        <h3>Runtime Telemetry (/debug/vars)</h3>
                    </div>
                    <p>Inspect internal application runtime metrics, memory allocation stats, system environment paths, and database configuration metadata.</p>
                </div>
                <div>
                    <a href="/debug/vars" target="_blank" class="action-btn secondary">View Raw Telemetry Feed &rarr;</a>
                </div>
            </div>
            <div class="action-card">
                <div>
                    <div class="action-card-header">
                        <span class="action-icon">🔐</span>
                        <h3>Supervisor Console (/login)</h3>
                    </div>
                    <p>Authenticate as an infrastructure operator to manage system keys and cluster parameters. Default administrative credentials apply for factory provisioning.</p>
                </div>
                <div>
                    <a href="/login" class="action-btn secondary">Open Administrator Sign In &rarr;</a>
                </div>
            </div>
        </div>

        <div class="section-title">
            <span>Recent System & Security Audit Events</span>
        </div>
        <div class="data-table-wrapper">
            <table class="enterprise-table">
                <thead>
                    <tr>
                        <th>Timestamp</th>
                        <th>Severity</th>
                        <th>Facility</th>
                        <th>Event Summary</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>02:00:14 UTC</td>
                        <td><span class="tag info">INFO</span></td>
                        <td>cron-backup</td>
                        <td>Nightly automated snapshot archive created: <code>/app/backups/backup_config.json.bak</code></td>
                    </tr>
                    <tr>
                        <td>04:15:22 UTC</td>
                        <td><span class="tag warning">WARNING</span></td>
                        <td>http-engine</td>
                        <td>Exposed debug interface initialized at <code>/debug/vars</code> (debug_mode enabled by developer flag)</td>
                    </tr>
                    <tr>
                        <td>06:30:00 UTC</td>
                        <td><span class="tag danger">CRITICAL</span></td>
                        <td>nginx-proxy</td>
                        <td>Security Audit: Directory browsing (autoindex) is currently ENABLED on path <code>/backups/</code></td>
                    </tr>
                    <tr>
                        <td>08:00:10 UTC</td>
                        <td><span class="tag warning">WARNING</span></td>
                        <td>auth-service</td>
                        <td>Default system administrator account (<code>admin</code>) remains configured with default initial password</td>
                    </tr>
                </tbody>
            </table>
        </div>
        "#;
        return (
            200,
            resp_headers,
            html_page("OpsGateway Operations Portal", content, session_user.as_deref()),
        );
    }

    // --- Login GET ---
    if (path == "/login" || path == "/login/") && method == "GET" {
        let form = r#"
        <div class="form-card">
            <div style="text-align: center; margin-bottom: 20px;">
                <div class="brand-logo" style="margin: 0 auto 12px; width: 44px; height: 44px; font-size: 20px;">OG</div>
                <h2>OpsGateway Administrator Console</h2>
                <p class="subtitle">Single Sign-On & Local Administrative Gateway</p>
            </div>

            <div class="alert-banner" style="margin-bottom: 20px; padding: 10px 14px;">
                <div class="alert-content">
                    <p style="font-size: 11px; color: #fbbf24;"><strong>Provisioning Notice:</strong> System is operating under initial bootstrap configuration. Default factory credentials (<code>admin</code> / <code>admin</code>) are active.</p>
                </div>
            </div>

            <form method="POST" action="/login">
                <div class="form-group">
                    <label>Operator Username</label>
                    <input type="text" class="form-input" name="username" placeholder="e.g. admin" required autofocus>
                </div>
                <div class="form-group">
                    <label>Supervisor Password</label>
                    <input type="password" class="form-input" name="password" placeholder="••••••••" required>
                </div>
                <button type="submit" class="submit-btn">Authenticate & Access Console</button>
            </form>
            <div style="margin-top: 20px; text-align: center;">
                <a href="/" style="font-size: 12px;">&larr; Return to Dashboard</a>
            </div>
        </div>
        "#;
        return (
            200,
            resp_headers,
            html_page("Supervisor Login", form, None),
        );
    }

    // --- Login POST ---
    if (path == "/login" || path == "/login/") && method == "POST" {
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
            let err = r#"
            <div class="form-card" style="text-align: center;">
                <h3 style="color: #ef4444; margin-bottom: 12px;">Authentication Refused</h3>
                <p style="color: var(--text-secondary); font-size: 13px; margin-bottom: 20px;">The provided administrative credentials were not recognized.</p>
                <a href="/login" class="action-btn" style="display: inline-block;">Try Again</a>
            </div>
            "#;
            return (
                401,
                resp_headers,
                html_page("Authentication Failed", err, None),
            );
        }
    }

    // --- Admin panel ---
    if path == "/admin" || path == "/admin/" {
        if session_user.is_none() {
            resp_headers.insert("Location".to_string(), "/login".to_string());
            return (302, resp_headers, "Redirecting...".to_string());
        }
        let content = format!(
            r#"
            <div class="flag-banner">
                <h2>🎉 ADMINISTRATIVE ACCESS ESTABLISHED!</h2>
                <p style="font-size: 13px; color: #a7f3d0; margin-bottom: 8px;">You logged in using default administrator credentials (<code>admin:admin</code>). Vulnerability OWASP A02:2025 verified!</p>
                <div class="flag-box">{}</div>
                <p style="font-size: 11px; color: #6ee7b7; margin-top: 4px;">Submit this token in ZITERA_LAB to complete the mission challenge.</p>
            </div>

            <div class="header-banner">
                <div>
                    <h1>Cluster Root Administrative Dashboard</h1>
                    <p>Logged in as: <strong>{}</strong> (Global System Administrator)</p>
                </div>
                <div>
                    <a href="/logout" class="action-btn secondary" style="font-size: 12px;">Sign Out</a>
                </div>
            </div>

            <div class="section-title">
                <span>Administrative Configuration & Control</span>
            </div>
            <div class="action-grid">
                <div class="action-card">
                    <div class="action-card-header">
                        <span class="action-icon">⚙️</span>
                        <h3>System Configuration</h3>
                    </div>
                    <p>Database: <code>internal-db.ops.local:5432</code><br>Storage root: <code>/app/backups/</code><br>Session secret: <code>default-ops-session-secret-2024</code></p>
                    <a href="/debug/vars" target="_blank" class="action-btn secondary">Inspect Debug Variables</a>
                </div>
                <div class="action-card">
                    <div class="action-card-header">
                        <span class="action-icon">📦</span>
                        <h3>Archive Manager</h3>
                    </div>
                    <p>Directory listing is active on <code>/backups/</code>. Sensitive backups containing database credentials can be directly retrieved by any unauthenticated client.</p>
                    <a href="/backups/" class="action-btn">Browse /backups/ Directory</a>
                </div>
            </div>
            "#,
            FLAG,
            session_user.as_deref().unwrap_or("admin")
        );
        return (
            200,
            resp_headers,
            html_page("Admin Console", &content, session_user.as_deref()),
        );
    }

    // --- Debug/vars endpoint (INTENTIONALLY UNAUTHENTICATED) ---
    if path == "/debug/vars" || path == "/debug/vars/" {
        state.debug_accessed = true;
        resp_headers.insert("Content-Type".to_string(), "application/json; charset=utf-8".to_string());
        let json = serde_json::json!({
            "app_name": "OpsGateway",
            "version": "2.1.4",
            "debug_mode": true,
            "environment": "production",
            "internal_storage": "/app/backups/",
            "database_host": "internal-db.ops.local",
            "database_port": 5432,
            "database_user": "opsadmin",
            "admin_email": "ops-admin@corp.internal",
            "session_secret": "default-ops-session-secret-2024",
            "backup_path": "/backups/",
            "admin_system_key": FLAG,
            "runtime_pid": std::process::id(),
            "uptime_seconds": 3600,
            "memstats": {
                "alloc": 18452920,
                "total_alloc": 42109840,
                "sys": 72410290,
                "num_gc": 4
            }
        });
        return (200, resp_headers, json.to_string());
    }

    // --- Directory listing on /backups/ (INTENTIONALLY VULNERABLE) ---
    if path == "/backups/" || path == "/backups" {
        state.backup_accessed = true;
        let content = r#"
        <div style="background-color: var(--bg-surface); border: 1px solid var(--border); border-radius: 8px; padding: 24px;">
            <div class="dir-header">Index of /backups/</div>
            <div class="alert-banner critical" style="margin: 16px 0;">
                <div style="font-size: 18px;">⚠️</div>
                <div class="alert-content">
                    <h4>Security Misconfiguration: Directory Browsing Enabled</h4>
                    <p>The web server configuration permits unrestricted directory indexing (<code>autoindex on;</code>). Anyone can list and download sensitive infrastructure configuration backups.</p>
                </div>
            </div>

            <table class="dir-table">
                <thead>
                    <tr>
                        <th style="width: 45%;">Name</th>
                        <th style="width: 25%;">Last modified</th>
                        <th style="width: 15%;">Size</th>
                        <th style="width: 15%;">Description</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>📁 <a href="/">Parent Directory (../)</a></td>
                        <td>-</td>
                        <td>-</td>
                        <td>Root Directory</td>
                    </tr>
                    <tr>
                        <td>📄 <a href="/backups/backup_config.json.bak" download style="font-weight: bold; color: #38bdf8;">backup_config.json.bak</a></td>
                        <td>2025-03-01 02:00:14</td>
                        <td>2.1 KB</td>
                        <td><span class="tag danger">Sensitive Config Backup</span></td>
                    </tr>
                    <tr>
                        <td>📄 <a href="/backups/ops_keys_old.txt" style="color: #94a3b8;">ops_keys_old.txt</a></td>
                        <td>2024-11-15 14:22:10</td>
                        <td>0.8 KB</td>
                        <td><span class="tag warning">Deprecated API Keys</span></td>
                    </tr>
                    <tr>
                        <td>📄 <a href="/backups/database_schema.sql.bak" style="color: #94a3b8;">database_schema.sql.bak</a></td>
                        <td>2025-01-20 09:15:00</td>
                        <td>18.4 KB</td>
                        <td><span class="tag info">Database DDL Dump</span></td>
                    </tr>
                </tbody>
            </table>

            <div class="dir-footer">
                Apache/2.4.52 (Ubuntu) Server at 127.0.0.1 Port 8012 &bull; Autoindex Module Active
            </div>
        </div>
        "#;
        return (
            200,
            resp_headers,
            html_page("Index of /backups/", content, session_user.as_deref()),
        );
    }

    // --- Backup config file (contains flag) ---
    if path == "/backups/backup_config.json.bak" {
        state.backup_accessed = true;
        resp_headers.insert(
            "Content-Type".to_string(),
            "application/json; charset=utf-8".to_string(),
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
        resp_headers.insert("Content-Type".to_string(), "text/plain; charset=utf-8".to_string());
        return (
            200,
            resp_headers,
            "# DEPRECATED: Old operations keys — DO NOT USE\n# Rotated: 2024-11-15\nOPS_API_KEY_V1=deprecated-key-12345\nOPS_API_KEY_V2=deprecated-key-67890\n".to_string(),
        );
    }

    // --- Database schema dump (adds realism) ---
    if path == "/backups/database_schema.sql.bak" {
        resp_headers.insert("Content-Type".to_string(), "text/plain; charset=utf-8".to_string());
        return (
            200,
            resp_headers,
            "-- OpsGateway PostgreSQL Schema Backup\n-- Generated 2025-01-20\nCREATE TABLE users (id SERIAL PRIMARY KEY, username VARCHAR(64), password_hash VARCHAR(128), role VARCHAR(32));\nCREATE TABLE system_keys (id SERIAL PRIMARY KEY, key_name VARCHAR(64), key_value TEXT);\nCREATE TABLE audit_logs (id SERIAL PRIMARY KEY, timestamp TIMESTAMP, message TEXT);\n".to_string(),
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
        "Page Not Found",
        r#"
        <div class="form-card" style="text-align: center;">
            <h2 style="color: #ef4444; margin-bottom: 8px;">404 &bull; Resource Not Found</h2>
            <p class="subtitle">The requested route does not exist on this OpsGateway server node.</p>
            <a href="/" class="action-btn">&larr; Return to Dashboard</a>
        </div>
        "#,
        session_user.as_deref(),
    );
    (404, resp_headers, not_found)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--help" || a == "-h" || a == "/?") {
        println!("Zitera Sandboxed Lab A02 (Security Misconfiguration) v{}", LAB_VERSION);
        println!("This native lab communicates with the host engine broker via stdin/stdout JSON IPC.");
        println!("Usage: a02-lab.exe");
        return;
    }
    if args.iter().any(|a| a == "--version" || a == "-v" || a == "-V") {
        println!("A02 v{} (native_sandboxed)", LAB_VERSION);
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
