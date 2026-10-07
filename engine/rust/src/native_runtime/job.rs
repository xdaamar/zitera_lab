use super::error::NativeRuntimeError;

/// Resource limit configuration for a Windows Job Object.
#[derive(Debug, Clone)]
pub struct JobLimits {
    pub kill_on_job_close: bool,
    pub active_process_limit: Option<u32>,
    pub job_memory_limit_bytes: Option<usize>,
    pub process_memory_limit_bytes: Option<usize>,
    pub die_on_unhandled_exception: bool,
}

impl Default for JobLimits {
    fn default() -> Self {
        Self {
            kill_on_job_close: true,
            active_process_limit: None,
            job_memory_limit_bytes: None,
            process_memory_limit_bytes: None,
            die_on_unhandled_exception: true,
        }
    }
}

/// Safe wrapper around a Windows Job Object handle.
pub struct JobObject {
    #[cfg(windows)]
    handle: windows_sys::Win32::Foundation::HANDLE,
}

// Safety: Windows Job Object handles can be sent between threads.
unsafe impl Send for JobObject {}
unsafe impl Sync for JobObject {}

#[cfg(windows)]
impl JobObject {
    /// Creates a new anonymous or named Job Object.
    pub fn create(name: Option<&str>) -> Result<Self, NativeRuntimeError> {
        use windows_sys::Win32::Foundation::GetLastError;
        use windows_sys::Win32::System::JobObjects::CreateJobObjectW;

        let wide_name: Option<Vec<u16>> =
            name.map(|n| n.encode_utf16().chain(std::iter::once(0)).collect());

        let name_ptr = match &wide_name {
            Some(w) => w.as_ptr(),
            None => std::ptr::null(),
        };

        let handle = unsafe { CreateJobObjectW(std::ptr::null(), name_ptr) };
        if handle == 0 as _ {
            let err = unsafe { GetLastError() };
            return Err(NativeRuntimeError::JobCreationFailed {
                os_code: err,
                message: format!("CreateJobObjectW failed with code {}", err),
            });
        }

        Ok(Self { handle })
    }

    /// Returns the raw Win32 HANDLE.
    pub fn raw_handle(&self) -> windows_sys::Win32::Foundation::HANDLE {
        self.handle
    }

    /// Sets limit policies on the Job Object (kill on close, process count, memory bounds).
    pub fn set_limits(&self, limits: &JobLimits) -> Result<(), NativeRuntimeError> {
        use windows_sys::Win32::Foundation::GetLastError;
        use windows_sys::Win32::System::JobObjects::{
            JobObjectExtendedLimitInformation, SetInformationJobObject,
            JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JOB_OBJECT_LIMIT_ACTIVE_PROCESS,
            JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION, JOB_OBJECT_LIMIT_JOB_MEMORY,
            JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE, JOB_OBJECT_LIMIT_PROCESS_MEMORY,
        };

        let mut info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = unsafe { std::mem::zeroed() };
        let mut flags: u32 = 0;

        if limits.kill_on_job_close {
            flags |= JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
        }
        if limits.die_on_unhandled_exception {
            flags |= JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION;
        }
        if let Some(proc_limit) = limits.active_process_limit {
            flags |= JOB_OBJECT_LIMIT_ACTIVE_PROCESS;
            info.BasicLimitInformation.ActiveProcessLimit = proc_limit;
        }
        if let Some(job_mem) = limits.job_memory_limit_bytes {
            flags |= JOB_OBJECT_LIMIT_JOB_MEMORY;
            info.JobMemoryLimit = job_mem;
        }
        if let Some(proc_mem) = limits.process_memory_limit_bytes {
            flags |= JOB_OBJECT_LIMIT_PROCESS_MEMORY;
            info.ProcessMemoryLimit = proc_mem;
        }

        info.BasicLimitInformation.LimitFlags = flags;

        let res = unsafe {
            SetInformationJobObject(
                self.handle,
                JobObjectExtendedLimitInformation,
                &info as *const _ as _,
                std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
            )
        };

        if res == 0 {
            let err = unsafe { GetLastError() };
            return Err(NativeRuntimeError::ProcessLaunchFailed {
                os_code: err,
                message: format!("SetInformationJobObject failed with code {}", err),
            });
        }

        Ok(())
    }

