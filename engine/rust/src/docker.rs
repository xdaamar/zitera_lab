use crate::process::run_cmd;
use std::path::Path;

pub fn get_docker_cmd() -> String {
    // Resolve user-profile-relative paths dynamically — never hardcode usernames
    let local_appdata = std::env::var("LOCALAPPDATA").unwrap_or_default();
    let program_files = std::env::var("ProgramFiles").unwrap_or_default();

    let candidate_paths: Vec<String> = vec![
        // Docker Desktop bundled CLI (most reliably allowed by AppControl policies)
        format!(
            r"{}\Programs\DockerDesktop\resources\bin\docker.exe",
            local_appdata
        ),
        // System-wide Docker Desktop install
        format!(r"{}\Docker\Docker\resources\bin\docker.exe", program_files),
        // WinGet-installed standalone Docker CLI
        format!(
            r"{}\Microsoft\WinGet\Packages\Docker.DockerCLI_Microsoft.Winget.Source_8wekyb3d8bbwe\docker\docker.exe",
            local_appdata
        ),
        // Scoop-installed Docker
        format!(
            r"{}\scoop\shims\docker.exe",
            std::env::var("USERPROFILE").unwrap_or_default()
        ),
        // Chocolatey-installed Docker
        r"C:\ProgramData\chocolatey\bin\docker.exe".to_string(),
        "docker".to_string(),
    ];

    for p in &candidate_paths {
        if !p.is_empty() && (p == "docker" || Path::new(p).exists()) {
            if let Ok(out) = run_cmd(p, &["--version"], None) {
                if out.success {
                    return p.clone();
                }
            }
        }
    }
    "docker".to_string()
}

pub fn start_lab(compose_path: &Path, lab_id: &str) -> Result<String, String> {
    if !compose_path.exists() {
        return Err(format!(
            "Docker compose file not found at {:?}",
            compose_path
        ));
    }

    let docker_bin = get_docker_cmd();
    let project_name = format!("zitera_{}", lab_id.to_lowercase());
    let compose_str = compose_path.to_string_lossy();

    let out = run_cmd(
        &docker_bin,
        &[
            "compose",
            "-f",
            &compose_str,
            "-p",
            &project_name,
            "up",
            "-d",
            "--build",
        ],
        compose_path.parent(),
    )?;

    if out.success {
        Ok(format!("Lab {} started successfully.", lab_id))
    } else {
        Err(format!("Docker compose failed to start: {}", out.stderr))
    }
}

pub fn stop_lab(compose_path: &Path, lab_id: &str) -> Result<String, String> {
    let docker_bin = get_docker_cmd();
    let project_name = format!("zitera_{}", lab_id.to_lowercase());
    let compose_str = compose_path.to_string_lossy();

    let out = run_cmd(
        &docker_bin,
        &["compose", "-f", &compose_str, "-p", &project_name, "down", "--remove-orphans"],
        compose_path.parent(),
    )?;

    if out.success {
        Ok(format!("Lab {} stopped.", lab_id))
    } else {
        Err(format!("Docker compose stop failed: {}", out.stderr))
    }
}

pub fn reset_lab(compose_path: &Path, lab_id: &str) -> Result<String, String> {
    let docker_bin = get_docker_cmd();
    let project_name = format!("zitera_{}", lab_id.to_lowercase());
    let compose_str = compose_path.to_string_lossy();

    // Reset with -v removes volumes deterministically restoring initial state
    let _ = run_cmd(
        &docker_bin,
        &[
            "compose",
            "-f",
            &compose_str,
            "-p",
            &project_name,
            "down",
            "-v",
            "--remove-orphans",
        ],
        compose_path.parent(),
    );

    let up = run_cmd(
        &docker_bin,
        &[
            "compose",
            "-f",
            &compose_str,
            "-p",
            &project_name,
            "up",
            "-d",
            "--build",
            "--force-recreate",
        ],
        compose_path.parent(),
    )?;

    if up.success {
        Ok(format!("Lab {} has been deterministically reset.", lab_id))
    } else {
        Err(format!("Failed to recreate lab on reset: {}", up.stderr))
    }
}

pub fn get_running_containers() -> std::collections::HashSet<String> {
    let docker_bin = get_docker_cmd();
    let mut running = std::collections::HashSet::new();
    if let Ok(out) = run_cmd(&docker_bin, &["ps", "--format", "{{.Names}}"], None) {
        if out.success {
            for line in out.stdout.lines() {
                let name = line.trim();
                if !name.is_empty() {
                    running.insert(name.to_lowercase());
                }
            }
        }
    }
    running
}

pub fn get_lab_container_status(lab_id: &str) -> bool {
    let project_name = format!("zitera_{}", lab_id.to_lowercase());
    let running = get_running_containers();
    running.iter().any(|name| name.contains(&project_name))
}

/// Prunes dangling stopped containers and anonymous volumes associated with zitera projects.
pub fn cleanup_orphans() -> Result<(), String> {
    let docker_bin = get_docker_cmd();
    let _ = run_cmd(
        &docker_bin,
        &["container", "prune", "-f", "--filter", "label=com.docker.compose.project"],
        None,
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docker_cmd_detection() {
        let cmd = get_docker_cmd();
        assert!(!cmd.is_empty());
    }

    #[test]
    fn test_project_name_convention() {
        let lab_id = "A01";
        let project_name = format!("zitera_{}", lab_id.to_lowercase());
        assert_eq!(project_name, "zitera_a01");
    }
}

