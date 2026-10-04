use std::env;
use std::fs;
use std::net::TcpStream;
use std::path::Path;
use std::process::exit;
use std::time::Duration;

#[cfg(windows)]
mod win {
    use windows_sys::Win32::Foundation::{CloseHandle, GetLastError, HANDLE};
    use windows_sys::Win32::Security::Authorization::ConvertSidToStringSidW;
    use windows_sys::Win32::Security::{
        GetTokenInformation, TokenAppContainerSid, TokenElevation, TokenIntegrityLevel,
        TokenIsAppContainer, TOKEN_APPCONTAINER_INFORMATION, TOKEN_ELEVATION,
        TOKEN_MANDATORY_LABEL, TOKEN_QUERY,
    };
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    pub fn inspect_token() -> Result<(), String> {
        unsafe {
            let mut token: HANDLE = 0 as _;
            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
                return Err(format!("OpenProcessToken failed: {}", GetLastError()));
            }

            // 1. IsAppContainer
            let mut is_app_container: u32 = 0;
            let mut ret_len: u32 = 0;
            if GetTokenInformation(
                token,
                TokenIsAppContainer,
                &mut is_app_container as *mut _ as _,
                std::mem::size_of::<u32>() as u32,
                &mut ret_len,
            ) == 0
            {
                CloseHandle(token);
                return Err(format!(
                    "GetTokenInformation(TokenIsAppContainer) failed: {}",
                    GetLastError()
                ));
            }
            println!("IS_APPCONTAINER: {}", is_app_container != 0);

            #[repr(C, align(8))]
            struct AlignedBuffer([u8; 128]);

            // 2. AppContainer SID
            if is_app_container != 0 {
                let mut app_container_info = AlignedBuffer([0; 128]);
                if GetTokenInformation(
                    token,
                    TokenAppContainerSid,
                    app_container_info.0.as_mut_ptr() as _,
                    app_container_info.0.len() as u32,
                    &mut ret_len,
                ) != 0
                {
                    let info =
                        app_container_info.0.as_ptr() as *const TOKEN_APPCONTAINER_INFORMATION;
                    let sid = (*info).TokenAppContainer;
                    if !sid.is_null() {
                        let mut str_ptr: *mut u16 = std::ptr::null_mut();
                        if ConvertSidToStringSidW(sid, &mut str_ptr) != 0 && !str_ptr.is_null() {
                            let mut len = 0;
                            while *str_ptr.add(len) != 0 {
                                len += 1;
                            }
                            let slice = std::slice::from_raw_parts(str_ptr, len);
                            let sid_str = String::from_utf16_lossy(slice);
                            windows_sys::Win32::Foundation::LocalFree(str_ptr as _);
                            println!("APPCONTAINER_SID: {}", sid_str);
                        }
                    }
                }
            }

            // 3. Token Integrity Level
            let mut label_buffer = AlignedBuffer([0; 128]);
            if GetTokenInformation(
                token,
                TokenIntegrityLevel,
                label_buffer.0.as_mut_ptr() as _,
                label_buffer.0.len() as u32,
                &mut ret_len,
            ) != 0
            {
                let label = label_buffer.0.as_ptr() as *const TOKEN_MANDATORY_LABEL;
                let sid = (*label).Label.Sid;
                if !sid.is_null() {
                    let sub_auth_count =
                        *windows_sys::Win32::Security::GetSidSubAuthorityCount(sid);
                    if sub_auth_count > 0 {
                        let rid = *windows_sys::Win32::Security::GetSidSubAuthority(
                            sid,
                            (sub_auth_count - 1) as u32,
                        );
                        println!("INTEGRITY_LEVEL_RID: 0x{:04X}", rid);
                    }
                }
            }

            // 4. Token Elevation
            let mut elevation: TOKEN_ELEVATION = std::mem::zeroed();
            if GetTokenInformation(
                token,
                TokenElevation,
                &mut elevation as *mut _ as _,
                std::mem::size_of::<TOKEN_ELEVATION>() as u32,
                &mut ret_len,
            ) != 0
            {
                println!("ELEVATED: {}", elevation.TokenIsElevated != 0);
            }

            CloseHandle(token);
            Ok(())
        }
    }
}

