use crate::catalog;
use crate::labs;
use crate::models::ApiResponse;
use crate::system;
use crate::tools;
use std::env;
use std::path::{Path, PathBuf};

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let json_mode = args.iter().any(|a| a == "--json");
    let filtered_args: Vec<String> = args.into_iter().skip(1).filter(|a| a != "--json").collect();

    let workspace_root = find_workspace_root();

    if filtered_args.is_empty() {
        print_help(json_mode);
        return;
    }

    match filtered_args[0].as_str() {
        "doctor" => handle_doctor(json_mode),
        "tool" => handle_tool(&filtered_args[1..], json_mode),
        "lab" => handle_lab(&filtered_args[1..], &workspace_root, json_mode),
        "catalog" => handle_catalog(&workspace_root, json_mode),
        "help" | "--help" | "-h" => print_help(json_mode),
        other => {
            if json_mode {
                let resp: ApiResponse<()> = ApiResponse::err(
                    other,
                    "UNKNOWN_COMMAND",
                    format!("Unrecognized command: {}", other),
                    false,
                );
                println!("{}", serde_json::to_string_pretty(&resp).unwrap());
            } else {
                eprintln!("Unknown command: '{}'. Run 'zitera help' for usage.", other);
            }
        }
    }
}

fn handle_doctor(json: bool) {
    let diag = system::diagnose_system();
    if json {
        let resp = ApiResponse::ok("doctor", diag);
        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
    } else {
        println!("==================================================");
        println!("  ZITERA_LAB — System Readiness Diagnostics       ");
        println!("==================================================");
        println!("OS            : [{}] {}", diag.os.status, diag.os.name);
        println!("Git           : [{}] {}", diag.git.status, diag.git.message);
        println!("WSL2          : [{}] {}", diag.wsl.status, diag.wsl.message);
        println!(
            "Docker CLI    : [{}] {}",
            diag.docker.status, diag.docker.message
        );
        println!(
            "Docker Daemon : [{}] {}",
            diag.docker_daemon.status, diag.docker_daemon.message
        );
        println!(
            "PowerShell    : [{}] {}",
            diag.powershell.status, diag.powershell.message
        );
        println!("--------------------------------------------------");
        println!(
            "Overall Readiness: {}",
            if diag.all_ready {
                "READY"
            } else {
                "ATTENTION REQUIRED"
            }
        );
        println!("==================================================");
    }
}

fn handle_tool(args: &[String], json: bool) {
    if args.is_empty() || args[0] == "list" {
        let tool_list = tools::list_tools();
        if json {
            let resp = ApiResponse::ok("tool.list", tool_list);
            println!("{}", serde_json::to_string_pretty(&resp).unwrap());
        } else {
            println!("==================================================");
            println!("  ZITERA_LAB — Security Tools Status              ");
            println!("==================================================");
            for t in tool_list {
                println!("{:<12} [{:<7}] Category: {}", t.name, t.status, t.category);
                if let Some(v) = t.version {
                    println!("  Version: {}", v);
                } else {
                    println!("  Guide: {}", t.install_guide);
                }
            }
            println!("==================================================");
        }
        return;
    }

    match args[0].as_str() {
        "status" => {
            let id = args.get(1).map(|s| s.as_str()).unwrap_or("");
            match tools::get_tool_status(id) {
                Ok(t) => {
                    if json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&ApiResponse::ok("tool.status", t))
                                .unwrap()
                        );
                    } else {
                        println!("Tool: {} [{}] Version: {:?}", t.name, t.status, t.version);
                    }
                }
                Err(e) => {
                    if json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&ApiResponse::<()>::err(
                                "tool.status",
                                "TOOL_NOT_FOUND",
                                e,
                                false
                            ))
                            .unwrap()
                        );
                    } else {
                        eprintln!("[ERROR] {}", e);
                    }
                }
            }
        }
        "install" => {
            let id = args.get(1).map(|s| s.as_str()).unwrap_or("");
            match tools::install_tool(id) {
                Ok(res) => {
                    if json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&ApiResponse::ok("tool.install", res))
                                .unwrap()
                        );
                    } else {
                        println!("[INSTALL] {}: {}", res.id, res.message);
                    }
                }
                Err(e) => {
                    if json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&ApiResponse::<()>::err(
                                "tool.install",
                                "TOOL_INSTALL_FAILED",
                                e,
                                false
                            ))
                            .unwrap()
                        );
                    } else {
                        eprintln!("[ERROR] {}", e);
                    }
                }
            }
        }
        other => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&ApiResponse::<()>::err(
                        "tool",
                        "UNKNOWN_TOOL_ACTION",
                        format!("Unknown tool action '{}'", other),
                        false
                    ))
                    .unwrap()
                );
            } else {
                eprintln!(
                    "Unknown tool command '{}'. Use: list, status, install",
                    other
                );
            }
        }
    }
}

