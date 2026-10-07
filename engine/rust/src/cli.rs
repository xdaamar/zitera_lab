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
        let exe_name = env::current_exe()
            .ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .unwrap_or_default();
        if exe_name.eq_ignore_ascii_case("lab.exe")
            || exe_name.to_ascii_lowercase().ends_with("-lab.exe")
        {
            crate::native_runtime::probe::handle_probe_cli(&["--mock-lab".to_string()]);
            return;
        }
        print_help(json_mode);
        return;
    }

    match filtered_args[0].as_str() {
        "doctor" => handle_doctor(&filtered_args[1..], &workspace_root, json_mode),
        "diagnostics" => handle_diagnostics(&filtered_args[1..], &workspace_root, json_mode),
        "tool" => handle_tool(&filtered_args[1..], json_mode),
        "lab" => handle_lab(&filtered_args[1..], &workspace_root, json_mode),
        "package" => handle_package(&filtered_args[1..], &workspace_root, json_mode),
        "catalog" => handle_catalog(&workspace_root, json_mode),
        "terminal" => handle_terminal(&filtered_args[1..], &workspace_root, json_mode),
        "storage" => handle_storage(&filtered_args[1..], json_mode),
        "sandbox-probe" => {
            crate::native_runtime::probe::handle_probe_cli(&filtered_args[1..]);
        }
        "help" | "--help" | "-h" => print_help(json_mode),
        "version" | "--version" | "-v" => {
            if json_mode {
                let resp: ApiResponse<serde_json::Value> = ApiResponse::ok(
                    "version",
                    serde_json::json!({
                        "version": "2.0.0",
                        "product": "Zitera Lab",
                        "target": "x86_64-pc-windows-msvc"
                    }),
                );
                println!("{}", serde_json::to_string_pretty(&resp).unwrap());
            } else {
                println!("Zitera Lab 2.0.0 (Windows Native Sandboxed Core)");
            }
        }
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

fn handle_doctor(args: &[String], workspace_root: &Path, json: bool) {
    if let Some(pos) = args.iter().position(|a| a == "--export-diagnostics" || a == "export") {
        let custom_path = args.get(pos + 1).map(|s| PathBuf::from(s));
        return handle_diagnostics_export(custom_path.as_deref(), workspace_root, json);
    }

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

fn handle_diagnostics(args: &[String], workspace_root: &Path, json: bool) {
    if args.is_empty() || args[0] == "export" {
        let custom_path = args.get(1).map(|s| PathBuf::from(s));
        handle_diagnostics_export(custom_path.as_deref(), workspace_root, json);
    } else {
        if json {
            let resp: ApiResponse<()> = ApiResponse::err(
                "diagnostics",
                "UNKNOWN_SUBCOMMAND",
                format!("Unrecognized diagnostics subcommand: {}", args[0]),
                false,
            );
            println!("{}", serde_json::to_string_pretty(&resp).unwrap());
        } else {
            eprintln!("Unknown subcommand: '{}'. Supported: export", args[0]);
        }
    }
}

fn handle_diagnostics_export(custom_out: Option<&Path>, workspace_root: &Path, json: bool) {
    match crate::diagnostics::export_diagnostics_archive(workspace_root, custom_out) {
        Ok(report) => {
            if json {
                let resp = ApiResponse::ok("diagnostics.export", report);
                println!("{}", serde_json::to_string_pretty(&resp).unwrap());
            } else {
                println!("==================================================");
                println!("  ZITERA_LAB — Privacy-Safe Diagnostics Export    ");
                println!("==================================================");
                println!("Archive Path  : {}", report.archive_path);
                println!("Archive Size  : {} bytes", report.archive_size_bytes);
                println!("Files Bundled : {}", report.file_count);
                println!("SHA256 Digest : {}", report.sha256_checksum);
                println!("Exported At   : {}", report.export_timestamp);
                println!("Privacy State : 100% SANITIZED (Flags & Secrets Redacted)");
                println!("==================================================");
            }
        }
        Err(e) => {
            if json {
                let resp: ApiResponse<()> = ApiResponse::err(
                    "diagnostics.export",
                    "EXPORT_ERROR",
                    e,
                    false,
                );
                println!("{}", serde_json::to_string_pretty(&resp).unwrap());
            } else {
                eprintln!("Failed to export diagnostics archive: {}", e);
            }
        }
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
                        let (code, rec) = crate::models::classify_recovery_error("lab.start", &e);
                        let resp: ApiResponse<()> =
                            ApiResponse::err_with_recovery("lab.start", code, e, rec, true);
                        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                    } else {
                        eprintln!("[ERROR] {}", e);
                    }
                }
            }
        }
        "serve" => {
            let id = args.get(1).map(|s| s.as_str()).unwrap_or("A01");
            if let Err(e) = labs::serve_lab(workspace_root, id) {
                eprintln!("[ERROR] Failed to serve lab {}: {}", id, e);
                std::process::exit(1);
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
                        let (code, rec) = crate::models::classify_recovery_error("lab.stop", &e);
                        let resp: ApiResponse<()> =
                            ApiResponse::err_with_recovery("lab.stop", code, e, rec, true);
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
                        let (code, rec) = crate::models::classify_recovery_error("lab.reset", &e);
                        let resp: ApiResponse<()> =
                            ApiResponse::err_with_recovery("lab.reset", code, e, rec, true);
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
                        let (code, rec) = crate::models::classify_recovery_error("lab.install", &e);
                        let resp: ApiResponse<()> =
                            ApiResponse::err_with_recovery("lab.install", code, e, rec, true);
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
                        let (code, rec) = crate::models::classify_recovery_error("lab.update", &e);
                        let resp: ApiResponse<()> =
                            ApiResponse::err_with_recovery("lab.update", code, e, rec, true);
                        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                    } else {
                        eprintln!("[ERROR] {}", e);
                    }
                }
            }
        }
        "install-package" | "update-package" => {
            let pkg_path_str = args.get(1).map(|s| s.as_str()).unwrap_or("");
            if pkg_path_str.is_empty() {
                if json {
                    let resp: ApiResponse<()> = ApiResponse::err_with_recovery(
                        "lab.install_package",
                        "MISSING_PACKAGE_PATH",
                        "Package file path required".to_string(),
                        "Provide a valid path to a .zlab package bundle.",
                        false,
                    );
                    println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                } else {
                    eprintln!("Error: package file path required. Usage: zitera lab install-package <path.zlab>");
                }
                return;
            }
            let pkg_path = PathBuf::from(pkg_path_str);
            match crate::package::install_or_update_package(&pkg_path, workspace_root) {
                Ok(report) => {
                    if json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&ApiResponse::ok(
                                "lab.install_package",
                                report
                            ))
                            .unwrap()
                        );
                    } else {
                        println!(
                            "[SUCCESS] Installed lab {} version {} (active: {})",
                            report.lab_id, report.new_version, report.active_path
                        );
                    }
                }
                Err(e) => {
                    if json {
                        let (code, rec) =
                            crate::models::classify_recovery_error("lab.install_package", &e);
                        let resp: ApiResponse<()> = ApiResponse::err_with_recovery(
                            "lab.install_package",
                            code,
                            e,
                            rec,
                            true,
                        );
                        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                    } else {
                        eprintln!("[ERROR] Package installation failed: {}", e);
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
                        let (code, rec) = crate::models::classify_recovery_error("lab.remove", &e);
                        let resp: ApiResponse<()> =
                            ApiResponse::err_with_recovery("lab.remove", code, e, rec, true);
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
        "validate" => {
            let id = args.get(1).map(|s| s.as_str()).unwrap_or("A01");
            let res = labs::validate_lab(workspace_root, id);
            if json {
                let resp = if res.valid {
                    ApiResponse::ok("lab.validate", res)
                } else {
                    ApiResponse::err_with_recovery(
                        "lab.validate",
                        "LAB_VALIDATION_FAILED",
                        res.summary.clone(),
                        "Check the failed verification points and update manifest/content accordingly.",
                        true,
                    )
                };
                println!("{}", serde_json::to_string_pretty(&resp).unwrap());
            } else {
                println!("==================================================");
                println!("  ZITERA_LAB — Lab Verification & Contract Check  ");
                println!("==================================================");
                println!("Lab ID  : {}", res.lab_id);
                println!("Result  : {}", if res.valid { "PASSED" } else { "FAILED" });
                println!("Summary : {}", res.summary);
                println!("--------------------------------------------------");
                for check in &res.checks {
                    let mark = if check.passed { "[PASS]" } else { "[FAIL]" };
                    println!("{} {}: {}", mark, check.name, check.message);
                    if let Some(rem) = &check.remediation {
                        println!("       -> Remediation: {}", rem);
                    }
                }
                println!("==================================================");
            }
        }
        "create" | "new" => {
            let id = args.get(1).map(|s| s.as_str()).unwrap_or("");
            if id.is_empty() {
                if json {
                    let resp: ApiResponse<()> = ApiResponse::err_with_recovery(
                        "lab.create",
                        "MISSING_LAB_ID",
                        "Lab ID required for scaffolding",
                        "Specify an alphanumeric lab ID (e.g., A11)",
                        false,
                    );
                    println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                } else {
                    eprintln!("Error: Lab ID required. Usage: zitera lab create <ID> [title] [category_id]");
                }
                return;
            }
            let title = args.get(2).map(|s| s.as_str()).unwrap_or("");
            let cat = args.get(3).map(|s| s.as_str()).unwrap_or("");
            match labs::create_lab_template(workspace_root, id, title, cat) {
                Ok(path) => {
                    if json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&ApiResponse::ok(
                                "lab.create",
                                format!("Lab {} scaffolded at {:?}", id, path)
                            ))
                            .unwrap()
                        );
                    } else {
                        println!("[SUCCESS] Lab {} created at {:?}", id, path);
                        println!(
                            "Run 'zitera lab validate {}' to verify curriculum compliance.",
                            id
                        );
                    }
                }
                Err(e) => {
                    if json {
                        let resp: ApiResponse<()> = ApiResponse::err_with_recovery(
                            "lab.create",
                            "LAB_CREATE_FAILED",
                            e,
                            "Ensure lab ID does not already exist and is valid alphanumeric format.",
                            true,
                        );
                        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                    } else {
                        eprintln!("[ERROR] Failed to create lab: {}", e);
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

fn handle_package(args: &[String], workspace_root: &Path, json: bool) {
    if args.is_empty() {
        if json {
            let resp: ApiResponse<()> = ApiResponse::err(
                "package",
                "MISSING_SUBCOMMAND",
                "Subcommand required: build or verify".to_string(),
                false,
            );
            println!("{}", serde_json::to_string_pretty(&resp).unwrap());
        } else {
            eprintln!("Usage: zitera package <build|verify> [args...]");
        }
        return;
    }

    match args[0].as_str() {
        "build" => {
            let src_str = args.get(1).map(|s| s.as_str()).unwrap_or("");
            let out_str = args.get(2).map(|s| s.as_str()).unwrap_or("");
            if src_str.is_empty() || out_str.is_empty() {
                if json {
                    let resp: ApiResponse<()> = ApiResponse::err(
                        "package.build",
                        "INVALID_ARGUMENTS",
                        "Usage: zitera package build <source_dir> <output.zlab>".to_string(),
                        false,
                    );
                    println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                } else {
                    eprintln!("Usage: zitera package build <source_dir> <output.zlab>");
                }
                return;
            }

            let src_path = workspace_root.join(src_str);
            let out_path = workspace_root.join(out_str);

            match crate::package::build_signed_package_from_dir(
                &src_path,
                &out_path,
                &crate::package::trust::DEV_PRIVATE_KEY_SEED,
            ) {
                Ok(verified) => {
                    if json {
                        let resp = ApiResponse::ok("package.build", serde_json::json!({
                            "lab_id": verified.manifest.id,
                            "version": verified.manifest.version,
                            "package_sha256": verified.package_sha256,
                            "content_digest": verified.content_digest,
                            "output_path": out_path.display().to_string(),
                        }));
                        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                    } else {
                        println!("[SUCCESS] Built and signed package {}", out_path.display());
                        println!("  Lab ID        : {}", verified.manifest.id);
                        println!("  Version       : {}", verified.manifest.version);
                        println!("  Package SHA256: {}", verified.package_sha256);
                        println!("  Content Digest: {}", verified.content_digest);
                    }
                }
                Err(e) => {
                    if json {
                        let resp: ApiResponse<()> = ApiResponse::err(
                            "package.build",
                            "PACKAGE_BUILD_FAILED",
                            e,
                            true,
                        );
                        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                    } else {
                        eprintln!("[ERROR] Package build failed: {}", e);
                    }
                }
            }
        }
        "verify" => {
            let pkg_str = args.get(1).map(|s| s.as_str()).unwrap_or("");
            if pkg_str.is_empty() {
                if json {
                    let resp: ApiResponse<()> = ApiResponse::err(
                        "package.verify",
                        "INVALID_ARGUMENTS",
                        "Usage: zitera package verify <package.zlab>".to_string(),
                        false,
                    );
                    println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                } else {
                    eprintln!("Usage: zitera package verify <package.zlab>");
                }
                return;
            }

            let pkg_path = workspace_root.join(pkg_str);
            match crate::package::verifier::verify_package(&pkg_path, None) {
                Ok(verified) => {
                    if json {
                        let resp = ApiResponse::ok("package.verify", serde_json::json!({
                            "lab_id": verified.manifest.id,
                            "version": verified.manifest.version,
                            "security_version": verified.manifest.security_version,
                            "minimum_core_version": verified.manifest.minimum_core_version,
                            "package_sha256": verified.package_sha256,
                            "content_digest": verified.content_digest,
                        }));
                        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                    } else {
                        println!("[VALID] Package verification PASSED for {}", pkg_path.display());
                        println!("  Lab ID              : {}", verified.manifest.id);
                        println!("  Version             : {}", verified.manifest.version);
                        println!("  Security Version    : {}", verified.manifest.security_version.unwrap_or(1));
                        println!("  Minimum Core Version: {}", verified.manifest.minimum_core_version.as_deref().unwrap_or("2.0.0"));
                        println!("  Package SHA-256     : {}", verified.package_sha256);
                        println!("  Content Digest      : {}", verified.content_digest);
                    }
                }
                Err(e) => {
                    if json {
                        let resp: ApiResponse<()> = ApiResponse::err(
                            "package.verify",
                            "VERIFICATION_FAILED",
                            e,
                            true,
                        );
                        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                    } else {
                        eprintln!("[REJECTED] Package verification failed: {}", e);
                    }
                }
            }
        }
        "install" => {
            let pkg_str = args.get(1).map(|s| s.as_str()).unwrap_or("");
            if pkg_str.is_empty() {
                if json {
                    let resp: ApiResponse<()> = ApiResponse::err(
                        "package.install",
                        "INVALID_ARGUMENTS",
                        "Usage: zitera package install <package.zlab>".to_string(),
                        false,
                    );
                    println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                } else {
                    eprintln!("Usage: zitera package install <package.zlab>");
                }
                return;
            }

            let candidate_path = PathBuf::from(pkg_str);
            let pkg_path = if candidate_path.exists() {
                candidate_path
            } else {
                workspace_root.join(pkg_str)
            };

            match crate::package::installer::install_or_update_package(&pkg_path, workspace_root) {
                Ok(rep) => {
                    if json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&ApiResponse::ok("package.install", &rep))
                                .unwrap()
                        );
                    } else {
                        println!("[SUCCESS] Package installed successfully:");
                        println!("  Lab ID       : {}", rep.lab_id);
                        println!("  Version      : {}", rep.new_version);
                        println!("  Status       : {}", rep.status);
                        println!("  Active Path  : {}", rep.active_path);
                    }
                }
                Err(e) => {
                    if json {
                        let resp: ApiResponse<()> = ApiResponse::err(
                            "package.install",
                            "INSTALL_FAILED",
                            e,
                            true,
                        );
                        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                    } else {
                        eprintln!("[ERROR] Package installation failed: {}", e);
                    }
                }
            }
        }
        other => {
            if json {
                let resp: ApiResponse<()> = ApiResponse::err(
                    "package",
                    "UNKNOWN_SUBCOMMAND",
                    format!("Unknown package subcommand '{}'", other),
                    false,
                );
                println!("{}", serde_json::to_string_pretty(&resp).unwrap());
            } else {
                eprintln!("Unknown package subcommand '{}'. Use: build, verify, install", other);
            }
        }
    }
}

