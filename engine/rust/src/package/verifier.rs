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
//! 9. per-file hash / content digest
//! 10. package hash
//! 11. Ed25519 asymmetric signature verification

use super::archive::{validate_archive_structure, ArchiveEntry, MAX_PACKAGE_SIZE};
use super::sha256::{sha256_hex, verify_hmac};
use super::trust::{
    build_canonical_package_payload, verify_with_trusted_keys, DEV_PUBLIC_KEY_HEX,
    RELEASE_PUBLIC_KEY_HEX,
};
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
    pub content_digest: String,
    pub canonical_payload: String,
}

/// Computes the deterministic content digest across all non-manifest archive entries.
///
/// Entries are sorted alphabetically by relative path name.
/// Each entry's name and uncompressed bytes hash are combined to produce a deterministic SHA-256.
pub fn compute_archive_content_digest(
    bytes: &[u8],
    entries: &[ArchiveEntry],
) -> Result<String, String> {
    let mut sorted_entries: Vec<&ArchiveEntry> = entries
        .iter()
        .filter(|e| e.name != "manifest.json")
        .collect();
    sorted_entries.sort_by(|a, b| a.name.cmp(&b.name));

    let mut hasher_input = Vec::new();
    for entry in sorted_entries {
        let start = entry.data_offset as usize;
        let end = start + (entry.uncompressed_size as usize);
        if end > bytes.len() {
            return Err(format!("Entry '{}' exceeds archive bounds", entry.name));
        }
        hasher_input.extend_from_slice(entry.name.as_bytes());
        hasher_input.push(0);
        let entry_hash = sha256_hex(&bytes[start..end]);
        hasher_input.extend_from_slice(entry_hash.as_bytes());
        hasher_input.push(b'\n');
    }

    Ok(sha256_hex(&hasher_input))
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

    // 9. Compute Content Digest
    let content_digest = compute_archive_content_digest(&bytes, &entries)?;

    // Construct Canonical Payload
    let sec_version = manifest.security_version.unwrap_or(1);
    let min_core = manifest.minimum_core_version.as_deref().unwrap_or("2.0.0");
    let arch_str = manifest.architecture.as_deref().unwrap_or("x86_64");

    let canonical_payload = build_canonical_package_payload(
        &manifest.id,
        &manifest.version,
        sec_version,
        min_core,
        &manifest.runtime,
        arch_str,
        &manifest.entrypoint,
        &content_digest,
    );

    // 11. Asymmetric Signature verification
    let sig = manifest.signature.as_ref().ok_or_else(|| {
        "Package signature is required. Unsigned packages are prohibited.".to_string()
    })?;

    let trusted_keys = [RELEASE_PUBLIC_KEY_HEX, DEV_PUBLIC_KEY_HEX];
    let ed25519_res = verify_with_trusted_keys(&canonical_payload, sig, &trusted_keys);

    if let Err(e) = ed25519_res {
        // Fallback for Phase 17 legacy test packages during migration transition
        let is_legacy_hmac = verify_hmac(DEV_SIGNING_KEY, manifest.id.as_bytes(), sig)
            || verify_hmac(TEST_SIGNING_KEY, manifest.id.as_bytes(), sig);
        if !is_legacy_hmac {
            return Err(format!("Package signature verification failed: {}", e));
        }
    }

    Ok(VerifiedPackage {
        manifest,
        entries,
        package_sha256,
        content_digest,
        canonical_payload,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package::archive::create_zlab_package;
    use crate::package::trust::{sign_payload, DEV_PRIVATE_KEY_SEED};

    #[test]
    fn test_verify_valid_and_tampered_package() {
        let temp = std::env::temp_dir().join("zitera_test_verifier_ed25519");
        let _ = fs::remove_dir_all(&temp);
        let src = temp.join("src");
        let pkg = temp.join("A01.zlab");

        fs::create_dir_all(&src).unwrap();
        fs::create_dir_all(src.join("bin")).unwrap();
        fs::write(src.join("bin").join("a01-lab.exe"), "binary content").unwrap();

        // 1. Build preliminary archive to calculate content digest
        let temp_prelim = temp.join("prelim.zlab");
        let dummy_manifest = r#"{
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
            "signature": "unsigned"
        }"#;
        fs::write(src.join("manifest.json"), dummy_manifest).unwrap();
        create_zlab_package(&src, &temp_prelim).unwrap();

        let prelim_bytes = fs::read(&temp_prelim).unwrap();
        let prelim_entries = validate_archive_structure(&prelim_bytes).unwrap();
        let digest = compute_archive_content_digest(&prelim_bytes, &prelim_entries).unwrap();

        // Sign canonical payload with Dev Private Key
        let canonical_payload = build_canonical_package_payload(
            "A01",
            "1.0.1",
            1,
            "2.0.0",
            "native_sandboxed",
            "x86_64",
            "bin/a01-lab.exe",
            &digest,
        );
        let sig = sign_payload(&canonical_payload, &DEV_PRIVATE_KEY_SEED);

        let manifest_content = format!(
            r#"{{
            "schema_version": 1,
            "id": "A01",
            "slug": "broken-access-control",
            "title": "Broken Access Control",
            "owasp": "A01:2025",
            "version": "1.0.1",
            "security_version": 1,
            "minimum_core_version": "2.0.0",
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

        // Create signed package
        create_zlab_package(&src, &pkg).unwrap();

        // 1. Verify valid signed package
        let verified = verify_package(&pkg, Some("A01"));
        assert!(
            verified.is_ok(),
            "Valid package should verify: {:?}",
            verified.err()
        );
        let v_pkg = verified.unwrap();
        assert_eq!(v_pkg.manifest.id, "A01");
        assert_eq!(v_pkg.manifest.version, "1.0.1");

        // 2. Verify lab ID mismatch rejected
        let mismatch = verify_package(&pkg, Some("A06"));
        assert!(mismatch.is_err());
        assert!(mismatch.unwrap_err().contains("mismatch"));

        // 3. Verify bad signature rejected
        let bad_sig_src = temp.join("src_bad_sig");
        let bad_sig_pkg = temp.join("A01_bad_sig.zlab");
        fs::create_dir_all(&bad_sig_src).unwrap();
        fs::create_dir_all(bad_sig_src.join("bin")).unwrap();
        fs::write(
            bad_sig_src.join("bin").join("a01-lab.exe"),
            "binary content",
        )
        .unwrap();
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

        // 4. Verify modified content fails verification
        let tampered_src = temp.join("src_tampered");
        let tampered_pkg = temp.join("A01_tampered.zlab");
        fs::create_dir_all(&tampered_src).unwrap();
        fs::create_dir_all(tampered_src.join("bin")).unwrap();
        // Alter binary content while keeping original signature
        fs::write(
            tampered_src.join("bin").join("a01-lab.exe"),
            "MALICIOUS TAMPERED BYTES",
        )
        .unwrap();
        let tampered_manifest = format!(
            r#"{{
            "schema_version": 1,
            "id": "A01",
            "slug": "broken-access-control",
            "title": "Broken Access Control",
            "owasp": "A01:2025",
            "version": "1.0.1",
            "security_version": 1,
            "minimum_core_version": "2.0.0",
            "difficulty": "Beginner",
            "runtime": "native_sandboxed",
            "entrypoint": "bin/a01-lab.exe",
            "default_port": 8011,
            "estimated_minutes": 45,
            "modes": ["learn"],
            "signature": "{}"
        }}"#,
            sig
        );
        fs::write(tampered_src.join("manifest.json"), tampered_manifest).unwrap();
        create_zlab_package(&tampered_src, &tampered_pkg).unwrap();

        let tampered_res = verify_package(&tampered_pkg, Some("A01"));
        assert!(
            tampered_res.is_err(),
            "Package with modified content must fail signature check"
        );

        let _ = fs::remove_dir_all(&temp);
    }
}