fn handle_lab(args: &[String], workspace_root: &Path, json: bool) {
    if args.is_empty() {
        handle_lab_list(workspace_root, json);
        return;
    }

    match args[0].as_str() {
        "list" => handle_lab_list(workspace_root, json),
        "status" => {
            let id = args.get(1).map(|s| s.as_str()).unwrap_or("A01");
            let st = labs::get_lab_status(workspace_root, id);
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&ApiResponse::ok("lab.status", st)).unwrap()
                );
            } else {
                println!(
                    "Lab: {} | Status: {} | Port: {}",
                    st.title, st.status, st.port
                );
                if let Some(url) = st.url {
                    println!("Access URL: {}", url);
                }
            }
        }
        "start" => {
            let id = args.get(1).map(|s| s.as_str()).unwrap_or("A01");
            match labs::start_lab(workspace_root, id) {
                Ok(msg) => {
                    if json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&ApiResponse::ok("lab.start", msg))
                                .unwrap()
                        );
                    } else {
                        println!("[SUCCESS] {}", msg);
                    }
                }
                Err(e) => {
                    if json {
                        let resp: ApiResponse<()> =
                            ApiResponse::err("lab.start", "LAB_START_FAILED", e, true);
                        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                    } else {
                        eprintln!("[ERROR] {}", e);
                    }
                }
            }
        }
        "stop" => {
            let id = args.get(1).map(|s| s.as_str()).unwrap_or("A01");
            match labs::stop_lab(workspace_root, id) {
                Ok(msg) => {
                    if json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&ApiResponse::ok("lab.stop", msg))
                                .unwrap()
                        );
                    } else {
                        println!("[SUCCESS] {}", msg);
                    }
                }
                Err(e) => {
                    if json {
                        let resp: ApiResponse<()> =
                            ApiResponse::err("lab.stop", "LAB_STOP_FAILED", e, true);
                        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                    } else {
                        eprintln!("[ERROR] {}", e);
                    }
                }
            }
        }
        "reset" => {
            let id = args.get(1).map(|s| s.as_str()).unwrap_or("A01");
            match labs::reset_lab(workspace_root, id) {
                Ok(msg) => {
                    if json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&ApiResponse::ok("lab.reset", msg))
                                .unwrap()
                        );
                    } else {
                        println!("[SUCCESS] {}", msg);
                    }
                }
                Err(e) => {
                    if json {
                        let resp: ApiResponse<()> =
                            ApiResponse::err("lab.reset", "LAB_RESET_FAILED", e, true);
                        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                    } else {
                        eprintln!("[ERROR] {}", e);
                    }
                }
            }
        }
        "install" => {
            let id = args.get(1).map(|s| s.as_str()).unwrap_or("A01");
            match labs::install_lab(workspace_root, id) {
                Ok(msg) => {
                    if json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&ApiResponse::ok("lab.install", msg))
                                .unwrap()
                        );
                    } else {
                        println!("[SUCCESS] {}", msg);
                    }
                }
                Err(e) => {
                    if json {
                        let resp: ApiResponse<()> =
                            ApiResponse::err("lab.install", "LAB_INSTALL_FAILED", e, true);
                        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                    } else {
                        eprintln!("[ERROR] {}", e);
                    }
                }
            }
        }
        "update" => {
            let id = args.get(1).map(|s| s.as_str()).unwrap_or("A01");
            match labs::update_lab(workspace_root, id) {
                Ok(msg) => {
                    if json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&ApiResponse::ok("lab.update", msg))
                                .unwrap()
                        );
                    } else {
                        println!("[SUCCESS] {}", msg);
                    }
                }
                Err(e) => {
                    if json {
                        let resp: ApiResponse<()> =
                            ApiResponse::err("lab.update", "LAB_UPDATE_FAILED", e, true);
                        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                    } else {
                        eprintln!("[ERROR] {}", e);
                    }
                }
            }
        }
        "remove" => {
            let id = args.get(1).map(|s| s.as_str()).unwrap_or("A01");
            match labs::remove_lab(workspace_root, id) {
                Ok(msg) => {
                    if json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&ApiResponse::ok("lab.remove", msg))
                                .unwrap()
                        );
                    } else {
                        println!("[SUCCESS] {}", msg);
                    }
                }
                Err(e) => {
                    if json {
                        let resp: ApiResponse<()> =
                            ApiResponse::err("lab.remove", "LAB_REMOVE_FAILED", e, true);
                        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                    } else {
                        eprintln!("[ERROR] {}", e);
                    }
                }
            }
        }
        "content" => {
            let id = args.get(1).map(|s| s.as_str()).unwrap_or("A01");
            match labs::get_lab_content(workspace_root, id) {
                Ok(content) => {
                    if json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&ApiResponse::ok("lab.content", content))
                                .unwrap()
                        );
                    } else {
                        println!(
                            "Lab Content: {} (v{})",
                            content.manifest.title, content.manifest.version
                        );
                        println!("Objective: {}", content.challenge_objective);
                        println!("Hints available: {}", content.hints.len());
                    }
                }
                Err(e) => {
                    if json {
                        let resp: ApiResponse<()> =
                            ApiResponse::err("lab.content", "LAB_CONTENT_FAILED", e, true);
                        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                    } else {
                        eprintln!("[ERROR] {}", e);
                    }
                }
            }
        }
        "validate-challenge" | "verify" => {
            let id = args.get(1).map(|s| s.as_str()).unwrap_or("A01");
            let submission = args.get(2).map(|s| s.as_str()).unwrap_or("");
            match labs::validate_challenge(workspace_root, id, submission) {
                Ok(verif) => {
                    if json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&ApiResponse::ok(
                                "lab.validate_challenge",
                                verif
                            ))
                            .unwrap()
                        );
                    } else {
                        println!("[{}] {}", verif.status, verif.message);
                    }
                }
                Err(e) => {
                    if json {
                        let resp: ApiResponse<()> = ApiResponse::err(
                            "lab.validate_challenge",
                            "CHALLENGE_VERIFY_FAILED",
                            e,
                            true,
                        );
                        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                    } else {
                        eprintln!("[ERROR] {}", e);
                    }
                }
            }
        }
        "practice-verify" => {
            let id = args.get(1).map(|s| s.as_str()).unwrap_or("A01");
            match labs::verify_practice(workspace_root, id) {
                Ok(res) => {
                    if json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&ApiResponse::ok(
                                "lab.practice_verify",
                                res
                            ))
                            .unwrap()
                        );
                    } else {
                        println!("[{}] {}", res.status, res.message);
                    }
                }
                Err(e) => {
                    if json {
                        let resp: ApiResponse<()> = ApiResponse::err(
                            "lab.practice_verify",
                            "PRACTICE_VERIFY_FAILED",
                            e,
                            true,
                        );
                        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                    } else {
                        eprintln!("[ERROR] {}", e);
                    }
                }
            }
        }
        other => {
            if json {
                let resp: ApiResponse<()> = ApiResponse::err(
                    "lab",
                    "UNKNOWN_SUBCOMMAND",
                    format!("Unknown lab subcommand: {}", other),
                    false,
                );
                println!("{}", serde_json::to_string_pretty(&resp).unwrap());
            } else {
                eprintln!("Unknown lab subcommand: {}", other);
            }
        }
    }
}

