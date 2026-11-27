// ZITERA CYBER_LAB — Tauri v2 Backend
// Provides IPC commands from the frontend to zitera-engine and local auth session.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use tauri::{Emitter, Manager};

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Locate `zitera-engine.exe` next to the current executable, bundled resources,
/// common workspace dev locations, or fall back to PATH.
fn engine_path(app: &tauri::AppHandle) -> String {
    // 1. Try next to current exe (production / bundle / run folder)
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            let candidate = parent.join("zitera-engine.exe");
            if candidate.exists() {
                return candidate.to_string_lossy().into_owned();
            }
            let candidate_res = parent.join("resources").join("zitera-engine.exe");
            if candidate_res.exists() {
                return candidate_res.to_string_lossy().into_owned();
            }
        }

        // Walk up from current exe directory to find repository engine binaries
        let mut cur = exe_path;
        while cur.pop() {
            let candidates = [
                cur.join("zitera-engine.exe"),
                cur.join("engine").join("zitera-engine.exe"),
                cur.join("engine").join("rust").join("target").join("release").join("zitera-engine.exe"),
                cur.join("engine").join("rust").join("target").join("debug").join("zitera-engine.exe"),
            ];
            for cand in &candidates {
                if cand.exists() {
                    return cand.to_string_lossy().into_owned();
                }
            }
        }
    }

    // 2. Try Tauri resource_dir
    if let Ok(res_dir) = app.path().resource_dir() {
        let candidate = res_dir.join("zitera-engine.exe");
        if candidate.exists() {
            return candidate.to_string_lossy().into_owned();
        }
    }

    // 3. Try relative to current working directory (walking up)
    if let Ok(mut cwd) = std::env::current_dir() {
        loop {
            let candidates = [
                cwd.join("zitera-engine.exe"),
                cwd.join("engine").join("zitera-engine.exe"),
                cwd.join("engine").join("rust").join("target").join("release").join("zitera-engine.exe"),
                cwd.join("engine").join("rust").join("target").join("debug").join("zitera-engine.exe"),
            ];
            for cand in &candidates {
                if cand.exists() {
                    return cand.to_string_lossy().into_owned();
                }
            }
            if !cwd.pop() {
                break;
            }
        }
    }

    // 4. Fallback: rely on system PATH
    "zitera-engine.exe".to_string()
}

/// Locates the project workspace root containing catalog and labs.
fn find_project_root() -> Option<PathBuf> {
    if let Ok(mut cwd) = std::env::current_dir() {
        loop {
            if cwd.join("catalog").join("catalog.json").exists() || cwd.join(".git").exists() {
                return Some(cwd);
            }
            if !cwd.pop() {
                break;
            }
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        let mut cur = exe;
        while cur.pop() {
            if cur.join("catalog").join("catalog.json").exists() || cur.join(".git").exists() {
                return Some(cur);
            }
        }
    }
    None
}

/// Run an engine sub-command and return its stdout as a JSON string.
/// argv: slice of args *after* the binary name, e.g. `&["--json", "lab", "list"]`
fn run_engine(app: &tauri::AppHandle, argv: &[&str]) -> Result<String, String> {
    let bin = engine_path(app);
    let mut cmd = Command::new(&bin);
    cmd.args(argv);
    if let Some(root) = find_project_root() {
        cmd.current_dir(root);
    }

    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to spawn {bin}: {e}"))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(format!(
            "engine exited with status {}: {stderr}",
            output.status.code().unwrap_or(-1)
        ))
    }
}

// ---------------------------------------------------------------------------
// Auth-session helpers
// ---------------------------------------------------------------------------

/// Returns `<AppData/Local>/ZiteraLab/user/auth_session.json` or `~/.zitera/user/auth_session.json`
fn auth_session_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    // Check ~/.zitera/user/auth_session.json first if it exists
    if let Ok(home) = app.path().home_dir() {
        let zitera_home = home.join(".zitera").join("user").join("auth_session.json");
        if zitera_home.exists() {
            return Ok(zitera_home);
        }
    }

    // Default to app_local_data_dir/user/auth_session.json
    let base = app
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Cannot resolve local data dir: {e}"))?;
    Ok(base.join("user").join("auth_session.json"))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthSession {
    pub token: String,
    pub user_id: String,
    pub email: String,
    pub username: String,
    pub expires_at: Option<String>,
}

// ---------------------------------------------------------------------------
// Tauri commands — Labs
// ---------------------------------------------------------------------------

/// Returns the full labs list as a JSON string.
#[tauri::command]
fn get_labs(app: tauri::AppHandle) -> Result<String, String> {
    run_engine(&app, &["--json", "lab", "list"])
}

/// Returns status JSON for a single lab.
#[tauri::command]
fn get_lab_status(app: tauri::AppHandle, lab_id: String) -> Result<String, String> {
    run_engine(&app, &["--json", "lab", "status", &lab_id])
}

