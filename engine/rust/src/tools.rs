use crate::models::{ToolInstallResult, ToolStatus};
use crate::process::run_cmd;
use std::env;
use std::path::{Path, PathBuf};

struct ToolSpec<'a> {
    id: &'a str,
    name: &'a str,
    category: &'a str,
    min_version: &'a str,
    version_args: &'a [&'a str],
    capabilities: &'a [&'a str],
    install_method: &'a str,
    install_guide: &'a str,
}

pub fn list_tools() -> Vec<ToolStatus> {
    vec![
        check_tool(&ToolSpec {
            id: "nmap",
            name: "Nmap",
            category: "Network Scanning",
            min_version: "7.80",
            version_args: &["--version"],
            capabilities: &["Port Scanning", "Host Discovery", "Service Detection"],
            install_method: "winget",
            install_guide: "Run 'winget install Insecure.Nmap' or download from https://nmap.org/download.html",
        }),
        check_tool(&ToolSpec {
            id: "sqlmap",
            name: "sqlmap",
            category: "Vulnerability Assessment",
            min_version: "1.7",
            version_args: &["--version"],
            capabilities: &["SQL Injection", "Database Fingerprinting", "Data Extraction"],
            install_method: "pip",
            install_guide: "Run 'pip install sqlmap' or clone from https://github.com/sqlmapproject/sqlmap",
        }),
        check_tool(&ToolSpec {
            id: "ffuf",
            name: "ffuf",
            category: "Web Fuzzing",
            min_version: "2.0",
            version_args: &["-V"],
            capabilities: &["Directory Fuzzing", "Parameter Discovery", "Header Fuzzing"],
            install_method: "winget",
            install_guide: "Run 'winget install ffuf.ffuf' or download binary from https://github.com/ffuf/ffuf/releases",
        }),
        check_tool(&ToolSpec {
            id: "gobuster",
            name: "Gobuster",
            category: "Content Discovery",
            min_version: "3.5",
            version_args: &["version"],
            capabilities: &["URI Discovery", "DNS Subdomain", "Vhost Enumeration"],
            install_method: "winget",
            install_guide: "Run 'winget install OJ.Gobuster' or 'go install github.com/OJ/gobuster/v3@latest'",
        }),
        check_tool(&ToolSpec {
            id: "linpeas",
            name: "LinPEAS",
            category: "Privilege Escalation",
            min_version: "1.0",
            version_args: &["-h"],
            capabilities: &["Local Privilege Escalation", "Misconfiguration Discovery"],
            install_method: "manual",
            install_guide: "Download script from https://github.com/carlospolop/PEASS-ng/releases",
        }),
        check_tool(&ToolSpec {
            id: "burpsuite",
            name: "Burp Suite Community",
            category: "Web Interception Proxy",
            min_version: "2023.1",
            version_args: &["--version"],
            capabilities: &["HTTP Proxy", "Repeater", "Intruder (Throttled)"],
            install_method: "guide",
            install_guide: "Download official installer from https://portswigger.net/burp/communitydownload (GUI tool: detection & guided launch only)",
        }),
        check_tool(&ToolSpec {
            id: "owasp-zap",
            name: "OWASP ZAP",
            category: "Application Security Scanner",
            min_version: "2.14",
            version_args: &["-version"],
            capabilities: &["Automated Scanner", "Proxy", "Fuzzing"],
            install_method: "guide",
            install_guide: "Download official installer from https://www.zaproxy.org/download/ (GUI tool: detection & guided launch only)",
        }),
    ]
}

pub fn get_tool_status(id: &str) -> Result<ToolStatus, String> {
    list_tools()
        .into_iter()
        .find(|t| t.id.eq_ignore_ascii_case(id))
        .ok_or_else(|| format!("Unknown tool ID: '{}'", id))
}

pub fn install_tool(id: &str) -> Result<ToolInstallResult, String> {
    // Audit & enforce strict allowlist per Phase 6 Security directives
    let tool = get_tool_status(id)?;
    if tool.installed {
        return Ok(ToolInstallResult {
            id: tool.id,
            success: true,
            message: format!(
                "Tool '{}' is already installed and verified ready.",
                tool.name
            ),
            method: tool.install_method,
        });
    }

    // GUI tool policy: NEVER silent install or bundle heavy GUI tools
    if tool.install_method == "guide" {
        return Ok(ToolInstallResult {
            id: tool.id,
            success: false,
            message: format!(
                "GUI tool '{}' cannot be silently installed per Zitera GUI Tool Policy. Please follow official installation guide: {}",
                tool.name, tool.install_guide
            ),
            method: "guide".to_string(),
        });
    }

    // CLI tools: Allowlisted, auditable package installation via Windows winget or pip
    match tool.id.as_str() {
        "nmap" => {
            let out = run_cmd(
                "winget",
                &[
                    "install",
                    "-e",
                    "--id",
                    "Insecure.Nmap",
                    "--accept-source-agreements",
                    "--accept-package-agreements",
                ],
                None,
            )?;
            Ok(ToolInstallResult {
                id: tool.id,
                success: out.success,
                message: if out.success {
                    "Nmap installation command executed successfully via winget.".to_string()
                } else {
                    format!(
                        "Winget exited with error: {}. Suggest manual setup: {}",
                        out.stderr, tool.install_guide
                    )
                },
                method: "winget".to_string(),
            })
        }
        "ffuf" => {
            let out = run_cmd(
                "winget",
                &[
                    "install",
                    "-e",
                    "--id",
                    "ffuf.ffuf",
                    "--accept-source-agreements",
                    "--accept-package-agreements",
                ],
                None,
            )?;
            Ok(ToolInstallResult {
                id: tool.id,
                success: out.success,
                message: if out.success {
                    "ffuf installation command executed successfully via winget.".to_string()
                } else {
                    format!(
                        "Winget exited with error: {}. Suggest manual setup: {}",
                        out.stderr, tool.install_guide
                    )
                },
                method: "winget".to_string(),
            })
        }
        "gobuster" => {
            let out = run_cmd(
                "winget",
                &[
                    "install",
                    "-e",
                    "--id",
                    "OJ.Gobuster",
                    "--accept-source-agreements",
                    "--accept-package-agreements",
                ],
                None,
            )?;
            Ok(ToolInstallResult {
                id: tool.id,
                success: out.success,
                message: if out.success {
                    "Gobuster installation command executed successfully via winget.".to_string()
                } else {
                    format!(
                        "Winget exited with error: {}. Suggest manual setup: {}",
                        out.stderr, tool.install_guide
                    )
                },
                method: "winget".to_string(),
            })
        }
        "sqlmap" => {
            let out = run_cmd("pip", &["install", "sqlmap"], None)?;
            Ok(ToolInstallResult {
                id: tool.id,
                success: out.success,
                message: if out.success {
                    "sqlmap installed successfully via pip.".to_string()
                } else {
                    format!(
                        "Pip installation failed: {}. Suggest manual setup: {}",
                        out.stderr, tool.install_guide
                    )
                },
                method: "pip".to_string(),
            })
        }
        other => Err(format!(
            "Tool '{}' does not support automated installation. Please follow: {}",
            other, tool.install_guide
        )),
    }
}

