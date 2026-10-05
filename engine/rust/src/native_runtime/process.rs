use super::error::NativeRuntimeError;
use super::job::JobObject;
use super::profile::LabIdentity;
use std::collections::HashMap;
use std::path::PathBuf;

/// Options for launching a process in an AppContainer sandbox.
#[derive(Debug, Clone)]
pub struct SandboxedProcessConfig {
    pub executable: PathBuf,
    pub arguments: Vec<String>,
    pub working_dir: PathBuf,
    pub environment: HashMap<String, String>,
}

/// Result of running a sandboxed command to completion.
#[derive(Debug, Clone)]
pub struct ProcessOutput {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

/// Handle to an active running sandboxed process.
pub struct SandboxedProcessHandle {
    pub pid: u32,
    #[cfg(windows)]
    h_process: windows_sys::Win32::Foundation::HANDLE,
    #[cfg(windows)]
    h_thread: windows_sys::Win32::Foundation::HANDLE,
    #[cfg(windows)]
    stdout_read: windows_sys::Win32::Foundation::HANDLE,
    #[cfg(windows)]
    stderr_read: windows_sys::Win32::Foundation::HANDLE,
}

#[cfg(windows)]
impl SandboxedProcessHandle {
    /// Returns the raw Win32 process handle.
    pub fn raw_process_handle(&self) -> windows_sys::Win32::Foundation::HANDLE {
        self.h_process
    }

    /// Waits for the process to exit and captures all stdout and stderr.
    pub fn wait(self) -> Result<ProcessOutput, NativeRuntimeError> {
        use std::io::Read;
        use std::os::windows::io::FromRawHandle;
        use windows_sys::Win32::Foundation::CloseHandle;
        use windows_sys::Win32::System::Threading::{
            GetExitCodeProcess, WaitForSingleObject, INFINITE,
        };

        unsafe {
            WaitForSingleObject(self.h_process, INFINITE);
        }

        let mut exit_code: u32 = 0;
        unsafe {
            GetExitCodeProcess(self.h_process, &mut exit_code);
            CloseHandle(self.h_process);
            CloseHandle(self.h_thread);
        }

        let mut stdout_file = unsafe { std::fs::File::from_raw_handle(self.stdout_read as _) };
        let mut stdout_buf = Vec::new();
        let _ = stdout_file.read_to_end(&mut stdout_buf);

        let mut stderr_file = unsafe { std::fs::File::from_raw_handle(self.stderr_read as _) };
        let mut stderr_buf = Vec::new();
        let _ = stderr_file.read_to_end(&mut stderr_buf);

        Ok(ProcessOutput {
            exit_code: exit_code as i32,
            stdout: String::from_utf8_lossy(&stdout_buf).to_string(),
            stderr: String::from_utf8_lossy(&stderr_buf).to_string(),
        })
    }

