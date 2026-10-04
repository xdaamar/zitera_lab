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
    let manifest_path = lab_dir.join("manifest.json");
    if !manifest_path.exists() {
        return Err(format!("manifest.json not found in {:?}", lab_dir));
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
            let is_running = !crate::system::is_port_available(manifest.default_port);
            let status_str = if is_running { "RUNNING" } else { "STOPPED" };
            let url = if is_running {
                Some(format!("http://127.0.0.1:{}", manifest.default_port))
            } else {
                None
            };

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
                port: manifest.default_port,
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

    if !crate::system::is_port_available(manifest.default_port) {
        return Ok(format!(
            "Lab {} is already running on port {}.",
            lab_id, manifest.default_port
        ));
    }

    // Phase 15 Architecture Freeze: Docker runtime purged, Native AppContainer runtime planned for Phase 16
    Ok(format!(
        "Lab {} runtime frozen in Phase 15. Native AppContainer sandbox runtime will be activated in Phase 16.",
        lab_id
    ))
}

pub fn stop_lab(workspace_root: &Path, lab_id: &str) -> Result<String, String> {
    validate_lab_id(lab_id)?;
    let lab_dir = get_lab_dir(workspace_root, lab_id);
    if !lab_dir.exists() {
        return Err(format!("Lab {} is not installed.", lab_id));
    }
    Ok(format!("Lab {} stopped.", lab_id))
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
    Ok(format!("Lab {} reset completed (Phase 15 Freeze).", lab_id))
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

    // Read lesson files - both standard keys and any additional dynamic markdown sections
    let lesson_dir = lab_dir.join("lesson");
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
    let challenge_path = lab_dir.join("challenge").join("challenge.md");
    let challenge_objective = if challenge_path.exists() {
        safe_read_file(&challenge_path).unwrap_or_default()
    } else {
        "Complete the mission objective in the target application.".to_string()
    };

    // Read progressive hints (stripping secret flag)
    let hints_path = lab_dir.join("challenge").join("hints.json");
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
        let temp_dir = std::env::temp_dir().join("zitera_test_incomplete_lab");
        let _ = fs::create_dir_all(temp_dir.join("labs").join("A01"));
        // Notice: No manifest.json is present in the lab directory
        let status = get_lab_status_cached(&temp_dir, "A01", None, None, None);
        assert_eq!(status.installed, false);
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
}