fn handle_lab_list(workspace_root: &Path, json: bool) {
    let all = labs::list_all_labs(workspace_root);
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&ApiResponse::ok("lab.list", all)).unwrap()
        );
    } else {
        println!("==================================================");
        println!("  ZITERA_LAB — Laboratory Catalog & Status        ");
        println!("==================================================");
        for l in all {
            println!(
                "{:<5} {:<24} [{:<12}] Port: {}",
                l.id, l.title, l.status, l.port
            );
        }
        println!("==================================================");
    }
}

fn handle_catalog(workspace_root: &Path, json: bool) {
    match catalog::load_catalog(workspace_root) {
        Ok(cat) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&ApiResponse::ok("catalog.list", cat)).unwrap()
                );
            } else {
                println!("ZITERA Lab Catalog (schema v{})", cat.schema_version);
                for l in cat.labs {
                    println!(
                        "- [{}] {} ({}) - {}",
                        l.id, l.title, l.version, l.description
                    );
                }
            }
        }
        Err(e) => {
            if json {
                let resp: ApiResponse<()> =
                    ApiResponse::err("catalog.list", "CATALOG_ERROR", e, true);
                println!("{}", serde_json::to_string_pretty(&resp).unwrap());
            } else {
                eprintln!("Error loading catalog: {}", e);
            }
        }
    }
}

fn find_workspace_root() -> PathBuf {
    // 1. Check relative to current working directory (development & standard execution)
    let mut current = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    loop {
        if current.join("catalog").join("catalog.json").exists() || current.join(".git").exists() {
            return current;
        }
        if let Some(parent) = current.parent() {
            current = parent.to_path_buf();
        } else {
            break;
        }
    }

    // 2. Check relative to current executable location (portable release package)
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let mut check = exe_dir.to_path_buf();
            loop {
                if check.join("catalog").join("catalog.json").exists()
                    || check.join(".git").exists()
                {
                    return check;
                }
                if let Some(parent) = check.parent() {
                    check = parent.to_path_buf();
                } else {
                    break;
                }
            }
        }
    }

    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn print_help(json: bool) {
    if json {
        let resp = ApiResponse::ok("help", "zitera CLI v0.1.0");
        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
    } else {
        println!("ZITERA_LAB Engine & CLI (zitera)");
        println!("Usage: zitera [--json] <command> [subcommand] [arguments]");
        println!();
        println!("Commands:");
        println!("  doctor                 Run system environment diagnostics");
        println!("  tool list              List supported security tools and status");
        println!("  lab list               List installed and available labs");
        println!("  lab status <id>        Show runtime status for a lab");
        println!("  lab content <id>       Get dynamic lesson and challenge content");
        println!("  lab validate-challenge <id> <flag> Validate a challenge submission");
        println!("  lab install <id>       Install lab from catalog");
        println!("  lab start <id>         Start a lab environment");
        println!("  lab stop <id>          Stop a lab environment");
        println!("  lab reset <id>         Deterministically reset a lab environment");
        println!("  lab update <id>        Update a lab environment to latest version");
        println!("  lab remove <id>        Remove an installed lab");
        println!("  catalog                Show central catalog");
    }
}
