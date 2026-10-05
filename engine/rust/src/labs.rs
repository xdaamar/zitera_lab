use crate::models::{LabManifest, LabStatus};
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_labs_dir(workspace_root: &Path) -> PathBuf {
    workspace_root.join("labs")
}

pub fn get_lab_dir(workspace_root: &Path, lab_id: &str) -> PathBuf {
    get_labs_dir(workspace_root).join(lab_id.to_uppercase())
}

/// SEC: Ensures user/manifest-controlled subpath is strictly contained within base_dir.
/// Rejects path traversal (..), drive letters (C:), absolute paths, and UNC network shares.
pub fn safe_subpath(
    base_dir: &Path,
    relative_subpath: impl AsRef<Path>,
) -> Result<PathBuf, String> {
    let sub = relative_subpath.as_ref();
    for comp in sub.components() {
        match comp {
            std::path::Component::Prefix(_) | std::path::Component::RootDir => {
                return Err(format!(
                    "Path traversal attempt: absolute or prefix path is forbidden: {:?}",
                    sub
                ));
            }
            std::path::Component::ParentDir => {
                return Err(format!(
                    "Path traversal attempt: parent directory ('..') is forbidden: {:?}",
                    sub
                ));
            }
            std::path::Component::CurDir | std::path::Component::Normal(_) => {}
        }
    }
    Ok(base_dir.join(sub))
}

pub fn read_manifest(lab_dir: &Path) -> Result<LabManifest, String> {
    let effective_dir = crate::package::resolve_effective_lab_dir(lab_dir);
    let manifest_path = effective_dir.join("manifest.json");
    if !manifest_path.exists() {
        return Err(format!("manifest.json not found in {:?}", effective_dir));
    }
    let data = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("Failed to read manifest at {:?}: {}", manifest_path, e))?;
    let manifest: LabManifest =
        serde_json::from_str(&data).map_err(|e| format!("Failed to parse manifest JSON: {}", e))?;

    if manifest.id.trim().is_empty() {
        return Err("Lab manifest 'id' cannot be empty.".to_string());
    }
    validate_lab_id(&manifest.id)?;

    if manifest.title.trim().is_empty() || manifest.title.len() > 128 {
        return Err("Lab manifest 'title' must be between 1 and 128 characters.".to_string());
    }
    if manifest.default_port < 1024 {
        return Err(format!(
            "Lab manifest 'default_port' ({}) must be >= 1024.",
            manifest.default_port
        ));
    }
    let supported_runtimes = ["docker", "native_sandboxed", "native_process", "mock"];
    if !supported_runtimes.contains(&manifest.runtime.as_str()) {
        return Err(format!(
            "Unsupported lab runtime '{}'. Supported runtimes for Phase 15/16: {:?}.",
            manifest.runtime, supported_runtimes
        ));
    }

    // Verify entrypoint is a safe relative subpath within lab_dir (e.g., docker-compose.yml)
    safe_subpath(lab_dir, Path::new(&manifest.entrypoint))?;

    Ok(manifest)
}

pub fn validate_lab_id(lab_id: &str) -> Result<(), String> {
    if lab_id.is_empty() || lab_id.len() > 16 {
        return Err("Lab ID must be between 1 and 16 characters".to_string());
    }
    if !lab_id
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        return Err(
            "Lab ID contains invalid characters; must be alphanumeric, hyphen, or underscore"
                .to_string(),
        );
    }
    Ok(())
}

