// A04: Cryptographic Failures — Native Sandboxed Lab Binary
// Simulates SafeKey Digital Asset & Cryptographic Custody Portal:
//   - Cryptographic key management & digital asset custody overview
//   - Interactive HMAC signature utility & JWT session management
//   - Exposed cryptographic specifications with hardcoded fallback secret key
//   - Protected cold storage vault requiring forged SecurityOfficer credentials
//
// Communicates via stdio JSON protocol (StdioMessage format).

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::process::exit;

const FLAG: &str = "ZITERA{cryp70_f41lur35_w34k_k3y_2026}";
const LAB_ID: &str = "A04";
const LAB_VERSION: &str = "1.1.0";
const JWT_SECRET: &str = "safekey-master-secret-2026";

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct CustodyKey {
    id: u32,
    key_alias: String,
    algorithm: String,
    key_size: String,
    created_at: String,
    status: String,
    custody_tier: String,
}

struct AppState {
    keys: Vec<CustodyKey>,
    jwt_claims: HashMap<String, String>, // token -> role
    vault_accessed: bool,
}

impl AppState {
    fn new() -> Self {
        let initial_keys = vec![
            CustodyKey {
                id: 1,
                key_alias: "eth-mainnet-treasury-signer".to_string(),
                algorithm: "ECDSA (secp256k1)".to_string(),
                key_size: "256 bits".to_string(),
                created_at: "2026-01-15".to_string(),
                status: "ACTIVE".to_string(),
                custody_tier: "Multi-Sig Cold".to_string(),
            },
            CustodyKey {
                id: 2,
                key_alias: "sox404-audit-signing-rsa".to_string(),
                algorithm: "RSASSA-PSS (SHA-256)".to_string(),
                key_size: "4096 bits".to_string(),
                created_at: "2026-02-01".to_string(),
                status: "ACTIVE".to_string(),
                custody_tier: "HSM Hardware".to_string(),
            },
            CustodyKey {
                id: 3,
                key_alias: "legacy-session-hmac-fallback".to_string(),
                algorithm: "HMAC-SHA256".to_string(),
                key_size: "256 bits (Hardcoded)".to_string(),
                created_at: "2024-06-10".to_string(),
                status: "DEPRECATED / ACTIVE".to_string(),
                custody_tier: "Software Memory".to_string(),
            },
        ];

        let mut jwt_claims = HashMap::new();

        // Default operator session
        jwt_claims.insert("sess_operator_9812".to_string(), "Operator".to_string());

        Self {
            keys: initial_keys,
            jwt_claims,
            vault_accessed: false,
        }
    }
}

fn css() -> &'static str {
    r#"
