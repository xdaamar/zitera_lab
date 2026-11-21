// ZITERA CYBER_LAB — Turso Database Native Security Enclave
// Securely executes LibSQL queries inside compiled native enclave with XOR-obfuscated credentials.
// Completely decouples database authentication from the client frontend bundle.

use serde::{Deserialize, Serialize};
use std::io::Write;
use std::process::{Command, Stdio};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

// Obfuscated cryptographic keys & pipeline endpoints (Rolling XOR)
const SECRET_KEY: &[u8] = &[90, 49, 116, 51, 114, 65, 95, 83, 51, 99, 85, 114, 49, 116, 89, 95, 69, 110, 67, 49, 52, 118, 51, 95, 50, 48, 50, 54, 33];
const ENC_TURSO_URL: &[u8] = &[50, 69, 0, 67, 1, 123, 112, 124, 73, 10, 33, 23, 67, 21, 53, 62, 39, 67, 59, 85, 85, 23, 94, 62, 64, 30, 83, 65, 82, 119, 80, 4, 30, 28, 46, 45, 39, 91, 6, 52, 1, 69, 89, 104, 113, 49, 27, 49, 66, 91, 88, 90, 48, 29, 70, 0, 25, 81, 51, 65, 17, 95, 27, 47, 58];
const ENC_TURSO_TOKEN: &[u8] = &[63, 72, 62, 91, 16, 6, 60, 58, 124, 10, 31, 52, 107, 49, 11, 11, 20, 61, 10, 66, 125, 24, 97, 106, 81, 115, 123, 0, 104, 49, 65, 44, 101, 49, 11, 102, 125, 86, 26, 31, 26, 120, 30, 54, 54, 38, 0, 32, 88, 120, 53, 121, 47, 107, 104, 99, 95, 110, 48, 116, 71, 124, 38, 16, 107, 30, 73, 34, 45, 63, 101, 57, 42, 22, 40, 2, 40, 120, 94, 25, 90, 18, 118, 118, 90, 123, 118, 15, 1, 57, 73, 39, 53, 6, 62, 126, 20, 24, 33, 1, 71, 22, 11, 46, 94, 15, 102, 114, 27, 105, 53, 81, 68, 127, 114, 73, 50, 104, 25, 126, 71, 14, 27, 9, 91, 58, 103, 63, 72, 61, 48, 40, 44, 15, 113, 93, 95, 63, 89, 48, 91, 101, 117, 90, 80, 3, 105, 46, 6, 22, 59, 14, 36, 81, 53, 23, 66, 82, 32, 15, 10, 33, 0, 18, 72, 121, 12, 87, 105, 99, 2, 11, 108, 115, 18, 65, 5, 98, 66, 11, 11, 50, 0, 19, 102, 37, 103, 16, 15, 59, 2, 26, 4, 101, 90, 30, 66, 62, 75, 121, 65, 127, 79, 16, 65, 46, 112, 59, 119, 22, 57, 117, 14, 15, 31, 120, 13, 3, 53, 3, 6, 15, 101, 121, 70, 126, 50, 119, 68, 124, 114, 96, 45, 124, 39, 2, 26, 15, 11, 9, 94, 47, 1, 22, 88, 45, 52, 18, 116, 33, 4, 123, 88, 59, 73, 26, 74, 106, 97, 124, 24, 116, 104, 89, 71, 59, 44, 110, 30, 3, 23, 57, 39, 112, 49, 35, 15, 12, 36, 27, 114, 3, 1, 11, 15, 96, 67, 90, 6, 16, 108, 104, 76, 64, 33, 49, 61, 22, 1, 87, 16, 29, 115, 14, 51, 21, 29, 87, 57, 8, 68, 71, 114, 21, 106, 123, 0, 68, 120, 14, 119, 45, 84, 59, 13, 29, 37, 105, 21, 47, 53, 99, 25, 23, 38, 26, 94, 45, 3, 126, 7, 101, 39, 68, 6, 125, 117, 86];

fn deobfuscate(enc: &[u8], key: &[u8]) -> String {
    let decrypted: Vec<u8> = enc.iter().enumerate().map(|(i, &b)| b ^ key[i % key.len()]).collect();
    String::from_utf8(decrypted).unwrap_or_default()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatementInput {
    pub sql: String,
    pub args: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TursoQueryResult {
    #[serde(rename = "affectedRowCount")]
    pub affected_row_count: usize,
    pub columns: Vec<String>,
    pub rows: Vec<serde_json::Map<String, serde_json::Value>>,
}

fn sanitize_sql(sql: &str) -> Result<(), String> {
    let s = sql.trim().to_uppercase();
    // 1. Forbid destructive DDL / DML
    let forbidden = ["DROP", "DELETE FROM USERS", "TRUNCATE", "ALTER", "ATTACH", "PRAGMA", "VACUUM"];
    for f in &forbidden {
        if s.contains(f) {
            return Err(format!("[SECURITY_ENCLAVE] Query rejected: contains forbidden keyword '{}'", f));
        }
    }
    // 2. Privilege Escalation Defense: strictly block updating tier/role/is_active
    if s.starts_with("UPDATE USERS") {
        if s.contains("TIER") || s.contains("ROLE") || s.contains("IS_ACTIVE") {
            return Err("[SECURITY_ENCLAVE] Privilege escalation blocked: modifying administrative user attributes is prohibited.".to_string());
        }
    }
    Ok(())
}

fn encode_arg(val: &serde_json::Value) -> serde_json::Value {
    match val {
        serde_json::Value::Null => serde_json::json!({ "type": "null" }),
        serde_json::Value::Bool(b) => serde_json::json!({ "type": "integer", "value": if *b { "1" } else { "0" } }),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                serde_json::json!({ "type": "integer", "value": i.to_string() })
            } else {
                serde_json::json!({ "type": "float", "value": n.to_string() })
            }
        }
        serde_json::Value::String(s) => serde_json::json!({ "type": "text", "value": s }),
        other => serde_json::json!({ "type": "text", "value": other.to_string() }),
    }
}