    /// Forcibly terminates the running process.
    pub fn terminate(&self, exit_code: u32) -> Result<(), NativeRuntimeError> {
        use windows_sys::Win32::Foundation::GetLastError;
        use windows_sys::Win32::System::Threading::TerminateProcess;

        let res = unsafe { TerminateProcess(self.h_process, exit_code) };
        if res == 0 {
            let err = unsafe { GetLastError() };
            return Err(NativeRuntimeError::ProcessLaunchFailed {
                os_code: err,
                message: format!("TerminateProcess failed with code {}", err),
            });
        }
        Ok(())
    }
}

#[cfg(not(windows))]
impl SandboxedProcessHandle {
    pub fn wait(self) -> Result<ProcessOutput, NativeRuntimeError> {
        Ok(ProcessOutput {
            exit_code: 0,
            stdout: String::new(),
            stderr: String::new(),
        })
    }
    pub fn terminate(&self, _exit_code: u32) -> Result<(), NativeRuntimeError> {
        Ok(())
    }
}

#[cfg(windows)]
pub fn spawn_sandboxed_process(
    identity: &LabIdentity,
    config: &SandboxedProcessConfig,
    job: Option<&JobObject>,
) -> Result<SandboxedProcessHandle, NativeRuntimeError> {
    use windows_sys::Win32::Foundation::{
        CloseHandle, GetLastError, SetHandleInformation, BOOL, HANDLE, HANDLE_FLAG_INHERIT, S_OK,
    };
    use windows_sys::Win32::Security::Isolation::DeriveAppContainerSidFromAppContainerName;
    use windows_sys::Win32::Security::{
        FreeSid, GetTokenInformation, TokenIsAppContainer, SECURITY_ATTRIBUTES,
        SECURITY_CAPABILITIES, TOKEN_QUERY,
    };
    use windows_sys::Win32::System::Threading::{
        CreateProcessW, DeleteProcThreadAttributeList, GetCurrentProcess,
        InitializeProcThreadAttributeList, OpenProcessToken, ResumeThread,
        UpdateProcThreadAttribute, CREATE_NO_WINDOW, CREATE_SUSPENDED, CREATE_UNICODE_ENVIRONMENT,
        EXTENDED_STARTUPINFO_PRESENT, PROCESS_INFORMATION, STARTF_USESTDHANDLES, STARTUPINFOEXW,
    };

    extern "system" {
        fn CreatePipe(
            hReadPipe: *mut HANDLE,
            hWritePipe: *mut HANDLE,
            lpPipeAttributes: *const SECURITY_ATTRIBUTES,
            nSize: u32,
        ) -> BOOL;
    }

    fn to_wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    // 1. Detect if current process is already in an AppContainer
    let mut is_parent_app_container: u32 = 0;
    unsafe {
        let mut cur_token: HANDLE = 0 as _;
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut cur_token) != 0 {
            let mut ret_len: u32 = 0;
            let _ = GetTokenInformation(
                cur_token,
                TokenIsAppContainer,
                &mut is_parent_app_container as *mut _ as _,
                std::mem::size_of::<u32>() as u32,
                &mut ret_len,
            );
            CloseHandle(cur_token);
        }
    }

    // 2. Derive AppContainer SID for the process
    let wide_profile_name = to_wide(identity.profile_name());
    let mut psid: *mut core::ffi::c_void = std::ptr::null_mut();
    let hr =
        unsafe { DeriveAppContainerSidFromAppContainerName(wide_profile_name.as_ptr(), &mut psid) };
    if hr != S_OK || psid.is_null() {
        return Err(NativeRuntimeError::ProcessLaunchFailed {
            os_code: hr as u32,
            message: format!(
                "Failed to derive AppContainer SID for process launch: HRESULT 0x{:08X}",
                hr as u32
            ),
        });
    }

    // 3. Set up Pipes for stdin/stdout/stderr redirection
    let mut sec_attrs: SECURITY_ATTRIBUTES = unsafe { std::mem::zeroed() };
    sec_attrs.nLength = std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32;
    sec_attrs.bInheritHandle = 1;

    let mut stdin_read: HANDLE = 0 as _;
    let mut stdin_write: HANDLE = 0 as _;
    let mut stdout_read: HANDLE = 0 as _;
    let mut stdout_write: HANDLE = 0 as _;
    let mut stderr_read: HANDLE = 0 as _;
    let mut stderr_write: HANDLE = 0 as _;

    unsafe {
        if CreatePipe(&mut stdin_read, &mut stdin_write, &sec_attrs, 0) == 0 {
            FreeSid(psid);
            return Err(NativeRuntimeError::ProcessLaunchFailed {
                os_code: GetLastError(),
                message: "Failed to create stdin pipe".to_string(),
            });
        }
        SetHandleInformation(stdin_write, HANDLE_FLAG_INHERIT, 0);

        if CreatePipe(&mut stdout_read, &mut stdout_write, &sec_attrs, 0) == 0 {
            CloseHandle(stdin_read);
            CloseHandle(stdin_write);
            FreeSid(psid);
            return Err(NativeRuntimeError::ProcessLaunchFailed {
                os_code: GetLastError(),
                message: "Failed to create stdout pipe".to_string(),
            });
        }
        SetHandleInformation(stdout_read, HANDLE_FLAG_INHERIT, 0);

        if CreatePipe(&mut stderr_read, &mut stderr_write, &sec_attrs, 0) == 0 {
            CloseHandle(stdin_read);
            CloseHandle(stdin_write);
            CloseHandle(stdout_read);
            CloseHandle(stdout_write);
            FreeSid(psid);
            return Err(NativeRuntimeError::ProcessLaunchFailed {
                os_code: GetLastError(),
                message: "Failed to create stderr pipe".to_string(),
            });
        }
        SetHandleInformation(stderr_read, HANDLE_FLAG_INHERIT, 0);
    }