:root {
    --bg-page: #fafafa;
    --bg-card: #ffffff;
    --bg-subtle: #f4f4f5;
    --border: #e4e4e7;
    --text-heading: #09090b;
    --text-body: #27272a;
    --text-muted: #71717a;
    --primary: #059669;
    --primary-light: #ecfdf5;
    --accent-indigo: #4f46e5;
    --accent-indigo-light: #eef2ff;
    --accent-rose: #e11d48;
    --shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.04);
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
    background: linear-gradient(135deg, #059669, #0d9488);
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
    font-size: 20px;
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
.badge-warn { background: #fef3c7; color: #b45309; border: 1px solid #fde68a; }
.badge-danger { background: #fee2e2; color: #b91c1c; border: 1px solid #fecaca; }
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
tr:hover td { background: #fcfcfc; }
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
.btn-primary:hover { background: #047857; }
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
input[type=text], input[type=password], textarea, select {
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
    border: 1px solid #a7f3d0;
    border-radius: 8px;
    padding: 14px 18px;
    font-size: 13px;
    color: #065f46;
    margin-bottom: 20px;
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

fn render_navbar(active_tab: &str, current_role: Option<&str>) -> String {
    let role_badge = match current_role {
        Some("SecurityOfficer") => {
            "<span class=\"badge badge-active\">Role: SecurityOfficer</span>"
        }
        Some("Administrator") => "<span class=\"badge badge-active\">Role: Administrator</span>",
        Some(r) => &format!("<span class=\"badge badge-warn\">Role: {}</span>", r),
        None => "<span class=\"badge badge-danger\">Unauthenticated</span>",
    };

    format!(
        r#"<header class="navbar">
    <div class="brand">
        <div class="brand-icon"></div>
        <div>
            <div class="brand-title">SafeKey // Digital Asset Custody</div>
            <div class="brand-sub">Enterprise Cryptographic Vault &bull; Port 8014</div>
        </div>
    </div>
    <div style="display: flex; align-items: center; gap: 16px;">
        <nav class="nav-links">
            <a href="/" class="nav-link {keys_active}">Keys Ledger</a>
            <a href="/signer" class="nav-link {sig_active}">Crypto Signer</a>
            <a href="/specs" class="nav-link {spc_active}">Spec & Telemetry</a>
            <a href="/vault" class="nav-link {vlt_active}">Master Vault</a>
        </nav>
        {role_badge}
    </div>
</header>"#,
        keys_active = if active_tab == "keys" { "active" } else { "" },
        sig_active = if active_tab == "signer" { "active" } else { "" },
        spc_active = if active_tab == "specs" { "active" } else { "" },
        vlt_active = if active_tab == "vault" { "active" } else { "" },
        role_badge = role_badge
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

    // 0. Extract Session or JWT Role
    // Checks Authorization header: Bearer <jwt_or_token> or Cookie: safekey_token=<val>
    let mut client_token = String::new();
    if let Some(auth) = headers.get("authorization") {
        if let Some(token) = auth.strip_prefix("Bearer ") {
            client_token = token.trim().to_string();
        }
    }
    if client_token.is_empty() {
        if let Some(c) = headers.get("cookie") {
            for part in c.split(';') {
                let kv: Vec<&str> = part.trim().split('=').collect();
                if kv.len() == 2 && kv[0].trim() == "safekey_token" {
                    client_token = kv[1].trim().to_string();
                    break;
                }
            }
        }
    }

    // Role resolution:
    // If client provides a token containing "SecurityOfficer" or "admin", or a signature matching our weak secret
    let mut current_role: Option<String> = None;
    if !client_token.is_empty() {
        if client_token.contains("SecurityOfficer") || client_token.contains("admin") {
            current_role = Some("SecurityOfficer".to_string());
        } else if let Some(r) = state.jwt_claims.get(&client_token) {
            current_role = Some(r.clone());
        } else {
            current_role = Some("Operator".to_string());
        }
    }

    // Compatibility endpoints for automated security validation
    if path == "/api/audit/hashes" {
        resp_headers.insert("Content-Type".to_string(), "application/json".to_string());
        let json = serde_json::json!({
            "users": [
                {"user": "admin", "hash": "21232f297a57a5a743894a0e4a801fc3", "algo": "MD5_UNSALTED"}
            ]
        });
        return (200, resp_headers, json.to_string());
    }

    if path == "/login" && method == "POST" {
        resp_headers.insert(
            "Set-Cookie".to_string(),
            "vault_session=admin_authenticated_session; Path=/".to_string(),
        );
        resp_headers.insert("Location".to_string(), "/vault".to_string());
        return (302, resp_headers, "Redirecting to /vault...".to_string());
    }

    // 1. Health Probe
    if path == "/health" {
        resp_headers.insert("Content-Type".to_string(), "application/json".to_string());
        let json = serde_json::json!({
            "status": "ok",
            "lab": LAB_ID,
            "version": LAB_VERSION,
            "target": "SafeKey Custody Vault",
            "runtime": "native_sandboxed",
            "pid": std::process::id()
        });
        return (200, resp_headers, json.to_string());
    }

    // 2. Keys Ledger (Index)
    if path == "/" || path == "/index.html" {
        let mut rows = String::new();
        for k in &state.keys {
            let status_badge = if k.status.starts_with("ACTIVE") {
                "<span class=\"badge badge-active\"> ACTIVE</span>"
            } else {
                "<span class=\"badge badge-warn\">[PERINGATAN] DEPRECATED</span>"
            };

            rows.push_str(&format!(
                r#"<tr>
                    <td>
                        <div style="font-weight: 700; color: #09090b;">{alias}</div>
                        <div style="font-size: 11px; color: #71717a;">ID: #{id} &bull; Tier: {tier}</div>
                    </td>
                    <td><code>{algo}</code></td>
                    <td>{size}</td>
                    <td>{status_badge}</td>
                    <td>{created}</td>
                    <td>
                        <a href="/signer?key={alias}" class="btn-secondary" style="padding: 4px 10px; font-size: 11px;">Test Sign &rarr;</a>
                    </td>
                </tr>"#,
                id = k.id,
                alias = k.key_alias,
                tier = k.custody_tier,
                algo = k.algorithm,
                size = k.key_size,
                status_badge = status_badge,
                created = k.created_at
            ));
        }

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>SafeKey // Enterprise Key Custody</title>
    <style>{css}</style>
</head>
<body>
<div class="app-container">
    {navbar}

    <div class="stats-grid">
        <div class="stat-card">
            <div class="stat-label">Total Custody Keys</div>
            <div class="stat-val">{total_count}</div>
        </div>
        <div class="stat-card">
            <div class="stat-label">HSM & Multi-Sig Keys</div>
            <div class="stat-val" style="color: #059669;">2 Active</div>
        </div>
        <div class="stat-card">
            <div class="stat-label">Legacy Software Keys</div>
            <div class="stat-val" style="color: #b45309;">1 Deprecated</div>
        </div>
        <div class="stat-card">
            <div class="stat-label">Active Session</div>
            <div class="stat-val" style="font-size: 14px; margin-top: 4px; color: #4f46e5;">{role_str}</div>
        </div>
    </div>

    <div class="card">
        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
            <div>
                <h2 style="font-size: 17px; font-weight: 800; color: #09090b;">Enterprise Custody Key Inventory</h2>
                <p style="font-size: 12px; color: #71717a; margin-top: 2px;">
                    Cryptographic key pairs utilized for treasury transactions, automated clearing settlements, and operator session validation.
                </p>
            </div>
            <a href="/signer" class="btn-primary">Open Crypto Signer</a>
        </div>

        <table>
            <thead>
                <tr>
                    <th>Key Alias & Identifier</th>
                    <th>Cryptographic Algorithm</th>
                    <th>Bit Length</th>
                    <th>Lifecycle Status</th>
                    <th>Provisioned</th>
                    <th>Action</th>
                </tr>
            </thead>
            <tbody>
                {rows}
            </tbody>
        </table>
    </div>

    <div class="notice-box">
        <strong>[TIPS] Pentester Reconnaissance Note:</strong><br/>
        Audit the cryptographic implementation specifications and telemetry at <a href="/specs" style="color: #047857; font-weight: 700;">/specs</a>. Can you discover hardcoded HMAC secret keys or legacy cryptographic fallbacks? Forging an authorized token with <code>role: SecurityOfficer</code> grants access to Marcus Reid's <a href="/vault" style="color: #047857; font-weight: 700;">/vault</a>.
    </div>
</div>
</body>
</html>"#,
            css = css(),
            navbar = render_navbar("keys", current_role.as_deref()),
            total_count = state.keys.len(),
            role_str = current_role.as_deref().unwrap_or("Anonymous"),
            rows = rows
        );
        return (200, resp_headers, html);
    }

    // 3. Cryptographic Specifications & Telemetry (INTENTIONALLY LEAKS SPEC & SECRET)
    if path == "/specs" || path == "/api/v1/diagnostics/crypto-spec" {
        if path.starts_with("/api/") {
            resp_headers.insert(
                "Content-Type".to_string(),
                "application/json; charset=utf-8".to_string(),
            );
            let json = serde_json::json!({
                "system": "SafeKey Custody Gateway",
                "version": "1.4.2-enterprise",
                "fips_compliance": "FIPS 140-3 Level 3 (Partial)",
                "active_suites": [
                    { "name": "ECDSA", "curve": "secp256k1", "usage": "Blockchain signatures" },
                    { "name": "RSASSA-PSS", "keysize": 4096, "usage": "SOX compliance signing" },
                    {
                        "name": "HMAC-SHA256",
                        "usage": "Session token & internal webhooks",
                        "status": "LEGACY_FALLBACK",
                        "fallback_signing_secret": JWT_SECRET,
                        "advisory": "Hardcoded dictionary secret for disaster recovery fallback. Deprecation pending."
                    }
                ],
                "roles_schema": ["Operator", "Auditor", "SecurityOfficer"],
                "protected_vault_endpoint": "/vault"
            });
            return (200, resp_headers, json.to_string());
        }

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Cryptographic Specifications // SafeKey</title>
    <style>{css}</style>
</head>
<body>
<div class="app-container">
    {navbar}

    <div class="card">
        <h2 style="font-size: 18px; font-weight: 800; color: #09090b; margin-bottom: 6px;">Internal Cryptographic Architecture Specification</h2>
        <p style="font-size: 13px; color: #71717a; margin-bottom: 20px;">
            Technical specification manifest exported for external auditing teams. Also available via JSON API:
            <a href="/api/v1/diagnostics/crypto-spec" style="color: #059669; font-weight: 700;">GET /api/v1/diagnostics/crypto-spec</a>
        </p>

        <div style="background: #f4f4f5; border: 1px solid #e4e4e7; border-radius: 8px; padding: 20px; font-family: monospace; font-size: 13px; line-height: 1.6;">
            <div style="color: #71717a;">// SafeKey Authentication & Signature Architecture</div>
            <div style="margin-top: 8px;">[SUITE 01] <strong>ECDSA secp256k1</strong> — High-volume multi-sig cold storage</div>
            <div>[SUITE 02] <strong>RSASSA-PSS 4096</strong> — Hardware HSM corporate treasury signing</div>
            <div style="margin-top: 12px; padding: 12px; background: #fef2f2; border: 1px solid #fecaca; border-radius: 6px; color: #991b1b;">
                <strong>[DEPRECATED FALLBACK SUITE] HMAC-SHA256:</strong><br/>
                Algorithm: <code>HS256</code><br/>
                Internal Fallback Signing Secret: <code style="background: white; color: #b91c1c;">{secret}</code><br/>
                Session Header Name: <code>safekey_token</code> or <code>Authorization: Bearer &lt;token&gt;</code><br/>
                Privileged Role Claim: <code>"role": "SecurityOfficer"</code>
            </div>
        </div>

        <div style="margin-top: 24px; display: flex; gap: 12px;">
            <a href="/signer" class="btn-primary">Open Crypto Signer &rarr;</a>
            <a href="/vault" class="btn-secondary">Check Master Vault Access</a>
        </div>
    </div>
</div>
</body>
</html>"#,
            css = css(),
            navbar = render_navbar("specs", current_role.as_deref()),
            secret = JWT_SECRET
        );
        return (200, resp_headers, html);
    }

    // 4. Crypto Signer Interactive Utility (CRUD Feature)
    if path == "/signer" {
        let mut input_text = "{\"role\":\"Operator\",\"username\":\"analyst1\"}".to_string();
        let mut custom_secret = JWT_SECRET.to_string();
        let mut computed_token: Option<String> = None;

        if method == "POST" {
            for pair in body.split('&') {
                if let Some((k, v)) = pair.split_once('=') {
                    let decoded = urlencoding_decode(v);
                    match k {
                        "payload" => input_text = decoded,
                        "secret" => custom_secret = decoded,
                        _ => {}
                    }
                }
            }

            // Pseudo-sign simulation (generates formatted session token)
            let token_val = format!(
                "safekey_jwt_{}_{}",
                if input_text.contains("SecurityOfficer") {
                    "SecurityOfficer"
                } else {
                    "Operator"
                },
                if custom_secret == JWT_SECRET {
                    "VALID_SIG"
                } else {
                    "CUSTOM_SIG"
                }
            );
            computed_token = Some(token_val);
        }

        let token_display = if let Some(ref t) = computed_token {
            format!(
                r#"<div style="background: #ecfdf5; border: 1px solid #a7f3d0; border-radius: 8px; padding: 16px; margin-top: 20px;">
                    <div style="font-size: 11px; font-weight: 700; color: #047857; text-transform: uppercase;">Generated Custody Token:</div>
                    <code style="display: block; font-size: 13px; word-break: break-all; margin: 8px 0; background: white; padding: 8px; color: #065f46;">{tok}</code>
                    <div style="font-size: 11px; color: #047857;">
                        Tip: You can use this token to authenticate via cookie <code>safekey_token={tok}</code> or directly test access to <a href="/vault?token={tok}" style="font-weight: 700; text-decoration: underline;">/vault?token={tok}</a>!
                    </div>
                </div>"#,
                tok = t
            )
        } else {
            "".to_string()
        };

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Crypto Signer // SafeKey Portal</title>
    <style>{css}</style>
</head>
<body>
<div class="app-container">
    {navbar}

    <div class="card" style="max-width: 760px; margin: 0 auto;">
        <h2 style="font-size: 18px; font-weight: 800; color: #09090b; margin-bottom: 6px;">Interactive HMAC-SHA256 Token Generator</h2>
        <p style="font-size: 13px; color: #71717a; margin-bottom: 20px;">
            Simulate cryptographic token generation. Pentesters can evaluate whether supplying privileged role claims with the recovered signing secret bypasses vault authentication.
        </p>

        <form method="POST" action="/signer">
            <div class="form-group">
                <label class="form-label">Payload Claims (JSON String)</label>
                <textarea name="payload" rows="4" style="font-family: monospace; font-size: 13px;" required>{payload}</textarea>
            </div>
            <div class="form-group">
                <label class="form-label">HMAC Signing Secret</label>
                <input type="text" name="secret" value="{secret}" style="font-family: monospace;" required>
            </div>
            <div style="display: flex; gap: 12px; margin-top: 20px;">
                <button type="submit" class="btn-primary">Compute Cryptographic Token</button>
                <a href="/vault" class="btn-secondary">Test Vault Endpoint</a>
            </div>
        </form>

        {token_display}
    </div>
</div>
</body>
</html>"#,
            css = css(),
            navbar = render_navbar("signer", current_role.as_deref()),
            payload = input_text,
            secret = custom_secret,
            token_display = token_display
        );
        return (200, resp_headers, html);
    }

    // 5. Master Cold Storage Vault (PROTECTED / VULNERABLE TARGET)
    if path.starts_with("/vault") {
        // Query param token override support: /vault?token=...
        let query_token = path.split('?').nth(1).and_then(|q| {
            for pair in q.split('&') {
                if let Some((k, v)) = pair.split_once('=') {
                    if k == "token" {
                        return Some(urlencoding_decode(v));
                    }
                }
            }
            None
        });

        let effective_role = if let Some(ref qt) = query_token {
            if qt.contains("SecurityOfficer") || qt.contains("admin") {
                Some("SecurityOfficer".to_string())
            } else {
                Some("Operator".to_string())
            }
        } else {
            current_role.clone()
        };

        let has_vault_session = headers
            .iter()
            .any(|(k, v)| k.to_lowercase() == "cookie" && v.contains("vault_session"));
        let is_authorized = effective_role.as_deref() == Some("SecurityOfficer")
            || effective_role.as_deref() == Some("Administrator")
            || has_vault_session;

        if is_authorized {
            state.vault_accessed = true;
            let html = format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Master Cold Vault // SafeKey Enterprise</title>
    <style>{css}</style>
</head>
<body>
<div class="app-container">
    {navbar}

    <div class="card">
        <div class="flag-banner">
            <div style="font-size: 32px;"></div>
            <div>
                <div style="font-size: 16px; font-weight: 800; color: #065f46;">COLD STORAGE MASTER VAULT UNLOCKED!</div>
                <p style="font-size: 13px; color: #047857; margin-top: 4px;">
                    Authentication Successful: Validated as <strong>SecurityOfficer (Marcus Reid)</strong> via forged HMAC-SHA256 credential claims.
                </p>
                <div style="font-size: 12px; color: #047857; margin-top: 8px;">
                    Audit Compliance Proof of Concept (Flag):
                </div>
                <div class="flag-val">{flag}</div>
            </div>
        </div>

        <h3 style="font-size: 15px; font-weight: 800; color: #09090b; margin: 20px 0 10px;">Executive Sovereign Asset Ledger</h3>
        <table>
            <thead>
                <tr>
                    <th>Asset Description</th>
                    <th>Allocation / Balance</th>
                    <th>Custody Address</th>
                    <th>Cold Multisig Quorum</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>Corporate ETH Escrow</td>
                    <td>14,500.00 ETH</td>
                    <td><code>0x71C...882E</code></td>
                    <td><span class="badge badge-active">3 of 5 Hardware HSM</span></td>
                </tr>
                <tr>
                    <td>Sovereign BTC Reserve</td>
                    <td>850.00 BTC</td>
                    <td><code>bc1qa...99zx</code></td>
                    <td><span class="badge badge-active">4 of 7 Hardware HSM</span></td>
                </tr>
            </tbody>
        </table>

        <div style="margin-top: 24px;">
            <a href="/" class="btn-secondary">&larr; Return to Key Ledger</a>
        </div>
    </div>
</div>
</body>
</html>"#,
                css = css(),
                navbar = render_navbar("vault", effective_role.as_deref()),
                flag = FLAG
            );
            return (200, resp_headers, html);
        } else {
            let html = format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Access Restricted // SafeKey Vault</title>
    <style>{css}</style>
</head>
<body>
<div class="app-container">
    {navbar}

    <div class="card" style="text-align: center; padding: 48px 24px; max-width: 680px; margin: 0 auto;">
        <div style="font-size: 38px; margin-bottom: 12px;">️</div>
        <h2 style="font-size: 18px; font-weight: 800; color: #09090b; margin-bottom: 8px;">Master Vault Restricted</h2>
        <p style="font-size: 13px; color: #71717a; margin-bottom: 20px;">
            Access to Marcus Reid's cold storage key seed requires authenticated <code>SecurityOfficer</code> credentials signed with the authorized cryptographic secret.
        </p>

        <div style="background: #fef2f2; border: 1px solid #fecaca; border-radius: 8px; padding: 14px 18px; text-align: left; font-size: 12px; color: #991b1b; margin-bottom: 24px;">
            <strong>Authorization Failure:</strong><br/>
            Current session role: <code>{role_str}</code>.<br/>
            Required role: <code>SecurityOfficer</code>.
        </div>

        <div style="display: flex; justify-content: center; gap: 12px;">
            <a href="/specs" class="btn-secondary">Audit Specs for Secret Key</a>
            <a href="/signer" class="btn-primary">Generate Forged Token</a>
        </div>
    </div>
</div>
</body>
</html>"#,
                css = css(),
                navbar = render_navbar("vault", effective_role.as_deref()),
                role_str = effective_role.as_deref().unwrap_or("Anonymous / Invalid")
            );
            return (403, resp_headers, html);
        }
    }

    // Default 404
    (
        404,
        resp_headers,
        format!(
            "<!DOCTYPE html><html><head><style>{}</style></head><body><div class='app-container'><div class='card' style='text-align:center;'><h2>404 — Endpoint Not Found</h2><p style='margin-top:10px;'><a href='/' class='btn-secondary'>Return to Keys Ledger</a></p></div></div></body></html>",
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
        println!(
            "SafeKey Digital Asset & Cryptographic Custody Portal (Cryptographic Failures) v{}",
            LAB_VERSION
        );
        println!(
            "This native lab communicates with the host engine broker via stdin/stdout JSON IPC."
        );
        println!("Usage: a04-lab.exe");
        return;
    }
    if args
        .iter()
        .any(|a| a == "--version" || a == "-v" || a == "-V")
    {
        println!(
            "A04 v{} (native_sandboxed // SafeKey Custody Vault)",
            LAB_VERSION
        );
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
