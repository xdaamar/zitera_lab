//! Zitera Terminal Engine Core
//!
//! Provides a secure, in-process pseudo-terminal for cybersecurity labs.
//! Guarantees:
//! - Zero host shell passthrough (no cmd.exe, no powershell)
//! - Complete virtual filesystem containment (Linux logical tree /)
//! - Deterministic Unix utilities (pwd, ls, cd, cat, grep, head, tail, etc.)
//! - Strict rejection of shell metacharacters and injection operators

pub mod commands;
pub mod fs;
pub mod parser;

use commands::execute_builtin;
use fs::TerminalFilesystem;
use parser::{parse_command, TerminalError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub cwd: String,
}

pub struct TerminalSession {
    vfs: TerminalFilesystem,
}

impl TerminalSession {
    /// Creates a new terminal session rooted at the specified physical sandbox directory.
    pub fn new(physical_root: PathBuf) -> Self {
        Self {
            vfs: TerminalFilesystem::new(physical_root),
        }
    }

    /// Returns the current logical working directory.
    pub fn cwd(&self) -> &str {
        self.vfs.current_virtual_dir()
    }

    /// Executes a single terminal command line safely.
    pub fn execute(&mut self, input: &str) -> TerminalResult {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return TerminalResult {
                stdout: String::new(),
                stderr: String::new(),
                exit_code: 0,
                cwd: self.cwd().to_string(),
            };
        }

        let parsed = match parse_command(trimmed) {
            Ok(cmd) => cmd,
            Err(e) => {
                return TerminalResult {
                    stdout: String::new(),
                    stderr: format!("{}\n", e),
                    exit_code: match e {
                        TerminalError::EmptyCommand => 0,
                        TerminalError::CommandNotAllowed(_) => 127,
                        TerminalError::ShellInjectionDetected(_) => 2,
                        TerminalError::UnmatchedQuotes => 2,
                        _ => 1,
                    },
                    cwd: self.cwd().to_string(),
                };
            }
        };

        // Handle tool commands (curl, nmap) if requested, or execute built-in
        let out = if parsed.name == "curl" || parsed.name == "nmap" {
            self.execute_tool(&parsed.name, &parsed.args)
        } else {
            execute_builtin(&parsed.name, &parsed.args, &mut self.vfs)
        };

