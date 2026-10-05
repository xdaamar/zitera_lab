//! Zitera Lab Package Format and Atomic Update Engine (Contract V3)
//!
//! Provides cryptographically verified, zero-recompile package updates (.zlab).

pub mod archive;
pub mod installer;
pub mod sha256;
pub mod verifier;

pub use archive::{create_zlab_package, extract_zlab_archive, validate_archive_structure};
pub use installer::{install_or_update_package, resolve_effective_lab_dir};
pub use verifier::{verify_package, DEV_SIGNING_KEY, TEST_SIGNING_KEY};

#[cfg(test)]
mod tests {
    use super::installer::*;
    use super::*;
    use crate::package::sha256::hmac_sha256_hex;
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

    fn make_test_manifest(id: &str, version: &str, lesson_text: &str) -> (String, String) {
        let sig = hmac_sha256_hex(DEV_SIGNING_KEY, id.as_bytes());
        let manifest = format!(
            r#"{{
            "schema_version": 1,
            "id": "{}",
            "slug": "broken-access-control",
            "title": "Broken Access Control",
            "owasp": "A01:2025",
            "version": "{}",
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
        (manifest, sig)
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

        let (manifest, _) = make_test_manifest(id, version, lesson_text);
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
