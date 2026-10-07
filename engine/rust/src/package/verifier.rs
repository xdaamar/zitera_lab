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
use super::sha256::sha256_hex;
use super::trust::{
    build_canonical_package_payload, verify_with_trusted_keys, DEV_PUBLIC_KEY_HEX,
    RELEASE_PUBLIC_KEY_HEX,
};
use crate::models::LabManifest;
use std::fs;
use std::path::Path;

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

    // Version format validation
    if manifest.version.trim().is_empty()
        || !manifest
            .version
            .chars()
            .all(|c| c.is_ascii_digit() || c == '.' || c == '-' || c.is_ascii_alphabetic())
    {
        return Err(format!(
            "Invalid package version format '{}'",
            manifest.version
        ));
    }

    // Security version validation (must be > 0 if specified)
    if let Some(sec_ver) = manifest.security_version {
        if sec_ver == 0 {
            return Err("Invalid security_version: must be greater than 0".to_string());
        }
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

    // 11. Asymmetric Signature verification (Strict Ed25519; zero HMAC fallback)
    let sig = manifest.signature.as_ref().ok_or_else(|| {
        "Package signature is required. Unsigned packages are prohibited.".to_string()
    })?;

    let trusted_keys = [RELEASE_PUBLIC_KEY_HEX, DEV_PUBLIC_KEY_HEX];
    verify_with_trusted_keys(&canonical_payload, sig, &trusted_keys)
        .map_err(|e| format!("Package signature verification failed: {}", e))?;

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

    #[test]
    fn test_adversarial_invalid_package_matrix() {
        let temp = std::env::temp_dir().join("zitera_adversarial_pkg_matrix");
        let _ = fs::remove_dir_all(&temp);
        let _ = fs::create_dir_all(&temp);

        // Helper to create a signed package with custom components
        let make_signed_package = |pkg_path: &Path,
                                   id: &str,
                                   version: &str,
                                   sec_ver: u32,
                                   extra_files: &[(&str, &[u8])],
                                   override_sig: Option<&str>| {
            let stage = pkg_path.with_extension("stage");
            let _ = fs::remove_dir_all(&stage);
            let _ = fs::create_dir_all(&stage);
            let _ = fs::create_dir_all(stage.join("bin"));
            fs::write(stage.join("bin").join("a01-lab.exe"), b"ELF_OR_PE_BINARY_BYTES").unwrap();

            for (rel_path, content) in extra_files {
                let p = stage.join(rel_path);
                if let Some(parent) = p.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                fs::write(p, content).unwrap();
            }

            // Prelim archive to calculate digest
            let prelim = pkg_path.with_extension("prelim.zlab");
            let dummy = r#"{"schema_version":1,"id":"A01","slug":"broken-access-control","title":"Broken Access Control","owasp":"A01:2025","version":"1.0.0","difficulty":"Beginner","runtime":"native_sandboxed","entrypoint":"bin/a01-lab.exe","default_port":8011,"estimated_minutes":45,"modes":["learn"],"signature":"unsigned"}"#;
            fs::write(stage.join("manifest.json"), dummy).unwrap();
            create_zlab_package(&stage, &prelim).unwrap();

            let bytes = fs::read(&prelim).unwrap();
            let entries = validate_archive_structure(&bytes).unwrap();
            let digest = compute_archive_content_digest(&bytes, &entries).unwrap();
            let _ = fs::remove_file(&prelim);

            let canonical = build_canonical_package_payload(
                id,
                version,
                sec_ver,
                "2.0.0",
                "native_sandboxed",
                "x86_64",
                "bin/a01-lab.exe",
                &digest,
            );

            let sig = match override_sig {
                Some(s) => s.to_string(),
                None => sign_payload(&canonical, &DEV_PRIVATE_KEY_SEED),
            };

            let manifest = format!(
                r#"{{
                "schema_version": 1,
                "id": "{}",
                "slug": "broken-access-control",
                "title": "Broken Access Control",
                "owasp": "A01:2025",
                "version": "{}",
                "security_version": {},
                "minimum_core_version": "2.0.0",
                "difficulty": "Beginner",
                "runtime": "native_sandboxed",
                "entrypoint": "bin/a01-lab.exe",
                "default_port": 8011,
                "estimated_minutes": 45,
                "modes": ["learn", "practice", "challenge"],
                "lesson": "Lesson text",
                "signature": "{}"
            }}"#,
                id, version, sec_ver, sig
            );
            fs::write(stage.join("manifest.json"), manifest).unwrap();

            create_zlab_package(&stage, pkg_path).unwrap();
            let _ = fs::remove_dir_all(&stage);
        };

        // 1. VALID
        let pkg_valid = temp.join("01_valid.zlab");
        make_signed_package(&pkg_valid, "A01", "1.0.0", 1, &[], None);
        let v_res = verify_package(&pkg_valid, Some("A01"));
        assert!(v_res.is_ok(), "Valid package must verify successfully");

        // 2. INVALID SIGNATURE (Signed with untrusted key seed)
        let pkg_invalid_sig = temp.join("02_invalid_sig.zlab");
        let untrusted_seed = [0x99u8; 32];
        let bogus_sig = sign_payload("some_canonical_payload", &untrusted_seed);
        make_signed_package(&pkg_invalid_sig, "A01", "1.0.0", 1, &[], Some(&bogus_sig));
        let err2 = verify_package(&pkg_invalid_sig, Some("A01")).unwrap_err();
        assert!(err2.contains("signature verification failed"));

        // 3. MISSING SIGNATURE
        let pkg_missing_sig = temp.join("03_missing_sig.zlab");
        let stage3 = temp.join("stage3");
        let _ = fs::create_dir_all(stage3.join("bin"));
        fs::write(stage3.join("bin").join("a01-lab.exe"), b"bin").unwrap();
        let unsigned_manifest = r#"{
            "schema_version": 1,
            "id": "A01",
            "slug": "broken-access-control",
            "title": "Broken Access Control",
            "owasp": "A01:2025",
            "version": "1.0.0",
            "difficulty": "Beginner",
            "runtime": "native_sandboxed",
            "entrypoint": "bin/a01-lab.exe",
            "default_port": 8011,
            "estimated_minutes": 45,
            "modes": ["learn"]
        }"#;
        fs::write(stage3.join("manifest.json"), unsigned_manifest).unwrap();
        create_zlab_package(&stage3, &pkg_missing_sig).unwrap();
        let _ = fs::remove_dir_all(&stage3);
        let err3 = verify_package(&pkg_missing_sig, Some("A01")).unwrap_err();
        assert!(err3.contains("Package signature is required"));

        // 4. CORRUPTED SIGNATURE (Wrong length / corrupt hex)
        let pkg_corrupt_sig = temp.join("04_corrupt_sig.zlab");
        make_signed_package(&pkg_corrupt_sig, "A01", "1.0.0", 1, &[], Some("deadbeef_corrupt"));
        let err4 = verify_package(&pkg_corrupt_sig, Some("A01")).unwrap_err();
        assert!(err4.contains("Invalid signature length") || err4.contains("Invalid hex"));

        // 5. MODIFIED MANIFEST (Version altered without re-signing)
        let pkg_mod_manifest = temp.join("05_mod_manifest.zlab");
        make_signed_package(&pkg_mod_manifest, "A01", "1.0.0", 1, &[], None);
        // Tamper with manifest in place
        let stage5 = temp.join("stage5");
        let _ = fs::create_dir_all(&stage5);
        super::super::archive::extract_zlab_archive(&pkg_mod_manifest, &stage5).unwrap();
        let tampered_m = fs::read_to_string(stage5.join("manifest.json"))
            .unwrap()
            .replace("\"1.0.0\"", "\"1.0.1\"");
        fs::write(stage5.join("manifest.json"), tampered_m).unwrap();
        create_zlab_package(&stage5, &pkg_mod_manifest).unwrap();
        let _ = fs::remove_dir_all(&stage5);
        let err5 = verify_package(&pkg_mod_manifest, Some("A01")).unwrap_err();
        assert!(err5.contains("signature verification failed"));

        // 6. MODIFIED BINARY
        let pkg_mod_bin = temp.join("06_mod_bin.zlab");
        make_signed_package(&pkg_mod_bin, "A01", "1.0.0", 1, &[], None);
        let stage6 = temp.join("stage6");
        let _ = fs::create_dir_all(&stage6);
        super::super::archive::extract_zlab_archive(&pkg_mod_bin, &stage6).unwrap();
        fs::write(stage6.join("bin").join("a01-lab.exe"), b"TAMPERED_MALICIOUS_BYTES").unwrap();
        create_zlab_package(&stage6, &pkg_mod_bin).unwrap();
        let _ = fs::remove_dir_all(&stage6);
        let err6 = verify_package(&pkg_mod_bin, Some("A01")).unwrap_err();
        assert!(err6.contains("signature verification failed"));

        // 7. MODIFIED LESSON
        let pkg_mod_lesson = temp.join("07_mod_lesson.zlab");
        make_signed_package(
            &pkg_mod_lesson,
            "A01",
            "1.0.0",
            1,
            &[("lesson/intro.md", b"Original lesson")],
            None,
        );
        let stage7 = temp.join("stage7");
        let _ = fs::create_dir_all(&stage7);
        super::super::archive::extract_zlab_archive(&pkg_mod_lesson, &stage7).unwrap();
        fs::write(stage7.join("lesson").join("intro.md"), b"Defaced lesson").unwrap();
        create_zlab_package(&stage7, &pkg_mod_lesson).unwrap();
        let _ = fs::remove_dir_all(&stage7);
        let err7 = verify_package(&pkg_mod_lesson, Some("A01")).unwrap_err();
        assert!(err7.contains("signature verification failed"));

        // 8. MODIFIED CHALLENGE
        let pkg_mod_challenge = temp.join("08_mod_challenge.zlab");
        make_signed_package(
            &pkg_mod_challenge,
            "A01",
            "1.0.0",
            1,
            &[("challenge/flag.txt", b"Original flag")],
            None,
        );
        let stage8 = temp.join("stage8");
        let _ = fs::create_dir_all(&stage8);
        super::super::archive::extract_zlab_archive(&pkg_mod_challenge, &stage8).unwrap();
        fs::write(stage8.join("challenge").join("flag.txt"), b"Fake flag").unwrap();
        create_zlab_package(&stage8, &pkg_mod_challenge).unwrap();
        let _ = fs::remove_dir_all(&stage8);
        let err8 = verify_package(&pkg_mod_challenge, Some("A01")).unwrap_err();
        assert!(err8.contains("signature verification failed"));

        // 9. MODIFIED HINT
        let pkg_mod_hint = temp.join("09_mod_hint.zlab");
        make_signed_package(
            &pkg_mod_hint,
            "A01",
            "1.0.0",
            1,
            &[("hints/hint1.md", b"Original hint")],
            None,
        );
        let stage9 = temp.join("stage9");
        let _ = fs::create_dir_all(&stage9);
        super::super::archive::extract_zlab_archive(&pkg_mod_hint, &stage9).unwrap();
        fs::write(stage9.join("hints").join("hint1.md"), b"Misleading hint").unwrap();
        create_zlab_package(&stage9, &pkg_mod_hint).unwrap();
        let _ = fs::remove_dir_all(&stage9);
        let err9 = verify_package(&pkg_mod_hint, Some("A01")).unwrap_err();
        assert!(err9.contains("signature verification failed"));

        // 10. EXTRA UNDECLARED FILE
        let pkg_extra = temp.join("10_extra_file.zlab");
        make_signed_package(&pkg_extra, "A01", "1.0.0", 1, &[], None);
        let stage10 = temp.join("stage10");
        let _ = fs::create_dir_all(&stage10);
        super::super::archive::extract_zlab_archive(&pkg_extra, &stage10).unwrap();
        fs::write(stage10.join("backdoor.sh"), b"evil payload").unwrap();
        create_zlab_package(&stage10, &pkg_extra).unwrap();
        let _ = fs::remove_dir_all(&stage10);
        let err10 = verify_package(&pkg_extra, Some("A01")).unwrap_err();
        assert!(err10.contains("signature verification failed"));

        // 11. MISSING DECLARED FILE (Entrypoint missing)
        let pkg_missing_entrypoint = temp.join("11_missing_entrypoint.zlab");
        let stage11 = temp.join("stage11");
        let _ = fs::create_dir_all(&stage11);
        let manifest11 = format!(
            r#"{{
            "schema_version": 1,
            "id": "A01",
            "slug": "broken-access-control",
            "title": "Broken Access Control",
            "owasp": "A01:2025",
            "version": "1.0.0",
            "difficulty": "Beginner",
            "runtime": "native_sandboxed",
            "entrypoint": "bin/nonexistent.exe",
            "default_port": 8011,
            "estimated_minutes": 45,
            "modes": ["learn"],
            "signature": "{}"
        }}"#,
            sign_payload("dummy", &DEV_PRIVATE_KEY_SEED)
        );
        fs::write(stage11.join("manifest.json"), manifest11).unwrap();
        create_zlab_package(&stage11, &pkg_missing_entrypoint).unwrap();
        let _ = fs::remove_dir_all(&stage11);
        let err11 = verify_package(&pkg_missing_entrypoint, Some("A01")).unwrap_err();
        assert!(err11.contains("Declared entrypoint 'bin/nonexistent.exe' not found"));

        // 12. INVALID PACKAGE ID
        let pkg_bad_id = temp.join("12_bad_id.zlab");
        make_signed_package(&pkg_bad_id, "INVALID_ID_TOO_LONG_12345", "1.0.0", 1, &[], None);
        let err12 = verify_package(&pkg_bad_id, None).unwrap_err();
        assert!(err12.contains("Lab ID must be between 1 and 16 characters"));

        // 13. INVALID VERSION
        let pkg_bad_ver = temp.join("13_bad_ver.zlab");
        make_signed_package(&pkg_bad_ver, "A01", "invalid version with spaces!", 1, &[], None);
        let err13 = verify_package(&pkg_bad_ver, Some("A01")).unwrap_err();
        assert!(err13.contains("Invalid package version format"));

        // 14. INVALID SECURITY VERSION (0)
        let pkg_bad_sec_ver = temp.join("14_bad_sec_ver.zlab");
        make_signed_package(&pkg_bad_sec_ver, "A01", "1.0.0", 0, &[], None);
        let err14 = verify_package(&pkg_bad_sec_ver, Some("A01")).unwrap_err();
        assert!(err14.contains("Invalid security_version: must be greater than 0"));

        let _ = fs::remove_dir_all(&temp);
    }
}
