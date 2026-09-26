use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

pub fn run_cmd(program: &str, args: &[&str], cwd: Option<&Path>) -> Result<CommandOutput, String> {
    let mut cmd = Command::new(program);
    cmd.args(args);
    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    match cmd.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            let success = output.status.success();
            Ok(CommandOutput {
                stdout,
                stderr,
                success,
            })
        }
        Err(e) => Err(format!("Failed to execute '{}': {}", program, e)),
    }
}
