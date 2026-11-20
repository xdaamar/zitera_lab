use std::fs;
use std::path::Path;
use zitera_engine::catalog::default_catalog;

fn main() {
    let catalog = default_catalog();
    let json_str = serde_json::to_string_pretty(&catalog).expect("Failed to serialize catalog");
    let dest = Path::new("catalog/catalog.json");
    if let Some(parent) = dest.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(dest, json_str).expect("Failed to write catalog.json");
    println!("Successfully exported and signed catalog with {} labs to catalog/catalog.json", catalog.labs.len());
}