/// Start a lab; returns engine response JSON.
#[tauri::command]
fn start_lab(app: tauri::AppHandle, lab_id: String) -> Result<String, String> {
    run_engine(&app, &["--json", "lab", "start", &lab_id])
}

/// Stop a running lab.
#[tauri::command]
fn stop_lab(app: tauri::AppHandle, lab_id: String) -> Result<String, String> {
    run_engine(&app, &["--json", "lab", "stop", &lab_id])
}

/// Reset a lab to its initial state.
#[tauri::command]
fn reset_lab(app: tauri::AppHandle, lab_id: String) -> Result<String, String> {
    run_engine(&app, &["--json", "lab", "reset", &lab_id])
}

/// Install a lab from the catalog.
#[tauri::command]
fn install_lab(app: tauri::AppHandle, lab_id: String) -> Result<String, String> {
    run_engine(&app, &["--json", "lab", "install", &lab_id])
}

/// Update an installed lab.
#[tauri::command]
fn update_lab(app: tauri::AppHandle, lab_id: String) -> Result<String, String> {
    run_engine(&app, &["--json", "lab", "update", &lab_id])
}

/// Return structured lab content / instructions for a lab.
#[tauri::command]
fn get_lab_content(app: tauri::AppHandle, lab_id: String) -> Result<String, String> {
    run_engine(&app, &["--json", "lab", "content", &lab_id])
}

// ---------------------------------------------------------------------------
// Tauri commands — Catalog & Health
// ---------------------------------------------------------------------------

/// Returns full catalog JSON.
#[tauri::command]
fn get_catalog(app: tauri::AppHandle) -> Result<String, String> {
    run_engine(&app, &["--json", "catalog", "list"])
}

/// Runs `zitera-engine --json doctor` and returns diagnostic JSON.
#[tauri::command]
fn check_doctor(app: tauri::AppHandle) -> Result<String, String> {
    run_engine(&app, &["--json", "doctor"])
}

// ---------------------------------------------------------------------------
// Tauri commands — Browser
// ---------------------------------------------------------------------------

/// Opens `url` in the default system browser.
/// Uses `cmd /c start "" <url>` on Windows; `xdg-open` on Linux; `open` on
/// macOS.  Returns Ok(()) on success.
#[tauri::command]
fn open_browser(url: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        // Use rundll32 FileProtocolHandler to preserve full URLs, tokens, and query strings
        // without CMD shell character parsing restrictions.
        let mut cmd = Command::new("rundll32.exe");
        cmd.args(["url.dll,FileProtocolHandler", &url]);
        cmd.creation_flags(CREATE_NO_WINDOW);
        if cmd.spawn().is_err() {
            let mut fallback = Command::new("cmd");
            fallback.args(["/c", "start", "", &url]);
            fallback.creation_flags(CREATE_NO_WINDOW);
            fallback.spawn().map_err(|e| format!("open_browser failed: {e}"))?;
        }
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("open_browser failed: {e}"))?;
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("open_browser failed: {e}"))?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Tauri commands — Auth session
// ---------------------------------------------------------------------------

/// Reads and returns the persisted auth session, or null if none.
#[tauri::command]
fn get_auth_session(app: tauri::AppHandle) -> Result<Option<serde_json::Value>, String> {
    let path = auth_session_path(&app)?;
    if !path.exists() {
        return Ok(None);
    }
    let raw = std::fs::read_to_string(&path)
        .map_err(|e| format!("Cannot read auth session: {e}"))?;
    let session: serde_json::Value =
        serde_json::from_str(&raw).map_err(|e| format!("Cannot parse auth session: {e}"))?;
    Ok(Some(session))
}

/// Persists the auth session to disk.
#[tauri::command]
fn save_auth_session(app: tauri::AppHandle, session: serde_json::Value) -> Result<(), String> {
    let path = auth_session_path(&app)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Cannot create auth dir: {e}"))?;
    }
    let json = serde_json::to_string_pretty(&session)
        .map_err(|e| format!("Cannot serialize auth session: {e}"))?;
    std::fs::write(&path, json).map_err(|e| format!("Cannot write auth session: {e}"))?;
    Ok(())
}

/// Deletes the persisted auth session.
#[tauri::command]
fn clear_auth_session(app: tauri::AppHandle) -> Result<(), String> {
    let path = auth_session_path(&app)?;
    if path.exists() {
        std::fs::remove_file(&path)
            .map_err(|e| format!("Cannot delete auth session: {e}"))?;
    }
    Ok(())
}

/// Sets the webview zoom factor natively (equivalent to browser Ctrl+ / Ctrl-).
#[tauri::command]
fn set_app_zoom(window: tauri::WebviewWindow, scale: f64) -> Result<(), String> {
    window.set_zoom(scale).map_err(|e| e.to_string())
}