fn handle_terminal(args: &[String], workspace_root: &Path, json: bool) {
    if args.is_empty() {
        if json {
            let resp: ApiResponse<()> = ApiResponse::err(
                "terminal",
                "MISSING_COMMAND",
                "No terminal command provided".to_string(),
                false,
            );
            println!("{}", serde_json::to_string_pretty(&resp).unwrap());
        } else {
            eprintln!("Usage: zitera terminal [--lab <id>] <command...>");
        }
        return;
    }

    let mut lab_id: Option<String> = None;
    let mut cmd_args: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--lab" && i + 1 < args.len() {
            lab_id = Some(args[i + 1].clone());
            i += 2;
        } else {
            cmd_args.push(args[i].clone());
            i += 1;
        }
    }

    if cmd_args.is_empty() {
        if json {
            let resp: ApiResponse<()> = ApiResponse::err(
                "terminal",
                "MISSING_COMMAND",
                "No terminal command specified after flags".to_string(),
                false,
            );
            println!("{}", serde_json::to_string_pretty(&resp).unwrap());
        } else {
            eprintln!("Error: no command provided to execute.");
        }
        return;
    }

    let sandbox_root = if let Some(ref id) = lab_id {
        let lab_dir = workspace_root.join("labs").join(id);
        if lab_dir.exists() {
            lab_dir
        } else {
            workspace_root.to_path_buf()
        }
    } else {
        let default_sandbox = workspace_root.join("sandbox");
        let _ = std::fs::create_dir_all(&default_sandbox);
        default_sandbox
    };

    let mut session = crate::terminal::TerminalSession::new(sandbox_root);
    let command_line = cmd_args.join(" ");
    let result = session.execute(&command_line);

    if json {
        let resp = ApiResponse::ok("terminal.execute", result);
        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
    } else {
        if !result.stdout.is_empty() {
            print!("{}", result.stdout);
        }
        if !result.stderr.is_empty() {
            eprint!("{}", result.stderr);
        }
        if result.exit_code != 0 {
            std::process::exit(result.exit_code);
        }
    }
}

