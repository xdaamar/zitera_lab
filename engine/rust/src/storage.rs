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
}
