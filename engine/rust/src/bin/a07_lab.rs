// A07: Authentication Failures — NexHealth Hospital Medical Records Portal
// Commercial Healthcare EHR & Clinical Staff Gateway
// Simulates an enterprise medical portal with intentional authentication vulnerabilities:
//   - Zero rate-limiting, progressive delay, or lockout protection on staff credentials
//   - Weak predictable PINs for medical staff (Dr. Marcus Reid / admin: 2026)
//   - Full interactive patient medical records CRUD
//   - Confidential Chief Medical Officer clinical research vault revealing the flag
//
// Communicates via stdio JSON protocol (StdioMessage format).

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::process::exit;

const FLAG: &str = "ZITERA{4uth_f41lur3_brut3_f0rc3_2026}";
const LAB_ID: &str = "A07";
const LAB_VERSION: &str = "2.0.0";
const ADMIN_USER: &str = "admin";
const DOCTOR_USER: &str = "mreid";
const ADMIN_PIN: &str = "2026";

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct PatientRecord {
    id: u32,
    patient_name: String,
    room: String,
    triage: String,
    physician: String,
    diagnosis: String,
    status: String,
}

struct AppState {
    sessions: HashMap<String, String>, // token -> username
    records: Vec<PatientRecord>,
    attempt_count: u32,
    brute_force_detected: bool,
    admin_unlocked: bool,
}

impl AppState {
    fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            records: vec![
                PatientRecord {
                    id: 101,
                    patient_name: "Jonathan Vance".to_string(),
                    room: "ICU-304".to_string(),
                    triage: "High".to_string(),
                    physician: "Dr. Sarah Chen".to_string(),
                    diagnosis: "Acute post-operative cardiovascular monitoring".to_string(),
                    status: "Admitted".to_string(),
                },
                PatientRecord {
                    id: 102,
                    patient_name: "Eleanor Sterling".to_string(),
                    room: "Ward-B12".to_string(),
                    triage: "Moderate".to_string(),
                    physician: "Dr. Emily Stone".to_string(),
                    diagnosis: "Type 2 diabetic ketoacidosis stabilization".to_string(),
                    status: "Stable".to_string(),
                },
                PatientRecord {
                    id: 103,
                    patient_name: "Alexander Wright".to_string(),
                    room: "Suite-501".to_string(),
                    triage: "Low".to_string(),
                    physician: "Dr. Marcus Reid".to_string(),
                    diagnosis: "Routine executive neurological screening".to_string(),
                    status: "Observation".to_string(),
                },
            ],
            attempt_count: 0,
            brute_force_detected: false,
            admin_unlocked: false,
        }
    }
}

fn css() -> &'static str {
    r#"
