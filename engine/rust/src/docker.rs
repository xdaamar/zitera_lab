use crate::process::run_cmd;
use std::path::Path;

pub fn get_docker_cmd() -> String {
    if run_cmd("docker", &["--version"], None).is_ok() {
        return "docker".to_string();
    }
    let fallback_paths = [
        r"C:\Program Files\Docker\Docker\resources\bin\docker.exe",
        r"C:\Users\Damar\AppData\Local\Programs\DockerDesktop\resources\bin\docker.exe",
    ];
    for p in &fallback_paths {
        if Path::new(p).exists() {
            return p.to_string();
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
        &["compose", "-f", &compose_str, "-p", &project_name, "down"],
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

pub fn get_lab_container_status(lab_id: &str) -> bool {
    let docker_bin = get_docker_cmd();
    let project_name = format!("zitera_{}", lab_id.to_lowercase());
    match run_cmd(
        &docker_bin,
        &[
            "ps",
            "--filter",
            &format!("name={}", project_name),
            "--format",
            "{{.Status}}",
        ],
        None,
    ) {
        Ok(out) => out.success && out.stdout.contains("Up"),
        Err(_) => false,
    }
}
