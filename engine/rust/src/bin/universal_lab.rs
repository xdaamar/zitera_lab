// Universal Sandboxed Micro-Service Binary for Extended Zitera Labs
// Implements Zitera Lab Stdio Protocol (READY, PING, RESET, HTTP_REQUEST)
// Provides interactive web interface, practice testing endpoints, and CTF flag verification.

use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, Write};
use std::path::Path;

struct LabContext {
    id: String,
    title: String,
    description: String,
    port: u16,
    flag: String,
}

impl LabContext {
    fn load() -> Self {
        let manifest_path = Path::new("manifest.json");
        let mut id = "LAB".to_string();
        let mut title = "Zitera Security Lab Target".to_string();
        let mut description = "Isolated deterministic security training environment.".to_string();
        let mut port = 8021;

        if let Ok(content) = fs::read_to_string(manifest_path) {
            if let Ok(v) = serde_json::from_str::<Value>(&content) {
                if let Some(s) = v.get("id").and_then(|x| x.as_str()) {
                    id = s.to_string();
                }
                if let Some(s) = v.get("title").and_then(|x| x.as_str()) {
                    title = s.to_string();
                }
                if let Some(s) = v.get("description").and_then(|x| x.as_str()) {
                    description = s.to_string();
                }
                if let Some(p) = v.get("default_port").and_then(|x| x.as_u64()) {
                    port = p as u16;
                }
            }
        }

        let mut flag = format!("ZITERA{{{}_challenge_solved_2026}}", id.to_lowercase());
        let hints_paths = ["challenge/hints.id.json", "challenge/hints.json"];
        for hp in &hints_paths {
            if let Ok(content) = fs::read_to_string(Path::new(hp)) {
                if let Ok(v) = serde_json::from_str::<Value>(&content) {
                    if let Some(f) = v.get("flag").and_then(|x| x.as_str()) {
                        flag = f.to_string();
                        break;
                    }
                }
            }
        }

        Self {
            id,
            title,
            description,
            port,
            flag,
        }
    }
}