/// SEC-011/SEC-012: Validate a repository URL is a safe HTTPS GitHub URL.
/// Rejects file://, ssh://, git://, arbitrary hosts, and argument-looking values.
/// Only allows https://github.com/<owner>/<repo>.git with safe identifier chars.
pub fn validate_repo_url(url: &str) -> Result<(), String> {
    if !url.starts_with("https://github.com/") {
        return Err(format!(
            "Repository URL '{}' is not a trusted HTTPS GitHub URL. Only https://github.com/ is allowed.",
            url
        ));
    }
    let path = url
        .strip_prefix("https://github.com/")
        .unwrap_or("")
        .trim_end_matches(".git");
    // Validate owner/repo: only alphanumeric, hyphens, underscores, and one slash
    let parts: Vec<&str> = path.splitn(2, '/').collect();
    if parts.len() != 2 {
        return Err(format!(
            "Repository '{}' must be in 'owner/repo' format.",
            url
        ));
    }
    let valid_ident = |s: &&str| {
        !s.is_empty()
            && s.chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    };
    if !parts.iter().all(valid_ident) {
        return Err(format!(
            "Repository '{}' contains invalid characters in owner or repo name.",
            url
        ));
    }
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct LabRuntimeState {
    pub pid: u32,
    pub broker_pid: u32,
    pub port: u16,
    pub session_id: String,
    pub entry_url: String,
    pub started_at: u64,
}

pub fn is_process_alive(pid: u32) -> bool {
    if pid == 0 {
        return false;
    }
    #[cfg(windows)]
    unsafe {
        use windows_sys::Win32::Foundation::CloseHandle;
        use windows_sys::Win32::System::Threading::{
            GetExitCodeProcess, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION,
        };
        let h = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
        if h == 0 as _ {
            return false;
        }
        let mut exit_code: u32 = 0;
        let res = GetExitCodeProcess(h, &mut exit_code);
        CloseHandle(h);
        res != 0 && exit_code == 259 // STILL_ACTIVE = 259
    }
    #[cfg(not(windows))]
    {
        false
    }
}

pub fn get_runtime_state(lab_dir: &Path) -> Option<LabRuntimeState> {
    let state_file = lab_dir.join(".runtime.json");
    if !state_file.exists() {
        return None;
    }
    if let Ok(content) = fs::read_to_string(&state_file) {
        if let Ok(state) = serde_json::from_str::<LabRuntimeState>(&content) {
            if is_process_alive(state.broker_pid) {
                return Some(state);
            }
        }
    }
    let _ = fs::remove_file(state_file);
    None
}

pub fn get_lab_status(workspace_root: &Path, lab_id: &str) -> LabStatus {
    get_lab_status_cached(workspace_root, lab_id, None, None, None)
}

pub fn get_lab_status_cached(
    workspace_root: &Path,
    lab_id: &str,
    cached_cat: Option<&crate::models::Catalog>,
    _running_containers: Option<&std::collections::HashSet<String>>,
    installed_tools: Option<&[String]>,
) -> LabStatus {
    if validate_lab_id(lab_id).is_err() {
        return LabStatus {
            id: lab_id.to_string(),
            title: lab_id.to_string(),
            installed: false,
            running: false,
            port: 0,
            url: None,
            version: "N/A".to_string(),
            status: "INVALID_ID".to_string(),
            learn_readiness: "NOT_READY".to_string(),
            practice_readiness: "NOT_READY".to_string(),
            challenge_readiness: "NOT_READY".to_string(),
            recommended_tools: Vec::new(),
        };
    }

    let lab_dir = get_lab_dir(workspace_root, lab_id);
    if !lab_dir.exists() {
        let title = if let Some(cat) = cached_cat {
            cat.labs
                .iter()
                .find(|l| l.id.eq_ignore_ascii_case(lab_id))
                .map(|l| l.title.clone())
                .unwrap_or_else(|| lab_id.to_string())
        } else {
            match crate::catalog::load_catalog(workspace_root) {
                Ok(cat) => cat
                    .labs
                    .iter()
                    .find(|l| l.id.eq_ignore_ascii_case(lab_id))
                    .map(|l| l.title.clone())
                    .unwrap_or_else(|| lab_id.to_string()),
                Err(_) => lab_id.to_string(),
            }
        };
        return LabStatus {
            id: lab_id.to_uppercase(),
            title,
            installed: false,
            running: false,
            port: 0,
            url: None,
            version: "N/A".to_string(),
            status: "NOT_INSTALLED".to_string(),
            learn_readiness: "NOT_READY".to_string(),
            practice_readiness: "NOT_READY".to_string(),
            challenge_readiness: "NOT_READY".to_string(),
            recommended_tools: Vec::new(),
        };
    }

    match read_manifest(&lab_dir) {
        Ok(manifest) => {
            let (is_running, port, url) = if manifest.runtime == "native_sandboxed" {
                if let Some(rt) = get_runtime_state(&lab_dir) {
                    (true, rt.port, Some(rt.entry_url))
                } else {
                    (false, 0, None)
                }
            } else {
                let running = !crate::system::is_port_available(manifest.default_port);
                let u = if running {
                    Some(format!("http://127.0.0.1:{}", manifest.default_port))
                } else {
                    None
                };
                (running, manifest.default_port, u)
            };
            let status_str = if is_running { "RUNNING" } else { "STOPPED" };

            let (challenge_readiness, recommended_tools) = if let Some(req) = &manifest.requirements
            {
                let tools_list: Vec<String> = if let Some(tools) = installed_tools {
                    tools.to_vec()
                } else {
                    crate::tools::list_tools()
                        .into_iter()
                        .filter(|t| t.installed)
                        .map(|t| t.id.to_lowercase())
                        .collect()
                };

                let missing_recommended: Vec<String> = req
                    .recommended_tools
                    .iter()
                    .filter(|t| !tools_list.contains(&t.to_lowercase()))
                    .cloned()
                    .collect();

                let missing_required: Vec<String> = req
                    .required_tools
                    .iter()
                    .filter(|t| !tools_list.contains(&t.to_lowercase()))
                    .cloned()
                    .collect();

                let readiness = if !missing_required.is_empty() {
                    "BLOCKED"
                } else if !missing_recommended.is_empty() {
                    "PARTIAL"
                } else {
                    "READY"
                };

                (readiness.to_string(), missing_recommended)
            } else {
                ("READY".to_string(), Vec::new())
            };

            LabStatus {
                id: manifest.id,
                title: manifest.title,
                installed: true,
                running: is_running,
                port,
                url,
                version: manifest.version,
                status: status_str.to_string(),
                learn_readiness: "READY".to_string(),
                practice_readiness: "READY".to_string(),
                challenge_readiness,
                recommended_tools,
            }
        }
        Err(_) => {
            let title = if let Some(cat) = cached_cat {
                cat.labs
                    .iter()
                    .find(|l| l.id.eq_ignore_ascii_case(lab_id))
                    .map(|l| l.title.clone())
                    .unwrap_or_else(|| lab_id.to_string())
            } else {
                match crate::catalog::load_catalog(workspace_root) {
                    Ok(cat) => cat
                        .labs
                        .iter()
                        .find(|l| l.id.eq_ignore_ascii_case(lab_id))
                        .map(|l| l.title.clone())
                        .unwrap_or_else(|| lab_id.to_string()),
                    Err(_) => lab_id.to_string(),
                }
            };
            LabStatus {
                id: lab_id.to_uppercase(),
                title,
                installed: false,
                running: false,
                port: 0,
                url: None,
                version: "UNKNOWN".to_string(),
                status: "INCOMPLETE_INSTALL".to_string(),
                learn_readiness: "NOT_READY".to_string(),
                practice_readiness: "NOT_READY".to_string(),
                challenge_readiness: "NOT_READY".to_string(),
                recommended_tools: Vec::new(),
            }
        }
    }
}

pub fn list_all_labs(workspace_root: &Path) -> Vec<LabStatus> {
    let cat_opt = crate::catalog::load_catalog(workspace_root).ok();
    let mut ids: Vec<String> = match &cat_opt {
        Some(cat) => cat.labs.iter().map(|l| l.id.clone()).collect(),
        None => Vec::new(),
    };

    // Also scan local workspace labs directory for installed labs not yet in catalog
    let labs_dir = workspace_root.join("labs");
    if let Ok(entries) = fs::read_dir(labs_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() && entry.path().join("manifest.json").exists() {
                if let Some(folder_name) = entry.file_name().to_str() {
                    let upper = folder_name.to_uppercase();
                    if !ids.iter().any(|id| id.eq_ignore_ascii_case(&upper)) {
                        ids.push(upper);
                    }
                }
            }
        }
    }

    ids.sort();

    // Query installed tools once in a single batch
    let installed_tools: Vec<String> = crate::tools::list_tools()
        .into_iter()
        .filter(|t| t.installed)
        .map(|t| t.id.to_lowercase())
        .collect();

    ids.into_iter()
        .map(|id| {
            get_lab_status_cached(
                workspace_root,
                &id,
                cat_opt.as_ref(),
                None,
                Some(&installed_tools),
            )
        })
        .collect()
}

