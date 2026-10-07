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