    // 4. Initialize Attribute List
    const PROC_THREAD_ATTRIBUTE_HANDLE_LIST: usize = 0x00020002;
    const PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES: usize = 0x00020009;

    let attribute_count: u32 = if is_parent_app_container != 0 { 1 } else { 2 };
    let mut attr_size: usize = 0;
    unsafe {
        InitializeProcThreadAttributeList(std::ptr::null_mut(), attribute_count, 0, &mut attr_size);
    }

    let mut attr_buffer: Vec<u8> = vec![0; attr_size];
    let attr_list = attr_buffer.as_mut_ptr() as *mut std::ffi::c_void;

    let init_res =
        unsafe { InitializeProcThreadAttributeList(attr_list, attribute_count, 0, &mut attr_size) };
    if init_res == 0 {
        unsafe {
            CloseHandle(stdin_read);
            CloseHandle(stdin_write);
            CloseHandle(stdout_read);
            CloseHandle(stdout_write);
            CloseHandle(stderr_read);
            CloseHandle(stderr_write);
            FreeSid(psid);
        }
        return Err(NativeRuntimeError::ProcessLaunchFailed {
            os_code: unsafe { GetLastError() },
            message: "InitializeProcThreadAttributeList failed".to_string(),
        });
    }

    let mut sec_caps: SECURITY_CAPABILITIES = unsafe { std::mem::zeroed() };
    if is_parent_app_container == 0 {
        sec_caps.AppContainerSid = psid;
        sec_caps.Capabilities = std::ptr::null_mut();
        sec_caps.CapabilityCount = 0;
        sec_caps.Reserved = 0;

        let update_sec = unsafe {
            UpdateProcThreadAttribute(
                attr_list,
                0,
                PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES,
                &mut sec_caps as *mut _ as _,
                std::mem::size_of::<SECURITY_CAPABILITIES>(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        };

        if update_sec == 0 {
            unsafe {
                DeleteProcThreadAttributeList(attr_list);
                CloseHandle(stdin_read);
                CloseHandle(stdin_write);
                CloseHandle(stdout_read);
                CloseHandle(stdout_write);
                CloseHandle(stderr_read);
                CloseHandle(stderr_write);
                FreeSid(psid);
            }
            return Err(NativeRuntimeError::ProcessLaunchFailed {
                os_code: unsafe { GetLastError() },
                message: "UpdateProcThreadAttribute for SECURITY_CAPABILITIES failed".to_string(),
            });
        }
    }

    let mut handles_to_inherit: [HANDLE; 3] = [stdin_read, stdout_write, stderr_write];
    let update_handles = unsafe {
        UpdateProcThreadAttribute(
            attr_list,
            0,
            PROC_THREAD_ATTRIBUTE_HANDLE_LIST,
            handles_to_inherit.as_mut_ptr() as _,
            handles_to_inherit.len() * std::mem::size_of::<HANDLE>(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };

    if update_handles == 0 {
        unsafe {
            DeleteProcThreadAttributeList(attr_list);
            CloseHandle(stdin_read);
            CloseHandle(stdin_write);
            CloseHandle(stdout_read);
            CloseHandle(stdout_write);
            CloseHandle(stderr_read);
            CloseHandle(stderr_write);
            FreeSid(psid);
        }
        return Err(NativeRuntimeError::ProcessLaunchFailed {
            os_code: unsafe { GetLastError() },
            message: "UpdateProcThreadAttribute for HANDLE_LIST failed".to_string(),
        });
    }

    // 5. Build Environment Block (Cleaned, explicit allowlist, Rule 27)
    let mut env_block: Vec<u16> = Vec::new();
    let mut env_map = config.environment.clone();
    if !env_map.contains_key("SystemRoot") {
        if let Ok(sr) = std::env::var("SystemRoot") {
            env_map.insert("SystemRoot".to_string(), sr);
        }
    }
    if !env_map.contains_key("PATH") {
        if let Ok(p) = std::env::var("PATH") {
            env_map.insert("PATH".to_string(), p);
        }
    }

    let mut keys: Vec<String> = env_map.keys().cloned().collect();
    keys.sort_by_key(|a| a.to_uppercase());
    for k in keys {
        let v = &env_map[&k];
        let entry = format!("{}={}\0", k, v);
        env_block.extend(entry.encode_utf16());
    }
    env_block.push(0);

    // 6. Build Command Line
    let mut cmd_line_str = format!("\"{}\"", config.executable.to_string_lossy());
    for arg in &config.arguments {
        cmd_line_str.push(' ');
        if arg.contains(' ') {
            cmd_line_str.push_str(&format!("\"{}\"", arg));
        } else {
            cmd_line_str.push_str(arg);
        }
    }
    let mut wide_cmd_line = to_wide(&cmd_line_str);
    let wide_working_dir = to_wide(&config.working_dir.to_string_lossy());

    // 7. Setup STARTUPINFOEXW
    let mut si_ex: STARTUPINFOEXW = unsafe { std::mem::zeroed() };
    si_ex.StartupInfo.cb = std::mem::size_of::<STARTUPINFOEXW>() as u32;
    si_ex.StartupInfo.dwFlags = STARTF_USESTDHANDLES;
    si_ex.StartupInfo.hStdOutput = stdout_write;
    si_ex.StartupInfo.hStdError = stderr_write;
    si_ex.StartupInfo.hStdInput = stdin_read;
    si_ex.lpAttributeList = attr_list;

    let mut pi: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

    // Always create suspended if a Job Object is provided, to ensure atomic containment
    let creation_flags = EXTENDED_STARTUPINFO_PRESENT
        | CREATE_NO_WINDOW
        | CREATE_UNICODE_ENVIRONMENT
        | if job.is_some() { CREATE_SUSPENDED } else { 0 };

    let create_res = unsafe {
        CreateProcessW(
            std::ptr::null(),
            wide_cmd_line.as_mut_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            1, // Inherit handles (for pipes)
            creation_flags,
            env_block.as_ptr() as _,
            wide_working_dir.as_ptr(),
            &si_ex.StartupInfo,
            &mut pi,
        )
    };

    // Free attribute list and derived SID handle, close parent's copies of write/stdin handles
    unsafe {
        DeleteProcThreadAttributeList(attr_list);
        FreeSid(psid);
        CloseHandle(stdin_read);
        CloseHandle(stdin_write);
        CloseHandle(stdout_write);
        CloseHandle(stderr_write);
    }

    if create_res == 0 {
        let err = unsafe { GetLastError() };
        unsafe {
            CloseHandle(stdout_read);
            CloseHandle(stderr_read);
        }
        return Err(NativeRuntimeError::ProcessLaunchFailed {
            os_code: err,
            message: format!(
                "CreateProcessW under AppContainer failed with Win32 code 0x{:08X}",
                err
            ),
        });
    }

    // 8. If Job Object is provided, assign the process BEFORE resuming execution
    if let Some(j) = job {
        let assign_res = unsafe { j.assign_process(pi.hProcess) };
        if let Err(e) = assign_res {
            unsafe {
                windows_sys::Win32::System::Threading::TerminateProcess(pi.hProcess, 1);
                CloseHandle(pi.hProcess);
                CloseHandle(pi.hThread);
                CloseHandle(stdout_read);
                CloseHandle(stderr_read);
            }
            return Err(e);
        }
        // Resume thread to start execution inside the Job Object
        unsafe {
            ResumeThread(pi.hThread);
        }
    }

    Ok(SandboxedProcessHandle {
        pid: pi.dwProcessId,
        h_process: pi.hProcess,
        h_thread: pi.hThread,
        stdout_read,
        stderr_read,
    })
}

#[cfg(not(windows))]
pub fn spawn_sandboxed_process(
    _identity: &LabIdentity,
    _config: &SandboxedProcessConfig,
    _job: Option<&JobObject>,
) -> Result<SandboxedProcessHandle, NativeRuntimeError> {
    Err(NativeRuntimeError::SecurityBoundaryViolation(
        "Native runtime process launching is only supported on Windows".to_string(),
    ))
}

pub fn run_sandboxed(
    identity: &LabIdentity,
    config: &SandboxedProcessConfig,
) -> Result<ProcessOutput, NativeRuntimeError> {
    run_sandboxed_with_job(identity, config, None)
}

pub fn run_sandboxed_with_job(
    identity: &LabIdentity,
    config: &SandboxedProcessConfig,
    job: Option<&JobObject>,
) -> Result<ProcessOutput, NativeRuntimeError> {
    let handle = spawn_sandboxed_process(identity, config, job)?;
    handle.wait()
}