pub fn serve_lab(workspace_root: &Path, lab_id: &str) -> Result<(), String> {
    use crate::broker::server::BrokerServer;
    use crate::broker::session::BrokerSessionManager;
    use crate::broker::transport::StdioLabChannel;
    use crate::native_runtime::job::{JobLimits, JobObject};
    use crate::native_runtime::process::{spawn_sandboxed_process, SandboxedProcessConfig};
    use crate::native_runtime::profile::AppContainerProfile;
    use crate::native_runtime::LabIdentity;
    use std::sync::Arc;
    use std::time::Duration;

    validate_lab_id(lab_id)?;
    let lab_dir = get_lab_dir(workspace_root, lab_id);
    if !lab_dir.exists() {
        return Err(format!("Lab {} is not installed.", lab_id));
    }
    let manifest = read_manifest(&lab_dir)?;
    if manifest.runtime != "native_sandboxed" {
        return Err(format!(
            "Lab {} runtime is '{}', expected 'native_sandboxed'.",
            lab_id, manifest.runtime
        ));
    }

    let entrypoint_path = lab_dir.join(&manifest.entrypoint);
    if !entrypoint_path.exists() {
        let filename = entrypoint_path.file_name().unwrap_or_default();
        let underscore_name = filename.to_str().unwrap_or("").replace('-', "_");
        let target_debug = workspace_root
            .join("engine/rust/target/debug")
            .join(filename);
        let target_release = workspace_root
            .join("engine/rust/target/release")
            .join(filename);
        let target_debug_under = workspace_root
            .join("engine/rust/target/debug")
            .join(&underscore_name);
        let target_release_under = workspace_root
            .join("engine/rust/target/release")
            .join(&underscore_name);

        let found = if target_debug.exists() {
            Some(target_debug)
        } else if target_release.exists() {
            Some(target_release)
        } else if target_debug_under.exists() {
            Some(target_debug_under)
        } else if target_release_under.exists() {
            Some(target_release_under)
        } else {
            None
        };

        if let Some(src) = found {
            if let Some(parent) = entrypoint_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::copy(&src, &entrypoint_path);
        } else {
            return Err(format!(
                "Lab entrypoint executable not found at {:?}",
                entrypoint_path
            ));
        }
    }

    let identity = LabIdentity::new(lab_id).map_err(|e| e.to_string())?;
    AppContainerProfile::create_or_open(&identity).map_err(|e| e.to_string())?;

    let job =
        JobObject::create(Some(&format!("ZITERA_LAB_{}", lab_id))).map_err(|e| e.to_string())?;
    let limits = JobLimits {
        kill_on_job_close: true,
        active_process_limit: Some(16),
        ..Default::default()
    };
    job.set_limits(&limits).map_err(|e| e.to_string())?;

    let config = SandboxedProcessConfig {
        executable: entrypoint_path,
        arguments: vec![],
        working_dir: lab_dir.clone(),
        environment: std::collections::HashMap::new(),
    };

    let mut handle =
        spawn_sandboxed_process(&identity, &config, Some(&job)).map_err(|e| e.to_string())?;
    let (stdin_opt, stdout_opt) = handle.take_io();
    let stdin = stdin_opt.ok_or_else(|| "Failed to capture stdin of lab process".to_string())?;
    let stdout = stdout_opt.ok_or_else(|| "Failed to capture stdout of lab process".to_string())?;

    let channel = Arc::new(StdioLabChannel::new(stdin, stdout));
    let _ = channel
        .wait_for_ready(Duration::from_secs(5))
        .map_err(|e| format!("Lab readiness failed: {}", e))?;

    let session_mgr = Arc::new(BrokerSessionManager::new(Duration::from_secs(3600)));
    let session = session_mgr.create_session(lab_id, None);

    let mut broker = BrokerServer::start(
        Arc::clone(&session_mgr),
        Arc::clone(&channel) as Arc<dyn crate::broker::server::LabRequestHandler>,
    )
    .map_err(|e| format!("Failed to start broker: {}", e))?;

    let entry_url = broker.entry_url(&session.session_id);

    let now_ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let state = LabRuntimeState {
        pid: handle.pid,
        broker_pid: std::process::id(),
        port: broker.port(),
        session_id: session.session_id.clone(),
        entry_url: entry_url.clone(),
        started_at: now_ts,
    };

    let state_file = lab_dir.join(".runtime.json");
    let _ = fs::write(&state_file, serde_json::to_string_pretty(&state).unwrap());

    println!("[READY] Lab {} running at {}", lab_id, entry_url);

    // Keep running until child process exits or stops
    while is_process_alive(handle.pid) {
        std::thread::sleep(Duration::from_millis(200));
    }

    // Cleanup
    let _ = channel.send_stop();
    broker.stop();
    let _ = fs::remove_file(state_file);
    let _ = handle.wait();
    let _ = AppContainerProfile::delete(&identity);
    Ok(())
}

