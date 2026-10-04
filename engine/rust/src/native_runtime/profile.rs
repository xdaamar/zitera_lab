use super::error::NativeRuntimeError;
use std::path::{Path, PathBuf};

pub const PROFILE_NAME_PREFIX: &str = "ZiteraLab_";
pub const MAX_PROFILE_NAME_LEN: usize = 64;

/// Strongly validated lab identity representing a deterministic AppContainer security context.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LabIdentity {
    raw_id: String,
    profile_name: String,
    display_name: String,
    description: String,
}

impl LabIdentity {
    /// Validates and constructs a deterministic LabIdentity.
    /// Strict validation rules:
    /// - 1 to 16 characters
    /// - ASCII alphanumeric, hyphen, or underscore only
    /// - Normalizes to uppercase for consistency
    pub fn new(id: &str) -> Result<Self, NativeRuntimeError> {
        let clean = id.trim().to_uppercase();
        if clean.is_empty() || clean.len() > 16 {
            return Err(NativeRuntimeError::InvalidLabId(id.to_string()));
        }
        if !clean
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err(NativeRuntimeError::InvalidLabId(id.to_string()));
        }

        // Sanitized profile name for Windows AppContainer API (no hyphens, safe identifier)
        let sanitized_id: String = clean
            .chars()
            .map(|c| if c == '-' { '_' } else { c })
            .collect();

        let profile_name = format!("{}{}", PROFILE_NAME_PREFIX, sanitized_id);
        if profile_name.len() > MAX_PROFILE_NAME_LEN {
            return Err(NativeRuntimeError::InvalidLabId(format!(
                "Generated profile name '{}' exceeds maximum allowed length of {}",
                profile_name, MAX_PROFILE_NAME_LEN
            )));
        }

        let display_name = format!("ZITERA_LAB {} Sandbox", clean);
        let description = format!(
            "Per-user isolated AppContainer sandbox for ZITERA_LAB lab {}",
            clean
        );

        Ok(Self {
            raw_id: clean,
            profile_name,
            display_name,
            description,
        })
    }

    pub fn id(&self) -> &str {
        &self.raw_id
    }

    pub fn profile_name(&self) -> &str {
        &self.profile_name
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

/// AppContainer configuration parameters scoped strictly to an isolated lab directory.
#[derive(Debug, Clone)]
pub struct AppContainerProfileConfig {
    pub identity: LabIdentity,
    pub lab_root: PathBuf,
    pub enable_network_loopback: bool,
}

impl AppContainerProfileConfig {
    pub fn new(
        identity: LabIdentity,
        workspace_root: &Path,
        enable_network_loopback: bool,
    ) -> Result<Self, NativeRuntimeError> {
        // Enforce workspace boundary
        let lab_dir = workspace_root.join("labs").join(identity.id());

        // Validate safe subpath
        if !lab_dir.starts_with(workspace_root) {
            return Err(NativeRuntimeError::PathTraversalAttempt(
                lab_dir.to_string_lossy().to_string(),
            ));
        }

        Ok(Self {
            identity,
            lab_root: lab_dir,
            enable_network_loopback,
        })
    }
}
