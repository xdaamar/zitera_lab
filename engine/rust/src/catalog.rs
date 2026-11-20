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
    let check_remote = std::env::var("ZITERA_CHECK_REMOTE").is_ok();
    load_catalog_with_remote_check(workspace_root, check_remote)
}

/// Explicitly checks REMOTE_CATALOG_URL first; falls back to local cache if network is unavailable or fails.
/// Corrupt remote responses NEVER overwrite valid local cache.
pub fn refresh_catalog(workspace_root: &Path) -> Result<Catalog, String> {
    load_catalog_with_remote_check(workspace_root, true)
}

pub fn load_catalog_with_remote_check(
    workspace_root: &Path,
    check_remote_first: bool,
) -> Result<Catalog, String> {
    let local_path = get_catalog_path(workspace_root);

    let mut loaded_catalog = if check_remote_first {
        // Try remote fetch first if explicitly requested
        let remote_res = fetch_remote_catalog(REMOTE_CATALOG_URL);
        if let Ok(ref remote_cat) = remote_res {
            if validate_catalog(remote_cat).is_ok() {
                if let Ok(serialized) = serde_json::to_string_pretty(remote_cat) {
                    if let Some(parent) = local_path.parent() {
                        let _ = fs::create_dir_all(parent);
                    }
                    let _ = fs::write(&local_path, serialized);
                }
                Some(remote_cat.clone())
            } else {
                None
            }
        } else {
            None
        }
        .or_else(|| load_local_cache(&local_path))
        .unwrap_or_else(default_catalog)
    } else {
        // Check local cache first (offline-first reliability)
        load_local_cache(&local_path)
            .or_else(|| {
                if let Ok(remote_cat) = fetch_remote_catalog(REMOTE_CATALOG_URL) {
                    if validate_catalog(&remote_cat).is_ok() {
                        if let Ok(serialized) = serde_json::to_string_pretty(&remote_cat) {
                            if let Some(parent) = local_path.parent() {
                                let _ = fs::create_dir_all(parent);
                            }
                            let _ = fs::write(&local_path, serialized);
                        }
                        return Some(remote_cat);
                    }
                }
                None
            })
            .unwrap_or_else(default_catalog)
    };

    enrich_catalog_with_installed_status(workspace_root, &mut loaded_catalog);
    Ok(loaded_catalog)
}

fn load_local_cache(local_path: &Path) -> Option<Catalog> {
    if local_path.exists() {
        if let Ok(meta) = fs::metadata(local_path) {
            if meta.len() <= MAX_CATALOG_FILE_SIZE {
                if let Ok(content) = fs::read_to_string(local_path) {
                    if let Ok(cached) = serde_json::from_str::<Catalog>(&content) {
                        if validate_catalog(&cached).is_ok() {
                            return Some(cached);
                        }
                    }
                }
            }
        }
    }
    None
}