pub fn start_lab(workspace_root: &Path, lab_id: &str) -> Result<String, String> {
    validate_lab_id(lab_id)?;
    let lab_dir = get_lab_dir(workspace_root, lab_id);
    if !lab_dir.exists() {
        return Err(format!(
            "Lab {} is not installed. Please install it first.",
            lab_id
        ));
    }
    let manifest = read_manifest(&lab_dir)?;

    if manifest.runtime == "native_sandboxed" {
        if let Some(state) = get_runtime_state(&lab_dir) {
            return Ok(format!(
                "Lab {} is already running at {}.",
                lab_id, state.entry_url
            ));
        }

        let current_exe = std::env::current_exe()
            .map_err(|e| format!("Failed to locate zitera-engine executable: {}", e))?;

        let is_engine_binary = current_exe
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| {
                n.eq_ignore_ascii_case("zitera-engine.exe")
                    || n.eq_ignore_ascii_case("zitera-engine")
            })
            .unwrap_or(false);

        let candidate_debug = workspace_root.join("engine/rust/target/debug/zitera-engine.exe");
        let engine_exe = if is_engine_binary && current_exe.exists() {
            current_exe
        } else if candidate_debug.exists() {
            candidate_debug
        } else {
            workspace_root.join("engine/rust/target/release/zitera-engine.exe")
        };

        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            const DETACHED_PROCESS: u32 = 0x00000008;

            let mut cmd = std::process::Command::new(engine_exe);
            cmd.args(["lab", "serve", lab_id]);
            cmd.current_dir(workspace_root);
            cmd.stdin(std::process::Stdio::null());
            cmd.stdout(std::process::Stdio::null());
            cmd.stderr(std::process::Stdio::null());
            cmd.creation_flags(CREATE_NO_WINDOW | DETACHED_PROCESS);
            cmd.spawn()
                .map_err(|e| format!("Failed to launch lab broker background process: {}", e))?;
        }

        #[cfg(not(windows))]
        {
            let mut cmd = std::process::Command::new(current_exe);
            cmd.args(["lab", "serve", lab_id]);
            cmd.current_dir(workspace_root);
            cmd.spawn()
                .map_err(|e| format!("Failed to launch lab broker background process: {}", e))?;
        }

        // Wait up to 5 seconds for .runtime.json to be created and healthy
        for _ in 0..50 {
            std::thread::sleep(std::time::Duration::from_millis(100));
            if let Some(state) = get_runtime_state(&lab_dir) {
                return Ok(format!("Lab {} started at {}", lab_id, state.entry_url));
            }
        }

        Err(format!(
            "Timed out waiting for Lab {} to initialize",
            lab_id
        ))
    } else {
        if !crate::system::is_port_available(manifest.default_port) {
            return Ok(format!(
                "Lab {} is already running on port {}.",
                lab_id, manifest.default_port
            ));
        }

        Ok(format!(
            "Lab {} uses legacy runtime '{}' which is frozen in ZITERA 2.0.",
            lab_id, manifest.runtime
        ))
    }
}

pub fn stop_lab(workspace_root: &Path, lab_id: &str) -> Result<String, String> {
    validate_lab_id(lab_id)?;
    let lab_dir = get_lab_dir(workspace_root, lab_id);
    if !lab_dir.exists() {
        return Err(format!("Lab {} is not installed.", lab_id));
    }

    if let Some(state) = get_runtime_state(&lab_dir) {
        #[cfg(windows)]
        unsafe {
            use windows_sys::Win32::Foundation::CloseHandle;
            use windows_sys::Win32::System::Threading::{
                OpenProcess, TerminateProcess, PROCESS_TERMINATE,
            };
            if state.broker_pid != 0 {
                let h_broker = OpenProcess(PROCESS_TERMINATE, 0, state.broker_pid);
                if h_broker != 0 as _ {
                    let _ = TerminateProcess(h_broker, 0);
                    CloseHandle(h_broker);
                }
            }
            if state.pid != 0 {
                let h_child = OpenProcess(PROCESS_TERMINATE, 0, state.pid);
                if h_child != 0 as _ {
                    let _ = TerminateProcess(h_child, 0);
                    CloseHandle(h_child);
                }
            }
        }
        let _ = fs::remove_file(lab_dir.join(".runtime.json"));
        Ok(format!("Lab {} stopped.", lab_id))
    } else {
        Ok(format!("Lab {} is not running.", lab_id))
    }
}

pub fn reset_lab(workspace_root: &Path, lab_id: &str) -> Result<String, String> {
    validate_lab_id(lab_id)?;
    let lab_dir = get_lab_dir(workspace_root, lab_id);
    if !lab_dir.exists() {
        return Err(format!(
            "Lab {} is not installed. Please install it first.",
            lab_id
        ));
    }
    let _ = stop_lab(workspace_root, lab_id);
    let _ = fs::remove_file(lab_dir.join(".runtime.json"));
    Ok(format!("Lab {} reset completed.", lab_id))
}

pub fn install_lab(workspace_root: &Path, lab_id: &str) -> Result<String, String> {
    validate_lab_id(lab_id)?;
    let lab_dir = get_lab_dir(workspace_root, lab_id);
    if lab_dir.exists() && lab_dir.join("manifest.json").exists() {
        return Ok(format!("Lab {} is already installed.", lab_id));
    }
    // Clean up partial/corrupt previous directory before installing
    if lab_dir.exists() {
        let _ = fs::remove_dir_all(&lab_dir);
    }

    let cat = crate::catalog::load_catalog(workspace_root)?;
    let item = cat
        .labs
        .into_iter()
        .find(|l| l.id.eq_ignore_ascii_case(lab_id))
        .ok_or_else(|| format!("Lab {} not found in catalog.", lab_id))?;

    let repo_url = if item.repository.starts_with("http") {
        item.repository
    } else {
        format!("https://github.com/{}.git", item.repository)
    };

    // SEC-011/012: Reject file://, ssh://, arbitrary hosts, and injection attempts.
    validate_repo_url(&repo_url)?;

    fs::create_dir_all(get_labs_dir(workspace_root))
        .map_err(|e| format!("Failed to create labs directory: {}", e))?;

    let clone_out = crate::process::run_cmd(
        "git",
        &[
            "clone",
            "--depth",
            "1",
            &repo_url,
            lab_dir.to_str().unwrap_or(""),
        ],
        None,
    )?;
    if clone_out.success && lab_dir.join("manifest.json").exists() {
        Ok(format!(
            "Lab {} ({}) successfully installed.",
            lab_id, item.title
        ))
    } else {
        if lab_dir.exists() {
            let _ = fs::remove_dir_all(&lab_dir);
        }
        Err(format!(
            "Failed to install lab {} from repository: {}",
            lab_id, clone_out.stderr
        ))
    }
}