fn check_tool(spec: &ToolSpec) -> ToolStatus {
    let resolved_path = find_tool_executable(spec.id);

    let exec_cmd = match &resolved_path {
        Some(p) => p.as_str(),
        None => spec.id,
    };

    match run_cmd(exec_cmd, spec.version_args, None) {
        Ok(out) if out.success => {
            let first_line = out
                .stdout
                .lines()
                .next()
                .unwrap_or("Installed")
                .trim()
                .chars()
                .take(120)
                .collect::<String>();

            ToolStatus {
                id: spec.id.to_string(),
                name: spec.name.to_string(),
                category: spec.category.to_string(),
                installed: true,
                version: Some(first_line),
                min_version: spec.min_version.to_string(),
                status: "READY".to_string(),
                path: resolved_path.or_else(|| Some(spec.id.to_string())),
                capabilities: spec.capabilities.iter().map(|s| s.to_string()).collect(),
                install_method: spec.install_method.to_string(),
                install_guide: spec.install_guide.to_string(),
            }
        }
        _ => {
            // If direct exec failed, check if we found path on disk
            let status = if resolved_path.is_some() {
                "BLOCKED".to_string()
            } else {
                "MISSING".to_string()
            };

            ToolStatus {
                id: spec.id.to_string(),
                name: spec.name.to_string(),
                category: spec.category.to_string(),
                installed: false,
                version: None,
                min_version: spec.min_version.to_string(),
                status,
                path: resolved_path,
                capabilities: spec.capabilities.iter().map(|s| s.to_string()).collect(),
                install_method: spec.install_method.to_string(),
                install_guide: spec.install_guide.to_string(),
            }
        }
    }
}

/// Bounded, fast PATH and known standard directory check on Windows
fn find_tool_executable(id: &str) -> Option<String> {
    // 1. Check `where.exe <id>` (fast PATH lookup on Windows)
    if let Ok(out) = run_cmd("where.exe", &[id], None) {
        if out.success {
            if let Some(first_line) = out.stdout.lines().next() {
                let trimmed = first_line.trim();
                if Path::new(trimmed).is_file() {
                    return Some(trimmed.to_string());
                }
            }
        }
    }

    // 2. Check standard Windows installation directories
    let program_files =
        env::var("ProgramFiles").unwrap_or_else(|_| r"C:\Program Files".to_string());
    let program_files_x86 =
        env::var("ProgramFiles(x86)").unwrap_or_else(|_| r"C:\Program Files (x86)".to_string());
    let local_app_data = env::var("LOCALAPPDATA").unwrap_or_default();
    let user_profile = env::var("USERPROFILE").unwrap_or_default();

    let candidate_paths: Vec<PathBuf> = match id {
        "nmap" => vec![
            PathBuf::from(&program_files).join("Nmap").join("nmap.exe"),
            PathBuf::from(&program_files_x86)
                .join("Nmap")
                .join("nmap.exe"),
        ],
        "ffuf" => vec![
            PathBuf::from(r"C:\tools\ffuf.exe"),
            PathBuf::from(&user_profile)
                .join("go")
                .join("bin")
                .join("ffuf.exe"),
            PathBuf::from(&local_app_data).join("bin").join("ffuf.exe"),
        ],
        "gobuster" => vec![PathBuf::from(&user_profile)
            .join("go")
            .join("bin")
            .join("gobuster.exe")],
        "burpsuite" => vec![PathBuf::from(&program_files)
            .join("BurpSuiteCommunity")
            .join("BurpSuiteCommunity.exe")],
        "owasp-zap" => vec![
            PathBuf::from(&program_files)
                .join("OWASP")
                .join("Zed Attack Proxy")
                .join("zap.bat"),
            PathBuf::from(&program_files)
                .join("ZAP")
                .join("Zed Attack Proxy")
                .join("zap.bat"),
        ],
        _ => Vec::new(),
    };

    for candidate in candidate_paths {
        if candidate.is_file() {
            return Some(candidate.to_string_lossy().to_string());
        }
    }

    None
}
