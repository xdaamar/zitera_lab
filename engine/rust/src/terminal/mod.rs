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

pub fn validate_curl_args(args: &[String]) -> Result<(), String> {
    if args.len() > 64 {
        return Err("curl: too many arguments (argument overflow limit exceeded)\n".to_string());
    }
    for arg in args {
        if arg.len() > 2048 {
            return Err("curl: argument length exceeds maximum allowed limit\n".to_string());
        }
        if arg.starts_with("-o") || arg.starts_with("--output") || arg.contains("file://") {
            return Err("curl: file output redirection is restricted in terminal mode\n".to_string());
        }
        let is_target = arg.starts_with("http://")
            || arg.starts_with("https://")
            || (!arg.starts_with('-')
                && (arg.contains(':')
                    || arg.contains("localhost")
                    || arg.starts_with("127.")
                    || arg.starts_with("192.")
                    || arg.starts_with("10.")
                    || arg.starts_with("172.")
                    || arg.contains(".com")
                    || arg.contains(".org")
                    || arg.contains(".net")
                    || arg.contains(".io")));

        if is_target {
            if arg.starts_with("https://") {
                return Err("curl: https protocol is not supported for local sandboxed lab endpoints\n".to_string());
            }
            if arg.contains('@') {
                return Err("curl: userinfo credentials in target URL are prohibited\n".to_string());
            }
            let without_proto = arg.trim_start_matches("http://");
            let host_port_end = without_proto.find('/').unwrap_or(without_proto.len());
            let host_port = &without_proto[..host_port_end];

            if host_port.is_empty() {
                return Err("curl: malformed target URL\n".to_string());
            }

            let (host, port_str) = if host_port.starts_with('[') {
                if let Some(close_bracket) = host_port.find(']') {
                    let h = &host_port[1..close_bracket];
                    let rem = &host_port[close_bracket + 1..];
                    if let Some(colon) = rem.find(':') {
                        (h, Some(&rem[colon + 1..]))
                    } else {
                        (h, None)
                    }
                } else {
                    return Err("curl: malformed IPv6 target address format\n".to_string());
                }
            } else if let Some(colon) = host_port.find(':') {
                (&host_port[..colon], Some(&host_port[colon + 1..]))
            } else {
                (host_port, None)
            };

            if host.is_empty() {
                return Err("curl: malformed target host in URL\n".to_string());
            }

            if let Some(p) = port_str {
                match p.parse::<u16>() {
                    Ok(val) if val > 0 => {}
                    _ => return Err("curl: invalid port specification in target URL\n".to_string()),
                }
            }

            let host_lower = host.to_lowercase();
            let is_loopback = host_lower == "127.0.0.1"
                || host_lower == "localhost"
                || host_lower == "::1";

            if !is_loopback {
                return Err("curl: access to external network or non-localhost target is denied by sandbox policy\n".to_string());
            }
        }
    }
    Ok(())
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
        if tool == "curl" {
            if let Err(msg) = validate_curl_args(args) {
                return commands::CommandOutput::err(msg, 1);
            }
        } else {
            // Enforce safe arguments for nmap / tools to prevent file overwrite escapes
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
        }

        let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        // Run tool through process helper without any shell wrapper
        match crate::process::run_cmd(tool, &str_args, None) {
            Ok(proc_out) => commands::CommandOutput {
                stdout: proc_out.stdout,
                stderr: proc_out.stderr,
                exit_code: if proc_out.success { 0 } else { 1 },
            },
            Err(e) => {
                if tool == "curl" {
                    if let Some(fallback) = self.try_builtin_curl(args) {
                        return fallback;
                    }
                }
                commands::CommandOutput::err(
                    format!("{}: tool execution failed: {}\n", tool, e),
                    127,
                )
            }
        }
    }

    fn try_builtin_curl(&self, args: &[String]) -> Option<commands::CommandOutput> {
        if let Err(msg) = validate_curl_args(args) {
            return Some(commands::CommandOutput::err(msg, 1));
        }

        let mut target_url = None;
        let mut method = "GET";
        let mut include_headers = false;
        let mut body_data = None;

        let mut idx = 0;
        while idx < args.len() {
            let arg = &args[idx];
            if arg == "-i" || arg == "-I" {
                include_headers = true;
            } else if arg == "-X" && idx + 1 < args.len() {
                idx += 1;
                method = &args[idx];
            } else if (arg == "-d" || arg == "--data") && idx + 1 < args.len() {
                idx += 1;
                body_data = Some(&args[idx]);
                if method == "GET" {
                    method = "POST";
                }
            } else if !arg.starts_with('-') {
                target_url = Some(arg);
            }
            idx += 1;
        }

        let url = target_url?;
        let clean = url.trim_start_matches("http://");
        let slash_pos = clean.find('/').unwrap_or(clean.len());
        let host_port = &clean[..slash_pos];
        let path = if slash_pos < clean.len() {
            &clean[slash_pos..]
        } else {
            "/"
        };

        let (host, port) = if host_port.starts_with('[') {
            if let Some(close_bracket) = host_port.find(']') {
                let h = &host_port[1..close_bracket];
                let rem = &host_port[close_bracket + 1..];
                let p = if let Some(colon) = rem.find(':') {
                    rem[colon + 1..].parse::<u16>().ok()?
                } else {
                    80
                };
                (h, p)
            } else {
                return Some(commands::CommandOutput::err("curl: malformed IPv6 address\n", 1));
            }
        } else if host_port.contains(':') {
            let parts: Vec<&str> = host_port.split(':').collect();
            (parts[0], parts[1].parse::<u16>().ok()?)
        } else {
            (host_port, 80)
        };

        if host != "127.0.0.1" && host != "localhost" && host != "::1" {
            return Some(commands::CommandOutput::err(
                "curl: access to external network or non-localhost target is denied by sandbox policy\n",
                1,
            ));
        }

        use std::io::{Read, Write};
        let mut stream = std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).ok()?;
        stream
            .set_read_timeout(Some(std::time::Duration::from_secs(3)))
            .ok();

        let req_body = body_data.map(|s| s.as_str()).unwrap_or("");
        let req_msg = format!(
            "{} {} HTTP/1.1\r\nHost: {}:{}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            method, path, host, port, req_body.len(), req_body
        );

        stream.write_all(req_msg.as_bytes()).ok()?;
        stream.flush().ok()?;

        let mut buf = Vec::new();
        let _ = stream.read_to_end(&mut buf);
        let resp_str = String::from_utf8_lossy(&buf).to_string();

        if include_headers {
            Some(commands::CommandOutput::ok(resp_str))
        } else {
            let body_only = if let Some(pos) = resp_str.find("\r\n\r\n") {
                &resp_str[pos + 4..]
            } else if let Some(pos) = resp_str.find("\n\n") {
                &resp_str[pos + 2..]
            } else {
                &resp_str
            };
            Some(commands::CommandOutput::ok(body_only))
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

    #[test]
    fn test_terminal_security_regression_matrix_cp02() {
        let (temp, mut term) = setup_test_env("security_cp02");

        // 1. ps process boundary verification
        let ps_res = term.execute("ps");
        assert_eq!(ps_res.exit_code, 0);
        assert!(ps_res.stdout.contains("init"));
        assert!(ps_res.stdout.contains("lab-daemon"));
        assert!(!ps_res.stdout.contains("explorer.exe"));
        assert!(!ps_res.stdout.contains("System"));

        let ps_ext = term.execute("ps -ef");
        assert_eq!(ps_ext.exit_code, 0);
        assert!(ps_ext.stdout.contains("UID"));
        assert!(ps_ext.stdout.contains("/sbin/init"));

        // 2. help educational system verification
        let help_gen = term.execute("help");
        assert_eq!(help_gen.exit_code, 0);
        assert!(help_gen.stdout.contains("Zitera Educational Terminal"));
        assert!(help_gen.stdout.contains("grep"));

        let help_grep = term.execute("help grep");
        assert_eq!(help_grep.exit_code, 0);
        assert!(help_grep.stdout.contains("COMMAND: grep"));

        let help_unknown = term.execute("help evil_cmd");
        assert_eq!(help_unknown.exit_code, 1);
        assert!(help_unknown.stderr.contains("no educational entry found"));

        // 3. curl security & loopback boundary matrix
        // 3a. Loopback allowed
        assert!(validate_curl_args(&["http://127.0.0.1:8080/api".to_string()]).is_ok());
        assert!(validate_curl_args(&["http://localhost:3000/".to_string()]).is_ok());
        assert!(validate_curl_args(&["http://[::1]:8011/session/".to_string()]).is_ok());

        // 3b. External targets strictly denied
        let ext_err = validate_curl_args(&["http://evil.com/leak".to_string()]);
        assert!(ext_err.is_err());
        assert!(ext_err.unwrap_err().contains("denied by sandbox policy"));

        let lan_err = validate_curl_args(&["http://192.168.1.1/".to_string()]);
        assert!(lan_err.is_err());

        let metadata_err = validate_curl_args(&["http://169.254.169.254/latest/meta-data".to_string()]);
        assert!(metadata_err.is_err());

        // 3c. Userinfo credentials prohibited
        let userinfo_err = validate_curl_args(&["http://admin:secret@127.0.0.1:8080".to_string()]);
        assert!(userinfo_err.is_err());
        assert!(userinfo_err.unwrap_err().contains("userinfo credentials"));

        // 3d. File output redirection restricted
        let file_out_err = validate_curl_args(&["-o".to_string(), "malware.exe".to_string()]);
        assert!(file_out_err.is_err());
        assert!(file_out_err.unwrap_err().contains("file output redirection is restricted"));

        let file_proto_err = validate_curl_args(&["file:///C:/Windows/win.ini".to_string()]);
        assert!(file_proto_err.is_err());

        // 3e. HTTPS protocol restricted for local lab endpoints
        let https_err = validate_curl_args(&["https://127.0.0.1:8080".to_string()]);
        assert!(https_err.is_err());
        assert!(https_err.unwrap_err().contains("https protocol is not supported"));

        // 3f. Malformed URL & Invalid port
        let port_err = validate_curl_args(&["http://127.0.0.1:99999".to_string()]);
        assert!(port_err.is_err());
        assert!(port_err.unwrap_err().contains("invalid port specification"));

        // 3g. Execution through TerminalSession
        let curl_exec_denied = term.execute("curl http://evil.com");
        assert_eq!(curl_exec_denied.exit_code, 1);
        assert!(curl_exec_denied.stderr.contains("denied by sandbox policy"));

        // 4. Shell injection rejection
        let inj1 = term.execute("ls; whoami");
        assert_eq!(inj1.exit_code, 2);
        assert!(inj1.stderr.contains("syntax error: shell operators"));

        let inj2 = term.execute("cat file.txt && rm -rf /");
        assert_eq!(inj2.exit_code, 2);

        let inj3 = term.execute("echo $(id)");
        assert_eq!(inj3.exit_code, 2);

        // 5. Path traversal blocked
        let trav1 = term.execute(r"cat C:\Windows\win.ini");
        assert!(trav1.stderr.contains("escapes terminal sandbox boundary"));

        let trav2 = term.execute(r"cat ..\..\secret");
        assert!(trav2.stderr.contains("escapes terminal sandbox boundary"));

        let trav3 = term.execute("cat ../../etc/passwd");
        assert!(trav3.stderr.contains("No such file or directory"));

        let _ = fs::remove_dir_all(&temp);
    }
}
