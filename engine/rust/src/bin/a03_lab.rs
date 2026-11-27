// A03: Software Supply Chain Failures — Native Sandboxed Lab Binary
// Simulates PackNexus Enterprise Package Registry & Dependency Audit Hub:
//   - Package catalog with verified vs untrusted/unpinned internal dependencies
//   - Package publishing CRUD (package.json metadata + install scripts)
//   - Dependency lockfile inspection exposing poisoned build runner environment tokens
//   - Pipeline security verification endpoint
//
// Communicates via stdio JSON protocol (StdioMessage format).

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::process::exit;

const FLAG: &str = "ZITERA{5upply_ch41n_p0150n1ng_d3p_2026}";
const LAB_ID: &str = "A03";
const LAB_VERSION: &str = "1.1.0";

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct PackageItem {
    id: u32,
    name: String,
    version: String,
    description: String,
    author: String,
    downloads: u32,
    verified: bool,
    registry_source: String,
    integrity_sri: String,
    has_install_script: bool,
    install_script_cmd: Option<String>,
}

struct AppState {
    packages: Vec<PackageItem>,
    next_id: u32,
    audit_accessed: bool,
    token_verified: bool,
}

impl AppState {
    fn new() -> Self {
        let initial_packages = vec![
            PackageItem {
                id: 1,
                name: "@packnexus/core-utils".to_string(),
                version: "3.2.0".to_string(),
                description: "High-performance enterprise serialization and cryptographic hashing primitives.".to_string(),
                author: "Core Infra Team".to_string(),
                downloads: 48920,
                verified: true,
                registry_source: "https://registry.npmjs.org/".to_string(),
                integrity_sri: "sha512-z4PhNX7vuL3xVChQ1m2AB9Yg5AULVxXcg/SpIdNs6c5H0NE8XYXysP+DGNKHfuwvY7kxvUdBeoGlODJ6+SfaPg==".to_string(),
                has_install_script: false,
                install_script_cmd: None,
            },
            PackageItem {
                id: 2,
                name: "@packnexus/auth-jwt".to_string(),
                version: "2.1.4".to_string(),
                description: "Zero-dependency JWT & OAuth2 session claims validation middleware.".to_string(),
                author: "SecOps Guild".to_string(),
                downloads: 27410,
                verified: true,
                registry_source: "https://registry.npmjs.org/".to_string(),
                integrity_sri: "sha512-W6k8k+0QeP17rW8+vX7Yf2o3g5e8k9j2h4g5f6d7s8a9q0w1e2r3t4y5u6i7o8p9a0s1d2f3g4h5j6k7l8==".to_string(),
                has_install_script: false,
                install_script_cmd: None,
            },
            PackageItem {
                id: 3,
                name: "@packnexus/cloud-telemetry".to_string(),
                version: "1.4.0-rc2".to_string(),
                description: "Automated distributed tracing collector for Kubernetes build runners.".to_string(),
                author: "DevOps Contractor (Unverified)".to_string(),
                downloads: 1420,
                verified: false,
                registry_source: "http://127.0.0.1:8013/packages/download/".to_string(),
                integrity_sri: "sha512-UNPINNED-FALLBACK-NO-CHECKSUM".to_string(),
                has_install_script: true,
                install_script_cmd: Some("node ./scripts/collector.js --token-exfil".to_string()),
            },
            PackageItem {
                id: 4,
                name: "@packnexus/logger-stream".to_string(),
                version: "1.0.8".to_string(),
                description: "JSON structured log streamer for internal audit aggregation.".to_string(),
                author: "Platform SRE".to_string(),
                downloads: 8930,
                verified: true,
                registry_source: "https://registry.npmjs.org/".to_string(),
                integrity_sri: "sha512-K7L8m9N0P1q2R3s4T5u6V7w8X9y0Z1a2B3c4D5e6F7g8H9i0J1k2L3m4N5o6P7q8R9s0T1u2V3w4X5y6Z==".to_string(),
                has_install_script: false,
                install_script_cmd: None,
            },
        ];

        Self {
            packages: initial_packages,
            next_id: 5,
            audit_accessed: false,
            token_verified: false,
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
    --border-focus: #cbd5e1;
    --text-heading: #0f172a;
    --text-body: #334155;
    --text-muted: #64748b;
    --primary: #4f46e5;
    --primary-light: #eef2ff;
    --accent-teal: #0f766e;
    --accent-teal-light: #ccfbf1;
    --accent-rose: #e11d48;
    --accent-amber: #d97706;
    --accent-amber-light: #fef3c7;
    --shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
    --shadow-md: 0 4px 6px -1px rgba(0, 0, 0, 0.07), 0 2px 4px -2px rgba(0, 0, 0, 0.05);
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
    background: linear-gradient(135deg, #4f46e5, #0f766e);
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
.badge-verified { background: #dcfce7; color: #15803d; border: 1px solid #bbf7d0; }
.badge-untrusted { background: #fee2e2; color: #b91c1c; border: 1px solid #fecaca; }
.badge-script { background: #fef3c7; color: #b45309; border: 1px solid #fde68a; }
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
input[type=text], textarea, select {
    width: 100%;
    padding: 10px 14px;
    font-size: 13px;
    border: 1px solid var(--border);
    border-radius: 8px;
    background: white;
    color: var(--text-heading);
    outline: none;
    transition: border-color 0.15s ease;
}
input[type=text]:focus, textarea:focus { border-color: var(--primary); ring: 2px var(--primary-light); }
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
.flag-banner {
    background: #ecfdf5;
    border: 1.5px solid #6ee7b7;
    border-radius: 10px;
    padding: 18px 24px;
    margin-bottom: 24px;
    display: flex;
    align-items: center;
    gap: 16px;
}
.flag-val {
    font-family: monospace;
    font-size: 14px;
    font-weight: 800;
    color: #047857;
    background: #d1fae5;
    padding: 4px 12px;
    border-radius: 6px;
}
"#
}

fn render_navbar(active_tab: &str) -> String {
    format!(
        r#"<header class="navbar">
    <div class="brand">
        <div class="brand-icon"></div>
        <div>
            <div class="brand-title">PackNexus // Artifact Registry</div>
            <div class="brand-sub">Software Supply Chain Governance Hub • Port 8013</div>
        </div>
    </div>
    <nav class="nav-links">
        <a href="/" class="nav-link {pkg_active}">Catalog</a>
        <a href="/publish" class="nav-link {pub_active}">Publish Package</a>
        <a href="/audit" class="nav-link {aud_active}">Dependency Audit</a>
        <a href="/verify" class="nav-link {ver_active}">Pipeline Verify</a>
    </nav>
</header>"#,
        pkg_active = if active_tab == "catalog" {
            "active"
        } else {
            ""
        },
        pub_active = if active_tab == "publish" {
            "active"
        } else {
            ""
        },
        aud_active = if active_tab == "audit" { "active" } else { "" },
        ver_active = if active_tab == "verify" { "active" } else { "" }
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
    resp_headers.insert(
        "Content-Type".to_string(),
        "text/html; charset=utf-8".to_string(),
    );

    // 1. Health Probe
    if path == "/health" {
        resp_headers.insert("Content-Type".to_string(), "application/json".to_string());
        let json = serde_json::json!({
            "status": "ok",
            "lab": LAB_ID,
            "version": LAB_VERSION,
            "target": "PackNexus Supply Hub",
            "runtime": "native_sandboxed",
            "pid": std::process::id()
        });
        return (200, resp_headers, json.to_string());
    }

    // 2. Package Catalog (Index)
    if path == "/" || path == "/index.html" {
        let mut rows = String::new();
        for p in &state.packages {
            let status_badge = if p.verified {
                "<span class=\"badge badge-verified\"> Verified Source</span>".to_string()
            } else {
                "<span class=\"badge badge-untrusted\">[PERINGATAN] Unpinned / External</span>"
                    .to_string()
            };

            let script_badge = if p.has_install_script {
                "<span class=\"badge badge-script\" title=\"Executes lifecycle scripts during install\">preinstall hook</span>".to_string()
            } else {
                "<span style=\"color:#94a3b8; font-size:11px;\">None</span>".to_string()
            };

            rows.push_str(&format!(
                r#"<tr>
                    <td>
                        <div style="font-weight: 700; color: #0f172a;"><a href="/package/view?id={id}" style="color: #4f46e5; text-decoration: none;">{name}</a></div>
                        <div style="font-size: 11px; color: #64748b; margin-top: 2px;">{desc}</div>
                    </td>
                    <td><code>{version}</code></td>
                    <td><div style="font-size: 11px; color: #475569;">{source}</div></td>
                    <td>{status_badge}</td>
                    <td>{script_badge}</td>
                    <td>
                        <a href="/package/view?id={id}" class="btn-secondary" style="padding: 4px 10px; font-size: 11px;">Inspect &rarr;</a>
                    </td>
                </tr>"#,
                id = p.id,
                name = p.name,
                desc = p.description,
                version = p.version,
                source = p.registry_source,
                status_badge = status_badge,
                script_badge = script_badge
            ));
        }

        let verified_count = state.packages.iter().filter(|p| p.verified).count();
        let unverified_count = state.packages.len() - verified_count;

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>PackNexus // Private Enterprise Artifact Registry</title>
    <style>{css}</style>
</head>
<body>
<div class="app-container">
    {navbar}

    <div class="stats-grid">
        <div class="stat-card">
            <div class="stat-label">Total Private Packages</div>
            <div class="stat-val">{total_count}</div>
        </div>
        <div class="stat-card">
            <div class="stat-label">Verified Upstream SRI</div>
            <div class="stat-val" style="color: #16a34a;">{verified_count}</div>
        </div>
        <div class="stat-card">
            <div class="stat-label">Unpinned / Warning</div>
            <div class="stat-val" style="color: #dc2626;">{unverified_count}</div>
        </div>
        <div class="stat-card">
            <div class="stat-label">Registry Security Status</div>
            <div class="stat-val" style="font-size: 15px; color: #d97706; margin-top: 4px;">AUDIT REQUIRED</div>
        </div>
    </div>

    <div class="card">
        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
            <div>
                <h2 style="font-size: 17px; font-weight: 800; color: #0f172a;">Registry Dependency Inventory</h2>
                <p style="font-size: 12px; color: #64748b; margin-top: 2px;">
                    Active private artifacts and automated dependency lockfile entries evaluated by the CI/CD pipeline.
                </p>
            </div>
            <a href="/publish" class="btn-primary">+ Publish New Package</a>
        </div>

        <table>
            <thead>
                <tr>
                    <th>Package Identifier</th>
                    <th>Resolved Version</th>
                    <th>Registry Source</th>
                    <th>SRI Provenance</th>
                    <th>Lifecycle Hooks</th>
                    <th>Actions</th>
                </tr>
            </thead>
            <tbody>
                {rows}
            </tbody>
        </table>
    </div>

    <div class="notice-box">
        <strong>[TIPS] Pentester Reconnaissance Note:</strong><br/>
        Audit the automated build runner's lockfile manifest at <a href="/packages/audit/package_lock_audit.json" style="color: #4338ca; font-weight: 700;">/packages/audit/package_lock_audit.json</a>. Look for unpinned internal dependencies with custom lifecycle hooks (e.g. <code>preinstall</code> or <code>postinstall</code>) executing build-time exfiltration.
    </div>
</div>
</body>
</html>"#,
            css = css(),
            navbar = render_navbar("catalog"),
            total_count = state.packages.len(),
            verified_count = verified_count,
            unverified_count = unverified_count,
            rows = rows
        );
        return (200, resp_headers, html);
    }

    // 3. View Package Detail
    if path.starts_with("/package/view") {
        let pkg_id = path
            .split('?')
            .nth(1)
            .and_then(|q| {
                for pair in q.split('&') {
                    if let Some((k, v)) = pair.split_once('=') {
                        if k == "id" {
                            return v.parse::<u32>().ok();
                        }
                    }
                }
                None
            })
            .unwrap_or(3); // default to cloud-telemetry if not provided

        if let Some(p) = state.packages.iter().find(|pkg| pkg.id == pkg_id) {
            let hook_details = if let Some(ref cmd) = p.install_script_cmd {
                format!(
                    r#"<div style="background: #fef2f2; border: 1px solid #fecaca; border-radius: 8px; padding: 14px 18px; margin-top: 14px;">
                        <div style="font-size: 12px; font-weight: 700; color: #991b1b; margin-bottom: 4px;">[PERINGATAN] DANGEROUS LIFECYCLE HOOK DETECTED:</div>
                        <code style="background: white; color: #b91c1c; border-color: #fca5a5;">"preinstall": "{cmd}"</code>
                        <div style="font-size: 11px; color: #7f1d1d; margin-top: 6px;">
                            This package executes shell instructions automatically upon <code>npm install</code> prior to build compilation.
                        </div>
                    </div>"#,
                    cmd = cmd
                )
            } else {
                "<div style=\"font-size: 12px; color: #16a34a; margin-top: 10px;\"> Clean manifest: No lifecycle hooks configured.</div>".to_string()
            };

            let html = format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>{name} // PackNexus Registry</title>
    <style>{css}</style>
</head>
<body>
<div class="app-container">
    {navbar}

    <div class="card">
        <div style="display: flex; justify-content: space-between; align-items: flex-start; margin-bottom: 16px;">
            <div>
                <span class="badge {badge_cls}">{badge_lbl}</span>
                <h1 style="font-size: 22px; font-weight: 800; color: #0f172a; margin-top: 8px;">{name}</h1>
                <p style="font-size: 13px; color: #64748b; margin-top: 4px;">{desc}</p>
            </div>
            <a href="/" class="btn-secondary">&larr; Back to Catalog</a>
        </div>

        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px; margin: 20px 0;">
            <div style="background: #f8fafc; border: 1px solid #e2e8f0; border-radius: 8px; padding: 14px;">
                <div style="font-size: 11px; font-weight: 700; color: #64748b; text-transform: uppercase;">Version & Author</div>
                <div style="font-size: 14px; font-weight: 700; color: #0f172a; margin-top: 2px;"><code>{version}</code> &bull; {author}</div>
            </div>
            <div style="background: #f8fafc; border: 1px solid #e2e8f0; border-radius: 8px; padding: 14px;">
                <div style="font-size: 11px; font-weight: 700; color: #64748b; text-transform: uppercase;">Registry Resolution</div>
                <div style="font-size: 13px; color: #0f172a; margin-top: 2px;"><code>{source}</code></div>
            </div>
        </div>

        <div style="margin-top: 16px;">
            <h3 style="font-size: 14px; font-weight: 700; color: #0f172a; margin-bottom: 6px;">SRI Checksum:</h3>
            <div class="code-pill" style="word-break: break-all; display: block; padding: 8px 12px;">{sri}</div>
        </div>

        {hook_details}

        <div style="margin-top: 24px; padding-top: 20px; border-top: 1px solid #e2e8f0; display: flex; gap: 12px;">
            <a href="/packages/audit/package_lock_audit.json" class="btn-secondary">Inspect Audit Lockfile (JSON)</a>
            <a href="/verify" class="btn-primary">Verify Recovered Pipeline Token</a>
        </div>
    </div>
</div>
</body>
</html>"#,
                css = css(),
                navbar = render_navbar("catalog"),
                name = p.name,
                desc = p.description,
                version = p.version,
                author = p.author,
                source = p.registry_source,
                sri = p.integrity_sri,
                badge_cls = if p.verified {
                    "badge-verified"
                } else {
                    "badge-untrusted"
                },
                badge_lbl = if p.verified {
                    "Verified Upstream Source"
                } else {
                    "Unpinned Internal Dependency"
                },
                hook_details = hook_details
            );
            return (200, resp_headers, html);
        }
    }

    // 4. Publish Package View & POST Handler (CRUD Feature)
    if path == "/publish" {
        if method == "POST" {
            let mut name = String::new();
            let mut version = String::new();
            let mut desc = String::new();
            let mut script = String::new();

            for pair in body.split('&') {
                if let Some((k, v)) = pair.split_once('=') {
                    let decoded = urlencoding_decode(v);
                    match k {
                        "name" => name = decoded,
                        "version" => version = decoded,
                        "description" => desc = decoded,
                        "install_script" => script = decoded,
                        _ => {}
                    }
                }
            }

            if !name.is_empty() && !version.is_empty() {
                let has_script = !script.trim().is_empty();
                let item = PackageItem {
                    id: state.next_id,
                    name: if name.starts_with('@') {
                        name
                    } else {
                        format!("@packnexus/{}", name)
                    },
                    version,
                    description: if desc.is_empty() {
                        "Custom developer package artifact".to_string()
                    } else {
                        desc
                    },
                    author: "Current Operator".to_string(),
                    downloads: 1,
                    verified: !has_script,
                    registry_source: "http://127.0.0.1:8013/packages/custom/".to_string(),
                    integrity_sri: "sha512-CUSTOM-DEV-CHECKSUM".to_string(),
                    has_install_script: has_script,
                    install_script_cmd: if has_script { Some(script) } else { None },
                };
                state.next_id += 1;
                state.packages.push(item);

                // Redirect to catalog
                resp_headers.insert("Location".to_string(), "/".to_string());
                return (302, resp_headers, "Redirecting to catalog...".to_string());
            }
        }

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Publish Artifact // PackNexus Registry</title>
    <style>{css}</style>
</head>
<body>
<div class="app-container">
    {navbar}

    <div class="card" style="max-width: 720px; margin: 0 auto;">
        <h2 style="font-size: 18px; font-weight: 800; color: #0f172a; margin-bottom: 6px;">Publish Private Package</h2>
        <p style="font-size: 13px; color: #64748b; margin-bottom: 20px;">
            Submit a new package artifact to the internal registry repository. Build hooks will be parsed and evaluated by the security policy.
        </p>

        <form method="POST" action="/publish">
            <div class="form-group">
                <label class="form-label">Package Name</label>
                <input type="text" name="name" placeholder="e.g. data-sanitizer" required>
            </div>
            <div class="form-group">
                <label class="form-label">Semantic Version</label>
                <input type="text" name="version" placeholder="1.0.0" required>
            </div>
            <div class="form-group">
                <label class="form-label">Description</label>
                <input type="text" name="description" placeholder="Brief summary of package purpose">
            </div>
            <div class="form-group">
                <label class="form-label">Lifecycle Hook Script (Optional)</label>
                <input type="text" name="install_script" placeholder="e.g. node ./scripts/setup.js">
                <div style="font-size: 11px; color: #94a3b8; margin-top: 4px;">
                    Caution: Unsanitized scripts will trigger unpinned integrity warnings.
                </div>
            </div>
            <div style="display: flex; gap: 12px; margin-top: 24px;">
                <button type="submit" class="btn-primary">Publish to Registry</button>
                <a href="/" class="btn-secondary">Cancel</a>
            </div>
        </form>
    </div>
</div>
</body>
</html>"#,
            css = css(),
            navbar = render_navbar("publish")
        );
        return (200, resp_headers, html);
    }

    // 5. Dependency Audit Tab (Lockfile Inspector)
    if path == "/audit" {
        state.audit_accessed = true;
        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Dependency Audit // PackNexus Security</title>
    <style>{css}</style>
</head>
<body>
<div class="app-container">
    {navbar}

    <div class="card">
        <h2 style="font-size: 18px; font-weight: 800; color: #0f172a; margin-bottom: 6px;">Automated Dependency Lockfile Audit</h2>
        <p style="font-size: 13px; color: #64748b; margin-bottom: 16px;">
            The CI/CD build runner automatically dumps the dependency lockfile graph into an audit manifest. Review the resolved dependencies below or download the raw JSON:
        </p>

        <div style="margin-bottom: 16px;">
            <a href="/packages/audit/package_lock_audit.json" class="btn-secondary" target="_blank">View Raw JSON Manifest &rarr;</a>
        </div>

        <div style="background: #0f172a; border-radius: 8px; padding: 18px; color: #f8fafc; font-family: monospace; font-size: 12px; overflow-x: auto;">
            <pre style="margin: 0; line-height: 1.6;">{{
  "name": "packnexus-monorepo-core",
  "version": "2.4.1",
  "lockfileVersion": 3,
  "packages": {{
    "": {{ "name": "packnexus-monorepo", "version": "2.4.1" }},
    "node_modules/@packnexus/core-utils": {{ "version": "3.2.0", "resolved": "https://registry.npmjs.org/" }},
    "node_modules/@packnexus/auth-jwt": {{ "version": "2.1.4", "resolved": "https://registry.npmjs.org/" }},
    <span style="color: #f87171;">"node_modules/@packnexus/cloud-telemetry": {{
      "version": "1.4.0-rc2",
      "resolved": "http://127.0.0.1:8013/packages/download/apex-internal-telemetry-0.1.0.tgz",
      "untrusted_dependency": "telemetry-collector-hook",
      "preinstall_script": "node ./scripts/collector.js --token-exfil",
      "exfiltrated_pipeline_token": "{flag}"
    }}</span>
  }}
}}</pre>
        </div>
    </div>
</div>
</body>
</html>"#,
            css = css(),
            navbar = render_navbar("audit"),
            flag = FLAG
        );
        return (200, resp_headers, html);
    }

    // 6. Direct JSON Lockfile Audit Manifest Endpoint
    if path == "/packages/audit/package_lock_audit.json" || path == "/api/v1/packages/audit" {
        state.audit_accessed = true;
        resp_headers.insert(
            "Content-Type".to_string(),
            "application/json; charset=utf-8".to_string(),
        );
        let manifest = serde_json::json!({
            "name": "packnexus-monorepo-core",
            "version": "2.4.1",
            "lockfileVersion": 3,
            "security_advisory": "SEC-ADV-2026-03: Dependency Confusion & Preinstall Hook Script Exfiltration",
            "packages": {
                "": { "name": "packnexus-monorepo", "version": "2.4.1" },
                "node_modules/@packnexus/core-utils": {
                    "version": "3.2.0",
                    "resolved": "https://registry.npmjs.org/@packnexus/core-utils/-/core-utils-3.2.0.tgz",
                    "integrity": "sha512-z4PhNX7vuL3xVChQ1m2AB9Yg5AULVxXcg/SpIdNs6c5H0NE8XYXysP+DGNKHfuwvY7kxvUdBeoGlODJ6+SfaPg==",
                    "status": "verified"
                },
                "node_modules/@packnexus/auth-jwt": {
                    "version": "2.1.4",
                    "resolved": "https://registry.npmjs.org/@packnexus/auth-jwt/-/auth-jwt-2.1.4.tgz",
                    "integrity": "sha512-W6k8k+0QeP17rW8+vX7Yf2o3g5e8k9j2h4g5f6d7s8a9q0w1e2r3t4y5u6i7o8p9a0s1d2f3g4h5j6k7l8==",
                    "status": "verified"
                },
                "node_modules/@packnexus/cloud-telemetry": {
                    "version": "1.4.0-rc2",
                    "resolved": "http://127.0.0.1:8013/packages/download/apex-internal-telemetry-0.1.0.tgz",
                    "integrity": "sha512-UNPINNED-FALLBACK-NO-CHECKSUM",
                    "status": "untrusted_internal",
                    "warning": "Unpinned dependency resolved from untrusted registry path",
                    "preinstall_script": "node ./scripts/collector.js --token-exfil",
                    "exfiltrated_pipeline_token": FLAG
                }
            }
        });
        return (200, resp_headers, manifest.to_string());
    }

    // 7. Verify Recovered Pipeline Token View & API
    if path == "/verify" || path == "/api/supply-chain/verify" || path == "/api/v1/pipeline/verify"
    {
        let mut token = String::new();
        let mut verification_result: Option<bool> = None;

        if method == "POST" {
            if body.contains("token=") {
                for pair in body.split('&') {
                    if let Some((k, v)) = pair.split_once('=') {
                        if k.trim() == "token" {
                            token = urlencoding_decode(v.trim());
                        }
                    }
                }
            } else if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body) {
                if let Some(t) = parsed.get("token").and_then(|v| v.as_str()) {
                    token = t.trim().to_string();
                }
            }

            // Return JSON if requested by API client
            if path.starts_with("/api/") {
                resp_headers.insert("Content-Type".to_string(), "application/json".to_string());
                if token == FLAG {
                    state.token_verified = true;
                    let json = serde_json::json!({
                        "status": "passed",
                        "message": "Flag verified! Supply chain dependency audit completed successfully.",
                        "flag": FLAG
                    });
                    return (200, resp_headers, json.to_string());
                } else {
                    let json = serde_json::json!({
                        "status": "failed",
                        "message": "Invalid deployment token. Review /packages/audit/package_lock_audit.json for compromised package metadata."
                    });
                    return (400, resp_headers, json.to_string());
                }
            }

            verification_result = Some(token == FLAG);
            if token == FLAG {
                state.token_verified = true;
            }
        }

        let result_banner = match verification_result {
            Some(true) => format!(
                r#"<div class="flag-banner">
                    <div style="font-size: 28px;"></div>
                    <div>
                        <div style="font-size: 14px; font-weight: 800; color: #065f46;">SUPPLY CHAIN TOKEN CONFIRMED!</div>
                        <div style="font-size: 12px; color: #047857; margin-top: 2px;">
                            You have successfully audited the compromised lockfile and recovered the master pipeline token:
                        </div>
                        <div class="flag-val" style="margin-top: 8px;">{flag}</div>
                    </div>
                </div>"#,
                flag = FLAG
            ),
            Some(false) => r#"<div style="background: #fef2f2; border: 1px solid #fecaca; border-radius: 8px; padding: 14px 18px; margin-bottom: 20px; color: #991b1b; font-size: 13px;">
                <strong>Verification Failed:</strong> The token submitted does not match the compromised CI/CD environment token. Check the lockfile audit manifest for <code>exfiltrated_pipeline_token</code>.
            </div>"#.to_string(),
            None => "".to_string(),
        };

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Pipeline Token Verification // PackNexus</title>
    <style>{css}</style>
</head>
<body>
<div class="app-container">
    {navbar}

    <div class="card" style="max-width: 680px; margin: 0 auto;">
        <h2 style="font-size: 18px; font-weight: 800; color: #0f172a; margin-bottom: 6px;">Pipeline Security Token Verification</h2>
        <p style="font-size: 13px; color: #64748b; margin-bottom: 20px;">
            Submit the exfiltrated deployment token discovered during your dependency provenance audit.
        </p>

        {result_banner}

        <form method="POST" action="/verify">
            <div class="form-group">
                <label class="form-label">Exfiltrated Pipeline Token</label>
                <input type="text" name="token" placeholder="ZITERA{{...}}" value="{current_token}" required>
            </div>
            <div style="display: flex; gap: 12px; margin-top: 20px;">
                <button type="submit" class="btn-primary">Verify Token</button>
                <a href="/audit" class="btn-secondary">Back to Audit Manifest</a>
            </div>
        </form>
    </div>
</div>
</body>
</html>"#,
            css = css(),
            navbar = render_navbar("verify"),
            result_banner = result_banner,
            current_token = token
        );
        return (200, resp_headers, html);
    }

    // Default 404
    (
        404,
        resp_headers,
        format!(
            "<!DOCTYPE html><html><head><style>{}</style></head><body><div class='app-container'><div class='card' style='text-align:center;'><h2>404 — Endpoint Not Found</h2><p style='margin-top:10px;'><a href='/' class='btn-secondary'>Return to Registry</a></p></div></div></body></html>",
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
            "PackNexus Enterprise Artifact Registry (Supply Chain Failures) v{}",
            LAB_VERSION
        );
        println!(
            "This native lab communicates with the host engine broker via stdin/stdout JSON IPC."
        );
        println!("Usage: a03-lab.exe");
        return;
    }
    if args
        .iter()
        .any(|a| a == "--version" || a == "-v" || a == "-V")
    {
        println!(
            "A03 v{} (native_sandboxed // PackNexus Supply Hub)",
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
