use crate::docker;
use crate::models::{LabManifest, LabStatus};
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_labs_dir(workspace_root: &Path) -> PathBuf {
    workspace_root.join("labs")
}

pub fn get_lab_dir(workspace_root: &Path, lab_id: &str) -> PathBuf {
    get_labs_dir(workspace_root).join(lab_id.to_uppercase())
}

pub fn read_manifest(lab_dir: &Path) -> Result<LabManifest, String> {
    let manifest_path = lab_dir.join("manifest.json");
    if !manifest_path.exists() {
        return Err(format!("manifest.json not found in {:?}", lab_dir));
    }
    let data = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("Failed to read manifest at {:?}: {}", manifest_path, e))?;
    serde_json::from_str(&data).map_err(|e| format!("Failed to parse manifest JSON: {}", e))
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
        };
    }

    let lab_dir = get_lab_dir(workspace_root, lab_id);
    if !lab_dir.exists() {
        return LabStatus {
            id: lab_id.to_uppercase(),
            title: lab_id.to_string(),
            installed: false,
            running: false,
            port: 0,
            url: None,
            version: "N/A".to_string(),
            status: "NOT_INSTALLED".to_string(),
        };
    }

    match read_manifest(&lab_dir) {
        Ok(manifest) => {
            let is_running = docker::get_lab_container_status(lab_id);
            let status_str = if is_running { "RUNNING" } else { "STOPPED" };
            let url = if is_running {
                Some(format!("http://127.0.0.1:{}", manifest.default_port))
            } else {
                None
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
            }
        }
        Err(_) => LabStatus {
            id: lab_id.to_uppercase(),
            title: lab_id.to_string(),
            installed: true,
            running: false,
            port: 0,
            url: None,
            version: "UNKNOWN".to_string(),
            status: "INVALID_MANIFEST".to_string(),
        },
    }
}

pub fn list_all_labs(workspace_root: &Path) -> Vec<LabStatus> {
    let ids: Vec<String> = match crate::catalog::load_catalog(workspace_root) {
        Ok(cat) => cat.labs.into_iter().map(|l| l.id).collect(),
        Err(_) => vec!["A01".to_string(), "A05".to_string()],
    };
    ids.into_iter()
        .map(|id| get_lab_status(workspace_root, &id))
        .collect()
}

pub fn start_lab(workspace_root: &Path, lab_id: &str) -> Result<String, String> {
    validate_lab_id(lab_id)?;
    let lab_dir = get_lab_dir(workspace_root, lab_id);
    let manifest = read_manifest(&lab_dir)?;
    if !crate::system::is_port_available(manifest.default_port)
        && !docker::get_lab_container_status(lab_id)
    {
        return Err(format!(
            "Port {} is already in use by another process. Please free the port before starting lab {}.",
            manifest.default_port, lab_id
        ));
    }
    let compose_path = lab_dir.join(&manifest.entrypoint);
    docker::start_lab(&compose_path, &manifest.id)
}

pub fn stop_lab(workspace_root: &Path, lab_id: &str) -> Result<String, String> {
    validate_lab_id(lab_id)?;
    let lab_dir = get_lab_dir(workspace_root, lab_id);
    let manifest = read_manifest(&lab_dir)?;
    let compose_path = lab_dir.join(&manifest.entrypoint);
    docker::stop_lab(&compose_path, &manifest.id)
}

pub fn reset_lab(workspace_root: &Path, lab_id: &str) -> Result<String, String> {
    validate_lab_id(lab_id)?;
    let lab_dir = get_lab_dir(workspace_root, lab_id);
    let manifest = read_manifest(&lab_dir)?;
    let compose_path = lab_dir.join(&manifest.entrypoint);
    docker::reset_lab(&compose_path, &manifest.id)
}

pub fn install_lab(workspace_root: &Path, lab_id: &str) -> Result<String, String> {
    validate_lab_id(lab_id)?;
    let lab_dir = get_lab_dir(workspace_root, lab_id);
    if lab_dir.exists() && lab_dir.join("manifest.json").exists() {
        return Ok(format!("Lab {} is already installed.", lab_id));
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
    if docker::get_lab_container_status(lab_id) {
        return Err(format!(
            "Lab {} is currently running. Please stop the lab before updating.",
            lab_id
        ));
    }

    let lab_dir = get_lab_dir(workspace_root, lab_id);
    if !lab_dir.exists() {
        return Err(format!(
            "Lab {} is not installed. Install it first.",
            lab_id
        ));
    }
    let current_manifest = read_manifest(&lab_dir)?;
    let cat = crate::catalog::load_catalog(workspace_root)?;
    let item = cat
        .labs
        .into_iter()
        .find(|l| l.id.eq_ignore_ascii_case(lab_id))
        .ok_or_else(|| format!("Lab {} not found in catalog.", lab_id))?;

    if lab_dir.join(".git").exists() {
        let pull_out = crate::process::run_cmd("git", &["pull"], Some(&lab_dir))?;
        if !pull_out.success {
            return Err(format!("Failed to update lab via git: {}", pull_out.stderr));
        }
        let updated_manifest = read_manifest(&lab_dir)?;
        Ok(format!(
            "Lab {} updated from v{} to v{}.",
            lab_id, current_manifest.version, updated_manifest.version
        ))
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

    // Read lesson files
    let lesson_dir = lab_dir.join("lesson");
    let mut lessons = std::collections::HashMap::new();
    let lesson_keys = [
        "analogy",
        "concept",
        "remediation",
        "walkthrough",
        "introduction",
    ];

    for key in &lesson_keys {
        let file_path = lesson_dir.join(format!("{}.md", key));
        if file_path.exists() {
            if let Ok(text) = safe_read_file(&file_path) {
                lessons.insert(key.to_string(), text);
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

/// Authoritative challenge validator (Phases 18, 19).
/// Verifies user submission against the lab repository's official challenge flag.
pub fn validate_challenge(
    workspace_root: &Path,
    lab_id: &str,
    user_submission: &str,
) -> Result<crate::models::ChallengeVerification, String> {
    validate_lab_id(lab_id)?;
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

    let submission_clean = user_submission.trim();
    let expected_clean = expected_flag.trim();

    if submission_clean == expected_clean {
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
