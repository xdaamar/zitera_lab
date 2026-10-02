use crate::labs;
use crate::models::{Catalog, CatalogLabItem};
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

/// Validates catalog schema, integrity, unique IDs, field bounds, and URL safety (Workstream B).
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
        if lab.repository.trim().is_empty() || lab.repository.len() > 256 {
            return Err(format!(
                "Lab {} has invalid repository (must be 1-256 characters).",
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

        // Validate repository structure (either owner/repo or full https://github.com/owner/repo.git)
        let full_url = if lab.repository.starts_with("https://") {
            lab.repository.clone()
        } else {
            format!("https://github.com/{}.git", lab.repository)
        };
        labs::validate_repo_url(&full_url)?;
    }
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

    // 3. Built-in default catalog fallback
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
    Catalog {
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
            },
            CatalogLabItem {
                id: "A02".to_string(),
                title: "Security Misconfiguration".to_string(),
                repository: "xdaamar/zitera_lab_a02".to_string(),
                version: "1.0.1".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A02:2025".to_string(),
                description: "Investigate default credentials, exposed debug interfaces, and unauthenticated directory listings leaking sensitive backups.".to_string(),
            },
            CatalogLabItem {
                id: "A03".to_string(),
                title: "Software Supply Chain Failures".to_string(),
                repository: "xdaamar/zitera_lab_a03".to_string(),
                version: "1.0.1".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A03:2025".to_string(),
                description: "Understand risks from unverified third-party dependencies, malicious package scripts, and vulnerable build artifacts.".to_string(),
            },
            CatalogLabItem {
                id: "A04".to_string(),
                title: "Cryptographic Failures".to_string(),
                repository: "xdaamar/zitera_lab_a04".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A04:2025".to_string(),
                description: "Explore the risks of legacy broken hashes (MD5), hardcoded encryption keys, and sensitive data leakage.".to_string(),
            },
            CatalogLabItem {
                id: "A05".to_string(),
                title: "Injection".to_string(),
                repository: "xdaamar/zitera_lab_a05".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A05:2025".to_string(),
                description: "Understand SQL injection root causes, malicious query manipulation, and parameterized query remediation.".to_string(),
            },
            CatalogLabItem {
                id: "A06".to_string(),
                title: "Insecure Design".to_string(),
                repository: "xdaamar/zitera_lab_a06".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Intermediate".to_string(),
                owasp: "A06:2025".to_string(),
                description: "Analyze flawed business logic, unverified workflow state transitions, and missing architectural security controls.".to_string(),
            },
            CatalogLabItem {
                id: "A07".to_string(),
                title: "Authentication Failures".to_string(),
                repository: "xdaamar/zitera_lab_a07".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A07:2025".to_string(),
                description: "Explore how weak credential requirements, lack of brute-force protection, and session fixation lead to account takeover.".to_string(),
            },
            CatalogLabItem {
                id: "A08".to_string(),
                title: "Software or Data Integrity Failures".to_string(),
                repository: "xdaamar/zitera_lab_a08".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Intermediate".to_string(),
                owasp: "A08:2025".to_string(),
                description: "Understand vulnerabilities from accepting unverified, untrusted software or configuration artifacts without cryptographic signatures.".to_string(),
            },
            CatalogLabItem {
                id: "A09".to_string(),
                title: "Security Logging & Alerting Failures".to_string(),
                repository: "xdaamar/zitera_lab_a09".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A09:2025".to_string(),
                description: "Discover how insufficient security telemetry, missing audit trails, and silent failures prevent incident detection.".to_string(),
            },
            CatalogLabItem {
                id: "A10".to_string(),
                title: "Mishandling of Exceptional Conditions".to_string(),
                repository: "xdaamar/zitera_lab_a10".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Intermediate".to_string(),
                owasp: "A10:2025".to_string(),
                description: "Examine dangerous fail-open exception handling where upstream service errors accidentally bypass authorization boundaries.".to_string(),
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_catalog_is_valid() {
        let cat = default_catalog();
        assert!(validate_catalog(&cat).is_ok());
        assert_eq!(cat.labs.len(), 10);
    }

    #[test]
    fn test_validate_catalog_duplicate_id() {
        let mut cat = default_catalog();
        let mut dup_lab = cat.labs[0].clone();
        dup_lab.title = "Duplicate Item".to_string();
        cat.labs.push(dup_lab);
        let res = validate_catalog(&cat);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Duplicate lab ID found"));
    }

    #[test]
    fn test_validate_catalog_invalid_id() {
        let mut cat = default_catalog();
        cat.labs[0].id = "../A01".to_string();
        let res = validate_catalog(&cat);
        assert!(res.is_err());
    }

    #[test]
    fn test_validate_catalog_empty_title() {
        let mut cat = default_catalog();
        cat.labs[0].title = "   ".to_string();
        let res = validate_catalog(&cat);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("invalid title"));
    }

    #[test]
    fn test_validate_catalog_untrusted_repo() {
        let mut cat = default_catalog();
        cat.labs[0].repository = "https://evil.attacker.com/malicious.git".to_string();
        let res = validate_catalog(&cat);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("not a trusted HTTPS GitHub URL"));
    }

    #[test]
    fn test_validate_catalog_oversized_description() {
        let mut cat = default_catalog();
        cat.labs[0].description = "A".repeat(1025);
        let res = validate_catalog(&cat);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("exceeding 1024 characters"));
    }
}
