use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}

impl<T> ApiResponse<T> {
    pub fn ok(action: impl Into<String>, data: T) -> Self {
        Self {
            success: true,
            action: action.into(),
            data: Some(data),
            error: None,
        }
    }

    pub fn err(
        action: impl Into<String>,
        code: impl Into<String>,
        message: impl Into<String>,
        recoverable: bool,
    ) -> Self {
        Self {
            success: false,
            action: action.into(),
            data: None,
            error: Some(ApiError {
                code: code.into(),
                message: message.into(),
                recoverable,
                details: None,
                recovery_action: None,
            }),
        }
    }

    pub fn err_with_recovery(
        action: impl Into<String>,
        code: impl Into<String>,
        message: impl Into<String>,
        recovery_action: impl Into<String>,
        recoverable: bool,
    ) -> Self {
        Self {
            success: false,
            action: action.into(),
            data: None,
            error: Some(ApiError {
                code: code.into(),
                message: message.into(),
                recoverable,
                details: None,
                recovery_action: Some(recovery_action.into()),
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub recoverable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_action: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComponentStatus {
    pub name: String,
    pub installed: bool,
    pub version: Option<String>,
    pub status: String, // READY, MISSING, OUTDATED, WARNING, BLOCKED, MANUAL_ACTION_REQUIRED
    pub message: String,
    pub recommendation: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemDiagnostics {
    pub os: ComponentStatus,
    pub git: ComponentStatus,
    pub wsl: ComponentStatus,
    pub docker: ComponentStatus,
    pub docker_daemon: ComponentStatus,
    pub powershell: ComponentStatus,
    pub memory_gb: f64,
    pub disk_free_gb: f64,
    pub all_ready: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolStatus {
    pub id: String,
    pub name: String,
    pub category: String,
    pub installed: bool,
    pub version: Option<String>,
    pub min_version: String,
    pub status: String, // READY, MISSING, OUTDATED, BLOCKED, ERROR, UNKNOWN
    pub path: Option<String>,
    pub capabilities: Vec<String>,
    pub install_method: String, // "winget", "pip", "manual", "guide"
    pub install_guide: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LabRequirements {
    #[serde(default)]
    pub tools: Vec<String>,
    #[serde(default)]
    pub required_tools: Vec<String>,
    #[serde(default)]
    pub recommended_tools: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LabManifest {
    pub schema_version: u32,
    pub id: String,
    #[serde(default)]
    pub package_id: Option<String>,
    pub slug: String,
    pub title: String,
    pub owasp: String,
    pub version: String,
    pub difficulty: String,
    pub runtime: String,
    pub entrypoint: String,
    pub default_port: u16,
    pub estimated_minutes: u32,
    pub modes: Vec<String>,
    #[serde(default)]
    pub short_description: Option<String>,
    #[serde(default)]
    pub standard: Option<String>,
    #[serde(default)]
    pub standard_version: Option<String>,
    #[serde(default)]
    pub category_id: Option<String>,
    #[serde(default)]
    pub category_name: Option<String>,
    #[serde(default)]
    pub estimated_time: Option<u32>,
    #[serde(default)]
    pub skills: Vec<String>,
    #[serde(default)]
    pub learning_objectives: Vec<String>,
    #[serde(default)]
    pub prerequisites: Vec<String>,
    #[serde(default)]
    pub requirements: Option<LabRequirements>,
    #[serde(default)]
    pub architecture: Option<String>,
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(default)]
    pub lesson: Option<String>,
    #[serde(default)]
    pub practice: Option<String>,
    #[serde(default)]
    pub challenge: Option<String>,
    #[serde(default)]
    pub package_size: Option<u64>,
    #[serde(default)]
    pub sha256: Option<String>,
    #[serde(default)]
    pub signature: Option<String>,
    #[serde(default)]
    pub security_version: Option<u32>,
    #[serde(default)]
    pub minimum_core_version: Option<String>,
}

impl LabManifest {
    pub fn package_id(&self) -> String {
        self.package_id
            .clone()
            .unwrap_or_else(|| format!("zitera-lab-{}", self.id.to_lowercase()))
    }

    pub fn standard(&self) -> String {
        self.standard
            .clone()
            .unwrap_or_else(|| "owasp-top10".to_string())
    }

    pub fn standard_version(&self) -> String {
        self.standard_version
            .clone()
            .unwrap_or_else(|| "2025".to_string())
    }

    pub fn category_id(&self) -> String {
        self.category_id
            .clone()
            .unwrap_or_else(|| self.id.clone())
    }

    pub fn category_name(&self) -> String {
        self.category_name
            .clone()
            .unwrap_or_else(|| self.title.clone())
    }

    pub fn short_description(&self) -> String {
        self.short_description
            .clone()
            .unwrap_or_else(|| format!("Interactive laboratory module for {}.", self.title))
    }

    pub fn security_version(&self) -> u32 {
        self.security_version.unwrap_or(1)
    }

    pub fn minimum_core_version(&self) -> String {
        self.minimum_core_version
            .clone()
            .unwrap_or_else(|| "2.0.0".to_string())
    }

    pub fn validate_curriculum_contract(&self) -> Result<(), String> {
        if self.id.trim().is_empty() {
            return Err("Manifest 'id' cannot be empty.".to_string());
        }
        if self.title.trim().is_empty() {
            return Err("Manifest 'title' cannot be empty.".to_string());
        }
        if self.version.trim().is_empty() {
            return Err("Manifest 'version' cannot be empty.".to_string());
        }
        if self.modes.is_empty() {
            return Err("Manifest must specify at least one mode.".to_string());
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LabStatus {
    pub id: String,
    pub title: String,
    pub installed: bool,
    pub running: bool,
    pub port: u16,
    pub url: Option<String>,
    pub version: String,
    pub status: String, // NOT_INSTALLED, STOPPED, RUNNING, ERROR
    #[serde(default = "default_ready")]
    pub learn_readiness: String, // READY
    #[serde(default = "default_ready")]
    pub practice_readiness: String, // READY
    #[serde(default = "default_ready")]
    pub challenge_readiness: String, // READY, PARTIAL, BLOCKED
    #[serde(default)]
    pub recommended_tools: Vec<String>,
}

fn default_ready() -> String {
    "READY".to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CatalogLabItem {
    pub id: String,
    #[serde(default)]
    pub package_id: Option<String>,
    pub title: String,
    pub repository: String,
    pub version: String,
    pub difficulty: String,
    pub owasp: String,
    pub description: String,
    #[serde(default)]
    pub short_description: Option<String>,
    #[serde(default)]
    pub standard: Option<String>,
    #[serde(default)]
    pub standard_version: Option<String>,
    #[serde(default)]
    pub category_id: Option<String>,
    #[serde(default)]
    pub category_name: Option<String>,
    #[serde(default)]
    pub security_version: Option<u32>,
    #[serde(default)]
    pub minimum_core_version: Option<String>,
    #[serde(default)]
    pub estimated_time: Option<u32>,
    #[serde(default)]
    pub skills: Vec<String>,
    #[serde(default)]
    pub learning_objectives: Vec<String>,
    #[serde(default)]
    pub prerequisites: Vec<String>,
    #[serde(default)]
    pub runtime: Option<String>,
    #[serde(default)]
    pub architecture: Option<String>,
    #[serde(default)]
    pub package_url: Option<String>,
    #[serde(default)]
    pub package_sha256: Option<String>,
}

impl CatalogLabItem {
    pub fn package_id(&self) -> String {
        self.package_id
            .clone()
            .unwrap_or_else(|| format!("zitera-lab-{}", self.id.to_lowercase()))
    }

    pub fn standard(&self) -> String {
        self.standard
            .clone()
            .unwrap_or_else(|| "owasp-top10".to_string())
    }

    pub fn standard_version(&self) -> String {
        self.standard_version
            .clone()
            .unwrap_or_else(|| "2025".to_string())
    }

    pub fn category_id(&self) -> String {
        self.category_id
            .clone()
            .unwrap_or_else(|| self.id.clone())
    }

    pub fn category_name(&self) -> String {
        self.category_name
            .clone()
            .unwrap_or_else(|| self.title.clone())
    }

    pub fn short_description(&self) -> String {
        self.short_description
            .clone()
            .unwrap_or_else(|| self.description.clone())
    }

    pub fn security_version(&self) -> u32 {
        self.security_version.unwrap_or(1)
    }

    pub fn minimum_core_version(&self) -> String {
        self.minimum_core_version
            .clone()
            .unwrap_or_else(|| "2.0.0".to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Catalog {
    pub schema_version: u32,
    pub labs: Vec<CatalogLabItem>,
    #[serde(default)]
    pub signature: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgressiveHint {
    pub tier: u32,
    #[serde(rename = "type")]
    pub hint_type: String,
    pub hint: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LabContent {
    pub manifest: LabManifest,
    pub lessons: std::collections::HashMap<String, String>,
    pub challenge_objective: String,
    pub hints: Vec<ProgressiveHint>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChallengeVerification {
    pub lab_id: String,
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PracticeVerification {
    pub lab_id: String,
    pub status: String, // passed, failed, unavailable, error
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolInstallResult {
    pub id: String,
    pub success: bool,
    pub message: String,
    pub method: String,
}

/// Classifies a raw error string into one of the canonical recovery states with actionable guidance.
pub fn classify_recovery_error(action: &str, raw_error: &str) -> (String, String) {
    let lower = raw_error.to_lowercase();
    let upper_action = action.to_uppercase().replace('.', "_");

    if lower.contains("timed out waiting for lab")
        || lower.contains("crashed")
        || lower.contains("unexpected exit")
        || lower.contains("died")
        || lower.contains("readiness failed")
    {
        (
            "LAB_CRASH".to_string(),
            "Restart the lab or reset its runtime state to default.".to_string(),
        )
    } else if lower.contains("rollback") || lower.contains("rolled back") {
        (
            "ROLLBACK_OCCURRED".to_string(),
            "Check previous installed versions or reinstall the package.".to_string(),
        )
    } else if lower.contains("staging")
        || lower.contains("interrupted")
        || lower.contains("staged manifest")
        || lower.contains("staged entrypoint")
    {
        (
            "UPDATE_INTERRUPTED".to_string(),
            "Retry the package update or clean the staging directory.".to_string(),
        )
    } else if lower.contains("appcontainer")
        || lower.contains("executable not found")
        || lower.contains("failed to locate zitera-engine")
        || lower.contains("jobobject")
        || lower.contains("sandbox runtime")
        || lower.contains("runtime is")
    {
        (
            "RUNTIME_UNAVAILABLE".to_string(),
            "Verify sandbox host executable integrity or run 'zitera doctor'.".to_string(),
        )
    } else if lower.contains("verification")
        || lower.contains("signature")
        || lower.contains("digest mismatch")
        || lower.contains("corrupted package")
        || lower.contains("downgrade")
        || lower.contains("checksum")
    {
        (
            "PACKAGE_VERIFICATION_FAILED".to_string(),
            "Re-download the lab package (.zlab) or verify signature.".to_string(),
        )
    } else if lower.contains("storage")
        || lower.contains("permission denied")
        || lower.contains("access is denied")
        || lower.contains("disk full")
        || lower.contains("file lock")
    {
        (
            "STORAGE_FAILURE".to_string(),
            "Purge cache, ensure write permissions for LocalAppData, and retry.".to_string(),
        )
    } else if lower.contains("broker")
        || lower.contains("failed to start broker")
        || lower.contains("port")
        || lower.contains("address already in use")
    {
        (
            "BROKER_UNAVAILABLE".to_string(),
            "Check port availability and restart the lab broker.".to_string(),
        )
    } else if lower.contains("not installed") {
        (
            "LAB_NOT_INSTALLED".to_string(),
            "Install the lab package first via catalog or 'zitera lab install'.".to_string(),
        )
    } else {
        (
            format!("{}_FAILED", upper_action),
            "Retry the operation, reset the lab, or export diagnostic report.".to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_failure_recovery_classification_states() {
        // 1. Lab Crash
        let (code1, rec1) = classify_recovery_error("lab.start", "Timed out waiting for Lab A01 to initialize");
        assert_eq!(code1, "LAB_CRASH");
        assert!(rec1.contains("Restart"));

        // 2. Runtime Unavailable
        let (code2, rec2) = classify_recovery_error("lab.start", "AppContainer profile creation failed");
        assert_eq!(code2, "RUNTIME_UNAVAILABLE");
        assert!(rec2.contains("zitera doctor"));

        // 3. Package Verification Failure
        let (code3, rec3) = classify_recovery_error("lab.install_package", "Signature verification failed: invalid key");
        assert_eq!(code3, "PACKAGE_VERIFICATION_FAILED");
        assert!(rec3.contains(".zlab"));

        // 4. Update Interrupted
        let (code4, rec4) = classify_recovery_error("lab.update", "Staged manifest.json not found: staging interrupted");
        assert_eq!(code4, "UPDATE_INTERRUPTED");
        assert!(rec4.contains("Retry"));

        // 5. Rollback Occurred
        let (code5, rec5) = classify_recovery_error("lab.update", "Post-install verification failed: rolled back to 1.0.0");
        assert_eq!(code5, "ROLLBACK_OCCURRED");
        assert!(rec5.contains("previous"));

        // 6. Storage Failure
        let (code6, rec6) = classify_recovery_error("storage.purge", "Storage permission denied on LocalAppData path");
        assert_eq!(code6, "STORAGE_FAILURE");
        assert!(rec6.contains("Purge cache"));

        // 7. Broker Unavailable
        let (code7, rec7) = classify_recovery_error("lab.start", "Failed to start broker: address already in use on port 8080");
        assert_eq!(code7, "BROKER_UNAVAILABLE");
        assert!(rec7.contains("port availability"));

        // Fallback
        let (code8, rec8) = classify_recovery_error("terminal.execute", "Command syntax error");
        assert_eq!(code8, "TERMINAL_EXECUTE_FAILED");
        assert!(rec8.contains("Retry"));
    }
}
