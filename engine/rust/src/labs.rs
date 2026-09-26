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

    fs::create_dir_all(get_labs_dir(workspace_root))
        .map_err(|e| format!("Failed to create labs directory: {}", e))?;

    let clone_out = crate::process::run_cmd(
        "git",
        &["clone", &repo_url, lab_dir.to_str().unwrap_or("")],
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
