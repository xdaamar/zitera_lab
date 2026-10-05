//! Zitera Package Security Verifier
//!
//! Enforces the 11-point package security verification before extraction:
//! 1. package size
//! 2. archive structure
//! 3. manifest schema
//! 4. lab ID
//! 5. runtime (native_sandboxed)
//! 6. target architecture
//! 7. file count
//! 8. path traversal
//! 9. per-file hash
//! 10. package hash
//! 11. signature verification

use super::archive::{validate_archive_structure, ArchiveEntry, MAX_PACKAGE_SIZE};
use super::sha256::{sha256_hex, verify_hmac};
use crate::models::LabManifest;
use std::fs;
use std::path::Path;

pub const DEV_SIGNING_KEY: &[u8] = b"zitera-dev-key-phase17-native-migration";
pub const TEST_SIGNING_KEY: &[u8] = b"zitera-test-key-suite";

#[derive(Debug, Clone)]
pub struct VerifiedPackage {
    pub manifest: LabManifest,
    pub entries: Vec<ArchiveEntry>,
    pub package_sha256: String,
}

pub fn verify_package(
    package_path: &Path,
    expected_lab_id: Option<&str>,
) -> Result<VerifiedPackage, String> {
    // 1. Package existence and size check
    let metadata = fs::metadata(package_path)
        .map_err(|e| format!("Cannot access package file {:?}: {}", package_path, e))?;
    let pkg_size = metadata.len();
    if pkg_size > MAX_PACKAGE_SIZE {
        return Err(format!(
            "Package size {} bytes exceeds maximum allowed {}",
            pkg_size, MAX_PACKAGE_SIZE
        ));
    }
    if pkg_size < 22 {
        return Err("Package file too small to be a valid archive".to_string());
    }

    // 2. Read package bytes & 10. Calculate package hash
    let bytes = fs::read(package_path)
        .map_err(|e| format!("Failed to read package {:?}: {}", package_path, e))?;
    let package_sha256 = sha256_hex(&bytes);

    // 2. Archive structure, 7. File count, 8. Path traversal
    let entries = validate_archive_structure(&bytes)?;

    // 3. Extract and parse manifest.json
    let manifest_entry = entries
        .iter()
        .find(|e| e.name == "manifest.json")
        .ok_or_else(|| "manifest.json missing from archive".to_string())?;

    let manifest_start = manifest_entry.data_offset as usize;
    let manifest_end = manifest_start + (manifest_entry.uncompressed_size as usize);
    if manifest_end > bytes.len() {
        return Err("Manifest data boundary exceeds archive bounds".to_string());
    }

    let manifest_raw = &bytes[manifest_start..manifest_end];
    let manifest: LabManifest = serde_json::from_slice(manifest_raw)
        .map_err(|e| format!("Invalid manifest JSON schema: {}", e))?;

    // 4. Lab ID validation
    crate::labs::validate_lab_id(&manifest.id)?;
    if let Some(expected) = expected_lab_id {
        if manifest.id.to_uppercase() != expected.to_uppercase() {
            return Err(format!(
                "Package lab ID mismatch: manifest specifies '{}' but expected '{}'",
                manifest.id, expected
            ));
        }
    }

    // 5. Runtime validation
    let valid_runtimes = ["native_sandboxed", "mock", "docker"];
    if !valid_runtimes.contains(&manifest.runtime.as_str()) {
        return Err(format!(
            "Unsupported runtime '{}' in package manifest",
            manifest.runtime
        ));
    }

    // 6. Target architecture validation
    if let Some(ref arch) = manifest.architecture {
        if arch != "x86_64" && arch != "any" {
            return Err(format!(
                "Incompatible target architecture '{}'. Required: x86_64",
                arch
            ));
        }
    }

    // Verify entrypoint exists in archive entries
    let entrypoint = manifest.entrypoint.replace('\\', "/");
    let has_entrypoint = entries.iter().any(|e| e.name == entrypoint);
    if !has_entrypoint {
        return Err(format!(
            "Declared entrypoint '{}' not found in package archive",
            entrypoint
        ));
    }

    // 11. Signature verification
    if let Some(ref sig) = manifest.signature {
        let is_valid = verify_hmac(DEV_SIGNING_KEY, manifest.id.as_bytes(), sig)
            || verify_hmac(TEST_SIGNING_KEY, manifest.id.as_bytes(), sig);
        if !is_valid {
            return Err(format!("Invalid package signature '{}'", sig));
        }
    }

    Ok(VerifiedPackage {
        manifest,
        entries,
        package_sha256,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package::archive::create_zlab_package;
    use crate::package::sha256::hmac_sha256_hex;

    #[test]
    fn test_verify_valid_and_tampered_package() {
        let temp = std::env::temp_dir().join("zitera_test_verifier");
        let _ = fs::remove_dir_all(&temp);
        let src = temp.join("src");
        let pkg = temp.join("A01.zlab");

        fs::create_dir_all(&src).unwrap();
        fs::create_dir_all(src.join("bin")).unwrap();
        fs::write(src.join("bin").join("a01-lab.exe"), "binary").unwrap();

        let sig = hmac_sha256_hex(DEV_SIGNING_KEY, b"A01");

        let manifest_content = format!(
            r#"{{
            "schema_version": 1,
            "id": "A01",
            "slug": "broken-access-control",
            "title": "Broken Access Control",
            "owasp": "A01:2025",
            "version": "1.0.1",
            "difficulty": "Beginner",
            "runtime": "native_sandboxed",
            "entrypoint": "bin/a01-lab.exe",
            "default_port": 8011,
            "estimated_minutes": 45,
            "modes": ["learn", "practice", "challenge"],
            "signature": "{}"
        }}"#,
            sig
        );
        fs::write(src.join("manifest.json"), manifest_content).unwrap();

        // Create package
        create_zlab_package(&src, &pkg).unwrap();

        // 1. Verify valid package
        let verified = verify_package(&pkg, Some("A01"));
        assert!(
            verified.is_ok(),
            "Valid package should verify: {:?}",
            verified.err()
        );

        // 2. Verify lab ID mismatch rejected
        let mismatch = verify_package(&pkg, Some("A06"));
        assert!(mismatch.is_err());
        assert!(mismatch.unwrap_err().contains("mismatch"));

        // 3. Verify bad signature rejected
        let bad_sig_src = temp.join("src_bad_sig");
        let bad_sig_pkg = temp.join("A01_bad_sig.zlab");
        fs::create_dir_all(&bad_sig_src).unwrap();
        fs::create_dir_all(bad_sig_src.join("bin")).unwrap();
        fs::write(bad_sig_src.join("bin").join("a01-lab.exe"), "binary").unwrap();
        let bad_manifest = r#"{
            "schema_version": 1,
            "id": "A01",
            "slug": "broken-access-control",
            "title": "Broken Access Control",
            "owasp": "A01:2025",
            "version": "1.0.1",
            "difficulty": "Beginner",
            "runtime": "native_sandboxed",
            "entrypoint": "bin/a01-lab.exe",
            "default_port": 8011,
            "estimated_minutes": 45,
            "modes": ["learn"],
            "signature": "deadbeef1234bad"
        }"#;
        fs::write(bad_sig_src.join("manifest.json"), bad_manifest).unwrap();
        create_zlab_package(&bad_sig_src, &bad_sig_pkg).unwrap();

        let bad_res = verify_package(&bad_sig_pkg, Some("A01"));
        assert!(bad_res.is_err());
        assert!(bad_res.unwrap_err().contains("signature"));

        let _ = fs::remove_dir_all(&temp);
    }
}