fn handle_storage(args: &[String], json: bool) {
    let paths = crate::storage::StoragePaths::resolve();
    let sub = args.first().map(|s| s.as_str()).unwrap_or("status");

    match sub {
        "status" | "info" => {
            let isolation = paths.verify_boundary_isolation();
            let is_isolated = isolation.is_ok();
            let isolation_msg = isolation.err();

            #[derive(serde::Serialize)]
            struct StorageReport {
                paths: crate::storage::StoragePaths,
                isolation_verified: bool,
                isolation_error: Option<String>,
                progress_file: String,
            }

            let report = StorageReport {
                progress_file: paths.user_progress_file().display().to_string(),
                paths: paths.clone(),
                isolation_verified: is_isolated,
                isolation_error: isolation_msg,
            };

            if json {
                let resp = ApiResponse::ok("storage.status", report);
                println!("{}", serde_json::to_string_pretty(&resp).unwrap());
            } else {
                println!("==================================================");
                println!("  ZITERA_LAB — Storage Boundaries & Isolation     ");
                println!("==================================================");
                println!("App Directory  : {}", paths.app_dir.display());
                println!("User Progress  : {}", paths.user_dir.display());
                println!("Lab Packages   : {}", paths.labs_dir.display());
                println!("Cache          : {}", paths.cache_dir.display());
                println!("Logs           : {}", paths.logs_dir.display());
                println!("Diagnostics    : {}", paths.diagnostics_dir.display());
                println!("Progress File  : {}", paths.user_progress_file().display());
                println!("--------------------------------------------------");
                println!("Boundary Check : {}", if is_isolated { "ISOLATED (OK)" } else { "VIOLATION" });
                println!("==================================================");
            }
        }
        "purge-cache" => {
            match paths.purge_cache() {
                Ok(cleared) => {
                    if json {
                        let resp = ApiResponse::ok("storage.purge_cache", cleared);
                        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                    } else {
                        println!("Cache purged successfully: {} items removed.", cleared);
                    }
                }
                Err(e) => {
                    if json {
                        let err_str = e.to_string();
                        let (code, rec) =
                            crate::models::classify_recovery_error("storage.purge_cache", &err_str);
                        let resp: ApiResponse<()> = ApiResponse::err_with_recovery(
                            "storage.purge_cache",
                            code,
                            err_str,
                            rec,
                            true,
                        );
                        println!("{}", serde_json::to_string_pretty(&resp).unwrap());
                    } else {
                        eprintln!("Error purging cache: {}", e);
                    }
                }
            }
        }
        other => {
            if json {
                let resp: ApiResponse<()> = ApiResponse::err(
                    "storage",
                    "UNKNOWN_SUBCOMMAND",
                    format!("Unrecognized storage subcommand: {}", other),
                    false,
                );
                println!("{}", serde_json::to_string_pretty(&resp).unwrap());
            } else {
                eprintln!("Unknown storage subcommand: '{}'. Supported: status, purge-cache", other);
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
        println!("  doctor [--export-diagnostics [out.zip]] Run diagnostics or export report");
        println!("  diagnostics export [out.zip] Export privacy-safe diagnostic report");
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
        println!("  lab install-package <file> Install or update lab from .zlab package");
        println!("  lab validate <id>      Verify curriculum contract, content & signature compliance");
        println!("  lab create <id>        Scaffold new lab template with curriculum contract");
        println!("  lab remove <id>        Remove an installed lab");
        println!("  package build <src> <out.zlab> Build and sign a deterministic .zlab package");
        println!("  package verify <file.zlab> Cryptographically verify a .zlab package");
        println!("  terminal [--lab <id>] <cmd> Execute safe command in Zitera Terminal");
        println!("  storage [status|purge-cache] Manage storage boundaries & cache");
        println!("  catalog                Show central catalog");
    }
}
