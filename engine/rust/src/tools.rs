use crate::models::ToolStatus;
use crate::process::run_cmd;

pub fn list_tools() -> Vec<ToolStatus> {
    vec![
        check_tool(
            "nmap",
            "Nmap",
            "Network Scanning",
            "7.80",
            &["--version"],
            "Download official Windows setup from https://nmap.org/download.html",
        ),
        check_tool(
            "sqlmap",
            "sqlmap",
            "Vulnerability Assessment",
            "1.7",
            &["--version"],
            "Install via Python 'pip install sqlmap' or clone from https://github.com/sqlmapproject/sqlmap",
        ),
        check_tool(
            "ffuf",
            "ffuf",
            "Web Fuzzing",
            "2.0",
            &["-V"],
            "Download Windows binary release from https://github.com/ffuf/ffuf/releases",
        ),
        check_tool(
            "gobuster",
            "Gobuster",
            "Content Discovery",
            "3.5",
            &["version"],
            "Download binary release from https://github.com/OJ/gobuster/releases or 'go install github.com/OJ/gobuster/v3@latest'",
        ),
        check_tool(
            "linpeas",
            "LinPEAS",
            "Privilege Escalation",
            "1.0",
            &["-h"],
            "Obtain script from https://github.com/carlospolop/PEASS-ng/releases",
        ),
    ]
}

fn check_tool(
    id: &str,
    name: &str,
    category: &str,
    min_version: &str,
    version_args: &[&str],
    install_guide: &str,
) -> ToolStatus {
    match run_cmd(id, version_args, None) {
        Ok(out) if out.success => {
            let first_line = out.stdout.lines().next().unwrap_or("Installed").to_string();
            ToolStatus {
                id: id.to_string(),
                name: name.to_string(),
                category: category.to_string(),
                installed: true,
                version: Some(first_line),
                min_version: min_version.to_string(),
                status: "READY".to_string(),
                install_guide: install_guide.to_string(),
            }
        }
        _ => ToolStatus {
            id: id.to_string(),
            name: name.to_string(),
            category: category.to_string(),
            installed: false,
            version: None,
            min_version: min_version.to_string(),
            status: "MISSING".to_string(),
            install_guide: install_guide.to_string(),
        },
    }
}