/// Compares each lab item in catalog against locally installed lab in labs/<ID>/manifest.json.
/// Sets `installed`, `installed_version`, `latest_version`, and `has_update`.
pub fn enrich_catalog_with_installed_status(workspace_root: &Path, catalog: &mut Catalog) {
    for lab in &mut catalog.labs {
        lab.latest_version = lab.version.clone();
        let lab_dir = crate::labs::get_lab_dir(workspace_root, &lab.id);
        if lab_dir.exists() {
            if let Ok(manifest) = crate::labs::read_manifest(&lab_dir) {
                lab.installed = true;
                lab.installed_version = Some(manifest.version.clone());
                lab.has_update = crate::package::installer::compare_semver(&lab.version, &manifest.version)
                    == std::cmp::Ordering::Greater;
            } else {
                lab.installed = false;
                lab.installed_version = None;
                lab.has_update = false;
            }
        } else {
            lab.installed = false;
            lab.installed_version = None;
            lab.has_update = false;
        }
    }
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
                package_id: Some("zitera-lab-a01".to_string()),
                title: "Broken Access Control".to_string(),
                repository: "xdaamar/zitera_lab_a01".to_string(),
                version: "1.0.1".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A01:2025".to_string(),
                standard: Some("owasp-top10".to_string()),
                standard_version: Some("2025".to_string()),
                category_id: Some("A01".to_string()),
                category_name: Some("Broken Access Control".to_string()),
                description: "Learn how authorization flaws allow unauthorized users to view, tamper with, or delete sensitive data.".to_string(),
                short_description: Some("Insecure Direct Object References and multi-tenant boundary bypass.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(45),
                skills: vec!["Authorization Matrix".to_string(), "IDOR Detection".to_string(), "Parameter Tampering".to_string()],
                learning_objectives: vec![
                    "Identify flawed object-level access controls".to_string(),
                    "Audit multi-tenant boundary isolation".to_string(),
                    "Implement server-side authorization enforcement".to_string(),
                ],
                prerequisites: vec!["HTTP basics".to_string(), "User session concepts".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8011),
            },
            CatalogLabItem {
                id: "A02".to_string(),
                package_id: Some("zitera-lab-a02".to_string()),
                title: "Security Misconfiguration".to_string(),
                repository: "xdaamar/zitera_lab_a02".to_string(),
                version: "1.1.0".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A02:2025".to_string(),
                standard: Some("owasp-top10".to_string()),
                standard_version: Some("2025".to_string()),
                category_id: Some("A02".to_string()),
                category_name: Some("Security Misconfiguration".to_string()),
                description: "Investigate default credentials, exposed debug interfaces, and unauthenticated directory listings leaking sensitive backups.".to_string(),
                short_description: Some("Default credentials, diagnostic endpoints, and backup exposures.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(45),
                skills: vec!["Directory Listing Audit".to_string(), "Credential Hardening".to_string(), "Interface Lockdown".to_string()],
                learning_objectives: vec![
                    "Identify exposed debug and diagnostic endpoints".to_string(),
                    "Detect dangerous unauthenticated directory browsing".to_string(),
                    "Harden default server credentials and configs".to_string(),
                ],
                prerequisites: vec!["Web server basics".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8012),
            },
            CatalogLabItem {
                id: "A03".to_string(),
                package_id: Some("zitera-lab-a03".to_string()),
                title: "Software Supply Chain Failures".to_string(),
                repository: "xdaamar/zitera_lab_a03".to_string(),
                version: "1.0.1".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A03:2025".to_string(),
                standard: Some("owasp-top10".to_string()),
                standard_version: Some("2025".to_string()),
                category_id: Some("A03".to_string()),
                category_name: Some("Software Supply Chain Failures".to_string()),
                description: "Understand risks from unverified third-party dependencies, malicious package scripts, and vulnerable build artifacts.".to_string(),
                short_description: Some("Dependency poisoning, unpinned telemetry packages, and lockfile tampering.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(45),
                skills: vec!["Lockfile Auditing".to_string(), "Dependency Provenance".to_string(), "Package Verification".to_string()],
                learning_objectives: vec![
                    "Analyze compromised package manifests and lockfiles".to_string(),
                    "Trace unauthorized credential exfiltration scripts".to_string(),
                    "Implement cryptographic package provenance verification".to_string(),
                ],
                prerequisites: vec!["Package manager basics (npm/cargo)".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8013),
            },
            CatalogLabItem {
                id: "A04".to_string(),
                package_id: Some("zitera-lab-a04".to_string()),
                title: "Cryptographic Failures".to_string(),
                repository: "xdaamar/zitera_lab_a04".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A04:2025".to_string(),
                standard: Some("owasp-top10".to_string()),
                standard_version: Some("2025".to_string()),
                category_id: Some("A04".to_string()),
                category_name: Some("Cryptographic Failures".to_string()),
                description: "Explore the risks of legacy broken hashes (MD5), hardcoded encryption keys, and sensitive data leakage.".to_string(),
                short_description: Some("Broken MD5 hashes, salt omissions, and rainbow table reversing.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(45),
                skills: vec!["Cryptographic Hash Audit".to_string(), "Salt Verification".to_string(), "Password Storage Standards".to_string()],
                learning_objectives: vec![
                    "Recognize obsolete cryptographic algorithms".to_string(),
                    "Understand how unsalted hashes succumb to precomputed tables".to_string(),
                    "Deploy modern password hashing algorithms (Argon2/bcrypt)".to_string(),
                ],
                prerequisites: vec!["Basic hashing knowledge".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8014),
            },
            CatalogLabItem {
                id: "A05".to_string(),
                package_id: Some("zitera-lab-a05".to_string()),
                title: "Injection".to_string(),
                repository: "xdaamar/zitera_lab_a05".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A05:2025".to_string(),
                standard: Some("owasp-top10".to_string()),
                standard_version: Some("2025".to_string()),
                category_id: Some("A05".to_string()),
                category_name: Some("Injection".to_string()),
                description: "Understand SQL injection root causes, malicious query manipulation, and parameterized query remediation.".to_string(),
                short_description: Some("SQL query manipulation, string escaping breakout, and UNION extraction.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(45),
                skills: vec!["SQL Syntax Injection".to_string(), "Query Parameterization".to_string(), "Database Defense".to_string()],
                learning_objectives: vec![
                    "Distinguish data from query syntax execution".to_string(),
                    "Execute UNION-based database schema exfiltration".to_string(),
                    "Refactor raw SQL strings into prepared statements".to_string(),
                ],
                prerequisites: vec!["Basic SQL queries".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8015),
            },
            CatalogLabItem {
                id: "A06".to_string(),
                package_id: Some("zitera-lab-a06".to_string()),
                title: "Insecure Design".to_string(),
                repository: "xdaamar/zitera_lab_a06".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Intermediate".to_string(),
                owasp: "A06:2025".to_string(),
                standard: Some("owasp-top10".to_string()),
                standard_version: Some("2025".to_string()),
                category_id: Some("A06".to_string()),
                category_name: Some("Insecure Design".to_string()),
                description: "Analyze flawed business logic, unverified workflow state transitions, and missing architectural security controls.".to_string(),
                short_description: Some("Flawed procurement state transitions and missing business logic controls.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(45),
                skills: vec!["Threat Modeling".to_string(), "State Machine Invariants".to_string(), "Business Logic Validation".to_string()],
                learning_objectives: vec![
                    "Differentiate design flaws from implementation vulnerabilities".to_string(),
                    "Detect illegal state transitions in enterprise workflows".to_string(),
                    "Establish architectural security invariants in business logic".to_string(),
                ],
                prerequisites: vec!["Workflow / State machine concepts".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8016),
            },
            CatalogLabItem {
                id: "A07".to_string(),
                package_id: Some("zitera-lab-a07".to_string()),
                title: "Authentication Failures".to_string(),
                repository: "xdaamar/zitera_lab_a07".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A07:2025".to_string(),
                standard: Some("owasp-top10".to_string()),
                standard_version: Some("2025".to_string()),
                category_id: Some("A07".to_string()),
                category_name: Some("Authentication Failures".to_string()),
                description: "Explore how weak credential requirements, lack of brute-force protection, and session fixation lead to account takeover.".to_string(),
                short_description: Some("Missing lockout mechanisms, predictable PINs, and automated credential stuffing.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(45),
                skills: vec!["Rate Limiting".to_string(), "Lockout Enforcement".to_string(), "Authentication Defense".to_string()],
                learning_objectives: vec![
                    "Assess authentication mechanisms against automated guessing".to_string(),
                    "Identify failure modes of unrestricted login endpoints".to_string(),
                    "Implement multi-layered rate limiting and progressive delays".to_string(),
                ],
                prerequisites: vec!["HTTP POST / form submissions".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8017),
            },
            CatalogLabItem {
                id: "A08".to_string(),
                package_id: Some("zitera-lab-a08".to_string()),
                title: "Software or Data Integrity Failures".to_string(),
                repository: "xdaamar/zitera_lab_a08".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Intermediate".to_string(),
                owasp: "A08:2025".to_string(),
                standard: Some("owasp-top10".to_string()),
                standard_version: Some("2025".to_string()),
                category_id: Some("A08".to_string()),
                category_name: Some("Software or Data Integrity Failures".to_string()),
                description: "Understand vulnerabilities from accepting unverified, untrusted software or configuration artifacts without cryptographic signatures.".to_string(),
                short_description: Some("Unsigned firmware configurations and missing digital signature verification.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(45),
                skills: vec!["Digital Signatures".to_string(), "Integrity Validation".to_string(), "Secure Deserialization".to_string()],
                learning_objectives: vec![
                    "Evaluate the risks of accepting untrusted configuration updates".to_string(),
                    "Demonstrate code/config tampering without integrity checks".to_string(),
                    "Implement asymmetric signature verification for payload ingestion".to_string(),
                ],
                prerequisites: vec!["Asymmetric cryptography concepts".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8018),
            },
            CatalogLabItem {
                id: "A09".to_string(),
                package_id: Some("zitera-lab-a09".to_string()),
                title: "Security Logging & Alerting Failures".to_string(),
                repository: "xdaamar/zitera_lab_a09".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A09:2025".to_string(),
                standard: Some("owasp-top10".to_string()),
                standard_version: Some("2025".to_string()),
                category_id: Some("A09".to_string()),
                category_name: Some("Security Logging & Alerting Failures".to_string()),
                description: "Discover how insufficient security telemetry, missing audit trails, and silent failures prevent incident detection.".to_string(),
                short_description: Some("Silent security incidents, missing privilege escalation logs, and audit blindness.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(45),
                skills: vec!["Security Event Telemetry".to_string(), "Audit Trail Design".to_string(), "SIEM Alert Rules".to_string()],
                learning_objectives: vec![
                    "Audit high-privilege operations for logging coverage".to_string(),
                    "Identify blind spots that permit undetected adversary movement".to_string(),
                    "Design tamper-evident structured security audit events".to_string(),
                ],
                prerequisites: vec!["Logging & system administration basics".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8019),
            },
            CatalogLabItem {
                id: "A10".to_string(),
                package_id: Some("zitera-lab-a10".to_string()),
                title: "Mishandling of Exceptional Conditions".to_string(),
                repository: "xdaamar/zitera_lab_a10".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Intermediate".to_string(),
                owasp: "A10:2025".to_string(),
                standard: Some("owasp-top10".to_string()),
                standard_version: Some("2025".to_string()),
                category_id: Some("A10".to_string()),
                category_name: Some("Mishandling of Exceptional Conditions".to_string()),
                description: "Examine dangerous fail-open exception handling where upstream service errors accidentally bypass authorization boundaries.".to_string(),
                short_description: Some("Fail-open exception handling allowing unauthorized security gate traversal.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(45),
                skills: vec!["Fail-Safe Defaults".to_string(), "Defensive Exception Handling".to_string(), "Boundary Guarding".to_string()],
                learning_objectives: vec![
                    "Identify fail-open security flaws in exception handling blocks".to_string(),
                    "Induce upstream failure states that bypass authorization checks".to_string(),
                    "Enforce strict fail-closed defaults across all boundary guards".to_string(),
                ],
                prerequisites: vec!["Error/Exception handling basics".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8020),
            },
            CatalogLabItem {
                id: "W01".to_string(),
                package_id: Some("zitera-lab-w01".to_string()),
                title: "Cross-Site Request Forgery (CSRF)".to_string(),
                repository: "xdaamar/zitera_lab_w01".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Intermediate".to_string(),
                owasp: "W01:2026".to_string(),
                standard: Some("web-security".to_string()),
                standard_version: Some("2026".to_string()),
                category_id: Some("W01".to_string()),
                category_name: Some("Web Security".to_string()),
                description: "Pelajari bagaimana browser secara otomatis menyertakan cookie sesi pada request lintas origin dan cara mengamankannya dengan Anti-CSRF Token serta SameSite cookies.".to_string(),
                short_description: Some("Cross-Site Request Forgery & anti-CSRF token bypass.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(45),
                skills: vec!["CSRF".to_string(), "SameSite Cookie".to_string(), "Anti-CSRF Token".to_string()],
                learning_objectives: vec![
                    "Memahami alur request lintas origin browser".to_string(),
                    "Mengeksploitasi endpoint tanpa token validasi".to_string(),
                    "Menerapkan pertahanan SameSite dan token sinkron".to_string(),
                ],
                prerequisites: vec!["HTTP basics".to_string(), "Cookies".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8021),
            },
            CatalogLabItem {
                id: "W02".to_string(),
                package_id: Some("zitera-lab-w02".to_string()),
                title: "Server-Side Request Forgery (SSRF)".to_string(),
                repository: "xdaamar/zitera_lab_w02".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Advanced".to_string(),
                owasp: "W02:2026".to_string(),
                standard: Some("web-security".to_string()),
                standard_version: Some("2026".to_string()),
                category_id: Some("W02".to_string()),
                category_name: Some("Web Security".to_string()),
                description: "Eksploitasi fitur fetch URL internal untuk memindai port privat lokal dan mengakses metadata service yang tersembunyi dari publik.".to_string(),
                short_description: Some("Server-Side Request Forgery on internal cloud metadata.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(50),
                skills: vec!["SSRF".to_string(), "Cloud Metadata".to_string(), "Loopback Bypass".to_string()],
                learning_objectives: vec![
                    "Memahami risiko endpoint fetching URL eksternal".to_string(),
                    "Memindai port loopback internal dari celah server".to_string(),
                    "Menerapkan IP allowlisting dan subnet restriction".to_string(),
                ],
                prerequisites: vec!["HTTP basics".to_string(), "IP Addressing".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8022),
            },
            CatalogLabItem {
                id: "W03".to_string(),
                package_id: Some("zitera-lab-w03".to_string()),
                title: "Broken Object Level Authorization (API BOLA)".to_string(),
                repository: "xdaamar/zitera_lab_w03".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Intermediate".to_string(),
                owasp: "W03:2026".to_string(),
                standard: Some("api-security".to_string()),
                standard_version: Some("2026".to_string()),
                category_id: Some("W03".to_string()),
                category_name: Some("API Security".to_string()),
                description: "Identifikasi celah otorisasi level objek pada REST API modern di mana manipulasi ID pesanan membocorkan transaksi pelanggan lain.".to_string(),
                short_description: Some("API Object Level Authorization bypass in modern JSON endpoints.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(40),
                skills: vec!["API BOLA".to_string(), "Object Ownership".to_string(), "REST Security".to_string()],
                learning_objectives: vec![
                    "Mengaudit otorisasi objek pada RESTful endpoints".to_string(),
                    "Membedakan validasi token bearer vs kepemilikan record".to_string(),
                    "Menerapkan record-level security middleware".to_string(),
                ],
                prerequisites: vec!["REST API".to_string(), "JSON".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8023),
            },
            CatalogLabItem {
                id: "W04".to_string(),
                package_id: Some("zitera-lab-w04".to_string()),
                title: "Arbitrary File Upload & MIME Sniffing".to_string(),
                repository: "xdaamar/zitera_lab_w04".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Advanced".to_string(),
                owasp: "W04:2026".to_string(),
                standard: Some("web-security".to_string()),
                standard_version: Some("2026".to_string()),
                category_id: Some("W04".to_string()),
                category_name: Some("Web Security".to_string()),
                description: "Uji kelemahan validasi berkas unggahan, manipulasi MIME type, bypass ekstensi ganda, dan eksekusi payload webshell terisolasi.".to_string(),
                short_description: Some("Unrestricted file upload and MIME validation bypass.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(45),
                skills: vec!["File Upload".to_string(), "MIME Validation".to_string(), "Webshell Analysis".to_string()],
                learning_objectives: vec![
                    "Menganalisis proses penanganan berkas unggahan".to_string(),
                    "Memanipulasi header Content-Type dan ekstensi".to_string(),
                    "Menerapkan isolasi penyimpanan berkas di luar webroot".to_string(),
                ],
                prerequisites: vec!["HTTP POST".to_string(), "Multipart Form".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8024),
            },
            CatalogLabItem {
                id: "W05".to_string(),
                package_id: Some("zitera-lab-w05".to_string()),
                title: "Path Traversal & Local File Inclusion".to_string(),
                repository: "xdaamar/zitera_lab_w05".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Intermediate".to_string(),
                owasp: "W05:2026".to_string(),
                standard: Some("web-security".to_string()),
                standard_version: Some("2026".to_string()),
                category_id: Some("W05".to_string()),
                category_name: Some("Web Security".to_string()),
                description: "Pahami bagaimana karakter dot-dot-slash memanipulasi path sistem berkas server untuk membaca file konfigurasi rahasia.".to_string(),
                short_description: Some("Path Traversal and Local File Inclusion directory escape.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(40),
                skills: vec!["Path Traversal".to_string(), "LFI".to_string(), "Filesystem Boundaries".to_string()],
                learning_objectives: vec![
                    "Mengidentifikasi parameter path yang tidak divalidasi".to_string(),
                    "Menggunakan urutan traversal dot-dot-slash terarah".to_string(),
                    "Menerapkan canonical path verification".to_string(),
                ],
                prerequisites: vec!["File System Basics".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8025),
            },
            CatalogLabItem {
                id: "F01".to_string(),
                package_id: Some("zitera-lab-f01".to_string()),
                title: "Dasar Protokol HTTP & Web Architecture".to_string(),
                repository: "xdaamar/zitera_lab_f01".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "F01:2026".to_string(),
                standard: Some("foundations".to_string()),
                standard_version: Some("2026".to_string()),
                category_id: Some("F01".to_string()),
                category_name: Some("Foundations".to_string()),
                description: "Kuasai dasar protokol HTTP: analisis method GET/POST/PUT, status code 200/403/500, dan struktur header permintaan web.".to_string(),
                short_description: Some("HTTP Protocol fundamentals, status codes, and header anatomy.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(25),
                skills: vec!["HTTP Protocol".to_string(), "Headers".to_string(), "Status Codes".to_string()],
                learning_objectives: vec![
                    "Membaca dan menyusun HTTP request terstruktur".to_string(),
                    "Menganalisis kode status respons server".to_string(),
                    "Mengidentifikasi header keamanan penting".to_string(),
                ],
                prerequisites: vec!["Dasar Komputer".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8026),
            },
            CatalogLabItem {
                id: "F02".to_string(),
                package_id: Some("zitera-lab-f02".to_string()),
                title: "Cookies, Tokens & Session Security".to_string(),
                repository: "xdaamar/zitera_lab_f02".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "F02:2026".to_string(),
                standard: Some("foundations".to_string()),
                standard_version: Some("2026".to_string()),
                category_id: Some("F02".to_string()),
                category_name: Some("Foundations".to_string()),
                description: "Pahami sifat stateless HTTP, pemeriksaan atribut keamanan cookie (HttpOnly, Secure, SameSite), dan siklus hidup token JWT.".to_string(),
                short_description: Some("Stateless HTTP, secure cookies, and session lifecycle.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(30),
                skills: vec!["Cookies".to_string(), "HttpOnly".to_string(), "SameSite".to_string(), "Session Security".to_string()],
                learning_objectives: vec![
                    "Menganalisis atribut keamanan cookie".to_string(),
                    "Memahami alur pertukaran sesi stateless".to_string(),
                    "Mencegah kebocoran token sesi pada klien".to_string(),
                ],
                prerequisites: vec!["HTTP Basics".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8027),
            },
            CatalogLabItem {
                id: "F03".to_string(),
                package_id: Some("zitera-lab-f03".to_string()),
                title: "Authentication vs Authorization & Trust Boundaries".to_string(),
                repository: "xdaamar/zitera_lab_f03".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "F03:2026".to_string(),
                standard: Some("foundations".to_string()),
                standard_version: Some("2026".to_string()),
                category_id: Some("F03".to_string()),
                category_name: Some("Foundations".to_string()),
                description: "Pahami perbedaan fundamental 'siapa Anda' (AuthN) dan 'apa izin Anda' (AuthZ), serta batas kepercayaan antara browser dan backend.".to_string(),
                short_description: Some("AuthN vs AuthZ differences and client-server trust boundaries.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(25),
                skills: vec!["AuthN vs AuthZ".to_string(), "Trust Boundary".to_string(), "Server Validation".to_string()],
                learning_objectives: vec![
                    "Menegaskan batas antara identitas dan wewenang".to_string(),
                    "Mengidentifikasi trust boundary aplikasi web".to_string(),
                    "Mencegah bypass kontrol berbasis browser".to_string(),
                ],
                prerequisites: vec!["Dasar Web".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8028),
            },
            CatalogLabItem {
                id: "F04".to_string(),
                package_id: Some("zitera-lab-f04".to_string()),
                title: "Kriptografi: Encoding vs Hashing vs Enkripsi".to_string(),
                repository: "xdaamar/zitera_lab_f04".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "F04:2026".to_string(),
                standard: Some("foundations".to_string()),
                standard_version: Some("2026".to_string()),
                category_id: Some("F04".to_string()),
                category_name: Some("Foundations".to_string()),
                description: "Hilangkan kerancuan antara Base64, SHA-256, dan AES. Pelajari sifat satu arah cryptographic hash dan peran salt.".to_string(),
                short_description: Some("Encoding vs Hashing vs Encryption mathematical foundations.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(30),
                skills: vec!["Base64".to_string(), "SHA-256".to_string(), "AES".to_string(), "Salting".to_string()],
                learning_objectives: vec![
                    "Membedakan encoding, hash satu arah, dan cipher simetris".to_string(),
                    "Menganalisis ketahanan hash terhadap serangan brute force".to_string(),
                    "Menerapkan fungsi hash ber-salt pada penyimpanan password".to_string(),
                ],
                prerequisites: vec!["Dasar Kriptografi".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8029),
            },
            CatalogLabItem {
                id: "F05".to_string(),
                package_id: Some("zitera-lab-f05".to_string()),
                title: "Dasar Query Database & Pemisahan Data/Perintah".to_string(),
                repository: "xdaamar/zitera_lab_f05".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "F05:2026".to_string(),
                standard: Some("foundations".to_string()),
                standard_version: Some("2026".to_string()),
                category_id: Some("F05".to_string()),
                category_name: Some("Foundations".to_string()),
                description: "Pelajari bagaimana parser SQL mengompilasi string query dan amati titik bahaya di mana data input mengubah pohon eksekusi perintah.".to_string(),
                short_description: Some("Database query parsing and code-data separation principles.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(30),
                skills: vec!["SQL Parser".to_string(), "AST".to_string(), "Parameterized Query".to_string()],
                learning_objectives: vec![
                    "Memahami alur kompilasi query database".to_string(),
                    "Mendeteksi perubahan sintaks oleh masukan data".to_string(),
                    "Menerapkan parameter binding secara konsisten".to_string(),
                ],
                prerequisites: vec!["Dasar SQL".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8030),
            },
            CatalogLabItem {
                id: "F06".to_string(),
                package_id: Some("zitera-lab-f06".to_string()),
                title: "Mekanisme Lab ZITERA & Pengiriman Flag CTF".to_string(),
                repository: "xdaamar/zitera_lab_f06".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "F06:2026".to_string(),
                standard: Some("foundations".to_string()),
                standard_version: Some("2026".to_string()),
                category_id: Some("F06".to_string()),
                category_name: Some("Foundations".to_string()),
                description: "Orientasi lengkap alur kerja ZITERA LAB: kontrol sandbox, status readiness, terminal interaktif, dan format verifikasi flag ZITERA{...}.".to_string(),
                short_description: Some("Zitera Lab mechanics, interactive terminal, and CTF verification.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(20),
                skills: vec!["CTF Workflow".to_string(), "Flag Verification".to_string(), "Sandbox Controls".to_string()],
                learning_objectives: vec![
                    "Mengoperasikan antarmuka laboratorium offline".to_string(),
                    "Memahami format flag ZITERA{...}".to_string(),
                    "Memverifikasi flag ke engine grader lokal".to_string(),
                ],
                prerequisites: vec!["Pengenalan ZITERA LAB".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8031),
            },
            CatalogLabItem {
                id: "D01".to_string(),
                package_id: Some("zitera-lab-d01".to_string()),
                title: "Web Server Log Analysis & Threat Detection".to_string(),
                repository: "xdaamar/zitera_lab_d01".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Intermediate".to_string(),
                owasp: "D01:2026".to_string(),
                standard: Some("defensive-security".to_string()),
                standard_version: Some("2026".to_string()),
                category_id: Some("D01".to_string()),
                category_name: Some("Defensive Security".to_string()),
                description: "Analisis berkas log web server, identifikasi jejak scanning botnet, injeksi SQL, dan rekonstruksi timeline insiden keamanan.".to_string(),
                short_description: Some("Web server log forensics and incident timeline reconstruction.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(45),
                skills: vec!["Log Forensics".to_string(), "Threat Detection".to_string(), "Incident Response".to_string()],
                learning_objectives: vec![
                    "Membaca anatomi catatan log Combined Nginx/Apache".to_string(),
                    "Membedakan lalu lintas normal vs scanning otomatis".to_string(),
                    "Mengidentifikasi indikator keberhasilan serangan".to_string(),
                ],
                prerequisites: vec!["HTTP Basics".to_string(), "Log Format".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8032),
            },
            CatalogLabItem {
                id: "D02".to_string(),
                package_id: Some("zitera-lab-d02".to_string()),
                title: "Binary & Package Cryptographic Integrity".to_string(),
                repository: "xdaamar/zitera_lab_d02".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Intermediate".to_string(),
                owasp: "D02:2026".to_string(),
                standard: Some("defensive-security".to_string()),
                standard_version: Some("2026".to_string()),
                category_id: Some("D02".to_string()),
                category_name: Some("Defensive Security".to_string()),
                description: "Verifikasi ringkasan hash SHA-256 dan tanda tangan digital asimetris Ed25519 untuk mendeteksi manipulasi biner pihak ketiga.".to_string(),
                short_description: Some("Cryptographic integrity verification and Ed25519 signature checks.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(45),
                skills: vec!["Ed25519".to_string(), "SHA-256 Checksum".to_string(), "Binary Authenticity".to_string()],
                learning_objectives: vec![
                    "Menghitung dan membandingkan hash SHA-256 biner".to_string(),
                    "Mendeteksi modifikasi 1-bit via avalanche effect".to_string(),
                    "Memverifikasi tanda tangan digital kunci asimetris".to_string(),
                ],
                prerequisites: vec!["Dasar Kriptografi".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8033),
            },
            CatalogLabItem {
                id: "A11".to_string(),
                package_id: Some("zitera-lab-a11".to_string()),
                title: "API Mass Assignment & Privilege Escalation".to_string(),
                repository: "xdaamar/zitera_lab_a11".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Intermediate".to_string(),
                owasp: "A11:2026".to_string(),
                standard: Some("advanced-attacks".to_string()),
                standard_version: Some("2026".to_string()),
                category_id: Some("A11".to_string()),
                category_name: Some("Advanced Attacks".to_string()),
                description: "Eksploitasi fitur binding otomatis payload JSON ke model database yang memungkinkan pengguna biasa mengangkat diri menjadi administrator.".to_string(),
                short_description: Some("API Mass Assignment and automated model binding privilege escalation.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(45),
                skills: vec!["Mass Assignment".to_string(), "Privilege Escalation".to_string(), "DTO Validation".to_string()],
                learning_objectives: vec![
                    "Mengaudit endpoint pendaftaran dan profil pengguna".to_string(),
                    "Menyisipkan atribut tersembunyi seperti is_admin".to_string(),
                    "Menerapkan Data Transfer Object (DTO) allowlist".to_string(),
                ],
                prerequisites: vec!["JSON API".to_string(), "Database Models".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8034),
            },
            CatalogLabItem {
                id: "A12".to_string(),
                package_id: Some("zitera-lab-a12".to_string()),
                title: "OAuth 2.0 & Account Linking Takeover".to_string(),
                repository: "xdaamar/zitera_lab_a12".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Advanced".to_string(),
                owasp: "A12:2026".to_string(),
                standard: Some("advanced-attacks".to_string()),
                standard_version: Some("2026".to_string()),
                category_id: Some("A12".to_string()),
                category_name: Some("Advanced Attacks".to_string()),
                description: "Audit kelemahan implementasi parameter state pada alur OAuth 2.0 Authorization Code yang memungkinkan pembajakan akun pengguna.".to_string(),
                short_description: Some("OAuth 2.0 State parameter absence and account linking hijacking.".to_string()),
                security_version: Some(1),
                minimum_core_version: Some("2.0.0".to_string()),
                estimated_time: Some(50),
                skills: vec!["OAuth 2.0".to_string(), "State Parameter".to_string(), "Account Takeover".to_string()],
                learning_objectives: vec![
                    "Menganalisis alur Authorization Code Grant OAuth 2.0".to_string(),
                    "Mengeksploitasi ketiadaan validasi parameter state".to_string(),
                    "Mengimplementasikan PKCE dan cryptographically random state".to_string(),
                ],
                prerequisites: vec!["OAuth Basics".to_string(), "HTTP Redirects".to_string()],
                runtime: Some("native_sandboxed".to_string()),
                architecture: Some("x86_64".to_string()),
                package_url: None,
                package_sha256: None,
                installed: false,
                installed_version: None,
                has_update: false,
                latest_version: String::new(),
                default_port: Some(8035),
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
        assert_eq!(cat.labs.len(), 25);
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

    #[test]
    fn test_catalog_enrichment_and_update_detection() {
        let temp = std::env::temp_dir().join(format!("zitera_enrich_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp);
        let workspace = temp.join("workspace");
        fs::create_dir_all(&workspace).unwrap();

        // 1. Initially without labs installed -> installed = false, has_update = false
        let cat = load_catalog(&workspace).expect("Catalog must load");
        let a01 = cat.labs.iter().find(|l| l.id == "A01").unwrap();
        assert_eq!(a01.installed, false);
        assert_eq!(a01.has_update, false);
        assert_eq!(a01.latest_version, a01.version);

        // 2. Install lab A01 with older version (1.0.0) while catalog is 1.0.1
        let a01_dir = workspace.join("labs").join("A01");
        fs::create_dir_all(&a01_dir).unwrap();
        let old_manifest = r#"{
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
            "estimated_minutes": 35,
            "modes": ["learn"]
        }"#;
        fs::write(a01_dir.join("manifest.json"), old_manifest).unwrap();

        let cat_updated = load_catalog(&workspace).expect("Catalog must load");
        let a01_enriched = cat_updated.labs.iter().find(|l| l.id == "A01").unwrap();
        assert_eq!(a01_enriched.installed, true);
        assert_eq!(a01_enriched.installed_version, Some("1.0.0".to_string()));
        assert_eq!(a01_enriched.has_update, true);
        assert_eq!(a01_enriched.latest_version, "1.0.1");

        // 3. Check lab list also reflects installed & update status
        let lab_list = crate::labs::list_all_labs(&workspace);
        let a01_status = lab_list.iter().find(|l| l.id == "A01").unwrap();
        assert_eq!(a01_status.installed, true);
        assert_eq!(a01_status.version, "1.0.0");
        assert_eq!(a01_status.has_update, true);
        assert_eq!(a01_status.latest_version, "1.0.1");

        // 4. Test atomic 1-click update
        let update_res = crate::labs::update_lab(&workspace, "A01").expect("Update must succeed");
        assert!(update_res.contains("successfully updated"));

        // 5. Re-check catalog & status: has_update should now be false, version 1.0.1
        let cat_after = load_catalog(&workspace).expect("Catalog must load");
        let a01_after = cat_after.labs.iter().find(|l| l.id == "A01").unwrap();
        assert_eq!(a01_after.installed, true);
        assert_eq!(a01_after.installed_version, Some("1.0.1".to_string()));
        assert_eq!(a01_after.has_update, false);

        let status_after = crate::labs::get_lab_status(&workspace, "A01");
        assert_eq!(status_after.installed, true);
        assert_eq!(status_after.version, "1.0.1");
        assert_eq!(status_after.has_update, false);

        // 6. Running update again reports already up to date
        let rerun = crate::labs::update_lab(&workspace, "A01").expect("Rerun update");
        assert!(rerun.contains("already up to date"));

        let _ = fs::remove_dir_all(&temp);
    }
}