pub fn update_lab(workspace_root: &Path, lab_id: &str) -> Result<String, String> {
    validate_lab_id(lab_id)?;
    let lab_dir = get_lab_dir(workspace_root, lab_id);
    if !lab_dir.exists() {
        return Err(format!(
            "Lab {} is not installed. Install it first.",
            lab_id
        ));
    }
    let current_manifest = read_manifest(&lab_dir)?;
    let is_running = !crate::system::is_port_available(current_manifest.default_port);
    if is_running {
        return Err(format!(
            "Lab {} is currently running. Please stop the lab before updating.",
            lab_id
        ));
    }
    let cat = crate::catalog::load_catalog(workspace_root)?;
    let item = cat
        .labs
        .into_iter()
        .find(|l| l.id.eq_ignore_ascii_case(lab_id))
        .ok_or_else(|| format!("Lab {} not found in catalog.", lab_id))?;

    if lab_dir.join(".git").exists() {
        // Interruption recovery: abort any previously stuck merge and reset dirty index
        let _ = crate::process::run_cmd("git", &["merge", "--abort"], Some(&lab_dir));
        let _ = crate::process::run_cmd("git", &["clean", "-fd"], Some(&lab_dir));

        let rev_out = crate::process::run_cmd("git", &["rev-parse", "HEAD"], Some(&lab_dir))?;
        let previous_head = rev_out.stdout.trim().to_string();
        if previous_head.is_empty() {
            return Err(
                "Unable to determine current repository commit for rollback safety.".to_string(),
            );
        }

        // Staged update: Fetch first without altering working tree
        let fetch_out = crate::process::run_cmd("git", &["fetch", "--depth", "1"], Some(&lab_dir))?;
        if !fetch_out.success {
            return Err(format!("Update network fetch failed: {}", fetch_out.stderr));
        }

        let pull_out = crate::process::run_cmd("git", &["pull"], Some(&lab_dir))?;
        if !pull_out.success {
            // Restore immediately if pull was interrupted
            let _ = crate::process::run_cmd(
                "git",
                &["reset", "--hard", &previous_head],
                Some(&lab_dir),
            );
            let _ = crate::process::run_cmd("git", &["clean", "-fd"], Some(&lab_dir));
            return Err(format!("Failed to update lab via git: {}", pull_out.stderr));
        }

        match read_manifest(&lab_dir) {
            Ok(updated_manifest) => Ok(format!(
                "Lab {} updated from v{} to v{}.",
                lab_id, current_manifest.version, updated_manifest.version
            )),
            Err(e) => {
                // Atomic rollback on manifest or contract integrity violation
                let _ = crate::process::run_cmd(
                    "git",
                    &["reset", "--hard", &previous_head],
                    Some(&lab_dir),
                );
                let _ = crate::process::run_cmd("git", &["clean", "-fd"], Some(&lab_dir));
                Err(format!(
                    "Update rejected due to invalid manifest: {}. Rolled back to previous valid state.",
                    e
                ))
            }
        }
    } else {
        if current_manifest.version == item.version {
            Ok(format!(
                "Lab {} is already up to date (v{}).",
                lab_id, current_manifest.version
            ))
        } else {
            Ok(format!(
                "Lab {} update staged (current: v{}, catalog: v{}).",
                lab_id, current_manifest.version, item.version
            ))
        }
    }
}

pub fn remove_lab(workspace_root: &Path, lab_id: &str) -> Result<String, String> {
    validate_lab_id(lab_id)?;
    let lab_dir = get_lab_dir(workspace_root, lab_id);
    if !lab_dir.exists() {
        return Ok(format!("Lab {} is not installed.", lab_id));
    }
    let _ = stop_lab(workspace_root, lab_id);
    fs::remove_dir_all(&lab_dir).map_err(|e| format!("Failed to remove lab directory: {}", e))?;
    Ok(format!("Lab {} has been removed.", lab_id))
}

const MAX_CONTENT_FILE_SIZE: u64 = 65536; // 64 KB limit per content file (SEC / bounded reads)

fn safe_read_file(path: &Path) -> Result<String, String> {
    if !path.exists() {
        return Err(format!("File not found: {:?}", path));
    }
    let meta =
        fs::metadata(path).map_err(|e| format!("Failed to read metadata for {:?}: {}", path, e))?;
    if meta.len() > MAX_CONTENT_FILE_SIZE {
        return Err(format!(
            "File {:?} exceeds maximum allowed size ({} bytes > {} bytes limit)",
            path,
            meta.len(),
            MAX_CONTENT_FILE_SIZE
        ));
    }
    fs::read_to_string(path).map_err(|e| format!("Failed to read {:?}: {}", path, e))
}

/// Dynamic content loader (Phases 8, 9, 10). Loads manifest, lesson markdown files,
/// and progressive hints dynamically from the installed repository.
/// Secret flags are NEVER included in the returned content.
pub fn get_lab_content(
    workspace_root: &Path,
    lab_id: &str,
) -> Result<crate::models::LabContent, String> {
    validate_lab_id(lab_id)?;
    let lab_dir = get_lab_dir(workspace_root, lab_id);
    if !lab_dir.exists() {
        return Err(format!("Lab {} is not installed.", lab_id));
    }

    let manifest = read_manifest(&lab_dir)?;
    let effective_dir = crate::package::resolve_effective_lab_dir(&lab_dir);

    // Read lesson files - both standard keys and any additional dynamic markdown sections
    let lesson_dir = effective_dir.join("lesson");
    let mut lessons = std::collections::HashMap::new();

    if let Ok(entries) = fs::read_dir(&lesson_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("md") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    let stem_clean = stem.to_lowercase();
                    if stem_clean
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.')
                    {
                        if let Ok(text) = safe_read_file(&path) {
                            lessons.insert(stem_clean, text);
                        }
                    }
                }
            }
        }
    }

    // Read challenge objective
    let challenge_path = effective_dir.join("challenge").join("challenge.md");
    let challenge_objective = if challenge_path.exists() {
        safe_read_file(&challenge_path).unwrap_or_default()
    } else {
        "Complete the mission objective in the target application.".to_string()
    };

    // Read progressive hints (stripping secret flag)
    let hints_path = effective_dir.join("challenge").join("hints.json");
    let mut progressive_hints = Vec::new();
    if hints_path.exists() {
        if let Ok(raw_json) = safe_read_file(&hints_path) {
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&raw_json) {
                if let Some(arr) = parsed.get("hints").and_then(|h| h.as_array()) {
                    for item in arr {
                        let tier = item.get("tier").and_then(|v| v.as_u64()).unwrap_or(1) as u32;
                        let hint_type = item
                            .get("type")
                            .and_then(|v| v.as_str())
                            .unwrap_or("general")
                            .to_string();
                        let hint = item
                            .get("hint")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        progressive_hints.push(crate::models::ProgressiveHint {
                            tier,
                            hint_type,
                            hint,
                        });
                    }
                }
            }
        }
    }

    Ok(crate::models::LabContent {
        manifest,
        lessons,
        challenge_objective,
        hints: progressive_hints,
    })
}

