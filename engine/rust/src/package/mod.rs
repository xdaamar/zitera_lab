//! Zitera Lab Package Format and Atomic Update Engine (Contract V3)
//!
//! Provides cryptographically verified, zero-recompile package updates (.zlab).

pub mod archive;
pub mod installer;
pub mod sha256;
pub mod trust;
pub mod verifier;

pub use archive::{create_zlab_package, extract_zlab_archive, validate_archive_structure};
pub use installer::{
    activate_version, install_or_update_package, reconcile_lab_version, resolve_effective_lab_dir,
    rollback_to_previous_version,
};
pub use trust::{
    build_canonical_package_payload, sign_payload, verify_signature, verify_with_trusted_keys,
    DEV_PRIVATE_KEY_SEED, DEV_PUBLIC_KEY_HEX, RELEASE_PUBLIC_KEY_HEX,
};
pub use verifier::verify_package;
use std::path::Path;

/// Builds and cryptographically signs a production-ready .zlab package from a source directory.
///
/// Ensures deterministic file ordering, calculates content digest across all resources,
/// signs the canonical payload using Ed25519, and verifies the generated package.
pub fn build_signed_package_from_dir(
    source_dir: &Path,
    output_zlab: &Path,
    signer_seed: &[u8; 32],
) -> Result<verifier::VerifiedPackage, String> {
    if !source_dir.exists() {
        return Err(format!("Source directory {:?} does not exist", source_dir));
    }
    let manifest_file = source_dir.join("manifest.json");
    if !manifest_file.exists() {
        return Err(format!("manifest.json not found in {:?}", source_dir));
    }

    let manifest_str = std::fs::read_to_string(&manifest_file)
        .map_err(|e| format!("Failed to read manifest.json: {}", e))?;
    let mut manifest_val: serde_json::Value = serde_json::from_str(&manifest_str)
        .map_err(|e| format!("Failed to parse manifest.json: {}", e))?;

    let lab_id = manifest_val.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let version = manifest_val.get("version").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let sec_version = manifest_val.get("security_version").and_then(|v| v.as_u64()).unwrap_or(1) as u32;
    let min_core = manifest_val.get("minimum_core_version").and_then(|v| v.as_str()).unwrap_or("2.0.0").to_string();
    let runtime = manifest_val.get("runtime").and_then(|v| v.as_str()).unwrap_or("native_sandboxed").to_string();
    let target_arch = manifest_val.get("target_arch").and_then(|v| v.as_str()).unwrap_or("x86_64").to_string();
    let entrypoint = manifest_val.get("entrypoint").and_then(|v| v.as_str()).unwrap_or("bin/lab.exe").to_string();

    let pid = std::process::id();
    let rand_nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let stage_dir = std::env::temp_dir().join(format!("zlab_pack_stage_{}_{}", pid, rand_nonce));
    let _ = std::fs::remove_dir_all(&stage_dir);
    std::fs::create_dir_all(&stage_dir)
        .map_err(|e| format!("Failed to create staging directory: {}", e))?;

    fn copy_filtered(src: &Path, dst: &Path) -> Result<(), String> {
        for entry in std::fs::read_dir(src).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') || name == ".runtime.json" || name == "versions" || name == "active_version.txt" {
                continue;
            }
            let target = dst.join(&name);
            if path.is_dir() {
                std::fs::create_dir_all(&target).map_err(|e| e.to_string())?;
                copy_filtered(&path, &target)?;
            } else {
                std::fs::copy(&path, &target).map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    if let Err(e) = copy_filtered(source_dir, &stage_dir) {
        let _ = std::fs::remove_dir_all(&stage_dir);
        return Err(format!("Failed to copy source files: {}", e));
    }

    // 1. Create preliminary archive to calculate content digest
    let prelim_pkg = stage_dir.join("_prelim.zlab");
    if let Err(e) = create_zlab_package(&stage_dir, &prelim_pkg) {
        let _ = std::fs::remove_dir_all(&stage_dir);
        return Err(format!("Failed to build preliminary archive: {}", e));
    }

    let bytes = std::fs::read(&prelim_pkg).map_err(|e| e.to_string())?;
    let entries = validate_archive_structure(&bytes).map_err(|e| e.to_string())?;
    let digest = verifier::compute_archive_content_digest(&bytes, &entries)?;
    let _ = std::fs::remove_file(&prelim_pkg);

    // 2. Canonicalize payload and sign
    let canonical_payload = build_canonical_package_payload(
        &lab_id,
        &version,
        sec_version,
        &min_core,
        &runtime,
        &target_arch,
        &entrypoint,
        &digest,
    );
    let signature = sign_payload(&canonical_payload, signer_seed);

    // 3. Inject signature, security_version, and minimum_core_version into manifest
    manifest_val["signature"] = serde_json::Value::String(signature);
    manifest_val["security_version"] = serde_json::Value::Number(serde_json::Number::from(sec_version));
    manifest_val["minimum_core_version"] = serde_json::Value::String(min_core);

    let final_manifest_str = serde_json::to_string_pretty(&manifest_val)
        .map_err(|e| format!("Failed to serialize manifest: {}", e))?;
    std::fs::write(stage_dir.join("manifest.json"), final_manifest_str)
        .map_err(|e| format!("Failed to write final manifest: {}", e))?;

    // 4. Ensure destination parent directory exists
    if let Some(parent) = output_zlab.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    // 5. Build final deterministic package
    let create_res = create_zlab_package(&stage_dir, output_zlab);
    let _ = std::fs::remove_dir_all(&stage_dir);
    create_res?;

    // 6. Authoritative verification of generated package
    let verified = verifier::verify_package(output_zlab, Some(&lab_id))?;
    Ok(verified)
}

#[cfg(test)]
mod tests {
    use super::installer::*;
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn setup_package_env(name: &str) -> (PathBuf, PathBuf) {
        let temp = std::env::temp_dir().join(format!("zitera_pkg_test_{}", name));
        let _ = fs::remove_dir_all(&temp);
        let _ = fs::create_dir_all(&temp);
        let workspace = temp.join("workspace");
        let _ = fs::create_dir_all(&workspace);
        (temp, workspace)
    }


    fn build_test_package(pkg_path: &std::path::Path, id: &str, version: &str, lesson_text: &str) {
        let stage = pkg_path.with_extension("stage_dir");
        let _ = fs::create_dir_all(&stage);
        let _ = fs::create_dir_all(stage.join("bin"));
        let _ = fs::create_dir_all(stage.join("lesson"));

        fs::write(
            stage.join("bin").join("a01-lab.exe"),
            format!("binary payload for {} v{}", id, version),
        )
        .unwrap();
        fs::write(
            stage.join("lesson").join("intro.md"),
            format!("Lesson content: {}", lesson_text),
        )
        .unwrap();

        // 1. Prelim archive to calculate content digest
        let prelim_pkg = pkg_path.with_extension("prelim.zlab");
        let dummy = r#"{"schema_version":1,"id":"A01","slug":"broken-access-control","title":"Broken Access Control","owasp":"A01:2025","version":"1.0.0","difficulty":"Beginner","runtime":"native_sandboxed","entrypoint":"bin/a01-lab.exe","default_port":8011,"estimated_minutes":45,"modes":["learn"],"signature":"unsigned"}"#;
        fs::write(stage.join("manifest.json"), dummy).unwrap();
        create_zlab_package(&stage, &prelim_pkg).unwrap();

        let bytes = fs::read(&prelim_pkg).unwrap();
        let entries = validate_archive_structure(&bytes).unwrap();
        let digest = verifier::compute_archive_content_digest(&bytes, &entries).unwrap();
        let _ = fs::remove_file(&prelim_pkg);

        // 2. Sign canonical payload using Ed25519 dev seed
        let canonical = build_canonical_package_payload(
            id,
            version,
            1,
            "2.0.0",
            "native_sandboxed",
            "x86_64",
            "bin/a01-lab.exe",
            &digest,
        );
        let sig = sign_payload(&canonical, &DEV_PRIVATE_KEY_SEED);

        let manifest = format!(
            r#"{{
            "schema_version": 1,
            "id": "{}",
            "slug": "broken-access-control",
            "title": "Broken Access Control",
            "owasp": "A01:2025",
            "version": "{}",
            "security_version": 1,
            "minimum_core_version": "2.0.0",
            "difficulty": "Beginner",
            "runtime": "native_sandboxed",
            "entrypoint": "bin/a01-lab.exe",
            "default_port": 8011,
            "estimated_minutes": 45,
            "modes": ["learn", "practice", "challenge"],
            "lesson": "{}",
            "signature": "{}"
        }}"#,
            id, version, lesson_text, sig
        );
        fs::write(stage.join("manifest.json"), manifest).unwrap();

        create_zlab_package(&stage, pkg_path).unwrap();
        let _ = fs::remove_dir_all(&stage);
    }

    #[test]
    fn test_checkpoint_8_package_update_and_atomic_rollback() {
        let (temp, workspace) = setup_package_env("cp8_proof");

        // Step 1: Create A01 1.0.0 package
        let pkg_v1 = temp.join("A01_1.0.0.zlab");
        build_test_package(&pkg_v1, "A01", "1.0.0", "Lesson Version 1.0.0 Content");

        // Step 2: Install 1.0.0
        let res_v1 =
            install_or_update_package(&pkg_v1, &workspace).expect("Install v1.0.0 should succeed");
        assert_eq!(res_v1.new_version, "1.0.0");
        assert_eq!(res_v1.status, "INSTALLED_ACTIVE");

        let lab_dir = workspace.join("labs").join("A01");
        assert_eq!(get_active_version(&lab_dir), Some("1.0.0".to_string()));

        let effective_v1 = resolve_effective_lab_dir(&lab_dir);
        let lesson_v1 = fs::read_to_string(effective_v1.join("lesson").join("intro.md")).unwrap();
        assert!(lesson_v1.contains("Lesson Version 1.0.0 Content"));

        // Step 3: Create A01 1.0.1 package with modified visible lesson content
        let pkg_v2 = temp.join("A01_1.0.1.zlab");
        build_test_package(
            &pkg_v2,
            "A01",
            "1.0.1",
            "Updated Lesson Version 1.0.1 Content",
        );

        // Step 4: Perform One-Click Update 1.0.0 -> 1.0.1
        let res_v2 = install_or_update_package(&pkg_v2, &workspace)
            .expect("Update to v1.0.1 should succeed");
        assert_eq!(res_v2.new_version, "1.0.1");
        assert_eq!(res_v2.previous_version, Some("1.0.0".to_string()));
        assert_eq!(get_active_version(&lab_dir), Some("1.0.1".to_string()));

        let effective_v2 = resolve_effective_lab_dir(&lab_dir);
        let lesson_v2 = fs::read_to_string(effective_v2.join("lesson").join("intro.md")).unwrap();
        assert!(lesson_v2.contains("Updated Lesson Version 1.0.1 Content"));

        // Step 5: Verify old version 1.0.0 is still preserved in versions/1.0.0
        let preserved_v1 = lab_dir
            .join("versions")
            .join("1.0.0")
            .join("lesson")
            .join("intro.md");
        assert!(preserved_v1.exists(), "Old version 1.0.0 must be preserved");

        // Step 6: Test Corruption 1: Corrupted manifest schema
        let bad_manifest_pkg = temp.join("A01_bad_manifest.zlab");
        let bad_stage = temp.join("bad_stage");
        fs::create_dir_all(&bad_stage).unwrap();
        fs::create_dir_all(bad_stage.join("bin")).unwrap();
        fs::write(bad_stage.join("bin").join("a01-lab.exe"), "bin").unwrap();
        fs::write(bad_stage.join("manifest.json"), "{ NOT VALID JSON").unwrap();
        create_zlab_package(&bad_stage, &bad_manifest_pkg).unwrap();
        let _ = fs::remove_dir_all(&bad_stage);

        let err_manifest = install_or_update_package(&bad_manifest_pkg, &workspace);
        assert!(err_manifest.is_err(), "Corrupted manifest must be rejected");
        assert_eq!(get_active_version(&lab_dir), Some("1.0.1".to_string()));

        // Step 7: Test Corruption 2: Invalid signature
        let bad_sig_pkg = temp.join("A01_bad_sig.zlab");
        let bad_sig_stage = temp.join("bad_sig_stage");
        fs::create_dir_all(&bad_sig_stage).unwrap();
        fs::create_dir_all(bad_sig_stage.join("bin")).unwrap();
        fs::write(bad_sig_stage.join("bin").join("a01-lab.exe"), "bin").unwrap();
        let bad_sig_manifest = r#"{
            "schema_version": 1,
            "id": "A01",
            "slug": "broken-access-control",
            "title": "Broken Access Control",
            "owasp": "A01:2025",
            "version": "1.0.2",
            "difficulty": "Beginner",
            "runtime": "native_sandboxed",
            "entrypoint": "bin/a01-lab.exe",
            "default_port": 8011,
            "estimated_minutes": 45,
            "modes": ["learn"],
            "signature": "forged_signature_00000"
        }"#;
        fs::write(bad_sig_stage.join("manifest.json"), bad_sig_manifest).unwrap();
        create_zlab_package(&bad_sig_stage, &bad_sig_pkg).unwrap();
        let _ = fs::remove_dir_all(&bad_sig_stage);

        let err_sig = install_or_update_package(&bad_sig_pkg, &workspace);
        assert!(err_sig.is_err(), "Tampered signature must be rejected");
        assert_eq!(get_active_version(&lab_dir), Some("1.0.1".to_string()));

        // Step 8: Test Corruption 3: Corrupted archive structure (truncated/junk bytes)
        let junk_pkg = temp.join("A01_junk.zlab");
        fs::write(&junk_pkg, b"PK\x03\x04random truncated junk data").unwrap();
        let err_junk = install_or_update_package(&junk_pkg, &workspace);
        assert!(err_junk.is_err(), "Corrupted archive must be rejected");
        assert_eq!(get_active_version(&lab_dir), Some("1.0.1".to_string()));

        // Step 9: Verify active lab is still intact and functioning at version 1.0.1
        let final_active = resolve_effective_lab_dir(&lab_dir);
        let final_lesson =
            fs::read_to_string(final_active.join("lesson").join("intro.md")).unwrap();
        assert!(final_lesson.contains("Updated Lesson Version 1.0.1 Content"));

        let _ = fs::remove_dir_all(&temp);
    }
}