    /// Assigns an existing process to this Job Object.
    ///
    /// # Safety
    /// `process_handle` must be a valid, unclosed Win32 process handle with PROCESS_SET_QUOTA and PROCESS_TERMINATE rights.
    pub unsafe fn assign_process(
        &self,
        process_handle: windows_sys::Win32::Foundation::HANDLE,
    ) -> Result<(), NativeRuntimeError> {
        use windows_sys::Win32::Foundation::GetLastError;
        use windows_sys::Win32::System::JobObjects::AssignProcessToJobObject;

        let res = unsafe { AssignProcessToJobObject(self.handle, process_handle) };
        if res == 0 {
            let err = unsafe { GetLastError() };
            return Err(NativeRuntimeError::ProcessLaunchFailed {
                os_code: err,
                message: format!("AssignProcessToJobObject failed with code {}", err),
            });
        }
        Ok(())
    }

    /// Forcibly terminates all processes currently associated with this Job Object.
    pub fn terminate(&self, exit_code: u32) -> Result<(), NativeRuntimeError> {
        use windows_sys::Win32::Foundation::GetLastError;
        use windows_sys::Win32::System::JobObjects::TerminateJobObject;

        let res = unsafe { TerminateJobObject(self.handle, exit_code) };
        if res == 0 {
            let err = unsafe { GetLastError() };
            return Err(NativeRuntimeError::ProcessLaunchFailed {
                os_code: err,
                message: format!("TerminateJobObject failed with code {}", err),
            });
        }
        Ok(())
    }

    /// Queries the list of Process IDs currently assigned to this Job Object.
    pub fn query_process_ids(&self) -> Result<Vec<u32>, NativeRuntimeError> {
        use windows_sys::Win32::Foundation::GetLastError;
        use windows_sys::Win32::System::JobObjects::{
            JobObjectBasicProcessIdList, QueryInformationJobObject, JOBOBJECT_BASIC_PROCESS_ID_LIST,
        };

        // Allocate buffer for header + up to 128 PIDs
        let max_pids = 128;
        let buf_size = std::mem::size_of::<JOBOBJECT_BASIC_PROCESS_ID_LIST>()
            + (max_pids - 1) * std::mem::size_of::<usize>();
        let mut buffer = vec![0u8; buf_size];

        let mut return_length: u32 = 0;
        let res = unsafe {
            QueryInformationJobObject(
                self.handle,
                JobObjectBasicProcessIdList,
                buffer.as_mut_ptr() as _,
                buf_size as u32,
                &mut return_length,
            )
        };

        if res == 0 {
            let err = unsafe { GetLastError() };
            return Err(NativeRuntimeError::ProcessLaunchFailed {
                os_code: err,
                message: format!("QueryInformationJobObject failed with code {}", err),
            });
        }

        let header = buffer.as_ptr() as *const JOBOBJECT_BASIC_PROCESS_ID_LIST;
        let count = unsafe { (*header).NumberOfProcessIdsInList } as usize;
        let pids_ptr = unsafe { (*header).ProcessIdList.as_ptr() };

        let mut pids = Vec::with_capacity(count);
        for i in 0..count {
            let pid = unsafe { *pids_ptr.add(i) } as u32;
            pids.push(pid);
        }

        Ok(pids)
    }

    /// Queries the total number of processes currently active in the job.
    pub fn query_process_count(&self) -> Result<u32, NativeRuntimeError> {
        let pids = self.query_process_ids()?;
        Ok(pids.len() as u32)
    }
}

#[cfg(windows)]
impl Drop for JobObject {
    fn drop(&mut self) {
        use windows_sys::Win32::Foundation::CloseHandle;
        unsafe {
            CloseHandle(self.handle);
        }
    }
}

#[cfg(not(windows))]
impl JobObject {
    pub fn create(_name: Option<&str>) -> Result<Self, NativeRuntimeError> {
        Ok(Self {})
    }
    pub fn set_limits(&self, _limits: &JobLimits) -> Result<(), NativeRuntimeError> {
        Ok(())
    }
    pub unsafe fn assign_process(&self, _process_handle: usize) -> Result<(), NativeRuntimeError> {
        Ok(())
    }
    pub fn terminate(&self, _exit_code: u32) -> Result<(), NativeRuntimeError> {
        Ok(())
    }
    pub fn query_process_ids(&self) -> Result<Vec<u32>, NativeRuntimeError> {
        Ok(Vec::new())
    }
    pub fn query_process_count(&self) -> Result<u32, NativeRuntimeError> {
        Ok(0)
    }
}