:root {
    --bg-page: #f8fafc;
    --card-bg: #ffffff;
    --text-primary: #0f172a;
    --text-secondary: #475569;
    --text-muted: #94a3b8;
    --border-color: #e2e8f0;
    --border-hover: #cbd5e1;
    --accent: #0d9488;
    --accent-hover: #0f766e;
    --accent-light: #f0fdfa;
    --success: #10b981;
    --success-light: #ecfdf5;
    --danger: #ef4444;
    --danger-light: #fef2f2;
    --warning: #f59e0b;
    --warning-light: #fffbeb;
}
* { box-sizing: border-box; margin: 0; padding: 0; }
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
    background-color: var(--bg-page);
    color: var(--text-primary);
    line-height: 1.5;
    padding: 32px 20px;
}
.container { max-width: 1000px; margin: 0 auto; }
.card {
    background: var(--card-bg);
    border: 1px solid var(--border-color);
    border-radius: 12px;
    box-shadow: 0 1px 3px rgba(0,0,0,0.04), 0 1px 2px rgba(0,0,0,0.02);
    padding: 24px 28px;
    margin-bottom: 24px;
}
.header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid var(--border-color);
    padding-bottom: 20px;
    margin-bottom: 24px;
}
.header-title h1 { font-size: 20px; font-weight: 700; color: var(--text-primary); display: flex; align-items: center; gap: 8px; }
.header-title p { font-size: 13px; color: var(--text-secondary); margin-top: 4px; }
.badge {
    display: inline-flex;
    align-items: center;
    padding: 4px 10px;
    font-size: 12px;
    font-weight: 600;
    border-radius: 9999px;
}
.badge-teal { background: var(--accent-light); color: var(--accent); border: 1px solid #ccfbf1; }
.badge-emerald { background: var(--success-light); color: var(--success); border: 1px solid #a7f3d0; }
.badge-amber { background: var(--warning-light); color: #d97706; border: 1px solid #fde68a; }
.badge-rose { background: var(--danger-light); color: var(--danger); border: 1px solid #fecaca; }
.banner-info {
    background: #f0fdfa;
    border-left: 4px solid var(--accent);
    padding: 14px 18px;
    border-radius: 6px;
    font-size: 13px;
    color: #134e4a;
    margin-bottom: 24px;
    line-height: 1.6;
}
.banner-danger {
    background: var(--danger-light);
    border-left: 4px solid var(--danger);
    padding: 14px 18px;
    border-radius: 6px;
    font-size: 13px;
    color: #991b1b;
    margin-bottom: 24px;
}
.grid-2 { display: grid; grid-template-columns: 2fr 1fr; gap: 24px; }
@media (max-width: 800px) { .grid-2 { grid-template-columns: 1fr; } }
.table-wrap { overflow-x: auto; border: 1px solid var(--border-color); border-radius: 8px; }
table { width: 100%; border-collapse: collapse; text-align: left; font-size: 13px; }
th { background: #f1f5f9; padding: 12px 16px; font-weight: 600; color: var(--text-secondary); border-bottom: 1px solid var(--border-color); }
td { padding: 12px 16px; border-bottom: 1px solid var(--border-color); color: var(--text-primary); }
tr:last-child td { border-bottom: none; }
tr:hover td { background: #f8fafc; }
.field { margin-bottom: 16px; }
label { display: block; font-size: 12px; font-weight: 600; color: var(--text-secondary); margin-bottom: 6px; }
input[type="text"], input[type="password"], select, textarea {
    width: 100%;
    padding: 10px 12px;
    background: #ffffff;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    font-size: 13px;
    color: var(--text-primary);
    outline: none;
    transition: border-color 0.15s ease;
}
input:focus, select:focus, textarea:focus { border-color: var(--accent); ring: 2px solid #ccfbf1; }
.btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 10px 16px;
    font-size: 13px;
    font-weight: 600;
    border-radius: 6px;
    cursor: pointer;
    border: none;
    text-decoration: none;
    transition: background-color 0.15s ease;
}
.btn-primary { background: var(--accent); color: white; }
.btn-primary:hover { background: var(--accent-hover); }
.btn-secondary { background: #f1f5f9; color: var(--text-secondary); border: 1px solid var(--border-color); }
.btn-secondary:hover { background: #e2e8f0; }
.btn-danger { background: var(--danger); color: white; }
.btn-danger:hover { background: #dc2626; }
.flag-box {
    background: #ecfdf5;
    border: 2px dashed #10b981;
    border-radius: 8px;
    padding: 18px;
    margin: 20px 0;
    color: #065f46;
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 15px;
    font-weight: 700;
    text-align: center;
}
.stat-box {
    background: #f8fafc;
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 14px 16px;
    margin-bottom: 12px;
}
.stat-title { font-size: 11px; text-transform: uppercase; letter-spacing: 0.05em; color: var(--text-muted); font-weight: 700; }
.stat-value { font-size: 18px; font-weight: 700; color: var(--text-primary); margin-top: 4px; }
"#
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn parse_form_body(body: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in body.split('&') {
        let mut parts = pair.splitn(2, '=');
        if let Some(key) = parts.next() {
            let val = parts.next().unwrap_or("");
            map.insert(urldecode(key), urldecode(val));
        }
    }
    map
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

fn extract_cookie(headers: &HashMap<String, String>, cookie_name: &str) -> Option<String> {
    let cookie_hdr = headers.get("cookie")?;
    for part in cookie_hdr.split(';') {
        let mut kv = part.splitn(2, '=');
        if let Some(name) = kv.next() {
            if name.trim().eq_ignore_ascii_case(cookie_name) {
                return kv.next().map(|v| v.trim().to_string());
            }
        }
    }
    None
}

fn render_portal(state: &AppState, user: Option<&str>, error: Option<&str>, search_q: Option<&str>) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\">");
    html.push_str("<title>NexHealth Portal — Electronic Health Records</title>");
    html.push_str("<style>");
    html.push_str(css());
    html.push_str("</style></head><body><div class=\"container\">");

    // Header
    html.push_str("<div class=\"card\"><div class=\"header\">");
    html.push_str("<div class=\"header-title\"><h1>[CLINICAL] NexHealth Medical EHR</h1>");
    html.push_str("<p>Hospital Clinical Records & Medical Staff Authorization Gateway (OWASP A07:2025)</p></div>");
    html.push_str("<div style=\"display: flex; gap: 8px; align-items: center;\">");
    if let Some(u) = user {
        html.push_str(&format!("<span class=\"badge badge-emerald\">[USER] Signed in: {}</span>", html_escape(u)));
        if u == ADMIN_USER || u == DOCTOR_USER {
            html.push_str("<a href=\"/records/vault\" class=\"btn btn-primary\" style=\"font-size: 12px; padding: 6px 12px;\">Chief Officer's Vault</a>");
        }
        html.push_str("<a href=\"/logout\" class=\"btn btn-secondary\" style=\"font-size: 12px; padding: 6px 12px;\">Log Out</a>");
    } else {
        html.push_str("<span class=\"badge badge-amber\"> Staff Authentication Required</span>");
        html.push_str("<a href=\"/login\" class=\"btn btn-primary\" style=\"font-size: 12px; padding: 6px 12px;\">Staff Sign In</a>");
    }
    html.push_str("</div></div>");

    // Notice banner
    html.push_str("<div class=\"banner-info\">");
    html.push_str("<strong>CLINICAL AUDIT DIRECTIVE:</strong> NexHealth manages emergency health records and clinical prescriptions. ");
    html.push_str("Hospital management recently migrated to a streamlined 4-digit PIN authentication protocol for all physicians. ");
    html.push_str("Notice the staff onboarding memo: all newly provisioned physician PINs (including Dr. Marcus Reid) default to the hospital's annual deployment cycle. ");
    html.push_str("Test the portal's rate-limiting policy to identify brute-force vulnerabilities.</div>");

    if let Some(err) = error {
        html.push_str(&format!("<div class=\"banner-danger\">[PERINGATAN] {}</div>", html_escape(err)));
    }

    // Grid layout
    html.push_str("<div class=\"grid-2\">");

    // Left Column: Patient Records
    html.push_str("<div>");
    html.push_str("<div style=\"display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;\">");
    html.push_str("<h2 style=\"font-size: 16px; font-weight: 700;\">Active Inpatient Roster</h2>");
    html.push_str("<form method=\"GET\" action=\"/\" style=\"display: flex; gap: 6px;\">");
    html.push_str(&format!("<input type=\"text\" name=\"q\" placeholder=\"Search patient or doctor...\" value=\"{}\" style=\"padding: 6px 10px; width: 180px;\">", html_escape(search_q.unwrap_or(""))));
    html.push_str("<button type=\"submit\" class=\"btn btn-secondary\" style=\"padding: 6px 10px; font-size: 12px;\">Filter</button>");
    html.push_str("</form></div>");

    html.push_str("<div class=\"table-wrap\"><table><thead><tr>");
    html.push_str("<th>ID</th><th>Patient Name</th><th>Room</th><th>Attending Physician</th><th>Triage</th><th>Status</th>");
    html.push_str("</tr></thead><tbody>");

    let filtered: Vec<&PatientRecord> = if let Some(q) = search_q {
        let q_lower = q.to_lowercase();
        state.records.iter().filter(|r| {
            r.patient_name.to_lowercase().contains(&q_lower) ||
            r.physician.to_lowercase().contains(&q_lower) ||
            r.diagnosis.to_lowercase().contains(&q_lower)
        }).collect()
    } else {
        state.records.iter().collect()
    };

    if filtered.is_empty() {
        html.push_str("<tr><td colspan=\"6\" style=\"text-align: center; color: var(--text-muted); padding: 20px;\">No clinical records found matching criteria.</td></tr>");
    } else {
        for r in filtered {
            let badge_class = match r.triage.as_str() {
                "High" => "badge-rose",
                "Moderate" => "badge-amber",
                _ => "badge-teal",
            };
            html.push_str("<tr>");
            html.push_str(&format!("<td><strong>#{}</strong></td>", r.id));
            html.push_str(&format!("<td><strong>{}</strong><br/><span style=\"font-size: 11px; color: var(--text-secondary);\">{}</span></td>", html_escape(&r.patient_name), html_escape(&r.diagnosis)));
            html.push_str(&format!("<td><code>{}</code></td>", html_escape(&r.room)));
            html.push_str(&format!("<td>{}</td>", html_escape(&r.physician)));
            html.push_str(&format!("<td><span class=\"badge {}\">{}</span></td>", badge_class, html_escape(&r.triage)));
            html.push_str(&format!("<td><span class=\"badge badge-emerald\">{}</span></td>", html_escape(&r.status)));
            html.push_str("</tr>");
        }
    }
    html.push_str("</tbody></table></div>");

    // Add Patient Form
    html.push_str("<div style=\"margin-top: 24px; padding-top: 20px; border-top: 1px solid var(--border-color);\">");
    html.push_str("<h3 style=\"font-size: 14px; font-weight: 700; margin-bottom: 12px;\">Admit New Clinical Patient</h3>");
    html.push_str("<form method=\"POST\" action=\"/records/new\">");
    html.push_str("<div style=\"display: grid; grid-template-columns: 1fr 1fr; gap: 12px;\">");
    html.push_str("<div class=\"field\"><label>Patient Full Name</label><input type=\"text\" name=\"patient_name\" placeholder=\"e.g. Margaret Taylor\" required></div>");
    html.push_str("<div class=\"field\"><label>Assigned Room</label><input type=\"text\" name=\"room\" placeholder=\"e.g. Ward-C08\" required></div>");
    html.push_str("</div>");
    html.push_str("<div style=\"display: grid; grid-template-columns: 1fr 1fr; gap: 12px;\">");
    html.push_str("<div class=\"field\"><label>Attending Physician</label><input type=\"text\" name=\"physician\" placeholder=\"e.g. Dr. Sarah Chen\" required></div>");
    html.push_str("<div class=\"field\"><label>Triage Priority</label><select name=\"triage\"><option value=\"Low\">Low - Standard Observation</option><option value=\"Moderate\">Moderate - Routine Treatment</option><option value=\"High\">High - Critical Monitoring</option></select></div>");
    html.push_str("</div>");
    html.push_str("<div class=\"field\"><label>Clinical Diagnosis & Treatment Plan</label><input type=\"text\" name=\"diagnosis\" placeholder=\"e.g. Bilateral pneumonia therapy\" required></div>");
    html.push_str("<button type=\"submit\" class=\"btn btn-primary\" style=\"width: 100%;\">Create Patient Record</button>");
    html.push_str("</form></div>");
    html.push_str("</div>");

    // Right Column: Staff Directory & Security Status
    html.push_str("<div>");
    html.push_str("<div class=\"stat-box\">");
    html.push_str("<div class=\"stat-title\">Attacking Vector</div>");
    html.push_str("<div class=\"stat-value\" style=\"color: var(--accent); font-size: 14px;\">Zero Rate-Limiting / Weak PIN</div>");
    html.push_str(&format!("<div style=\"font-size: 12px; color: var(--text-secondary); margin-top: 6px;\">Recorded Login Attempts: <strong>{}</strong></div>", state.attempt_count));
    html.push_str("</div>");

    html.push_str("<div class=\"card\" style=\"padding: 16px; margin-bottom: 16px;\">");
    html.push_str("<h3 style=\"font-size: 13px; font-weight: 700; color: var(--text-primary); margin-bottom: 8px;\">[CHECKLIST] Clinical Staff Directory</h3>");
    html.push_str("<ul style=\"list-style: none; font-size: 12px; color: var(--text-secondary); line-height: 1.8;\">");
    html.push_str("<li>• <strong>Dr. Marcus Reid</strong> (<code>admin</code> / <code>mreid</code>)<br/><span style=\"color: var(--text-muted); font-size: 11px;\">Chief Medical Officer & Clinical Director</span></li>");
    html.push_str("<li>• <strong>Dr. Sarah Chen</strong> (<code>schen</code>)<br/><span style=\"color: var(--text-muted); font-size: 11px;\">Cardiovascular Surgery Lead</span></li>");
    html.push_str("<li>• <strong>Nurse Joy Vance</strong> (<code>nurse_joy</code>)<br/><span style=\"color: var(--text-muted); font-size: 11px;\">Triage Coordination Unit (PIN: <code>1122</code>)</span></li>");
    html.push_str("</ul>");
    html.push_str("</div>");

    html.push_str("<div class=\"card\" style=\"padding: 16px; background: #fffbeb; border-color: #fde68a;\">");
    html.push_str("<h3 style=\"font-size: 12px; font-weight: 700; color: #b45309; margin-bottom: 6px;\">[NOTE] Staff Memo #2026-IT</h3>");
    html.push_str("<p style=\"font-size: 11px; color: #92400e; line-height: 1.5;\">\"All physicians must use their 4-digit setup PIN on the first login. New system rollout credentials match the deployment year (<strong>2026</strong>). No lockouts are configured to avoid blocking emergency doctors during shifts.\"</p>");
    html.push_str("</div>");

    html.push_str("</div>"); // End Right Column
    html.push_str("</div>"); // End Grid
    html.push_str("</div></div></body></html>");
    html
}

fn render_login_page(state: &AppState, error: Option<&str>) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\">");
    html.push_str("<title>Staff Authentication — NexHealth EHR</title>");
    html.push_str("<style>");
    html.push_str(css());
    html.push_str(".login-card { max-width: 440px; margin: 40px auto; }");
    html.push_str("</style></head><body><div class=\"container\">");
    html.push_str("<div class=\"card login-card\">");
    html.push_str("<div style=\"text-align: center; margin-bottom: 24px;\">");
    html.push_str("<div style=\"font-size: 32px;\"></div>");
    html.push_str("<h1 style=\"font-size: 20px; font-weight: 700; color: var(--text-primary); margin-top: 8px;\">NexHealth Staff Gateway</h1>");
    html.push_str("<p style=\"font-size: 13px; color: var(--text-secondary); margin-top: 4px;\">Clinical Staff Electronic Health Portal (Port 8017)</p>");
    html.push_str("</div>");

    if let Some(err) = error {
        html.push_str(&format!("<div class=\"banner-danger\" style=\"padding: 10px 14px; font-size: 12px;\">[PERINGATAN] {}</div>", html_escape(err)));
    }

    html.push_str("<form method=\"POST\" action=\"/login\">");
    html.push_str("<div class=\"field\"><label>Staff Identifier / Username</label>");
    html.push_str("<input type=\"text\" name=\"username\" placeholder=\"admin, mreid, or nurse_joy\" value=\"admin\" required autofocus></div>");
    html.push_str("<div class=\"field\"><label>Clinical Security PIN (4 Digits)</label>");
    html.push_str("<input type=\"password\" name=\"password\" placeholder=\"e.g. 2026\" required></div>");
    html.push_str("<button type=\"submit\" class=\"btn btn-primary\" style=\"width: 100%; padding: 12px;\">Authenticate to EHR</button>");
    html.push_str("</form>");

    html.push_str("<div style=\"margin-top: 20px; padding-top: 16px; border-top: 1px solid var(--border-color); font-size: 12px; color: var(--text-muted); text-align: center;\">");
    html.push_str(&format!("Login attempts without rate-limit: <strong>{}</strong><br/>", state.attempt_count));
    html.push_str("<a href=\"/\" style=\"color: var(--accent); text-decoration: none;\">← Return to Inpatient Directory</a>");
    html.push_str("</div>");

    html.push_str("</div></div></body></html>");
    html
}

fn render_vault_page(username: &str) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\">");
    html.push_str("<title>Chief Officer's Medical Vault — NexHealth</title>");
    html.push_str("<style>");
    html.push_str(css());
    html.push_str("</style></head><body><div class=\"container\"><div class=\"card\">");
    html.push_str("<div class=\"header\"><div class=\"header-title\">");
    html.push_str("<h1> Chief Medical Officer // Confidential Clinical Research</h1>");
    html.push_str(&format!("<p>Dr. Marcus Reid's Classified Research Repository (Authenticated as: {})</p></div>", html_escape(username)));
    html.push_str("<a href=\"/\" class=\"btn btn-secondary\" style=\"font-size: 12px;\">← Return to Clinical Portal</a>");
    html.push_str("</div>");

    html.push_str("<div class=\"banner-info\" style=\"background: #ecfdf5; border-color: var(--success); color: #065f46;\">");
    html.push_str("<strong>AUTHORIZATION COMPLETED:</strong> You successfully exploited authentication deficiencies (lack of lockout and predictable 4-digit PIN) to gain access to Dr. Marcus Reid's executive medical research vault.</div>");

    html.push_str("<div class=\"flag-box\">");
    html.push_str("<div style=\"font-size: 12px; text-transform: uppercase; letter-spacing: 0.05em; margin-bottom: 6px;\">CTF FLAG VERIFIED:</div>");
    html.push_str(&format!("<div>{}</div>", FLAG));
    html.push_str("</div>");

    html.push_str("<div class=\"card\" style=\"background: #f8fafc; border-color: var(--border-color); padding: 18px;\">");
    html.push_str("<h3 style=\"font-size: 14px; font-weight: 700; margin-bottom: 8px;\">Classified Protocol: Project Aegis-Bio Q1</h3>");
    html.push_str("<p style=\"font-size: 13px; color: var(--text-secondary); line-height: 1.6;\">Clinical Trial Dossier: Experimental gene therapy and targeted oncological vector trials. All patient participants anonymized under ISO-27799 / HIPAA compliance standards.</p>");
    html.push_str("</div>");

    html.push_str("</div></div></body></html>");
    html
}

fn handle_request(
    state: &mut AppState,
    method: &str,
    path: &str,
    headers: &HashMap<String, String>,
    body: &str,
) -> (u16, HashMap<String, String>, String) {
    let mut resp_headers = HashMap::new();
    let clean_path = path.split('?').next().unwrap_or("/");

    let query_map: HashMap<String, String> = if let Some(idx) = path.find('?') {
        parse_form_body(&path[idx + 1..])
    } else {
        HashMap::new()
    };

    if clean_path == "/health" {
        resp_headers.insert("content-type".to_string(), "application/json".to_string());
        let body = serde_json::json!({
            "status": "ok",
            "lab": LAB_ID,
            "port": 8017
        })
        .to_string();
        return (200, resp_headers, body);
    }

    let auth_user = extract_cookie(headers, "nexhealth_session")
        .or_else(|| extract_cookie(headers, "auth_session"))
        .and_then(|tok| state.sessions.get(&tok).cloned());

    if clean_path == "/" {
        resp_headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
        let q = query_map.get("q").map(|s| s.as_str());
        return (200, resp_headers, render_portal(state, auth_user.as_deref(), None, q));
    }

    if clean_path == "/records/new" && method == "POST" {
        let form = parse_form_body(body);
        let name = form.get("patient_name").cloned().unwrap_or_else(|| "Unknown".to_string());
        let room = form.get("room").cloned().unwrap_or_else(|| "Unassigned".to_string());
        let physician = form.get("physician").cloned().unwrap_or_else(|| "Dr. On Call".to_string());
        let triage = form.get("triage").cloned().unwrap_or_else(|| "Moderate".to_string());
        let diagnosis = form.get("diagnosis").cloned().unwrap_or_else(|| "General consultation".to_string());

        let next_id = (state.records.len() as u32) + 101;
        state.records.push(PatientRecord {
            id: next_id,
            patient_name: name,
            room,
            triage,
            physician,
            diagnosis,
            status: "Admitted".to_string(),
        });

        resp_headers.insert("Location".to_string(), "/".to_string());
        return (302, resp_headers, String::new());
    }

    if clean_path == "/login" {
        if method == "POST" {
            state.attempt_count += 1;
            let form = parse_form_body(body);
            let user = form.get("username").map(|s| s.trim()).unwrap_or("");
            let pw = form.get("password").map(|s| s.trim()).unwrap_or("");

            if state.attempt_count >= 3 {
                state.brute_force_detected = true;
            }

            if (user == ADMIN_USER || user == DOCTOR_USER) && pw == ADMIN_PIN {
                let token = format!("nex_sess_{}_{}", state.attempt_count, 2026);
                state.sessions.insert(token.clone(), user.to_string());
                state.admin_unlocked = true;
                resp_headers.insert("Set-Cookie".to_string(), format!("auth_session={}; nexhealth_session={}; Path=/", token, token));
                resp_headers.insert("Location".to_string(), "/records/vault".to_string());
                return (302, resp_headers, String::new());
            } else if user == "nurse_joy" && pw == "1122" {
                let token = format!("nex_sess_nurse_{}", state.attempt_count);
                state.sessions.insert(token.clone(), "nurse_joy".to_string());
                resp_headers.insert("Set-Cookie".to_string(), format!("auth_session={}; nexhealth_session={}; Path=/", token, token));
                resp_headers.insert("Location".to_string(), "/".to_string());
                return (302, resp_headers, String::new());
            }

            resp_headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
            let html = render_login_page(state, Some("Invalid credentials. Please verify your clinical security PIN or staff username."));
            return (200, resp_headers, html);
        }

        resp_headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
        return (200, resp_headers, render_login_page(state, None));
    }

    if clean_path == "/api/login" && method == "POST" {
        state.attempt_count += 1;
        let (user, pw) = if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(body) {
            let u = json_val.get("username").and_then(|v| v.as_str()).unwrap_or("").trim().to_string();
            let p = json_val.get("password").and_then(|v| v.as_str()).unwrap_or("").trim().to_string();
            (u, p)
        } else {
            let form = parse_form_body(body);
            let u = form.get("username").map(|s| s.trim()).unwrap_or("").to_string();
            let p = form.get("password").map(|s| s.trim()).unwrap_or("").to_string();
            (u, p)
        };

        if state.attempt_count >= 3 {
            state.brute_force_detected = true;
        }

        resp_headers.insert("content-type".to_string(), "application/json".to_string());
        if (user == ADMIN_USER || user == DOCTOR_USER) && pw == ADMIN_PIN {
            state.admin_unlocked = true;
            let token = format!("nex_token_admin_{}", state.attempt_count);
            state.sessions.insert(token.clone(), user.clone());
            resp_headers.insert("Set-Cookie".to_string(), format!("auth_session={}; nexhealth_session={}; Path=/", token, token));
            let resp = serde_json::json!({
                "status": "passed",
                "message": "Authentication successful as Chief Medical Officer!",
                "user": user,
                "token": token,
                "flag": FLAG
            });
            return (200, resp_headers, resp.to_string());
        }

        return (401, resp_headers, serde_json::json!({
            "status": "failed",
            "error": "Invalid credentials",
            "attempt": state.attempt_count
        }).to_string());
    }

    if clean_path == "/records/vault" || clean_path == "/admin" {
        let has_cookie_auth = headers.iter().any(|(k, v)| k.to_lowercase() == "cookie" && v.contains("auth_session="));
        if auth_user.is_some() || has_cookie_auth {
            resp_headers.insert("content-type".to_string(), "text/html; charset=utf-8".to_string());
            return (200, resp_headers, render_vault_page("admin"));
        }
        resp_headers.insert("Location".to_string(), "/login".to_string());
        return (302, resp_headers, String::new());
    }

    if clean_path == "/logout" {
        resp_headers.insert("Set-Cookie".to_string(), "nexhealth_session=; Path=/; Max-Age=0".to_string());
        resp_headers.insert("Location".to_string(), "/".to_string());
        return (302, resp_headers, String::new());
    }

    (404, resp_headers, "404 Not Found".to_string())
}

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut state = AppState::new();

    // 1. Initial READY handshake
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
                let success = state.admin_unlocked || state.brute_force_detected || state.attempt_count > 0;
                let message = if state.admin_unlocked {
                    "Practice verified: You authenticated to the administrative portal using weak credential brute-force!"
                } else if state.brute_force_detected {
                    "Practice verified: Rapid credential attempts observed without rate limiting or account lockout."
                } else if state.attempt_count > 0 {
                    "Practice partially verified: Login attempt recorded. Test brute-forcing weak 4-digit PINs."
                } else {
                    "Practice incomplete: Submit credentials to /login or /api/login to evaluate rate-limiting."
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
                        "Correct! Authentication failures challenge complete."
                    } else {
                        "Incorrect flag. Hint: Authenticate as admin or mreid with the 4-digit onboarding PIN to unlock the vault."
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
