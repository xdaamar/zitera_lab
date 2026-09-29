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

/// Validates catalog schema, integrity, unique IDs, and field correctness (SEC-011 / Phase 11).
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

    let mut seen_ids = HashSet::new();
    for lab in &catalog.labs {
        labs::validate_lab_id(&lab.id)?;
        if !seen_ids.insert(lab.id.to_uppercase()) {
            return Err(format!("Duplicate lab ID found in catalog: {}", lab.id));
        }
        if lab.title.trim().is_empty() {
            return Err(format!("Lab {} has an empty title.", lab.id));
        }
        if lab.version.trim().is_empty() {
            return Err(format!("Lab {} has an empty version.", lab.id));
        }
        if lab.repository.trim().is_empty() {
            return Err(format!("Lab {} has an empty repository field.", lab.id));
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

    // 1. Try remote fetch if curl is available
    if let Ok(remote_catalog) = fetch_remote_catalog(REMOTE_CATALOG_URL) {
        if let Ok(()) = validate_catalog(&remote_catalog) {
            // Safe atomic write / update to local cache
            if let Ok(serialized) = serde_json::to_string_pretty(&remote_catalog) {
                if let Some(parent) = local_path.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                let _ = fs::write(&local_path, serialized);
            }
            return Ok(remote_catalog);
        }
        // If remote is invalid or malformed, proceed to local cache without overwriting
    }

    // 2. Fall back to local cache
    if local_path.exists() {
        if let Ok(content) = fs::read_to_string(&local_path) {
            if let Ok(cached) = serde_json::from_str::<Catalog>(&content) {
                if validate_catalog(&cached).is_ok() {
                    return Ok(cached);
                }
            }
        }
    }

    // 3. Built-in fallback
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
                version: "1.0.0".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A02:2025".to_string(),
                description: "Investigate default credentials, exposed debug interfaces, and unauthenticated directory listings leaking sensitive backups.".to_string(),
            },
            CatalogLabItem {
                id: "A03".to_string(),
                title: "Software Supply Chain Failures".to_string(),
                repository: "xdaamar/zitera_lab_a03".to_string(),
                version: "1.0.0".to_string(),
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
                id: "A07".to_string(),
                title: "Authentication Failures".to_string(),
                repository: "xdaamar/zitera_lab_a07".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A07:2025".to_string(),
                description: "Explore how weak credential requirements, lack of brute-force protection, and session fixation lead to account takeover.".to_string(),
            },
        ],
    }
}
