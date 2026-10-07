//! Zitera Lab Package Installer and Atomic Update Engine
//!
//! Implements Contract V3 Versioned Installation & Update Model:
//! - Full pre-extraction security verification
//! - Anti-downgrade protection (security_version & version monotonic checks)
//! - Minimum core version compatibility validation
//! - Safe extraction to staging directory
//! - Atomic version activation (`labs/<ID>/active_version.txt`)
//! - Non-destructive rollback: old version remains intact upon any failure

use super::archive::extract_zlab_archive;
use super::verifier::verify_package;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs;
use std::path::{Path, PathBuf};

pub const CURRENT_CORE_VERSION: &str = "2.0.0";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallReport {
    pub lab_id: String,
    pub new_version: String,
    pub previous_version: Option<String>,
    pub package_sha256: String,
    pub status: String,
    pub active_path: String,
}

/// Parses a semantic version string into (major, minor, patch).
pub fn parse_semver(v: &str) -> (u32, u32, u32) {
    let clean = v.trim().trim_start_matches('v');
    let mut parts = clean.split('.');
    let major = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
    let patch = parts
        .next()
        .and_then(|p| p.split('-').next().and_then(|x| x.parse().ok()))
        .unwrap_or(0);
    (major, minor, patch)
}

/// Compares two semantic version strings.
pub fn compare_semver(v1: &str, v2: &str) -> Ordering {
    let (maj1, min1, pat1) = parse_semver(v1);
    let (maj2, min2, pat2) = parse_semver(v2);
    (maj1, min1, pat1).cmp(&(maj2, min2, pat2))
}

pub fn install_or_update_package(
    package_path: &Path,
    workspace_root: &Path,
) -> Result<InstallReport, String> {
    install_or_update_package_with_policy(package_path, workspace_root, false)
}

pub fn install_or_update_package_with_policy(
    package_path: &Path,
    workspace_root: &Path,
    allow_downgrade: bool,
) -> Result<InstallReport, String> {
    // 1. Verify package security before touching any lab state
    let verified = verify_package(package_path, None)?;
    let lab_id = verified.manifest.id.to_uppercase();
    let version = verified.manifest.version.clone();

    // Check minimum core version compatibility
    if let Some(ref min_core) = verified.manifest.minimum_core_version {
        if compare_semver(CURRENT_CORE_VERSION, min_core) == Ordering::Less {
            return Err(format!(
                "This lab update requires a newer ZITERA_LAB core (requires {}, current: {}). Please update ZITERA_LAB before installing this lab.",
                min_core, CURRENT_CORE_VERSION
            ));
        }
    }

    let lab_dir = workspace_root.join("labs").join(&lab_id);
    let _ = fs::create_dir_all(&lab_dir);

    // Anti-Downgrade Protection: check currently active installation
    let active_marker = lab_dir.join("active_version.txt");
    let prev_version = fs::read_to_string(&active_marker)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let installed_dir = resolve_effective_lab_dir(&lab_dir);
    let installed_manifest_path = installed_dir.join("manifest.json");
    if installed_manifest_path.exists() {
        if let Ok(content) = fs::read_to_string(&installed_manifest_path) {
            if let Ok(installed_manifest) =
                serde_json::from_str::<crate::models::LabManifest>(&content)
            {
                let installed_sec_ver = installed_manifest.security_version.unwrap_or(1);
                let incoming_sec_ver = verified.manifest.security_version.unwrap_or(1);

                // Anti-downgrade rule 1: security_version must never decrease
                if incoming_sec_ver < installed_sec_ver && !allow_downgrade {
                    return Err(format!(
                        "Anti-downgrade violation: incoming security version ({}) is lower than installed security version ({})",
                        incoming_sec_ver, installed_sec_ver
                    ));
                }

                // Anti-downgrade rule 2: package version must never decrease unless downgrade authorized
                if compare_semver(&verified.manifest.version, &installed_manifest.version)
                    == Ordering::Less
                    && !allow_downgrade
                {
                    return Err(format!(
                        "Downgrade rejected: incoming version ({}) is lower than installed version ({})",
                        verified.manifest.version, installed_manifest.version
                    ));
                }
            }
        }
    }

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

/// Scans available version directories and reconciles active_version.txt if it is missing,
/// corrupted, or pointing to a non-existent version directory.
pub fn reconcile_lab_version(lab_dir: &Path) -> Option<String> {
    let current_active = get_active_version(lab_dir);
    if let Some(ref ver) = current_active {
        let active_dir = lab_dir.join("versions").join(ver);
        if active_dir.join("manifest.json").exists() {
            return Some(ver.clone());
        }
    }

    // Active version is missing or invalid: search versions/ directory for the highest valid version
    let versions_dir = lab_dir.join("versions");
    if !versions_dir.exists() {
        return None;
    }

    let mut valid_versions: Vec<String> = Vec::new();
    if let Ok(entries) = fs::read_dir(&versions_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() && entry.path().join("manifest.json").exists() {
                if let Some(name) = entry.file_name().to_str() {
                    valid_versions.push(name.to_string());
                }
            }
        }
    }

    if valid_versions.is_empty() {
        return None;
    }

    // Sort descending by semantic version
    valid_versions.sort_by(|a, b| compare_semver(b, a));
    let recovered_version = valid_versions[0].clone();

    // Reconcile marker and mirror
    let _ = fs::write(lab_dir.join("active_version.txt"), &recovered_version);
    let recovered_dir = versions_dir.join(&recovered_version);
    let _ = copy_dir_all(&recovered_dir, lab_dir);

    Some(recovered_version)
}