fn decode_cell(cell: &serde_json::Value) -> serde_json::Value {
    if let Some(obj) = cell.as_object() {
        let t = obj.get("type").and_then(|v| v.as_str()).unwrap_or("");
        let val = obj.get("value");
        match t {
            "null" => serde_json::Value::Null,
            "integer" => {
                if let Some(v_str) = val.and_then(|v| v.as_str()) {
                    if let Ok(num) = v_str.parse::<i64>() {
                        return serde_json::json!(num);
                    }
                }
                val.cloned().unwrap_or(serde_json::Value::Null)
            }
            "float" => {
                if let Some(v_str) = val.and_then(|v| v.as_str()) {
                    if let Ok(num) = v_str.parse::<f64>() {
                        return serde_json::json!(num);
                    }
                }
                val.cloned().unwrap_or(serde_json::Value::Null)
            }
            "text" | "blob" => val.cloned().unwrap_or(serde_json::Value::Null),
            _ => val.cloned().unwrap_or(serde_json::Value::Null),
        }
    } else {
        serde_json::Value::Null
    }
}

pub fn execute_batch(statements: Vec<StatementInput>) -> Result<Vec<TursoQueryResult>, String> {
    for stmt in &statements {
        sanitize_sql(&stmt.sql)?;
    }

    let mut requests = Vec::new();
    for stmt in &statements {
        let mut stmt_obj = serde_json::Map::new();
        stmt_obj.insert("sql".to_string(), serde_json::Value::String(stmt.sql.clone()));
        if let Some(args) = &stmt.args {
            if !args.is_empty() {
                let encoded_args: Vec<serde_json::Value> = args.iter().map(encode_arg).collect();
                stmt_obj.insert("args".to_string(), serde_json::Value::Array(encoded_args));
            }
        }
        requests.push(serde_json::json!({
            "type": "execute",
            "stmt": stmt_obj
        }));
    }
    requests.push(serde_json::json!({ "type": "close" }));

    let payload = serde_json::json!({ "requests": requests });
    let payload_str = serde_json::to_string(&payload)
        .map_err(|e| format!("Serialization error: {e}"))?;

    let url = deobfuscate(ENC_TURSO_URL, SECRET_KEY);
    let token = deobfuscate(ENC_TURSO_TOKEN, SECRET_KEY);

    let mut cmd = Command::new("curl.exe");
    cmd.args([
        "-s",
        "--connect-timeout", "4",
        "--max-time", "8",
        "-X", "POST",
        "-H", &format!("Authorization: Bearer {}", token),
        "-H", "Content-Type: application/json",
        "--data-binary", "@-",
        &url,
    ])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());

    #[cfg(target_os = "windows")]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let mut child = cmd.spawn().map_err(|e| format!("Failed to invoke secure HTTP channel: {e}"))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(payload_str.as_bytes())
            .map_err(|e| format!("Failed to pipe request payload: {e}"))?;
    }

    let output = child.wait_with_output()
        .map_err(|e| format!("Failed to read HTTP enclave response: {e}"))?;

    if !output.status.success() {
        let err_text = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Secure channel network error: {}", err_text));
    }

    let raw_resp_str = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&raw_resp_str)
        .map_err(|e| format!("Enclave response decode error: {e} (Raw: {raw_resp_str})"))?;

    let raw_results = parsed.get("results")
        .and_then(|r| r.as_array())
        .ok_or_else(|| "Invalid Turso protocol response: missing 'results'".to_string())?;

    let mut final_results = Vec::new();
    for (i, _) in statements.iter().enumerate() {
        let res_item = raw_results.get(i).ok_or_else(|| format!("Missing result index {i}"))?;
        if let Some(item_type) = res_item.get("type").and_then(|t| t.as_str()) {
            if item_type == "error" {
                let err_msg = res_item.get("error")
                    .and_then(|e| e.get("message"))
                    .and_then(|m| m.as_str())
                    .unwrap_or("Unknown Turso SQL error");
                return Err(format!("SQL Execution Error: {}", err_msg));
            }
        }

        let resp_obj = res_item.get("response").and_then(|r| r.as_object());
        let result_obj = resp_obj.and_then(|r| r.get("result")).and_then(|r| r.as_object());

        let mut col_names = Vec::new();
        if let Some(cols) = result_obj.and_then(|r| r.get("cols")).and_then(|c| c.as_array()) {
            for c in cols {
                if let Some(name) = c.get("name").and_then(|n| n.as_str()) {
                    col_names.push(name.to_string());
                }
            }
        }

        let affected_row_count = result_obj
            .and_then(|r| r.get("affected_row_count"))
            .and_then(|n| n.as_u64())
            .unwrap_or(0) as usize;

        let mut rows = Vec::new();
        if let Some(raw_rows) = result_obj.and_then(|r| r.get("rows")).and_then(|r| r.as_array()) {
            for cells in raw_rows {
                if let Some(cell_list) = cells.as_array() {
                    let mut row_map = serde_json::Map::new();
                    for (j, col) in col_names.iter().enumerate() {
                        let cell_val = cell_list.get(j).unwrap_or(&serde_json::Value::Null);
                        row_map.insert(col.clone(), decode_cell(cell_val));
                    }
                    rows.push(row_map);
                }
            }
        }

        final_results.push(TursoQueryResult {
            affected_row_count,
            columns: col_names,
            rows,
        });
    }

    Ok(final_results)
}