/// Returns persistent hardware machine identifier for single-device licensing binding.
#[tauri::command]
fn get_device_fingerprint() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("reg")
            .args(["query", "HKLM\\SOFTWARE\\Microsoft\\Cryptography", "/v", "MachineGuid"])
            .creation_flags(CREATE_NO_WINDOW)
            .output();
        if let Ok(out) = output {
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                if line.contains("MachineGuid") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 3 {
                        let guid = parts[2];
                        let hostname = std::env::var("COMPUTERNAME").unwrap_or_else(|_| "WIN-PC".into());
                        return Ok(format!("{}_{}", hostname, guid));
                    }
                }
            }
        }
    }
    let host = std::env::var("COMPUTERNAME").or_else(|_| std::env::var("HOSTNAME")).unwrap_or_else(|_| "DEVICE".into());
    Ok(format!("{}_NODEVICEGUID", host))
}

// ---------------------------------------------------------------------------
// Auxiliary feature windows (Tanya Mentor / Burp Suite Lite)
// ---------------------------------------------------------------------------

/// Shared lab context handed to detached feature windows.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
struct FeatureWindowContext {
    lab_id: String,
    lab_title: String,
    port: u16,
    tier: String,
}

#[derive(Default)]
struct FeatureWindowsState {
    context: Mutex<FeatureWindowContext>,
}

/// Pushes fresh lab context into windows that are already open.
#[tauri::command]
fn set_feature_window_context(
    app: tauri::AppHandle,
    context: FeatureWindowContext,
) -> Result<(), String> {
    {
        let state = app.state::<FeatureWindowsState>();
        let mut stored = state
            .context
            .lock()
            .map_err(|_| "Feature window context lock poisoned".to_string())?;
        *stored = context.clone();
    }

    for kind in ["mentor", "burp"] {
            let label = format!("zitera-{kind}");
            if app.get_webview_window(&label).is_some() {
                let _ = app.emit_to(&label, "zitera:feature-context", &context);
            }
        }
    Ok(())
}

/// Returns the latest lab context for the calling feature window.
#[tauri::command]
fn get_feature_window_context(
    state: tauri::State<'_, FeatureWindowsState>,
) -> Result<FeatureWindowContext, String> {
    state
        .context
        .lock()
        .map(|context| context.clone())
        .map_err(|_| "Feature window context lock poisoned".to_string())
}

mod cloud_enclave;

#[cfg(target_os = "windows")]
extern "system" {
    fn IsDebuggerPresent() -> i32;
    fn CheckRemoteDebuggerPresent(hProcess: *mut std::ffi::c_void, pbDebuggerPresent: *mut i32) -> i32;
    fn GetCurrentProcess() -> *mut std::ffi::c_void;
}

fn check_debugger_tampering() -> Result<(), String> {
    #[cfg(all(target_os = "windows", not(debug_assertions)))]
    unsafe {
        if IsDebuggerPresent() != 0 {
            return Err("[ANTI_REVERSE] Execution halted: active debugger detected.".into());
        }
        let mut remote: i32 = 0;
        CheckRemoteDebuggerPresent(GetCurrentProcess(), &mut remote);
        if remote != 0 {
            return Err("[ANTI_REVERSE] Execution halted: remote debugger attached.".into());
        }
    }
    Ok(())
}

/// Executes an authorized batch of SQL statements within the native security enclave.
#[tauri::command]
fn cloud_execute_batch(
    statements: Vec<cloud_enclave::StatementInput>,
) -> Result<Vec<cloud_enclave::CloudQueryResult>, String> {
    check_debugger_tampering()?;
    cloud_enclave::execute_batch(statements)
}

// ---------------------------------------------------------------------------
// App entry-point
// ---------------------------------------------------------------------------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(FeatureWindowsState::default());
            app.handle().plugin(
                tauri_plugin_log::Builder::default()
                    .level(log::LevelFilter::Info)
                    .build(),
            )?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                if let Some(kind) = window.label().strip_prefix("zitera-") {
                    let _ = window.app_handle().emit_to(
                        "main",
                        "zitera:feature-window",
                        serde_json::json!({ "kind": kind, "open": false }),
                    );
                }
            }
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            // Labs
            get_labs,
            get_lab_status,
            start_lab,
            stop_lab,
            reset_lab,
            install_lab,
            update_lab,
            get_lab_content,
            // Catalog & Health
            get_catalog,
            check_doctor,
            // Browser
            open_browser,
            // Auth session
            get_auth_session,
            save_auth_session,
            clear_auth_session,
            // Window Zoom
            set_app_zoom,
            // Device Hardware Binding
            get_device_fingerprint,
            // Native Security Enclave
            cloud_execute_batch,
            // Detached feature windows
            set_feature_window_context,
            get_feature_window_context,
        ])
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
