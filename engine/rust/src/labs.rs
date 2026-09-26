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

pub fn get_lab_status(workspace_root: &Path, lab_id: &str) -> LabStatus {
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
    let known_ids = vec!["A01", "A05"];
    known_ids
        .into_iter()
        .map(|id| get_lab_status(workspace_root, id))
        .collect()
}

pub fn start_lab(workspace_root: &Path, lab_id: &str) -> Result<String, String> {
    let lab_dir = get_lab_dir(workspace_root, lab_id);
    let manifest = read_manifest(&lab_dir)?;
    if !crate::system::is_port_available(manifest.default_port) && !docker::get_lab_container_status(lab_id) {
        return Err(format!(
            "Port {} is already in use by another process. Please free the port before starting lab {}.",
            manifest.default_port, lab_id
        ));
    }
    let compose_path = lab_dir.join(&manifest.entrypoint);
    docker::start_lab(&compose_path, &manifest.id)
}

pub fn stop_lab(workspace_root: &Path, lab_id: &str) -> Result<String, String> {
    let lab_dir = get_lab_dir(workspace_root, lab_id);
    let manifest = read_manifest(&lab_dir)?;
    let compose_path = lab_dir.join(&manifest.entrypoint);
    docker::stop_lab(&compose_path, &manifest.id)
}

pub fn reset_lab(workspace_root: &Path, lab_id: &str) -> Result<String, String> {
    let lab_dir = get_lab_dir(workspace_root, lab_id);
    let manifest = read_manifest(&lab_dir)?;
    let compose_path = lab_dir.join(&manifest.entrypoint);
    docker::reset_lab(&compose_path, &manifest.id)
}
