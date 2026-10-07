use crate::labs;
use crate::models::{Catalog, CatalogLabItem};
use crate::package::trust::{
    build_canonical_catalog_payload, sign_catalog, verify_with_trusted_keys, DEV_PRIVATE_KEY_SEED,
    DEV_PUBLIC_KEY_HEX, RELEASE_PUBLIC_KEY_HEX,
};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub const REMOTE_CATALOG_URL: &str =
    "https://raw.githubusercontent.com/xdaamar/zitera_lab/main/catalog/catalog.json";

pub fn get_catalog_path(workspace_root: &Path) -> PathBuf {
    workspace_root.join("catalog").join("catalog.json")
}

pub const MAX_CATALOG_FILE_SIZE: u64 = 524_288; // 512 KB limit to prevent unbounded memory allocation
pub const MAX_LABS_COUNT: usize = 100;

/// Validates catalog schema, integrity, unique IDs, field bounds, URL safety, and Ed25519 signature.
pub fn validate_catalog(catalog: &Catalog) -> Result<(), String> {
    if catalog.schema_version != 1 {
        return Err(format!(
            "Unsupported catalog schema version: {}",
            catalog.schema_version
        ));
    }
    if catalog.labs.is_empty() {
        return Err("Catalog contains no labs.".to_string());
    }
    if catalog.labs.len() > MAX_LABS_COUNT {
        return Err(format!(
            "Catalog exceeds maximum lab capacity ({} > {}).",
            catalog.labs.len(),
            MAX_LABS_COUNT
        ));
    }

    let mut seen_ids = HashSet::new();
    for lab in &catalog.labs {
        labs::validate_lab_id(&lab.id)?;
        if !seen_ids.insert(lab.id.to_uppercase()) {
            return Err(format!("Duplicate lab ID found in catalog: {}", lab.id));
        }
        if lab.title.trim().is_empty() || lab.title.len() > 128 {
            return Err(format!(
                "Lab {} has invalid title (must be 1-128 characters).",
                lab.id
            ));
        }
        if lab.version.trim().is_empty() || lab.version.len() > 32 {
            return Err(format!(
                "Lab {} has invalid version (must be 1-32 characters).",
                lab.id
            ));
        }

        // Validate semantic version format (must contain numbers and dots)
        if !lab
            .version
            .chars()
            .all(|c| c.is_ascii_digit() || c == '.' || c == '-' || c.is_ascii_alphabetic())
        {
            return Err(format!(
                "Lab {} has invalid characters in version string.",
                lab.id
            ));
        }

        if lab.repository.trim().is_empty() || lab.repository.len() > 256 {
            return Err(format!(
                "Lab {} has invalid repository (must be 1-256 characters).",
                lab.id
            ));
        }

        // Strict protocol check: Disallow arbitrary protocols (ftp, file, gopher, unc)
        let repo_lower = lab.repository.to_lowercase();
        if repo_lower.starts_with("ftp://")
            || repo_lower.starts_with("file://")
            || repo_lower.starts_with("http://")
            || repo_lower.starts_with("\\\\")
        {
            return Err(format!(
                "Lab {} specifies forbidden network protocol in repository URL.",
                lab.id
            ));
        }

        if lab.description.len() > 1024 {
            return Err(format!(
                "Lab {} has description exceeding 1024 characters.",
                lab.id
            ));
        }
        if lab.owasp.len() > 64 {
            return Err(format!(
                "Lab {} has owasp field exceeding 64 characters.",
                lab.id
            ));
        }
        if lab.difficulty.len() > 32 {
            return Err(format!(
                "Lab {} has difficulty field exceeding 32 characters.",
                lab.id
            ));
        }

        // Validate runtime if present
        if let Some(ref rt) = lab.runtime {
            let valid_runtimes = ["native_sandboxed", "mock", "docker"];
            if !valid_runtimes.contains(&rt.as_str()) {
                return Err(format!("Lab {} has invalid runtime '{}'.", lab.id, rt));
            }
        }

        // Validate architecture if present
        if let Some(ref arch) = lab.architecture {
            if arch != "x86_64" && arch != "any" {
                return Err(format!(
                    "Lab {} specifies unsupported architecture '{}'.",
                    lab.id, arch
                ));
            }
        }

        // Validate package_url if present
        if let Some(ref pkg_url) = lab.package_url {
            let url_lower = pkg_url.to_lowercase();
            if !url_lower.starts_with("https://") {
                return Err(format!(
                    "Lab {} package URL must use HTTPS protocol.",
                    lab.id
                ));
            }
            if url_lower.contains("..") || url_lower.contains('\\') {
                return Err(format!(
                    "Lab {} package URL contains invalid path sequences.",
                    lab.id
                ));
            }
        }

        // Validate repository structure (either owner/repo or full https://github.com/owner/repo.git)
        let full_url = if lab.repository.starts_with("https://") {
            lab.repository.clone()
        } else {
            format!("https://github.com/{}.git", lab.repository)
        };
        labs::validate_repo_url(&full_url)?;
    }

    // Cryptographic Signature Verification
    let sig = catalog.signature.as_ref().ok_or_else(|| {
        "Catalog signature is required. Unsigned catalogs are prohibited.".to_string()
    })?;

    let canonical_payload = build_canonical_catalog_payload(catalog);
    let trusted_keys = [RELEASE_PUBLIC_KEY_HEX, DEV_PUBLIC_KEY_HEX];
    verify_with_trusted_keys(&canonical_payload, sig, &trusted_keys)
        .map_err(|e| format!("Catalog cryptographic verification failed: {}", e))?;

    Ok(())
}