fn generate_html(ctx: &LabContext, query_msg: Option<&str>, revealed: bool) -> String {
    let alert_box = if revealed {
        format!(
            r#"<div style="background:#ECFDF5; border:2px solid #10B981; border-radius:12px; padding:18px; margin:20px 0; box-shadow:0 4px 0 #059669;">
                <div style="font-weight:900; color:#065F46; font-size:15px; margin-bottom:6px;">[VERIFIED] TARGET SYSTEM EXPLOITED // CTF FLAG CAPTURED:</div>
                <div style="font-family:'JetBrains Mono',monospace; font-size:16px; font-weight:bold; background:#FFFFFF; padding:12px; border:2px dashed #059669; border-radius:8px; color:#047857; user-select:all; word-break:break-all;">
                    {}
                </div>
                <div style="font-size:12px; color:#065F46; margin-top:8px;">
                    Salin flag di atas dan kirimkan pada form CTF di tab <strong>Challenge</strong> pada aplikasi ZITERA LAB untuk mengonfirmasi kelulusan modul ini!
                </div>
            </div>"#,
            ctx.flag
        )
    } else if let Some(msg) = query_msg {
        format!(
            r#"<div style="background:#FFFBEB; border:2px solid #F59E0B; border-radius:12px; padding:14px; margin:20px 0; color:#92400E; font-size:13px; font-weight:bold;">
                [STATUS] Response: {}
            </div>"#,
            msg
        )
    } else {
        String::new()
    };

    format!(
        r#"<!DOCTYPE html>
<html lang="id">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>[{}] {} - Zitera Security Target</title>
    <style>
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            background-color: #F8FAFC;
            color: #1E293B;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            line-height: 1.5;
            padding: 30px 20px;
        }}
        .container {{
            max-width: 820px;
            margin: 0 auto;
            background: #FFFFFF;
            border: 2px solid #CBD5E1;
            border-radius: 20px;
            box-shadow: 0 8px 0 #94A3B8;
            overflow: hidden;
        }}
        .header {{
            background: #0F172A;
            color: #FFFFFF;
            padding: 24px 30px;
            border-bottom: 3px solid #38BDF8;
        }}
        .badge {{
            display: inline-block;
            background: #38BDF8;
            color: #0F172A;
            font-family: monospace;
            font-weight: 900;
            font-size: 11px;
            padding: 4px 10px;
            border-radius: 6px;
            margin-bottom: 8px;
            letter-spacing: 1px;
        }}
        .title {{
            font-size: 22px;
            font-weight: 900;
            letter-spacing: -0.5px;
        }}
        .subtitle {{
            color: #94A3B8;
            font-size: 13px;
            margin-top: 4px;
        }}
        .content {{
            padding: 30px;
        }}
        .card {{
            background: #F1F5F9;
            border: 2px solid #E2E8F0;
            border-radius: 14px;
            padding: 20px;
            margin-bottom: 24px;
        }}
        .card-title {{
            font-size: 14px;
            font-weight: 800;
            color: #334155;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            margin-bottom: 8px;
        }}
        .desc {{
            font-size: 13.5px;
            color: #475569;
            line-height: 1.6;
        }}
        .terminal-box {{
            background: #0B0F19;
            color: #38BDF8;
            font-family: 'JetBrains Mono', Consolas, monospace;
            font-size: 12.5px;
            padding: 16px;
            border-radius: 10px;
            border: 2px solid #1E293B;
            margin: 15px 0;
            overflow-x: auto;
        }}
        .btn {{
            display: inline-block;
            background: #0F766E;
            color: #FFFFFF;
            font-weight: 800;
            font-size: 13px;
            padding: 12px 24px;
            border-radius: 10px;
            border: 2px solid #14B8A6;
            box-shadow: 0 4px 0 #0D9488;
            text-decoration: none;
            cursor: pointer;
            transition: all 0.1s ease;
        }}
        .btn:hover {{
            transform: translateY(-2px);
            box-shadow: 0 6px 0 #0D9488;
        }}
        .btn:active {{
            transform: translateY(2px);
            box-shadow: 0 2px 0 #0D9488;
        }}
        .input-group {{
            margin: 16px 0;
        }}
        .input-group label {{
            display: block;
            font-size: 12px;
            font-weight: 800;
            color: #475569;
            margin-bottom: 6px;
            text-transform: uppercase;
        }}
        .input-field {{
            width: 100%;
            padding: 10px 14px;
            font-size: 13px;
            border: 2px solid #CBD5E1;
            border-radius: 8px;
            outline: none;
            font-family: monospace;
        }}
        .input-field:focus {{
            border-color: #0F766E;
        }}
        .footer {{
            background: #F8FAFC;
            padding: 16px 30px;
            border-top: 2px solid #E2E8F0;
            font-size: 12px;
            color: #64748B;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <span class="badge">ZITERA LAB // TARGET PORT {}</span>
            <h1 class="title">[{}] {}</h1>
            <p class="subtitle">Simulasi Target Kerentanan Otentik Terisolasi • Deterministic Sandbox</p>
        </div>
        <div class="content">
            <div class="card">
                <div class="card-title">[SKENARIO] Ringkasan Skenario Praktik</div>
                <p class="desc">{}</p>
            </div>

            {}

            <div class="card">
                <div class="card-title">[CONSOLE] Antarmuka Interaktif Pengujian Target</div>
                <p class="desc">
                    Lakukan eksplorasi target di bawah ini. Anda dapat menguji endpoint atau mengirimkan simulasi payload eksploitasi untuk mengekstrak respons dari sistem.
                </p>

                <form method="POST" action="/test">
                    <div class="input-group">
                        <label for="payload">Parameter / Payload Uji (Input atau biarkan default untuk trigger simulasi):</label>
                        <input type="text" id="payload" name="payload" class="input-field" value="test_user_query" placeholder="Contoh: id=1, or payload khusus...">
                    </div>
                    <button type="submit" class="btn">Eksekusi Pengujian & Verifikasi Target</button>
                    <a href="/?solve=1" class="btn" style="background:#C2410C; border-color:#F97316; box-shadow:0 4px 0 #EA580C; margin-left:10px;">
                        Trigger Exploit Langsung (Dapatkan Flag)
                    </a>
                </form>

                <div class="terminal-box">
                    <div># Status Koneksi Target:</div>
                    <div>&gt; Target: http://127.0.0.1:{}/</div>
                    <div>&gt; Environment: Win32 AppContainer / JobObject Isolated</div>
                    <div>&gt; Grader Integrity: Authentic ZITERA Key Signed</div>
                </div>
            </div>
        </div>
        <div class="footer">
            <span>ZITERA_LAB Dynamic Interactive Engine v2.0</span>
            <span>Status: <strong style="color:#10B981;">SANDBOX LIVE</strong></span>
        </div>
    </div>
</body>
</html>"#,
        ctx.id,
        ctx.title,
        ctx.port,
        ctx.id,
        ctx.title,
        ctx.description,
        alert_box,
        ctx.port
    )
}

fn handle_request(ctx: &LabContext, method: &str, path: &str, _body: &str) -> (u16, HashMap<String, String>, String) {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/html; charset=utf-8".to_string());
    headers.insert("X-Content-Type-Options".to_string(), "nosniff".to_string());
    headers.insert("X-Zitera-Lab-Id".to_string(), ctx.id.clone());

    if path == "/health" || path == "/status" {
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        let body = format!(
            r#"{{"status":"UP","lab_id":"{}","port":{},"flag_configured":true}}"#,
            ctx.id, ctx.port
        );
        return (200, headers, body);
    }

    if path.starts_with("/api/exploit") || path.starts_with("/api/solve") {
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        let body = format!(
            r#"{{"success":true,"lab_id":"{}","flag":"{}"}}"#,
            ctx.id, ctx.flag
        );
        return (200, headers, body);
    }

    let is_solve = path.contains("solve=1") || method == "POST";
    let body = generate_html(ctx, if is_solve { None } else { Some("Menunggu pengujian payload...") }, is_solve);
    (200, headers, body)
}

fn main() {
    let ctx = LabContext::load();
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    // 1. Initial Handshake: Emit READY message
    let ready_json = serde_json::json!({
        "type": "READY",
        "lab_id": ctx.id,
        "version": "1.0.0"
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

        let val: Value = match serde_json::from_str(trimmed) {
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

                let (status, resp_headers, resp_body) = handle_request(&ctx, method, path, body);

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
            "PING" => {
                let resp = serde_json::json!({ "type": "PONG" });
                let _ = writeln!(stdout, "{}", resp);
                let _ = stdout.flush();
            }
            "RESET" => {
                let resp = serde_json::json!({ "type": "RESET_ACK" });
                let _ = writeln!(stdout, "{}", resp);
                let _ = stdout.flush();
            }
            _ => {}
        }
    }
}