pub fn handle_probe_cli(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: sandbox-probe <action> [args...]");
        exit(1);
    }

    let command = args[0].as_str();
    match command {
        "--read-file" => {
            if args.len() < 2 {
                eprintln!("Missing path for --read-file");
                exit(1);
            }
            let path = Path::new(&args[1]);
            match fs::read_to_string(path) {
                Ok(content) => {
                    println!("ALLOWED_READ: {}", content);
                    exit(0);
                }
                Err(e) => {
                    println!("DENIED_READ: {}", e);
                    exit(2);
                }
            }
        }
        "--write-file" => {
            if args.len() < 3 {
                eprintln!("Missing path or content for --write-file");
                exit(1);
            }
            let path = Path::new(&args[1]);
            let content = &args[2];
            match fs::write(path, content) {
                Ok(_) => {
                    println!("ALLOWED_WRITE: {}", path.display());
                    exit(0);
                }
                Err(e) => {
                    println!("DENIED_WRITE: {}", e);
                    exit(2);
                }
            }
        }
        "--inspect-token" => {
            #[cfg(windows)]
            {
                if let Err(e) = win::inspect_token() {
                    eprintln!("Token inspection failed: {}", e);
                    exit(1);
                }
                exit(0);
            }
            #[cfg(not(windows))]
            {
                println!("IS_APPCONTAINER: false");
                println!("ELEVATED: false");
                exit(0);
            }
        }
        "--check-env" => {
            if args.len() < 2 {
                eprintln!("Missing env var name for --check-env");
                exit(1);
            }
            let var_name = &args[1];
            match env::var(var_name) {
                Ok(val) => {
                    println!("ENV_PRESENT: {}={}", var_name, val);
                    exit(0);
                }
                Err(_) => {
                    println!("ENV_ABSENT: {}", var_name);
                    exit(0);
                }
            }
        }
        "--dump-env-keys" => {
            let mut keys: Vec<String> = env::vars().map(|(k, _)| k).collect();
            keys.sort();
            for k in keys {
                println!("ENV_KEY: {}", k);
            }
            exit(0);
        }
        "--connect-network" => {
            if args.len() < 3 {
                eprintln!("Missing host and port for --connect-network");
                exit(1);
            }
            let host = &args[1];
            let port: u16 = args[2].parse().unwrap_or(80);
            let addr = format!("{}:{}", host, port);
            match TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_millis(1500)) {
                Ok(_) => {
                    println!("NETWORK_CONNECTED: {}", addr);
                    exit(0);
                }
                Err(e) => {
                    println!("NETWORK_BLOCKED: {} ({})", addr, e);
                    exit(2);
                }
            }
        }
        "--sleep-ms" => {
            let ms: u64 = if args.len() >= 2 {
                args[1].parse().unwrap_or(1000)
            } else {
                1000
            };
            println!("SLEEPING_MS: {}", ms);
            std::thread::sleep(Duration::from_millis(ms));
            println!("AWAKE");
            exit(0);
        }
        "--allocate-mb" => {
            let mb: usize = if args.len() >= 2 {
                args[1].parse().unwrap_or(10)
            } else {
                10
            };
            let mut data: Vec<u8> = vec![0; mb * 1024 * 1024];
            for i in (0..data.len()).step_by(4096) {
                data[i] = 0xAA;
            }
            println!("ALLOCATED_MB: {}", mb);
            exit(0);
        }
        "--spawn-child" => {
            if args.len() < 2 {
                eprintln!("Missing executable for --spawn-child");
                exit(1);
            }
            let child_exe = &args[1];
            let child_args = &args[2..];
            let mut child = match std::process::Command::new(child_exe)
                .args(child_args)
                .spawn()
            {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Failed to spawn child: {}", e);
                    exit(1);
                }
            };
            println!("CHILD_PID: {}", child.id());
            let status = child.wait().unwrap();
            exit(status.code().unwrap_or(0));
        }
        "--exit-with-code" => {
            let code: i32 = if args.len() >= 2 {
                args[1].parse().unwrap_or(0)
            } else {
                0
            };
            println!("EXITING_WITH_CODE: {}", code);
            exit(code);
        }
        other => {
            eprintln!("Unknown probe command: {}", other);
            exit(1);
        }
    }
}
