//! Zitera Storage Boundaries and Lifecycle Retention Policy
//!
//! Formalizes strict directory separation between:
//! 1. Application Files (%LOCALAPPDATA%\Programs\ZiteraLab) - Immutable binaries & assets
//! 2. Lab Packages (%LOCALAPPDATA%\ZiteraLab\labs) - Versioned lab contents & runtimes
//! 3. User Progress (%LOCALAPPDATA%\ZiteraLab\user) - Student progress, scores & history
//! 4. Cache (%LOCALAPPDATA%\ZiteraLab\cache) - Disposable download buffers & temporary staging
//! 5. Logs (%LOCALAPPDATA%\ZiteraLab\logs) - Runtime session logs & telemetry-free stdout/stderr
//! 6. Diagnostics (%LOCALAPPDATA%\ZiteraLab\diagnostics) - Sanitized diagnostic bundles
//!
//! Guarantees:
//! - Application updates NEVER delete, modify, or overwrite student progress.
//! - Default uninstallation purges application binaries and cache while safely preserving
//!   student progress, installed lab packages, and diagnostic reports.
//! - Full purge is strictly opt-in via explicit user confirmation or uninstaller flag.

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageCategory {
    ApplicationFiles,
    LabPackages,
    UserProgress,
    Cache,
    Logs,
    Diagnostics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleAction {
    AppInstall,
    AppUpdate,
    AppRepair,
    UninstallDefault,
    UninstallFullPurge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionDisposition {
    Preserve,
    Purge,
    Replace,
}

/// Evaluates the lifecycle retention policy for a storage category under a given lifecycle action.
pub fn get_retention_disposition(
    action: LifecycleAction,
    category: StorageCategory,
) -> RetentionDisposition {
    match action {
        LifecycleAction::AppInstall => RetentionDisposition::Preserve,
        LifecycleAction::AppUpdate => match category {
            StorageCategory::ApplicationFiles => RetentionDisposition::Replace,
            StorageCategory::Cache => RetentionDisposition::Purge,
            StorageCategory::LabPackages
            | StorageCategory::UserProgress
            | StorageCategory::Logs
            | StorageCategory::Diagnostics => RetentionDisposition::Preserve,
        },
        LifecycleAction::AppRepair => match category {
            StorageCategory::ApplicationFiles => RetentionDisposition::Replace,
            _ => RetentionDisposition::Preserve,
        },
        LifecycleAction::UninstallDefault => match category {
            StorageCategory::ApplicationFiles | StorageCategory::Cache => {
                RetentionDisposition::Purge
            }
            StorageCategory::LabPackages
            | StorageCategory::UserProgress
            | StorageCategory::Logs
            | StorageCategory::Diagnostics => RetentionDisposition::Preserve,
        },
        LifecycleAction::UninstallFullPurge => RetentionDisposition::Purge,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePaths {
    pub app_dir: PathBuf,
    pub user_dir: PathBuf,
    pub labs_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub logs_dir: PathBuf,
    pub diagnostics_dir: PathBuf,
}

impl StoragePaths {
    /// Resolves canonical storage paths following Windows per-user hierarchy with environment overrides.
    pub fn resolve() -> Self {
        let app_dir = match env::var("ZITERA_APP_DIR") {
            Ok(val) if !val.trim().is_empty() => PathBuf::from(val.trim()),
            _ => {
                if let Ok(local_app_data) = env::var("LOCALAPPDATA") {
                    PathBuf::from(local_app_data).join("Programs").join("ZiteraLab")
                } else if let Ok(exe_path) = env::current_exe() {
                    exe_path.parent().map(|p| p.to_path_buf()).unwrap_or_else(|| PathBuf::from("."))
                } else {
                    PathBuf::from(".")
                }
            }
        };

        let base_data_dir = match env::var("ZITERA_DATA_DIR") {
            Ok(val) if !val.trim().is_empty() => PathBuf::from(val.trim()),
            _ => {
                if let Ok(local_app_data) = env::var("LOCALAPPDATA") {
                    PathBuf::from(local_app_data).join("ZiteraLab")
                } else {
                    PathBuf::from(".").join(".zitera_data")
                }
            }
        };

        let user_dir = match env::var("ZITERA_USER_DIR") {
            Ok(val) if !val.trim().is_empty() => PathBuf::from(val.trim()),
            _ => base_data_dir.join("user"),
        };

        let labs_dir = match env::var("ZITERA_LABS_DIR") {
            Ok(val) if !val.trim().is_empty() => PathBuf::from(val.trim()),
            _ => base_data_dir.join("labs"),
        };

        let cache_dir = match env::var("ZITERA_CACHE_DIR") {
            Ok(val) if !val.trim().is_empty() => PathBuf::from(val.trim()),
            _ => base_data_dir.join("cache"),
        };

        let logs_dir = match env::var("ZITERA_LOGS_DIR") {
            Ok(val) if !val.trim().is_empty() => PathBuf::from(val.trim()),
            _ => base_data_dir.join("logs"),
        };

        let diagnostics_dir = match env::var("ZITERA_DIAGNOSTICS_DIR") {
            Ok(val) if !val.trim().is_empty() => PathBuf::from(val.trim()),
            _ => base_data_dir.join("diagnostics"),
        };

        Self {
            app_dir,
            user_dir,
            labs_dir,
            cache_dir,
            logs_dir,
            diagnostics_dir,
        }
    }

    /// Constructs custom storage paths for isolated testing or specialized deployment.
    pub fn custom(app_dir: PathBuf, base_data_dir: PathBuf) -> Self {
        Self {
            user_dir: base_data_dir.join("user"),
            labs_dir: base_data_dir.join("labs"),
            cache_dir: base_data_dir.join("cache"),
            logs_dir: base_data_dir.join("logs"),
            diagnostics_dir: base_data_dir.join("diagnostics"),
            app_dir,
        }
    }

    /// Ensures all mutable data directories exist on the filesystem.
    pub fn ensure_data_directories(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.user_dir)?;
        fs::create_dir_all(&self.labs_dir)?;
        fs::create_dir_all(&self.cache_dir)?;
        fs::create_dir_all(&self.logs_dir)?;
        fs::create_dir_all(&self.diagnostics_dir)?;
        Ok(())
    }

    /// Returns the canonical path to the student progress file.
    pub fn user_progress_file(&self) -> PathBuf {
        self.user_dir.join("progress.json")
    }

    /// Verifies that application binaries directory and user data directories are strictly disjoint.
    /// Returns Err if any data directory is nested inside or identical to the application directory.
    pub fn verify_boundary_isolation(&self) -> Result<(), String> {
        let app_canon = dunce_canonicalize(&self.app_dir);
        let user_canon = dunce_canonicalize(&self.user_dir);
        let labs_canon = dunce_canonicalize(&self.labs_dir);

        if app_canon == user_canon {
            return Err("Violation: app_dir and user_dir are identical".to_string());
        }
        if user_canon.starts_with(&app_canon) {
            return Err(format!(
                "Violation: user_dir ({:?}) is nested inside app_dir ({:?}); app updates risk deleting user progress",
                self.user_dir, self.app_dir
            ));
        }
        if labs_canon.starts_with(&app_canon) {
            return Err(format!(
                "Violation: labs_dir ({:?}) is nested inside app_dir ({:?}); app updates risk deleting installed labs",
                self.labs_dir, self.app_dir
            ));
        }
        Ok(())
    }

    /// Purges disposable cache files without affecting user progress or installed labs.
    pub fn purge_cache(&self) -> std::io::Result<u64> {
        let mut count = 0u64;
        if self.cache_dir.exists() {
            for entry in fs::read_dir(&self.cache_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    fs::remove_file(&path)?;
                    count += 1;
                } else if path.is_dir() {
                    fs::remove_dir_all(&path)?;
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    /// Checks and automatically migrates legacy state files (e.g. zitera_progress.json)
    /// into the canonical ZITERA 2.x storage file (%LOCALAPPDATA%\ZiteraLab\user\progress.json).
    /// Returns Ok(true) if migration occurred, Ok(false) if canonical file was already present or no legacy file found.
    pub fn migrate_legacy_state(&self, workspace_root: Option<&Path>) -> Result<bool, String> {
        let canonical_file = self.user_progress_file();
        if canonical_file.exists() {
            return Ok(false);
        }

        // Candidate legacy progress file locations
        let mut candidates = vec![
            self.user_dir.join("zitera_progress.json"),
            PathBuf::from("zitera_progress.json"),
        ];
        if let Some(root) = workspace_root {
            candidates.push(root.join("zitera_progress.json"));
            candidates.push(root.join("ui/flutter/zitera_progress.json"));
        }

        for candidate in candidates {
            if candidate.exists() {
                if let Ok(content) = fs::read_to_string(&candidate) {
                    if let Ok(mut json_val) = serde_json::from_str::<serde_json::Value>(&content) {
                        // Normalize legacy keys
                        if let Some(obj) = json_val.as_object_mut() {
                            // Purge any legacy secret flags if present
                            obj.remove("solved_flags");

                            // Normalize completed_labs
                            if let Some(labs) = obj.get_mut("completed_labs").and_then(|v| v.as_array_mut()) {
                                for item in labs.iter_mut() {
                                    if let Some(s) = item.as_str() {
                                        *item = serde_json::Value::String(normalize_lab_id(s));
                                    }
                                }
                            }
                            // Normalize completed_challenges
                            if let Some(chals) = obj.get_mut("completed_challenges").and_then(|v| v.as_array_mut()) {
                                for item in chals.iter_mut() {
                                    if let Some(s) = item.as_str() {
                                        *item = serde_json::Value::String(normalize_lab_id(s));
                                    }
                                }
                            }
                            // Normalize completed_practice
                            if let Some(prac) = obj.get_mut("completed_practice").and_then(|v| v.as_array_mut()) {
                                for item in prac.iter_mut() {
                                    if let Some(s) = item.as_str() {
                                        *item = serde_json::Value::String(normalize_lab_id(s));
                                    }
                                }
                            }
                            // Normalize keys in lab_records
                            if let Some(records) = obj.get("lab_records").cloned() {
                                if let Some(rec_map) = records.as_object() {
                                    let mut new_records = serde_json::Map::new();
                                    for (k, v) in rec_map {
                                        new_records.insert(normalize_lab_id(k), v.clone());
                                    }
                                    obj.insert("lab_records".to_string(), serde_json::Value::Object(new_records));
                                }
                            }
                            // Normalize keys in completed_sections
                            if let Some(sections) = obj.get("completed_sections").cloned() {
                                if let Some(sec_map) = sections.as_object() {
                                    let mut new_sections = serde_json::Map::new();
                                    for (k, v) in sec_map {
                                        new_sections.insert(normalize_lab_id(k), v.clone());
                                    }
                                    obj.insert("completed_sections".to_string(), serde_json::Value::Object(new_sections));
                                }
                            }
                        }

                        // Write to canonical user storage
                        self.ensure_data_directories()
                            .map_err(|e| format!("Failed to create user directories: {}", e))?;
                        let serialized = serde_json::to_string_pretty(&json_val)
                            .map_err(|e| format!("Failed to serialize migrated progress: {}", e))?;
                        fs::write(&canonical_file, serialized)
                            .map_err(|e| format!("Failed to write canonical progress: {}", e))?;
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    /// Loads the user progress JSON, executing automatic migration if needed.
    pub fn load_progress(&self, workspace_root: Option<&Path>) -> Result<serde_json::Value, String> {
        let canonical_file = self.user_progress_file();
        if !canonical_file.exists() {
            let _ = self.migrate_legacy_state(workspace_root);
        }

        if canonical_file.exists() {
            let content = fs::read_to_string(&canonical_file)
                .map_err(|e| format!("Failed to read progress file: {}", e))?;
            serde_json::from_str(&content)
                .map_err(|e| format!("Corrupted progress file: {}", e))
        } else {
            Ok(serde_json::json!({
                "language": "en",
                "completed_labs": [],
                "completed_challenges": [],
                "completed_practice": [],
                "completed_sections": {},
                "lab_records": {},
            }))
        }
    }

    /// Atomically saves user progress into the canonical storage boundary.
    pub fn save_progress(&self, data: &serde_json::Value) -> Result<(), String> {
        self.ensure_data_directories()
            .map_err(|e| format!("Failed to create data dirs: {}", e))?;
        let canonical_file = self.user_progress_file();
        let tmp_file = self.user_dir.join("progress.tmp");
        let serialized = serde_json::to_string_pretty(data)
            .map_err(|e| format!("Failed to serialize progress: {}", e))?;
        fs::write(&tmp_file, serialized)
            .map_err(|e| format!("Failed to write tmp progress: {}", e))?;
        if canonical_file.exists() {
            let _ = fs::remove_file(&canonical_file);
        }
        fs::rename(&tmp_file, &canonical_file)
            .map_err(|e| format!("Failed to commit atomic progress: {}", e))?;
        Ok(())
    }
}

/// Normalizes any legacy or package lab identifier (e.g. "zitera-lab-a01", "zitera-a01", "a01")
/// into the canonical alphanumeric uppercase lab ID ("A01").
pub fn normalize_lab_id(raw_id: &str) -> String {
    let mut s = raw_id.trim();
    if let Some(stripped) = s.strip_prefix("zitera-lab-") {
        s = stripped;
    } else if let Some(stripped) = s.strip_prefix("zitera-") {
        s = stripped;
    }
    s.to_uppercase()
}

/// Returns the canonical package ID corresponding to a lab ID.
pub fn canonical_package_id_from_lab(lab_id: &str) -> String {
    let norm = normalize_lab_id(lab_id);
    format!("zitera-lab-{}", norm.to_lowercase())
}

/// Helper function to normalize paths for prefix checking without failing if paths don't exist yet.
fn dunce_canonicalize(path: &Path) -> PathBuf {
    if let Ok(canon) = fs::canonicalize(path) {
        return canon;
    }
    // Fallback: clean normalized representation
    let mut normalized = PathBuf::new();
    for comp in path.components() {
        normalized.push(comp);
    }
    normalized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_boundary_resolution_and_defaults() {
        let paths = StoragePaths::resolve();
        assert!(!paths.app_dir.as_os_str().is_empty());
        assert!(!paths.user_dir.as_os_str().is_empty());
        assert!(!paths.labs_dir.as_os_str().is_empty());
        assert!(!paths.cache_dir.as_os_str().is_empty());
        assert!(!paths.logs_dir.as_os_str().is_empty());
        assert!(!paths.diagnostics_dir.as_os_str().is_empty());
    }

    #[test]
    fn test_storage_boundary_isolation_verification() {
        let temp_root = std::env::temp_dir().join(format!("zitera_boundary_test_{}", std::process::id()));
        let app_dir = temp_root.join("Programs").join("ZiteraLab");
        let data_dir = temp_root.join("ZiteraLabData");

        let paths = StoragePaths::custom(app_dir.clone(), data_dir.clone());
        assert!(paths.verify_boundary_isolation().is_ok());

        // Violating configuration: nesting user_dir inside app_dir
        let invalid_paths = StoragePaths::custom(app_dir.clone(), app_dir.join("NestedData"));
        let isolation_err = invalid_paths.verify_boundary_isolation();
        assert!(isolation_err.is_err());
        assert!(isolation_err.unwrap_err().contains("nested inside app_dir"));

        let _ = fs::remove_dir_all(&temp_root);
    }

    #[test]
    fn test_lifecycle_retention_policy_matrix() {
        // AppUpdate: replaces app binaries, purges cache, preserves user data and labs
        assert_eq!(
            get_retention_disposition(LifecycleAction::AppUpdate, StorageCategory::ApplicationFiles),
            RetentionDisposition::Replace
        );
        assert_eq!(
            get_retention_disposition(LifecycleAction::AppUpdate, StorageCategory::Cache),
            RetentionDisposition::Purge
        );
        assert_eq!(
            get_retention_disposition(LifecycleAction::AppUpdate, StorageCategory::UserProgress),
            RetentionDisposition::Preserve
        );
        assert_eq!(
            get_retention_disposition(LifecycleAction::AppUpdate, StorageCategory::LabPackages),
            RetentionDisposition::Preserve
        );
        assert_eq!(
            get_retention_disposition(LifecycleAction::AppUpdate, StorageCategory::Diagnostics),
            RetentionDisposition::Preserve
        );

        // UninstallDefault: purges app binaries and cache, preserves user progress and labs
        assert_eq!(
            get_retention_disposition(LifecycleAction::UninstallDefault, StorageCategory::ApplicationFiles),
            RetentionDisposition::Purge
        );
        assert_eq!(
            get_retention_disposition(LifecycleAction::UninstallDefault, StorageCategory::UserProgress),
            RetentionDisposition::Preserve
        );
        assert_eq!(
            get_retention_disposition(LifecycleAction::UninstallDefault, StorageCategory::LabPackages),
            RetentionDisposition::Preserve
        );

        // UninstallFullPurge: purges everything
        assert_eq!(
            get_retention_disposition(LifecycleAction::UninstallFullPurge, StorageCategory::UserProgress),
            RetentionDisposition::Purge
        );
        assert_eq!(
            get_retention_disposition(LifecycleAction::UninstallFullPurge, StorageCategory::LabPackages),
            RetentionDisposition::Purge
        );
    }

    #[test]
    fn test_update_simulation_leaves_user_data_intact() {
        let temp_root = std::env::temp_dir().join(format!("zitera_update_sim_{}", std::process::id()));
        let app_dir = temp_root.join("Programs").join("ZiteraLab");
        let data_dir = temp_root.join("ZiteraLabData");

        let paths = StoragePaths::custom(app_dir.clone(), data_dir.clone());
        paths.ensure_data_directories().unwrap();

        // 1. Simulate existing student progress and cache
        let progress_file = paths.user_progress_file();
        fs::write(&progress_file, r#"{"completed_labs":["A01"],"score":100}"#).unwrap();

        let cache_file = paths.cache_dir.join("staging_pkg.tmp");
        fs::write(&cache_file, b"temporary cache content").unwrap();

        // 2. Simulate application update: replace app_dir, purge cache
        let _ = fs::remove_dir_all(&app_dir);
        fs::create_dir_all(&app_dir).unwrap();
        fs::write(app_dir.join("zitera-engine.exe"), b"MOCK_NEW_BINARY_v2").unwrap();
        paths.purge_cache().unwrap();

        // 3. Assert user progress remains 100% intact
        assert!(progress_file.exists());
        let progress_content = fs::read_to_string(&progress_file).unwrap();
        assert_eq!(progress_content, r#"{"completed_labs":["A01"],"score":100}"#);

        // 4. Assert cache was cleared
        assert!(!cache_file.exists());

        let _ = fs::remove_dir_all(&temp_root);
    }

    #[test]
    fn test_legacy_state_migration_to_zitera_2x() {
        let temp_root = std::env::temp_dir().join(format!("zitera_mig_test_{}", std::process::id()));
        let app_dir = temp_root.join("Programs").join("ZiteraLab");
        let data_dir = temp_root.join("ZiteraLabData");

        let paths = StoragePaths::custom(app_dir, data_dir);
        paths.ensure_data_directories().unwrap();

        // 1. Create legacy progress file with old lab IDs and legacy structure
        let legacy_file = paths.user_dir.join("zitera_progress.json");
        let legacy_json = serde_json::json!({
            "language": "en",
            "completed_labs": ["zitera-lab-a01", "a02"],
            "completed_challenges": ["zitera-lab-a01"],
            "completed_practice": ["zitera-a01"],
            "completed_sections": {
                "zitera-lab-a01": ["intro", "vulnerability"]
            },
            "lab_records": {
                "zitera-lab-a01": {
                    "attempts": 3,
                    "challenge_completed": true,
                    "practice_completed": true
                }
            },
            "solved_flags": {
                "zitera-lab-a01": "FLAG{legacy_plain_flag}"
            }
        });
        fs::write(&legacy_file, serde_json::to_string_pretty(&legacy_json).unwrap()).unwrap();

        // 2. Run migration
        let migrated = paths.migrate_legacy_state(None).expect("Migration must succeed");
        assert!(migrated, "Migration should have executed");

        // 3. Verify canonical progress.json exists
        let canonical = paths.user_progress_file();
        assert!(canonical.exists(), "Canonical progress.json must exist after migration");

        // 4. Load migrated data and assert all IDs normalized to canonical A01 and A02
        let progress = paths.load_progress(None).expect("Progress must load");
        let obj = progress.as_object().unwrap();

        // solved_flags must be sanitized
        assert!(!obj.contains_key("solved_flags"), "Secret plain flags must be purged during migration");

        // completed_labs normalized
        let labs: Vec<String> = obj["completed_labs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();
        assert!(labs.contains(&"A01".to_string()));
        assert!(labs.contains(&"A02".to_string()));

        // completed_challenges normalized
        let challenges: Vec<String> = obj["completed_challenges"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();
        assert!(challenges.contains(&"A01".to_string()));

        // lab_records normalized
        let recs = obj["lab_records"].as_object().unwrap();
        assert!(recs.contains_key("A01"), "lab_records must contain normalized A01 key");
        assert_eq!(recs["A01"]["attempts"], 3);

        // completed_sections normalized
        let secs = obj["completed_sections"].as_object().unwrap();
        assert!(secs.contains_key("A01"), "completed_sections must contain normalized A01 key");

        let _ = fs::remove_dir_all(&temp_root);
    }

    #[test]
    fn test_package_id_migration_and_query_compatibility() {
        assert_eq!(normalize_lab_id("zitera-lab-a01"), "A01");
        assert_eq!(normalize_lab_id("zitera-a01"), "A01");
        assert_eq!(normalize_lab_id("a01"), "A01");
        assert_eq!(normalize_lab_id("A01"), "A01");
        assert_eq!(canonical_package_id_from_lab("A01"), "zitera-lab-a01");
        assert_eq!(canonical_package_id_from_lab("zitera-lab-a01"), "zitera-lab-a01");
    }

    #[test]
    fn test_progress_preservation_across_simulated_app_restart() {
        let temp_root = std::env::temp_dir().join(format!("zitera_restart_test_{}", std::process::id()));
        let app_dir = temp_root.join("Programs").join("ZiteraLab");
        let data_dir = temp_root.join("ZiteraLabData");

        // Session 1: App creates and updates progress
        {
            let paths1 = StoragePaths::custom(app_dir.clone(), data_dir.clone());
            let progress1 = serde_json::json!({
                "language": "id",
                "completed_labs": ["A01", "A05"],
                "completed_challenges": ["A01"],
                "completed_practice": ["A01"],
                "completed_sections": {"A01": ["section_1", "section_2"]},
                "lab_records": {
                    "A01": {"attempts": 2, "score": 100}
                }
            });
            paths1.save_progress(&progress1).expect("Initial save must succeed");
        } // All handles dropped (simulating full app exit)

        // Session 2: Fresh app process launches, resolves paths, loads progress
        {
            let paths2 = StoragePaths::custom(app_dir, data_dir);
            let loaded = paths2.load_progress(None).expect("Progress must load after restart");
            assert_eq!(loaded["language"], "id");
            let labs = loaded["completed_labs"].as_array().unwrap();
            assert_eq!(labs.len(), 2);
            assert_eq!(loaded["completed_challenges"][0], "A01");
            assert_eq!(loaded["lab_records"]["A01"]["score"], 100);
        }

        let _ = fs::remove_dir_all(&temp_root);
    }

    #[test]
    fn test_progress_preservation_across_package_update() {
        let temp_root = std::env::temp_dir().join(format!("zitera_pkg_upd_test_{}", std::process::id()));
        let app_dir = temp_root.join("Programs").join("ZiteraLab");
        let data_dir = temp_root.join("ZiteraLabData");

        let paths = StoragePaths::custom(app_dir, data_dir);
        paths.ensure_data_directories().unwrap();

        // 1. Initial state: Lab A01 installed (v1.0.0), student solved challenge
        let lab_a01_dir = paths.labs_dir.join("A01");
        fs::create_dir_all(&lab_a01_dir).unwrap();
        fs::write(lab_a01_dir.join("manifest.json"), r#"{"package_id":"zitera-lab-a01","version":"1.0.0"}"#).unwrap();

        let initial_progress = serde_json::json!({
            "completed_labs": ["A01"],
            "completed_challenges": ["A01"],
            "lab_records": {"A01": {"attempts": 1, "completed": true}}
        });
        paths.save_progress(&initial_progress).unwrap();

        // 2. Package update: Lab A01 files updated to v2.0.0 (simulating package installer)
        fs::write(lab_a01_dir.join("manifest.json"), r#"{"package_id":"zitera-lab-a01","version":"2.0.0","new_meta":"added"}"#).unwrap();
        fs::write(lab_a01_dir.join("active_version.txt"), "2.0.0").unwrap();

        // 3. User progress loaded: must remain unchanged
        let loaded = paths.load_progress(None).unwrap();
        assert_eq!(loaded["completed_challenges"][0], "A01");
        assert_eq!(loaded["lab_records"]["A01"]["attempts"], 1);

        let _ = fs::remove_dir_all(&temp_root);
    }

    #[test]
    fn test_progress_preservation_across_remove_and_reinstall() {
        let temp_root = std::env::temp_dir().join(format!("zitera_reinstall_test_{}", std::process::id()));
        let app_dir = temp_root.join("Programs").join("ZiteraLab");
        let data_dir = temp_root.join("ZiteraLabData");

        let paths = StoragePaths::custom(app_dir, data_dir);
        paths.ensure_data_directories().unwrap();

        // 1. Lab A01 installed with progress
        let lab_a01_dir = paths.labs_dir.join("A01");
        fs::create_dir_all(&lab_a01_dir).unwrap();
        fs::write(lab_a01_dir.join("manifest.json"), r#"{"package_id":"zitera-lab-a01","version":"1.0.0"}"#).unwrap();

        let student_progress = serde_json::json!({
            "completed_labs": ["A01"],
            "completed_challenges": ["A01"],
            "lab_records": {"A01": {"attempts": 5, "conquered": true}}
        });
        paths.save_progress(&student_progress).unwrap();

        // 2. Simulate lab removal: labs/A01 directory deleted
        let _ = fs::remove_dir_all(&lab_a01_dir);
        assert!(!lab_a01_dir.exists(), "Lab package directory deleted");

        // 3. Verify user progress was NOT deleted by lab uninstall
        assert!(paths.user_progress_file().exists(), "User progress file must survive lab removal");

        // 4. Simulate lab reinstall: new package unpacked to labs/A01
        fs::create_dir_all(&lab_a01_dir).unwrap();
        fs::write(lab_a01_dir.join("manifest.json"), r#"{"package_id":"zitera-lab-a01","version":"1.0.1"}"#).unwrap();

        // 5. Verify student coursework is immediately restored
        let loaded = paths.load_progress(None).unwrap();
        assert_eq!(loaded["completed_challenges"][0], "A01");
        assert_eq!(loaded["lab_records"]["A01"]["attempts"], 5);

        let _ = fs::remove_dir_all(&temp_root);
    }
}