        TerminalResult {
            stdout: out.stdout,
            stderr: out.stderr,
            exit_code: out.exit_code,
            cwd: self.cwd().to_string(),
        }
    }

    fn execute_tool(&self, tool: &str, args: &[String]) -> commands::CommandOutput {
        // Enforce safe arguments for curl / nmap to prevent file overwrite escapes
        for arg in args {
            if arg.starts_with("-o") || arg.starts_with("--output") || arg.contains("file://") {
                return commands::CommandOutput::err(
                    format!(
                        "{}: file output redirection is restricted in terminal mode\n",
                        tool
                    ),
                    1,
                );
            }
        }

        let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        // Run tool through process helper without any shell wrapper
        match crate::process::run_cmd(tool, &str_args, None) {
            Ok(proc_out) => commands::CommandOutput {
                stdout: proc_out.stdout,
                stderr: proc_out.stderr,
                exit_code: if proc_out.success { 0 } else { 1 },
            },
            Err(e) => commands::CommandOutput::err(
                format!("{}: tool execution failed: {}\n", tool, e),
                127,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup_test_env(name: &str) -> (PathBuf, TerminalSession) {
        let temp = std::env::temp_dir().join(format!("zitera_term_test_{}", name));
        let _ = fs::remove_dir_all(&temp);
        let _ = fs::create_dir_all(&temp);
        let session = TerminalSession::new(temp.clone());
        (temp, session)
    }

    #[test]
    fn test_terminal_basic_lifecycle() {
        let (temp, mut term) = setup_test_env("lifecycle");

        // 1. pwd initial
        let res = term.execute("pwd");
        assert_eq!(res.exit_code, 0);
        assert_eq!(res.stdout, "/\n");
        assert_eq!(res.cwd, "/");

        // 2. mkdir and cd
        let res_mkdir = term.execute("mkdir src");
        assert_eq!(res_mkdir.exit_code, 0);

        let res_cd = term.execute("cd src");
        assert_eq!(res_cd.exit_code, 0);
        assert_eq!(term.cwd(), "/src");

        // 3. echo into file
        let res_touch = term.execute("touch main.c");
        assert_eq!(res_touch.exit_code, 0);

        let res_ls = term.execute("ls");
        assert_eq!(res_ls.exit_code, 0);
        assert!(res_ls.stdout.contains("main.c"));

        // 4. cd back to root
        let res_cd_back = term.execute("cd ..");
        assert_eq!(res_cd_back.exit_code, 0);
        assert_eq!(term.cwd(), "/");

        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_terminal_shell_injection_rejection() {
        let (temp, mut term) = setup_test_env("injections");

        let attacks = [
            "ls; whoami",
            "ls && whoami",
            "cat /etc/passwd | grep root",
            "echo `whoami`",
            "echo $(id)",
            "cat foo > /root/bad",
            "cat foo >> /root/bad",
            "touch a || rm -rf /",
            "ls & cmd",
        ];

        for atk in attacks {
            let res = term.execute(atk);
            assert_eq!(
                res.exit_code, 2,
                "Expected shell injection rejection for: {}",
                atk
            );
            assert!(
                res.stderr.contains("syntax error: shell operators"),
                "Expected syntax error message, got: {}",
                res.stderr
            );
        }

        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_terminal_forbidden_commands_rejection() {
        let (temp, mut term) = setup_test_env("forbidden");

        let forbidden = [
            "cmd",
            "cmd.exe",
            "powershell",
            "pwsh",
            "wmic",
            "reg",
            "diskpart",
            "runas",
            "wsl",
            "bash",
            "sh",
        ];

        for cmd in forbidden {
            let res = term.execute(cmd);
            assert_eq!(
                res.exit_code, 127,
                "Forbidden command must return 127: {}",
                cmd
            );
            assert!(
                res.stderr.contains("command not found"),
                "Got: {}",
                res.stderr
            );
        }

        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_terminal_filesystem_escape_matrix() {
        let (temp, mut term) = setup_test_env("escape");

        let escape_attempts = [
            "cd C:/",
            "cd C:\\Windows",
            "cd ..\\..\\..",
            "cd //server/share",
            "cat C:/Windows/win.ini",
            "cat ..\\secret.txt",
        ];

        for esc in escape_attempts {
            let res = term.execute(esc);
            assert!(
                res.exit_code != 0,
                "Path escape attempt should fail: {}",
                esc
            );
            assert!(
                res.stderr.contains("access denied") || res.stderr.contains("No such file"),
                "Expected access denied, got: {}",
                res.stderr
            );
            // Verify working directory did NOT escape
            assert!(
                term.cwd().starts_with('/'),
                "Logical cwd was corrupted: {}",
                term.cwd()
            );
        }

        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_terminal_utilities_suite() {
        let (temp, mut term) = setup_test_env("utilities");

        // Create sample file
        term.vfs.write_file("data.log", "2026-01-01 INFO startup\n2026-01-02 WARN memory\n2026-01-03 ERROR crash\n2026-01-04 DEBUG done\n").unwrap();

        // test cat
        let cat_res = term.execute("cat data.log");
        assert_eq!(cat_res.exit_code, 0);
        assert!(cat_res.stdout.contains("INFO startup"));

        // test head -n 2
        let head_res = term.execute("head -n 2 data.log");
        assert_eq!(head_res.exit_code, 0);
        assert_eq!(
            head_res.stdout,
            "2026-01-01 INFO startup\n2026-01-02 WARN memory\n"
        );

        // test tail -n 2
        let tail_res = term.execute("tail -n 2 data.log");
        assert_eq!(tail_res.exit_code, 0);
        assert_eq!(
            tail_res.stdout,
            "2026-01-03 ERROR crash\n2026-01-04 DEBUG done\n"
        );

        // test grep ERROR
        let grep_res = term.execute("grep ERROR data.log");
        assert_eq!(grep_res.exit_code, 0);
        assert_eq!(grep_res.stdout, "2026-01-03 ERROR crash\n");

        // test grep -v ERROR
        let grep_v_res = term.execute("grep -v ERROR data.log");
        assert_eq!(grep_v_res.exit_code, 0);
        assert!(!grep_v_res.stdout.contains("ERROR crash"));

        // test cp
        let cp_res = term.execute("cp data.log backup.log");
        assert_eq!(cp_res.exit_code, 0);

        // test find
        let find_res = term.execute("find . -name *.log");
        assert_eq!(find_res.exit_code, 0);
        assert!(find_res.stdout.contains("data.log"));
        assert!(find_res.stdout.contains("backup.log"));

        // test mv
        let mv_res = term.execute("mv backup.log archived.log");
        assert_eq!(mv_res.exit_code, 0);

        // test rm
        let rm_res = term.execute("rm archived.log");
        assert_eq!(rm_res.exit_code, 0);

        // test whoami & uname
        let who_res = term.execute("whoami");
        assert_eq!(who_res.stdout, "learner\n");

        let uname_res = term.execute("uname");
        assert_eq!(uname_res.stdout, "Linux\n");

        let _ = fs::remove_dir_all(&temp);
    }
}
