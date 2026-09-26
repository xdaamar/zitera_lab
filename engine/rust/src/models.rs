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

    pub fn err(action: impl Into<String>, code: impl Into<String>, message: impl Into<String>, recoverable: bool) -> Self {
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
    pub status: String,
    pub install_guide: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LabManifest {
    pub schema_version: u32,
    pub id: String,
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CatalogLabItem {
    pub id: String,
    pub title: String,
    pub repository: String,
    pub version: String,
    pub difficulty: String,
    pub owasp: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Catalog {
    pub schema_version: u32,
    pub labs: Vec<CatalogLabItem>,
}
