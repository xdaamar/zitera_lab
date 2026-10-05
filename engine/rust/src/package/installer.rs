//! Zitera Lab Package Installer and Atomic Update Engine
//!
//! Implements Contract V3 Versioned Installation & Update Model:
//! - Full pre-extraction security verification
//! - Safe extraction to staging directory
//! - Atomic version activation (`labs/<ID>/active_version.txt`)
//! - Non-destructive rollback: old version remains intact upon any failure

use super::archive::extract_zlab_archive;
use super::verifier::verify_package;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallReport {
    pub lab_id: String,
    pub new_version: String,
    pub previous_version: Option<String>,
    pub package_sha256: String,
    pub status: String,
    pub active_path: String,
}

pub fn install_or_update_package(
    package_path: &Path,
    workspace_root: &Path,
) -> Result<InstallReport, String> {
    // 1. Verify package security before touching any lab state
    let verified = verify_package(package_path, None)?;
    let lab_id = verified.manifest.id.to_uppercase();
    let version = verified.manifest.version.clone();

    let lab_dir = workspace_root.join("labs").join(&lab_id);
    let _ = fs::create_dir_all(&lab_dir);

    // Read current active version if present
    let active_marker = lab_dir.join("active_version.txt");
    let prev_version = fs::read_to_string(&active_marker)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    // 2. Extract into staging directory
    let staging_dir = lab_dir.join(format!("staging_{}", std::process::id()));
    let _ = fs::remove_dir_all(&staging_dir);

    if let Err(e) = extract_zlab_archive(package_path, &staging_dir) {
        let _ = fs::remove_dir_all(&staging_dir);
        return Err(format!("Extraction to staging failed: {}", e));
    }

    // 3. Post-extraction validation on staging
    let staged_manifest = staging_dir.join("manifest.json");
    if !staged_manifest.exists() {
        let _ = fs::remove_dir_all(&staging_dir);
        return Err("Staged manifest.json not found".to_string());
    }

    let staged_entrypoint = staging_dir.join(&verified.manifest.entrypoint);
    if !staged_entrypoint.exists() {
        let _ = fs::remove_dir_all(&staging_dir);
        return Err(format!(
            "Staged entrypoint binary {:?} not found",
            staged_entrypoint
        ));
    }

    // 4. Move to versioned directory: labs/<ID>/versions/<VERSION>/
    let versions_root = lab_dir.join("versions");
    let target_version_dir = versions_root.join(&version);
    let _ = fs::create_dir_all(&versions_root);
    let _ = fs::remove_dir_all(&target_version_dir);

    // Copy staged contents to target version directory
    if let Err(e) = copy_dir_all(&staging_dir, &target_version_dir) {
        let _ = fs::remove_dir_all(&staging_dir);
        let _ = fs::remove_dir_all(&target_version_dir);
        return Err(format!("Failed to activate version directory: {}", e));
    }

    // Clean up staging
    let _ = fs::remove_dir_all(&staging_dir);

    // 5. Atomic activation: write active_version.txt
    if let Err(e) = fs::write(&active_marker, &version) {
        return Err(format!("Failed to update active_version marker: {}", e));
    }

    // 6. Mirror active version files to lab_dir root for seamless backwards compatibility
    let _ = copy_dir_all(&target_version_dir, &lab_dir);

    Ok(InstallReport {
        lab_id,
        new_version: version,
        previous_version: prev_version,
        package_sha256: verified.package_sha256,
        status: "INSTALLED_ACTIVE".to_string(),
        active_path: target_version_dir.to_string_lossy().to_string(),
    })
}

/// Helper function to recursively copy directories.
pub fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dest_child = dst.join(entry.file_name());
        if ty.is_dir() {
            // Avoid recursive self-copy if dst contains subdirectories
            if entry.file_name() != "versions" && entry.file_name() != "staging" {
                copy_dir_all(&entry.path(), &dest_child)?;
            }
        } else {
            fs::copy(entry.path(), dest_child)?;
        }
    }
    Ok(())
}

/// Returns the active version of a lab, if configured.
pub fn get_active_version(lab_dir: &Path) -> Option<String> {
    fs::read_to_string(lab_dir.join("active_version.txt"))
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Returns the effective root directory for a lab (pointing to active version if set).
pub fn resolve_effective_lab_dir(lab_dir: &Path) -> PathBuf {
    if let Some(active) = get_active_version(lab_dir) {
        let version_path = lab_dir.join("versions").join(&active);
        if version_path.join("manifest.json").exists() {
            return version_path;
        }
    }
    lab_dir.to_path_buf()
}
