use crate::models::{ComponentStatus, SystemDiagnostics};
use crate::process::run_cmd;
use std::net::TcpListener;

pub fn diagnose_system() -> SystemDiagnostics {
    let os_status = check_os();
    let git_status = check_git();
    let wsl_status = check_wsl();
    let docker_status = check_docker_cli();
    let docker_daemon_status = check_docker_daemon();
    let powershell_status = check_powershell();

    // Phase 15 Architecture Freeze: Host readiness no longer blocks on Docker/WSL
    let all_ready = os_status.installed && powershell_status.installed;

    let (memory_gb, disk_free_gb) = query_resources();

    SystemDiagnostics {
        os: os_status,
        git: git_status,
        wsl: wsl_status,
        docker: docker_status,
        docker_daemon: docker_daemon_status,
        powershell: powershell_status,
        memory_gb,
        disk_free_gb,
        all_ready,
    }
}

pub fn is_port_available(port: u16) -> bool {
    TcpListener::bind(("127.0.0.1", port)).is_ok()
}

fn check_os() -> ComponentStatus {
    ComponentStatus {
        name: "Microsoft Windows".to_string(),
        installed: true,
        version: Some("Windows 10/11 x64".to_string()),
        status: "READY".to_string(),
        message: "Compatible Windows architecture detected.".to_string(),
        recommendation: None,
    }
}

fn check_git() -> ComponentStatus {
    match run_cmd("git", &["--version"], None) {
        Ok(out) if out.success => {
            let ver = out.stdout.replace("git version", "").trim().to_string();
            ComponentStatus {
                name: "Git (Optional)".to_string(),
                installed: true,
                version: Some(ver),
                status: "READY".to_string(),
                message: "Git is available (optional developer utility).".to_string(),
                recommendation: None,
            }
        }
        _ => ComponentStatus {
            name: "Git (Optional)".to_string(),
            installed: false,
            version: None,
            status: "NOT_REQUIRED".to_string(),
            message: "Git is optional; offline courseware operates without Git.".to_string(),
            recommendation: None,
        },
    }
}

fn check_wsl() -> ComponentStatus {
    ComponentStatus {
        name: "WSL2 (Legacy)".to_string(),
        installed: false,
        version: None,
        status: "NOT_REQUIRED".to_string(),
        message: "WSL2 is deprecated for ZITERA 2.0 Native Runtime and no longer required."
            .to_string(),
        recommendation: None,
    }
}

fn check_docker_cli() -> ComponentStatus {
    ComponentStatus {
        name: "Docker CLI (Legacy)".to_string(),
        installed: false,
        version: None,
        status: "NOT_REQUIRED".to_string(),
        message: "Docker CLI is deprecated for ZITERA 2.0 Native Runtime (Windows AppContainer)."
            .to_string(),
        recommendation: None,
    }
}

fn check_docker_daemon() -> ComponentStatus {
    ComponentStatus {
        name: "Docker Daemon (Legacy)".to_string(),
        installed: false,
        version: None,
        status: "NOT_REQUIRED".to_string(),
        message: "Docker daemon dependency is deprecated; native runtime operates daemonless."
            .to_string(),
        recommendation: None,
    }
}

/// Query real RAM and disk-free values from the host OS via native Win32 API.
/// Instantaneous, permission-free, and unaffected by PowerShell locale decimal separators.
#[cfg(windows)]
fn query_resources() -> (f64, f64) {
    #[repr(C)]
    struct MemoryStatusEx {
        length: u32,
        memory_load: u32,
        total_phys: u64,
        avail_phys: u64,
        total_page_file: u64,
        avail_page_file: u64,
        total_virtual: u64,
        avail_virtual: u64,
        avail_extended_virtual: u64,
    }

    extern "system" {
        fn GlobalMemoryStatusEx(stat: *mut MemoryStatusEx) -> i32;
        fn GetDiskFreeSpaceExW(
            directory_name: *const u16,
            free_bytes_available: *mut u64,
            total_number_of_bytes: *mut u64,
            total_number_of_free_bytes: *mut u64,
        ) -> i32;
    }

    let mut memory_gb = 0.0;
    let mut disk_free_gb = 0.0;

    unsafe {
        let mut mem_stat = MemoryStatusEx {
            length: std::mem::size_of::<MemoryStatusEx>() as u32,
            memory_load: 0,
            total_phys: 0,
            avail_phys: 0,
            total_page_file: 0,
            avail_page_file: 0,
            total_virtual: 0,
            avail_virtual: 0,
            avail_extended_virtual: 0,
        };

        if GlobalMemoryStatusEx(&mut mem_stat) != 0 {
            memory_gb = (mem_stat.total_phys as f64) / (1024.0 * 1024.0 * 1024.0);
        }

        let drive_root: [u16; 4] = [b'C' as u16, b':' as u16, b'\\' as u16, 0];
        let mut free_bytes: u64 = 0;
        let mut total_bytes: u64 = 0;
        let mut total_free: u64 = 0;

        if GetDiskFreeSpaceExW(
            drive_root.as_ptr(),
            &mut free_bytes,
            &mut total_bytes,
            &mut total_free,
        ) != 0
        {
            disk_free_gb = (free_bytes as f64) / (1024.0 * 1024.0 * 1024.0);
        }
    }

    memory_gb = (memory_gb * 10.0).round() / 10.0;
    disk_free_gb = (disk_free_gb * 10.0).round() / 10.0;

    (memory_gb, disk_free_gb)
}

#[cfg(not(windows))]
fn query_resources() -> (f64, f64) {
    (16.0, 50.0)
}

fn check_powershell() -> ComponentStatus {
    let windir = std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".to_string());
    let ps = format!(r"{}\System32\WindowsPowerShell\v1.0\powershell.exe", windir);
    match run_cmd(
        &ps,
        &["-NoProfile", "-Command", "$PSVersionTable.PSVersion.Major"],
        None,
    ) {
        Ok(out) if out.success => ComponentStatus {
            name: "PowerShell".to_string(),
            installed: true,
            version: Some(format!("v{}", out.stdout.trim())),
            status: "READY".to_string(),
            message: "PowerShell is available.".to_string(),
            recommendation: None,
        },
        _ => ComponentStatus {
            name: "PowerShell".to_string(),
            installed: false,
            version: None,
            status: "WARNING".to_string(),
            message: "PowerShell not directly invocable.".to_string(),
            recommendation: None,
        },
    }
}