/// Dynamic practice orchestrator (Phase 6F).
/// Verifies local practice runtime health and response code directly via the engine.
pub fn verify_practice(
    workspace_root: &Path,
    lab_id: &str,
) -> Result<crate::models::PracticeVerification, String> {
    validate_lab_id(lab_id)?;
    let lab_dir = get_lab_dir(workspace_root, lab_id);
    if !lab_dir.exists() {
        return Ok(crate::models::PracticeVerification {
            lab_id: lab_id.to_uppercase(),
            status: "unavailable".to_string(),
            message: format!("Lab {} is not installed.", lab_id),
        });
    }

    let manifest = read_manifest(&lab_dir)?;
    let port = manifest.default_port;

    let is_running = !crate::system::is_port_available(port);
    if !is_running {
        return Ok(crate::models::PracticeVerification {
            lab_id: lab_id.to_uppercase(),
            status: "unavailable".to_string(),
            message: format!(
                "Lab {} runtime is stopped. Please start the lab environment first.",
                lab_id
            ),
        });
    }

    // 1. Authoritative Practice Verification Endpoint (Contract V2)
    // If the laboratory service exposes a stateful practice verification endpoint, query it.
    let practice_url = format!("http://127.0.0.1:{}/practice/verify", port);
    let practice_probe =
        crate::process::run_cmd("curl.exe", &["-s", "--max-time", "3", &practice_url], None);

    if let Ok(pout) = practice_probe {
        if pout.success && !pout.stdout.trim().is_empty() {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&pout.stdout) {
                if let (Some(status_val), Some(msg_val)) = (
                    json.get("status").and_then(|s| s.as_str()),
                    json.get("message").and_then(|m| m.as_str()),
                ) {
                    return Ok(crate::models::PracticeVerification {
                        lab_id: lab_id.to_uppercase(),
                        status: status_val.to_string(),
                        message: msg_val.to_string(),
                    });
                }
            }
        }
    }

    // 2. Fallback to health probe
    let url = format!("http://127.0.0.1:{}/health", port);

    // Fast bounded curl probe (3 seconds max)
    let probe = crate::process::run_cmd(
        "curl.exe",
        &[
            "-s",
            "-o",
            "nul",
            "-w",
            "%{http_code}",
            "--max-time",
            "3",
            &url,
        ],
        None,
    );

    match probe {
        Ok(out) if out.stdout.trim() == "200" => Ok(crate::models::PracticeVerification {
            lab_id: lab_id.to_uppercase(),
            status: "passed".to_string(),
            message: format!(
                "Local practice target service on port {} is active and healthy (HTTP 200).",
                port
            ),
        }),
        _ => {
            // Check root endpoint if /health returns non-200 or 404
            let root_url = format!("http://127.0.0.1:{}/", port);
            let root_probe = crate::process::run_cmd(
                "curl.exe",
                &[
                    "-s",
                    "-o",
                    "nul",
                    "-w",
                    "%{http_code}",
                    "--max-time",
                    "3",
                    &root_url,
                ],
                None,
            );
            if let Ok(rout) = root_probe {
                let code = rout.stdout.trim();
                if code.starts_with('2') || code.starts_with('3') {
                    return Ok(crate::models::PracticeVerification {
                        lab_id: lab_id.to_uppercase(),
                        status: "passed".to_string(),
                        message: format!(
                            "Local practice target service on port {} responded successfully (HTTP {}).",
                            port, code
                        ),
                    });
                }
            }

            Ok(crate::models::PracticeVerification {
                lab_id: lab_id.to_uppercase(),
                status: "failed".to_string(),
                message: format!(
                    "Practice target probe returned unexpected status on port {}.",
                    port
                ),
            })
        }
    }
}

/// Constant-time byte comparison to eliminate timing side-channel leakage.
pub fn timing_safe_compare(a: &str, b: &str) -> bool {
    let a_clean = a.trim();
    let b_clean = b.trim();
    a_clean.len() == b_clean.len()
        && a_clean
            .as_bytes()
            .iter()
            .zip(b_clean.as_bytes().iter())
            .fold(0u8, |acc, (&x, &y)| acc | (x ^ y))
            == 0
}