/// Attempts to load catalog from remote URL with timeout; falls back to local cache or default catalog.
/// Corrupt remote responses NEVER overwrite valid local cache.
pub fn load_catalog(workspace_root: &Path) -> Result<Catalog, String> {
    let local_path = get_catalog_path(workspace_root);

    // 1. Check local catalog cache first (ensures offline reliability and local authority)
    if local_path.exists() {
        if let Ok(meta) = fs::metadata(&local_path) {
            if meta.len() <= MAX_CATALOG_FILE_SIZE {
                if let Ok(content) = fs::read_to_string(&local_path) {
                    if let Ok(cached) = serde_json::from_str::<Catalog>(&content) {
                        if validate_catalog(&cached).is_ok() {
                            return Ok(cached);
                        }
                    }
                }
            }
        }
    }

    // 2. Try remote fetch if local cache is missing or corrupt
    if let Ok(remote_catalog) = fetch_remote_catalog(REMOTE_CATALOG_URL) {
        if let Ok(()) = validate_catalog(&remote_catalog) {
            if let Ok(serialized) = serde_json::to_string_pretty(&remote_catalog) {
                if let Some(parent) = local_path.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                let _ = fs::write(&local_path, serialized);
            }
            return Ok(remote_catalog);
        }
    }

    // 3. Built-in default signed catalog fallback
    Ok(default_catalog())
}

/// Fetch catalog JSON from remote HTTP URL using platform curl with bounded execution.
pub fn fetch_remote_catalog(url: &str) -> Result<Catalog, String> {
    // SEC-011: Enforce HTTPS
    if !url.starts_with("https://") {
        return Err("Only HTTPS catalog URLs are permitted.".to_string());
    }

    let out = crate::process::run_cmd("curl.exe", &["-s", "--max-time", "4", "--fail", url], None)?;

    if !out.success || out.stdout.trim().is_empty() {
        return Err(format!("Remote catalog request failed: {}", out.stderr));
    }

    if out.stdout.len() as u64 > MAX_CATALOG_FILE_SIZE {
        return Err("Remote catalog response exceeds maximum allowed size (512 KB).".to_string());
    }

    let catalog: Catalog = serde_json::from_str(&out.stdout)
        .map_err(|e| format!("Failed to parse remote catalog JSON: {}", e))?;
    Ok(catalog)
}