/// Returns the effective root directory for a lab (pointing to active version if set).
pub fn resolve_effective_lab_dir(lab_dir: &Path) -> PathBuf {
    if let Some(active) = reconcile_lab_version(lab_dir) {
        let version_path = lab_dir.join("versions").join(&active);
        if version_path.join("manifest.json").exists() {
            return version_path;
        }
    }
    lab_dir.to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package::archive::create_zlab_package;
    use crate::package::trust::{
        build_canonical_package_payload, sign_payload, DEV_PRIVATE_KEY_SEED,
    };
    use crate::package::verifier::compute_archive_content_digest;

    fn build_test_package_signed(dest_pkg: &Path, id: &str, version: &str, sec_version: u32) {
        let temp_dir = dest_pkg.with_extension("stage_build");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(temp_dir.join("bin")).unwrap();
        fs::write(temp_dir.join("bin").join("lab.exe"), "binary").unwrap();

        // 1. Prelim archive to calculate digest
        let prelim_pkg = dest_pkg.with_extension("prelim.zlab");
        let dummy = r#"{"schema_version":1,"id":"A05","slug":"inj","title":"Inj","owasp":"A05","version":"1.0.0","difficulty":"B","runtime":"native_sandboxed","entrypoint":"bin/lab.exe","default_port":8015,"estimated_minutes":30,"modes":["learn"],"signature":"unsigned"}"#;
        fs::write(temp_dir.join("manifest.json"), dummy).unwrap();
        create_zlab_package(&temp_dir, &prelim_pkg).unwrap();

        let bytes = fs::read(&prelim_pkg).unwrap();
        let entries = super::super::archive::validate_archive_structure(&bytes).unwrap();
        let digest = compute_archive_content_digest(&bytes, &entries).unwrap();
        let _ = fs::remove_file(&prelim_pkg);

        // Sign canonical payload
        let canonical_payload = build_canonical_package_payload(
            id,
            version,
            sec_version,
            "2.0.0",
            "native_sandboxed",
            "x86_64",
            "bin/lab.exe",
            &digest,
        );
        let sig = sign_payload(&canonical_payload, &DEV_PRIVATE_KEY_SEED);

        let manifest = format!(
            r#"{{
            "schema_version": 1,
            "id": "{}",
            "slug": "inj",
            "title": "Injection",
            "owasp": "A05:2025",
            "version": "{}",
            "security_version": {},
            "minimum_core_version": "2.0.0",
            "difficulty": "Beginner",
            "runtime": "native_sandboxed",
            "entrypoint": "bin/lab.exe",
            "default_port": 8015,
            "estimated_minutes": 30,
            "modes": ["learn"],
            "signature": "{}"
        }}"#,
            id, version, sec_version, sig
        );
        fs::write(temp_dir.join("manifest.json"), manifest).unwrap();
        create_zlab_package(&temp_dir, dest_pkg).unwrap();
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_checkpoint_4_anti_downgrade_matrix() {
        let temp = std::env::temp_dir().join("zitera_anti_downgrade_test");
        let _ = fs::remove_dir_all(&temp);
        let workspace = temp.join("workspace");
        fs::create_dir_all(&workspace).unwrap();

        let pkg_v100_s1 = temp.join("A05_1.0.0_s1.zlab");
        let pkg_v101_s1 = temp.join("A05_1.0.1_s1.zlab");
        let pkg_v101_s2 = temp.join("A05_1.0.1_s2.zlab");
        let pkg_v110_s2 = temp.join("A05_1.1.0_s2.zlab");
        let pkg_v101_s5 = temp.join("A05_1.0.1_s5.zlab");
        let pkg_v101_s3 = temp.join("A05_1.0.1_s3.zlab");

        build_test_package_signed(&pkg_v100_s1, "A05", "1.0.0", 1);
        build_test_package_signed(&pkg_v101_s1, "A05", "1.0.1", 1);
        build_test_package_signed(&pkg_v101_s2, "A05", "1.0.1", 2);
        build_test_package_signed(&pkg_v110_s2, "A05", "1.1.0", 2);
        build_test_package_signed(&pkg_v101_s5, "A05", "1.0.1", 5);
        build_test_package_signed(&pkg_v101_s3, "A05", "1.0.1", 3);

        // 1. Initial install 1.0.0 (security_version = 1) -> PASS
        let rep1 = install_or_update_package(&pkg_v100_s1, &workspace);
        assert!(rep1.is_ok(), "Initial install must succeed: {:?}", rep1);
        assert_eq!(rep1.unwrap().new_version, "1.0.0");

        // 2. 1.0.0 -> 1.0.1 (security_version = 1) -> PASS
        let rep2 = install_or_update_package(&pkg_v101_s1, &workspace);
        assert!(
            rep2.is_ok(),
            "1.0.0 -> 1.0.1 upgrade must succeed: {:?}",
            rep2
        );
        assert_eq!(rep2.unwrap().new_version, "1.0.1");

        // 3. 1.0.1 -> 1.1.0 (security_version = 2) -> PASS
        let rep3 = install_or_update_package(&pkg_v110_s2, &workspace);
        assert!(
            rep3.is_ok(),
            "1.0.1 -> 1.1.0 upgrade must succeed: {:?}",
            rep3
        );
        assert_eq!(rep3.unwrap().new_version, "1.1.0");

        // 4. 1.1.0 -> 1.0.1 with security_version 2 (version downgrade attempt) -> REJECT
        let rep4 = install_or_update_package(&pkg_v101_s2, &workspace);
        assert!(rep4.is_err(), "1.1.0 -> 1.0.1 downgrade must be rejected");
        assert!(rep4.unwrap_err().contains("Downgrade rejected"));

        // 5. Test security_version downgrade:
        // Install security_version 5 (with downgrade allowed for test setup)
        let rep5 = install_or_update_package_with_policy(&pkg_v101_s5, &workspace, true);
        assert!(rep5.is_ok());

        // Now attempt normal update with security_version 3 -> REJECT
        let rep6 = install_or_update_package(&pkg_v101_s3, &workspace);
        assert!(
            rep6.is_err(),
            "security_version 5 -> 3 rollback must be rejected"
        );
        assert!(rep6.unwrap_err().contains("Anti-downgrade violation"));

        // 6. Forged version with old signature -> REJECT at verification layer
        let tampered_pkg = temp.join("A05_forged.zlab");
        let stage_forged = temp.join("stage_forged");
        fs::create_dir_all(&stage_forged).unwrap();
        fs::create_dir_all(stage_forged.join("bin")).unwrap();
        fs::write(stage_forged.join("bin").join("lab.exe"), "binary").unwrap();
        // Manifest claiming 2.0.0 but using signature for 1.0.0
        let forged_manifest = r#"{
            "schema_version": 1,
            "id": "A05",
            "slug": "inj",
            "title": "Injection",
            "owasp": "A05:2025",
            "version": "2.0.0",
            "security_version": 10,
            "minimum_core_version": "2.0.0",
            "difficulty": "Beginner",
            "runtime": "native_sandboxed",
            "entrypoint": "bin/lab.exe",
            "default_port": 8015,
            "estimated_minutes": 30,
            "modes": ["learn"],
            "signature": "3a48e7badsignature1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
        }"#;
        fs::write(stage_forged.join("manifest.json"), forged_manifest).unwrap();
        create_zlab_package(&stage_forged, &tampered_pkg).unwrap();

        let rep_forged = install_or_update_package(&tampered_pkg, &workspace);
        assert!(
            rep_forged.is_err(),
            "Forged version with invalid signature must be rejected"
        );

        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_checkpoint_4_incompatible_core_version_rejection() {
        let temp = std::env::temp_dir().join("zitera_incompatible_core_test");
        let _ = fs::remove_dir_all(&temp);
        let workspace = temp.join("workspace");
        fs::create_dir_all(&workspace).unwrap();

        let pkg_future_core = temp.join("A01_future.zlab");
        let stage = temp.join("stage_future");
        let _ = fs::create_dir_all(&stage);
        let _ = fs::create_dir_all(stage.join("bin"));
        fs::write(stage.join("bin").join("lab.exe"), b"binary").unwrap();

        // Prelim archive to calculate digest
        let prelim_pkg = temp.join("prelim_future.zlab");
        let dummy = r#"{"schema_version":1,"id":"A01","slug":"a01","title":"A01","owasp":"A01","version":"1.0.0","difficulty":"B","runtime":"native_sandboxed","entrypoint":"bin/lab.exe","default_port":8011,"estimated_minutes":30,"modes":["learn"],"signature":"unsigned"}"#;
        fs::write(stage.join("manifest.json"), dummy).unwrap();
        create_zlab_package(&stage, &prelim_pkg).unwrap();

        let bytes = fs::read(&prelim_pkg).unwrap();
        let entries = super::super::archive::validate_archive_structure(&bytes).unwrap();
        let digest = compute_archive_content_digest(&bytes, &entries).unwrap();
        let _ = fs::remove_file(&prelim_pkg);

        // Package declares minimum_core_version = 99.0.0 (far newer than current 2.0.0)
        let canonical = build_canonical_package_payload(
            "A01",
            "1.0.0",
            1,
            "99.0.0",
            "native_sandboxed",
            "x86_64",
            "bin/lab.exe",
            &digest,
        );
        let sig = sign_payload(&canonical, &DEV_PRIVATE_KEY_SEED);

        let manifest = format!(
            r#"{{
            "schema_version": 1,
            "id": "A01",
            "slug": "a01",
            "title": "A01",
            "owasp": "A01:2025",
            "version": "1.0.0",
            "security_version": 1,
            "minimum_core_version": "99.0.0",
            "difficulty": "Beginner",
            "runtime": "native_sandboxed",
            "entrypoint": "bin/lab.exe",
            "default_port": 8011,
            "estimated_minutes": 30,
            "modes": ["learn"],
            "signature": "{}"
        }}"#,
            sig
        );
        fs::write(stage.join("manifest.json"), manifest).unwrap();
        create_zlab_package(&stage, &pkg_future_core).unwrap();
        let _ = fs::remove_dir_all(&stage);

        let install_res = install_or_update_package(&pkg_future_core, &workspace);
        assert!(install_res.is_err());
        let err = install_res.unwrap_err();
        assert!(err.contains("This lab update requires a newer ZITERA_LAB core"));
        assert!(err.contains("requires 99.0.0, current: 2.0.0"));

        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_checkpoint_6_crash_recovery_and_auto_reconciliation() {
        let temp = std::env::temp_dir().join("zitera_crash_recovery_test");
        let _ = fs::remove_dir_all(&temp);
        let workspace = temp.join("workspace");
        fs::create_dir_all(&workspace).unwrap();

        let pkg_v1 = temp.join("A01_v1.zlab");
        let pkg_v2 = temp.join("A01_v2.zlab");

        build_test_package_signed(&pkg_v1, "A01", "1.0.0", 1);
        build_test_package_signed(&pkg_v2, "A01", "1.0.1", 1);

        // Install 1.0.0 and 1.0.1
        let _ = install_or_update_package(&pkg_v1, &workspace).unwrap();
        let _ = install_or_update_package(&pkg_v2, &workspace).unwrap();

        let lab_dir = workspace.join("labs").join("A01");
        assert_eq!(get_active_version(&lab_dir), Some("1.0.1".to_string()));

        // Simulate crash/corruption 1: active_version.txt deleted
        let _ = fs::remove_file(lab_dir.join("active_version.txt"));
        assert_eq!(get_active_version(&lab_dir), None);

        // Self-healing: resolve_effective_lab_dir must auto-reconcile and recover 1.0.1
        let effective = resolve_effective_lab_dir(&lab_dir);
        assert_eq!(get_active_version(&lab_dir), Some("1.0.1".to_string()));
        assert!(effective.ends_with("1.0.1"));
        assert!(effective.join("manifest.json").exists());

        // Simulate crash/corruption 2: active_version.txt points to non-existent version
        fs::write(lab_dir.join("active_version.txt"), "9.9.9_corrupt").unwrap();
        let effective2 = resolve_effective_lab_dir(&lab_dir);
        assert_eq!(get_active_version(&lab_dir), Some("1.0.1".to_string()));
        assert!(effective2.ends_with("1.0.1"));
        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_checkpoint_17_clean_room_install_proof() {
        let temp = std::env::temp_dir().join("zitera_clean_room_install_proof");
        let _ = fs::remove_dir_all(&temp);
        let isolated_workspace = temp.join("clean_workspace");
        fs::create_dir_all(&isolated_workspace).unwrap();

        // 1. Build authentic signed release package for A01
        let pkg_v1 = temp.join("A01_release_v1.zlab");
        let pkg_v2 = temp.join("A01_release_v2.zlab");
        let pkg_tampered = temp.join("A01_tampered.zlab");

        build_test_package_signed(&pkg_v1, "A01", "1.0.1", 1);
        build_test_package_signed(&pkg_v2, "A01", "1.0.2", 2);

        // 2. Fresh clean-room installation into completely empty workspace
        let install_rep = install_or_update_package(&pkg_v1, &isolated_workspace)
            .expect("Clean-room fresh install must succeed");
        assert_eq!(install_rep.lab_id, "A01");
        assert_eq!(install_rep.new_version, "1.0.1");

        let lab_dir = isolated_workspace.join("labs").join("A01");
        assert!(lab_dir.exists(), "Target lab directory must exist");
        assert_eq!(get_active_version(&lab_dir), Some("1.0.1".to_string()));
        let effective_dir = resolve_effective_lab_dir(&lab_dir);
        assert!(effective_dir.join("manifest.json").exists());
        assert!(effective_dir.join("bin").join("lab.exe").exists());

        // 3. Clean-room atomic upgrade to v1.0.2
        let upgrade_rep = install_or_update_package(&pkg_v2, &isolated_workspace)
            .expect("Clean-room upgrade must succeed");
        assert_eq!(upgrade_rep.new_version, "1.0.2");
        assert_eq!(get_active_version(&lab_dir), Some("1.0.2".to_string()));
        let upgraded_effective = resolve_effective_lab_dir(&lab_dir);
        assert!(upgraded_effective.ends_with("1.0.2"));

        // 4. Attempt installing tampered package -> REJECTED, previous state preserved
        fs::copy(&pkg_v1, &pkg_tampered).unwrap();
        // Tamper bytes
        let mut tampered_bytes = fs::read(&pkg_tampered).unwrap();
        if tampered_bytes.len() > 100 {
            tampered_bytes[50] ^= 0xff;
        }
        fs::write(&pkg_tampered, tampered_bytes).unwrap();

        let fail_res = install_or_update_package(&pkg_tampered, &isolated_workspace);
        assert!(fail_res.is_err(), "Tampered package must be rejected");

        // 5. Verification that previous version remains usable and intact
        assert_eq!(get_active_version(&lab_dir), Some("1.0.2".to_string()));
        assert!(upgraded_effective.join("manifest.json").exists());

        let _ = fs::remove_dir_all(&temp);
    }
}