/// Authoritative challenge validator (Phases 18, 19).
/// Verifies user submission against the lab repository's official challenge flag.
pub fn validate_challenge(
    workspace_root: &Path,
    lab_id: &str,
    user_submission: &str,
) -> Result<crate::models::ChallengeVerification, String> {
    validate_lab_id(lab_id)?;

    // Bounded input length to prevent excessive comparisons or memory exhaustion
    if user_submission.len() > 512 {
        return Ok(crate::models::ChallengeVerification {
            lab_id: lab_id.to_uppercase(),
            status: "failed".to_string(),
            message: "Submission exceeds 512 characters maximum limit.".to_string(),
        });
    }

    let lab_dir = get_lab_dir(workspace_root, lab_id);
    if !lab_dir.exists() {
        return Err(format!("Lab {} is not installed.", lab_id));
    }

    let hints_path = lab_dir.join("challenge").join("hints.json");
    if !hints_path.exists() {
        return Err(format!(
            "Challenge definition not found for lab {}.",
            lab_id
        ));
    }

    let raw_json = safe_read_file(&hints_path)?;
    let parsed: serde_json::Value = serde_json::from_str(&raw_json)
        .map_err(|e| format!("Failed to parse challenge definition: {}", e))?;

    let expected_flag = parsed
        .get("flag")
        .and_then(|f| f.as_str())
        .ok_or_else(|| "Challenge does not define an expected flag.".to_string())?;

    let matches = timing_safe_compare(user_submission, expected_flag);

    if matches {
        Ok(crate::models::ChallengeVerification {
            lab_id: lab_id.to_uppercase(),
            status: "passed".to_string(),
            message: "EXCELLENT! Challenge Completed! Flag Verified.".to_string(),
        })
    } else {
        Ok(crate::models::ChallengeVerification {
            lab_id: lab_id.to_uppercase(),
            status: "failed".to_string(),
            message: "Invalid Flag. Keep investigating!".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_lab_id_valid() {
        assert!(validate_lab_id("A01").is_ok());
        assert!(validate_lab_id("A10").is_ok());
        assert!(validate_lab_id("zitera-lab").is_ok());
        assert!(validate_lab_id("LAB_01").is_ok());
    }

    #[test]
    fn test_validate_lab_id_path_traversals_rejected() {
        let malicious = [
            "../A01",
            "../../engine",
            r"..\..\engine",
            r"C:\Windows",
            r"C:\src",
            r"\\server\share",
            "/absolute/path",
            "A01/../../engine",
            r"A01\..\..\engine",
            "A01;whoami",
            "A01|calc",
            "A01 && dir",
        ];
        for test in &malicious {
            assert!(
                validate_lab_id(test).is_err(),
                "Malicious input '{}' should be rejected by validate_lab_id",
                test
            );
        }
    }

    #[test]
    fn test_safe_subpath_enforcement() {
        let base = Path::new("C:/zitera_lab/labs/A01");
        assert!(safe_subpath(base, "docker-compose.yml").is_ok());
        assert!(safe_subpath(base, "sub/dir/config.json").is_ok());

        assert!(safe_subpath(base, "../evil.yml").is_err());
        assert!(safe_subpath(base, "../../engine/src").is_err());
        assert!(safe_subpath(base, "/etc/passwd").is_err());
        assert!(safe_subpath(base, r"C:\Windows\System32").is_err());
        assert!(safe_subpath(base, r"\\share\file").is_err());
    }

    #[test]
    fn test_validate_repo_url_trusted() {
        assert!(validate_repo_url("https://github.com/xdaamar/zitera_lab_a01.git").is_ok());
        assert!(validate_repo_url("https://github.com/owner/valid-repo_123").is_ok());
    }

    #[test]
    fn test_validate_repo_url_untrusted() {
        assert!(validate_repo_url("file:///etc/passwd").is_err());
        assert!(validate_repo_url("ssh://git@github.com/owner/repo.git").is_err());
        assert!(validate_repo_url("http://github.com/owner/repo.git").is_err());
        assert!(validate_repo_url("https://evil.com/owner/repo.git").is_err());
        assert!(validate_repo_url("https://github.com/owner/repo;touch /tmp/pwn").is_err());
    }

    #[test]
    fn test_lifecycle_uninstalled_lab_fails_gracefully() {
        let dummy_root = Path::new("C:/dummy_zitera_nonexistent_workspace");
        let start_res = start_lab(dummy_root, "A99");
        assert!(start_res.is_err());
        assert!(start_res.unwrap_err().contains("not installed"));

        let reset_res = reset_lab(dummy_root, "A99");
        assert!(reset_res.is_err());
        assert!(reset_res.unwrap_err().contains("not installed"));

        let stop_res = stop_lab(dummy_root, "A99");
        assert!(stop_res.is_err());
        assert!(stop_res.unwrap_err().contains("not installed"));

        let remove_res = remove_lab(dummy_root, "A99");
        assert!(remove_res.is_ok());
        assert!(remove_res.unwrap().contains("not installed"));
    }

    #[test]
    fn test_incomplete_install_marked_uninstalled() {
        let temp_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("test_incomplete_lab");
        let lab_dir = temp_dir.join("labs").join("A01");
        let _ = fs::create_dir_all(&lab_dir);
        let status = get_lab_status_cached(&temp_dir, "A01", None, None, None);
        assert!(!status.installed);
        assert_eq!(status.status, "INCOMPLETE_INSTALL");
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_challenge_candidate_oversized_rejected() {
        let dummy_root = Path::new("C:/dummy_zitera_nonexistent_workspace");
        let huge_input = "A".repeat(600);
        let res = validate_challenge(dummy_root, "A01", &huge_input);
        assert!(res.is_ok());
        let verification = res.unwrap();
        assert_eq!(verification.status, "failed");
        assert!(verification.message.contains("exceeds 512 characters"));
    }

    #[test]
    fn test_timing_safe_compare() {
        assert!(timing_safe_compare("flag{zitera_123}", "flag{zitera_123}"));
        assert!(!timing_safe_compare("flag{zitera_123}", "flag{zitera_456}"));
        assert!(!timing_safe_compare("flag{zitera_123}", "short"));
    }

    #[test]
    #[cfg(windows)]
    fn test_a01_native_sandbox_lifecycle_and_challenge() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();

        let a01_bin = repo_root
            .join("labs")
            .join("A01")
            .join("bin")
            .join("a01-lab.exe");
        if !a01_bin.exists() {
            let built_bin = repo_root
                .join("engine")
                .join("rust")
                .join("target")
                .join("debug")
                .join("a01_lab.exe");
            if built_bin.exists() {
                let _ = fs::create_dir_all(a01_bin.parent().unwrap());
                let _ = fs::copy(&built_bin, &a01_bin);
            }
        }
        if !a01_bin.exists() {
            eprintln!("Skipping test: a01-lab.exe not deployed yet");
            return;
        }

        // Stop any previous run
        let _ = stop_lab(&repo_root, "A01");

        // 1. Initial status before start
        let st_init = get_lab_status(&repo_root, "A01");
        assert!(st_init.installed, "A01 must be installed");
        assert!(!st_init.running, "A01 should not be running initially");

        // 2. Start A01
        let start_res = start_lab(&repo_root, "A01");
        assert!(start_res.is_ok(), "start_lab must succeed: {:?}", start_res);

        // 3. Status when running
        let st_running = get_lab_status(&repo_root, "A01");
        assert!(st_running.running, "A01 must report running");
        assert!(st_running.url.is_some(), "A01 must have URL");
        let entry_url = st_running.url.unwrap();
        assert!(entry_url.starts_with("http://127.0.0.1:"));
        assert!(entry_url.contains("/session/"));

        // Helper to send HTTP via TCP socket to the broker port
        let port = st_running.port;
        let token = entry_url
            .split("/session/")
            .nth(1)
            .unwrap()
            .trim_end_matches('/');

        let send_req = |req_str: &str| -> String {
            use std::io::{Read, Write};
            let mut stream = std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
            stream.write_all(req_str.as_bytes()).unwrap();
            stream.flush().unwrap();
            let mut buf = Vec::new();
            let _ = stream.read_to_end(&mut buf);
            String::from_utf8_lossy(&buf).to_string()
        };

        // 4. Test GET / -> Login portal
        let req_root = format!(
            "GET /session/{}/ HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
            token, port
        );
        let resp_root = send_req(&req_root);
        assert!(resp_root.starts_with("HTTP/1.1 200 OK"));
        assert!(resp_root.contains("Internal Financial Accounting Portal"));

        // 5. Test POST /login -> Authenticate as alice
        let login_body = "username=alice&password=password123";
        let req_login = format!(
            "POST /session/{}/login HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nContent-Type: application/x-www-form-urlencoded\r\nContent-Length: {}\r\n\r\n{}",
            token, port, login_body.len(), login_body
        );
        let resp_login = send_req(&req_login);
        assert!(resp_login.starts_with("HTTP/1.1 302"));
        assert!(resp_login.contains("session_user=user_sess_1"));

        // 6. Test IDOR vulnerability: Access unowned Invoice #42
        let req_invoice42 = format!(
            "GET /session/{}/invoice/42 HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nCookie: session_user=user_sess_1\r\n\r\n",
            token, port
        );
        let resp_inv42 = send_req(&req_invoice42);
        assert!(resp_inv42.starts_with("HTTP/1.1 200 OK"));
        assert!(resp_inv42.contains("FLAG: ZITERA{b10k3n_4cc355_c0ntr01_m45t3r}"));

        // 7. Validate Challenge submission
        let verif =
            validate_challenge(&repo_root, "A01", "ZITERA{b10k3n_4cc355_c0ntr01_m45t3r}").unwrap();
        assert_eq!(
            verif.status, "passed",
            "Challenge flag must pass validation"
        );

        // 8. Stop A01
        let stop_res = stop_lab(&repo_root, "A01");
        assert!(stop_res.is_ok());

        // 9. Status after stop
        let st_stopped = get_lab_status(&repo_root, "A01");
        assert!(
            !st_stopped.running,
            "A01 must report stopped after stop_lab"
        );
    }

    #[test]
    #[cfg(windows)]
    fn test_a06_native_sandbox_lifecycle_and_challenge() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();

        let a06_bin = repo_root
            .join("labs")
            .join("A06")
            .join("bin")
            .join("a06-lab.exe");
        if !a06_bin.exists() {
            let built_bin = repo_root
                .join("engine")
                .join("rust")
                .join("target")
                .join("debug")
                .join("a06_lab.exe");
            if built_bin.exists() {
                let _ = fs::create_dir_all(a06_bin.parent().unwrap());
                let _ = fs::copy(&built_bin, &a06_bin);
            }
        }
        if !a06_bin.exists() {
            eprintln!("Skipping test: a06-lab.exe not deployed yet");
            return;
        }

        // Stop any previous run
        let _ = stop_lab(&repo_root, "A06");

        // 1. Initial status before start
        let st_init = get_lab_status(&repo_root, "A06");
        assert!(st_init.installed, "A06 must be installed");
        assert!(!st_init.running, "A06 should not be running initially");

        // 2. Start A06
        let start_res = start_lab(&repo_root, "A06");
        assert!(start_res.is_ok(), "start_lab must succeed: {:?}", start_res);

        // 3. Status when running
        let st_running = get_lab_status(&repo_root, "A06");
        assert!(st_running.running, "A06 must report running");
        assert!(st_running.url.is_some(), "A06 must have URL");
        let entry_url = st_running.url.unwrap();
        assert!(entry_url.starts_with("http://127.0.0.1:"));
        assert!(entry_url.contains("/session/"));

        let port = st_running.port;
        let token = entry_url
            .split("/session/")
            .nth(1)
            .unwrap()
            .trim_end_matches('/');

        let send_req = |req_str: &str| -> String {
            use std::io::{Read, Write};
            let mut stream = std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).unwrap();
            stream.write_all(req_str.as_bytes()).unwrap();
            stream.flush().unwrap();
            let mut buf = Vec::new();
            let _ = stream.read_to_end(&mut buf);
            String::from_utf8_lossy(&buf).to_string()
        };

        // 4. Test GET / -> Procurement portal
        let req_root = format!(
            "GET /session/{}/ HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
            token, port
        );
        let resp_root = send_req(&req_root);
        assert!(resp_root.starts_with("HTTP/1.1 200 OK"));
        assert!(resp_root.contains("Zitera Procurement Workflow"));
        assert!(resp_root.contains("Executive Datacenter Cluster"));

        // 5. Test Practice Verification before exploit -> Should report failed
        let req_verify_init = format!(
            "GET /session/{}/practice/verify HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
            token, port
        );
        let resp_verify_init = send_req(&req_verify_init);
        assert!(resp_verify_init.starts_with("HTTP/1.1 200 OK"));
        assert!(
            resp_verify_init.contains("\"status\":\"failed\"")
                || resp_verify_init.contains("\"status\": \"failed\"")
        );

        // 6. Test Insecure Design Flaw: Transition high-value order #9999 directly to DISPATCHED
        let transition_body = "{\"target_state\": \"DISPATCHED\"}";
        let req_transition = format!(
            "POST /session/{}/api/orders/9999/transition HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            token, port, transition_body.len(), transition_body
        );
        let resp_transition = send_req(&req_transition);
        assert!(resp_transition.starts_with("HTTP/1.1 200 OK"));
        assert!(resp_transition.contains("DISPATCHED"));
        assert!(resp_transition.contains("ZITERA{1n53cur3_d351gn_fl4w3d_w0rkfl0w}"));

        // 7. Test Practice Verification after exploit -> Should report passed
        let resp_verify_post = send_req(&req_verify_init);
        assert!(resp_verify_post.starts_with("HTTP/1.1 200 OK"));
        assert!(
            resp_verify_post.contains("\"status\":\"passed\"")
                || resp_verify_post.contains("\"status\": \"passed\"")
        );

        // 8. Validate Challenge submission
        let verif =
            validate_challenge(&repo_root, "A06", "ZITERA{1n53cur3_d351gn_fl4w3d_w0rkfl0w}")
                .unwrap();
        assert_eq!(
            verif.status, "passed",
            "Challenge flag must pass validation"
        );

        // 9. Stop A06
        let stop_res = stop_lab(&repo_root, "A06");
        assert!(stop_res.is_ok());

        // 10. Status after stop
        let st_stopped = get_lab_status(&repo_root, "A06");
        assert!(
            !st_stopped.running,
            "A06 must report stopped after stop_lab"
        );
    }
}