pub fn default_catalog() -> Catalog {
    let mut catalog = Catalog {
        schema_version: 1,
        labs: vec![
            CatalogLabItem {
                id: "A01".to_string(),
                title: "Broken Access Control".to_string(),
                repository: "xdaamar/zitera_lab_a01".to_string(),
                version: "1.0.1".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A01:2025".to_string(),
                description: "Learn how authorization flaws allow unauthorized users to view, tamper with, or delete sensitive data.".to_string(),
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
            },
            CatalogLabItem {
                id: "A02".to_string(),
                title: "Security Misconfiguration".to_string(),
                repository: "xdaamar/zitera_lab_a02".to_string(),
                version: "1.0.1".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A02:2025".to_string(),
                description: "Investigate default credentials, exposed debug interfaces, and unauthenticated directory listings leaking sensitive backups.".to_string(),
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
            },
            CatalogLabItem {
                id: "A03".to_string(),
                title: "Software Supply Chain Failures".to_string(),
                repository: "xdaamar/zitera_lab_a03".to_string(),
                version: "1.0.1".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A03:2025".to_string(),
                description: "Understand risks from unverified third-party dependencies, malicious package scripts, and vulnerable build artifacts.".to_string(),
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
            },
            CatalogLabItem {
                id: "A04".to_string(),
                title: "Cryptographic Failures".to_string(),
                repository: "xdaamar/zitera_lab_a04".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A04:2025".to_string(),
                description: "Explore the risks of legacy broken hashes (MD5), hardcoded encryption keys, and sensitive data leakage.".to_string(),
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
            },
            CatalogLabItem {
                id: "A05".to_string(),
                title: "Injection".to_string(),
                repository: "xdaamar/zitera_lab_a05".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A05:2025".to_string(),
                description: "Understand SQL injection root causes, malicious query manipulation, and parameterized query remediation.".to_string(),
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
            },
            CatalogLabItem {
                id: "A06".to_string(),
                title: "Insecure Design".to_string(),
                repository: "xdaamar/zitera_lab_a06".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Intermediate".to_string(),
                owasp: "A06:2025".to_string(),
                description: "Analyze flawed business logic, unverified workflow state transitions, and missing architectural security controls.".to_string(),
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
            },
            CatalogLabItem {
                id: "A07".to_string(),
                title: "Authentication Failures".to_string(),
                repository: "xdaamar/zitera_lab_a07".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A07:2025".to_string(),
                description: "Explore how weak credential requirements, lack of brute-force protection, and session fixation lead to account takeover.".to_string(),
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
            },
            CatalogLabItem {
                id: "A08".to_string(),
                title: "Software or Data Integrity Failures".to_string(),
                repository: "xdaamar/zitera_lab_a08".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Intermediate".to_string(),
                owasp: "A08:2025".to_string(),
                description: "Understand vulnerabilities from accepting unverified, untrusted software or configuration artifacts without cryptographic signatures.".to_string(),
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
            },
            CatalogLabItem {
                id: "A09".to_string(),
                title: "Security Logging & Alerting Failures".to_string(),
                repository: "xdaamar/zitera_lab_a09".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A09:2025".to_string(),
                description: "Discover how insufficient security telemetry, missing audit trails, and silent failures prevent incident detection.".to_string(),
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
            },
            CatalogLabItem {
                id: "A10".to_string(),
                title: "Mishandling of Exceptional Conditions".to_string(),
                repository: "xdaamar/zitera_lab_a10".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Intermediate".to_string(),
                owasp: "A10:2025".to_string(),
                description: "Examine dangerous fail-open exception handling where upstream service errors accidentally bypass authorization boundaries.".to_string(),
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
            },
        ],
        signature: None,
    };

    // Deterministically sign the default catalog with Dev Private Key
    sign_catalog(&mut catalog, &DEV_PRIVATE_KEY_SEED);
    catalog
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_catalog_is_valid() {
        let cat = default_catalog();
        assert!(validate_catalog(&cat).is_ok());
        assert_eq!(cat.labs.len(), 10);
        assert!(cat.signature.is_some());
    }

    #[test]
    fn test_checkpoint_3_signed_catalog_security_matrix() {
        let base_cat = default_catalog();

        // 1. VALID CATALOG -> PASS
        assert!(validate_catalog(&base_cat).is_ok());

        // 2. MODIFIED CATALOG (content changed without updating signature) -> FAIL
        let mut mod_cat = base_cat.clone();
        mod_cat.labs[0].version = "9.9.9".to_string();
        let err_mod = validate_catalog(&mod_cat);
        assert!(err_mod.is_err(), "Modified catalog must fail verification");
        assert!(err_mod
            .unwrap_err()
            .contains("cryptographic verification failed"));

        // 3. WRONG CATALOG SIGNATURE -> FAIL
        let mut wrong_sig_cat = base_cat.clone();
        wrong_sig_cat.signature = Some("deadbeef".repeat(16));
        let err_wrong_sig = validate_catalog(&wrong_sig_cat);
        assert!(err_wrong_sig.is_err());
        assert!(err_wrong_sig
            .unwrap_err()
            .contains("cryptographic verification failed"));

        // 4. UNTRUSTED SIGNING KEY -> FAIL
        let mut untrusted_cat = base_cat.clone();
        let rogue_seed = [0x77u8; 32];
        sign_catalog(&mut untrusted_cat, &rogue_seed);
        let err_untrusted = validate_catalog(&untrusted_cat);
        assert!(
            err_untrusted.is_err(),
            "Catalog signed with untrusted key must fail"
        );

        // 5. MALFORMED / EMPTY SIGNATURE -> FAIL
        let mut empty_sig_cat = base_cat.clone();
        empty_sig_cat.signature = None;
        assert!(validate_catalog(&empty_sig_cat).is_err());

        // 6. OVERSIZED CATALOG (Capacity limit) -> FAIL
        let mut oversized_cat = base_cat.clone();
        let item = oversized_cat.labs[0].clone();
        for i in 10..=MAX_LABS_COUNT + 1 {
            let mut extra = item.clone();
            extra.id = format!("X{:02}", i);
            oversized_cat.labs.push(extra);
        }
        sign_catalog(&mut oversized_cat, &DEV_PRIVATE_KEY_SEED);
        let err_over = validate_catalog(&oversized_cat);
        assert!(err_over.is_err());
        assert!(err_over.unwrap_err().contains("maximum lab capacity"));

        // 7. DUPLICATE ID -> FAIL
        let mut dup_cat = base_cat.clone();
        let mut dup_item = dup_cat.labs[0].clone();
        dup_item.title = "Duplicate Item".to_string();
        dup_cat.labs.push(dup_item);
        sign_catalog(&mut dup_cat, &DEV_PRIVATE_KEY_SEED);
        let err_dup = validate_catalog(&dup_cat);
        assert!(err_dup.is_err());
        assert!(err_dup.unwrap_err().contains("Duplicate lab ID"));

        // 8. INVALID LAB ID (Path traversal) -> FAIL
        let mut invalid_id_cat = base_cat.clone();
        invalid_id_cat.labs[0].id = "../A01".to_string();
        sign_catalog(&mut invalid_id_cat, &DEV_PRIVATE_KEY_SEED);
        assert!(validate_catalog(&invalid_id_cat).is_err());

        // 9. INVALID PACKAGE URL (Non-HTTPS / path traversal) -> FAIL
        let mut invalid_url_cat = base_cat.clone();
        invalid_url_cat.labs[0].package_url = Some("http://insecure.site/pkg.zlab".to_string());
        sign_catalog(&mut invalid_url_cat, &DEV_PRIVATE_KEY_SEED);
        let err_url = validate_catalog(&invalid_url_cat);
        assert!(err_url.is_err());
        assert!(err_url.unwrap_err().contains("must use HTTPS protocol"));

        // 10. ARBITRARY PROTOCOL (ftp:// or file://) -> FAIL
        let mut proto_cat = base_cat.clone();
        proto_cat.labs[0].repository = "ftp://malicious.host/repo.git".to_string();
        sign_catalog(&mut proto_cat, &DEV_PRIVATE_KEY_SEED);
        let err_proto = validate_catalog(&proto_cat);
        assert!(err_proto.is_err());
        assert!(err_proto
            .unwrap_err()
            .contains("forbidden network protocol"));

        // 11. INVALID RUNTIME -> FAIL
        let mut runtime_cat = base_cat.clone();
        runtime_cat.labs[0].runtime = Some("unrestricted_root_shell".to_string());
        sign_catalog(&mut runtime_cat, &DEV_PRIVATE_KEY_SEED);
        let err_rt = validate_catalog(&runtime_cat);
        assert!(err_rt.is_err());
        assert!(err_rt.unwrap_err().contains("invalid runtime"));

        // 12. INVALID ARCHITECTURE -> FAIL
        let mut arch_cat = base_cat.clone();
        arch_cat.labs[0].architecture = Some("mips_be_malicious".to_string());
        sign_catalog(&mut arch_cat, &DEV_PRIVATE_KEY_SEED);
        let err_arch = validate_catalog(&arch_cat);
        assert!(err_arch.is_err());
        assert!(err_arch.unwrap_err().contains("unsupported architecture"));
    }

    #[test]
    fn test_checkpoint_10_offline_first_operation() {
        let temp = std::env::temp_dir().join("zitera_offline_test");
        let _ = fs::remove_dir_all(&temp);
        let workspace = temp.join("workspace");
        fs::create_dir_all(&workspace).unwrap();

        // 1. Without network and without local cache -> fallback to built-in default signed catalog
        let cat_default = load_catalog(&workspace).expect("Default catalog must load offline");
        assert_eq!(cat_default.schema_version, 1);
        assert_eq!(cat_default.labs.len(), 10);
        for id in ["A01", "A02", "A03", "A04", "A05", "A06", "A07", "A08", "A09", "A10"] {
            assert!(
                cat_default.labs.iter().any(|l| l.id.eq_ignore_ascii_case(id)),
                "Default catalog must include {}",
                id
            );
        }

        // 2. Offline-first local cache priority: Cache exists and is valid -> returned directly
        let cat_dir = workspace.join("catalog");
        fs::create_dir_all(&cat_dir).unwrap();
        let mut custom_cat = cat_default.clone();
        custom_cat.labs[0].title = "Offline Cached Title".to_string();
        sign_catalog(&mut custom_cat, &DEV_PRIVATE_KEY_SEED);
        fs::write(
            cat_dir.join("catalog.json"),
            serde_json::to_string_pretty(&custom_cat).unwrap(),
        )
        .unwrap();

        let cat_cached = load_catalog(&workspace).expect("Local cached catalog must load offline");
        assert_eq!(cat_cached.labs[0].title, "Offline Cached Title");

        let _ = fs::remove_dir_all(&temp);
    }
}
