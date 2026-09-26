use crate::models::{Catalog, CatalogLabItem};
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_catalog_path(workspace_root: &Path) -> PathBuf {
    workspace_root.join("catalog").join("catalog.json")
}

pub fn load_catalog(workspace_root: &Path) -> Result<Catalog, String> {
    let path = get_catalog_path(workspace_root);
    if !path.exists() {
        // Return default built-in catalog if file doesn't exist yet
        return Ok(default_catalog());
    }

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read catalog at {:?}: {}", path, e))?;

    serde_json::from_str(&content).map_err(|e| format!("Failed to parse catalog JSON: {}", e))
}

pub fn default_catalog() -> Catalog {
    Catalog {
        schema_version: 1,
        labs: vec![
            CatalogLabItem {
                id: "A01".to_string(),
                title: "Broken Access Control".to_string(),
                repository: "xdaamar/zitera_lab_a01".to_string(),
                version: "1.0.0".to_string(),
                difficulty: "Beginner".to_string(),
                owasp: "A01:2025".to_string(),
                description: "Learn how authorization flaws allow unauthorized users to view, tamper with, or delete sensitive data.".to_string(),
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
        ],
    }
}
