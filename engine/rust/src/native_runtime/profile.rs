use super::error::NativeRuntimeError;
use std::path::{Path, PathBuf};

pub const PROFILE_NAME_PREFIX: &str = "ZiteraLab_";
pub const MAX_PROFILE_NAME_LEN: usize = 64;

/// Strongly validated lab identity representing a deterministic AppContainer security context.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LabIdentity {
    raw_id: String,
    profile_name: String,
    display_name: String,
    description: String,
}

impl LabIdentity {
    /// Validates and constructs a deterministic LabIdentity.
    /// Strict validation rules:
    /// - 1 to 16 characters
    /// - ASCII alphanumeric, hyphen, or underscore only
    /// - Normalizes to uppercase for consistency
    pub fn new(id: &str) -> Result<Self, NativeRuntimeError> {
        let clean = id.trim().to_uppercase();
        if clean.is_empty() || clean.len() > 16 {
            return Err(NativeRuntimeError::InvalidLabId(id.to_string()));
        }
        if !clean
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err(NativeRuntimeError::InvalidLabId(id.to_string()));
        }

        // Sanitized profile name for Windows AppContainer API (no hyphens, safe identifier)
        let sanitized_id: String = clean
            .chars()
            .map(|c| if c == '-' { '_' } else { c })
            .collect();

        let profile_name = format!("{}{}", PROFILE_NAME_PREFIX, sanitized_id);
        if profile_name.len() > MAX_PROFILE_NAME_LEN {
            return Err(NativeRuntimeError::InvalidLabId(format!(
                "Generated profile name '{}' exceeds maximum allowed length of {}",
                profile_name, MAX_PROFILE_NAME_LEN
            )));
        }

        let display_name = format!("ZITERA_LAB {} Sandbox", clean);
        let description = format!(
            "Per-user isolated AppContainer sandbox for ZITERA_LAB lab {}",
            clean
        );

        Ok(Self {
            raw_id: clean,
            profile_name,
            display_name,
            description,
        })
    }

    pub fn id(&self) -> &str {
        &self.raw_id
    }

    pub fn profile_name(&self) -> &str {
        &self.profile_name
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

/// AppContainer configuration parameters scoped strictly to an isolated lab directory.
#[derive(Debug, Clone)]
pub struct AppContainerProfileConfig {
    pub identity: LabIdentity,
    pub lab_root: PathBuf,
    pub enable_network_loopback: bool,
}

impl AppContainerProfileConfig {
    pub fn new(
        identity: LabIdentity,
        workspace_root: &Path,
        enable_network_loopback: bool,
    ) -> Result<Self, NativeRuntimeError> {
        // Enforce workspace boundary
        let lab_dir = workspace_root.join("labs").join(identity.id());

        // Validate safe subpath
        if !lab_dir.starts_with(workspace_root) {
            return Err(NativeRuntimeError::PathTraversalAttempt(
                lab_dir.to_string_lossy().to_string(),
            ));
        }

        Ok(Self {
            identity,
            lab_root: lab_dir,
            enable_network_loopback,
        })
    }
}

/// Concrete representation of an active, per-user Windows AppContainer profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppContainerProfile {
    pub identity: LabIdentity,
    pub sid_string: String,
    pub folder_path: Option<PathBuf>,
}

#[cfg(windows)]
fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

#[cfg(windows)]
unsafe fn sid_to_string(psid: *mut core::ffi::c_void) -> Result<String, NativeRuntimeError> {
    use windows_sys::Win32::Foundation::{GetLastError, LocalFree};
    use windows_sys::Win32::Security::Authorization::ConvertSidToStringSidW;

    let mut str_ptr: *mut u16 = std::ptr::null_mut();
    if ConvertSidToStringSidW(psid, &mut str_ptr) == 0 {
        let err = GetLastError();
        return Err(NativeRuntimeError::ProfileLookupFailed {
            lab_id: "UNKNOWN".to_string(),
            os_code: err,
            message: format!("ConvertSidToStringSidW failed with code {}", err),
        });
    }

    let mut len = 0;
    while *str_ptr.add(len) != 0 {
        len += 1;
    }
    let slice = std::slice::from_raw_parts(str_ptr, len);
    let sid_str = String::from_utf16_lossy(slice);
    LocalFree(str_ptr as _);
    Ok(sid_str)
}

impl AppContainerProfile {
    /// Deterministically derives the AppContainer SID for a given lab identity without side-effects.
    #[cfg(windows)]
    pub fn derive_sid(identity: &LabIdentity) -> Result<String, NativeRuntimeError> {
        use windows_sys::Win32::Foundation::S_OK;
        use windows_sys::Win32::Security::FreeSid;
        use windows_sys::Win32::Security::Isolation::DeriveAppContainerSidFromAppContainerName;

        let wide_name = to_wide(identity.profile_name());
        let mut psid: *mut core::ffi::c_void = std::ptr::null_mut();

        let hr =
            unsafe { DeriveAppContainerSidFromAppContainerName(wide_name.as_ptr(), &mut psid) };
        if hr != S_OK {
            return Err(NativeRuntimeError::ProfileLookupFailed {
                lab_id: identity.id().to_string(),
                os_code: hr as u32,
                message: format!(
                    "DeriveAppContainerSidFromAppContainerName failed (HRESULT 0x{:08X})",
                    hr as u32
                ),
            });
        }

        let sid_str = unsafe { sid_to_string(psid)? };
        unsafe { FreeSid(psid) };
        Ok(sid_str)
    }

    #[cfg(not(windows))]
    pub fn derive_sid(identity: &LabIdentity) -> Result<String, NativeRuntimeError> {
        Ok(format!("S-1-15-2-MOCK-{}", identity.id()))
    }

    /// Queries the OS folder path associated with this AppContainer SID.
    #[cfg(windows)]
    pub fn lookup_folder_path(sid_string: &str) -> Option<PathBuf> {
        use windows_sys::Win32::Foundation::S_OK;
        use windows_sys::Win32::Security::Isolation::GetAppContainerFolderPath;
        use windows_sys::Win32::System::Com::CoTaskMemFree;

        let wide_sid = to_wide(sid_string);
        let mut path_ptr: *mut u16 = std::ptr::null_mut();

        let hr = unsafe { GetAppContainerFolderPath(wide_sid.as_ptr(), &mut path_ptr) };
        if hr != S_OK || path_ptr.is_null() {
            return None;
        }

        let mut len = 0;
        unsafe {
            while *path_ptr.add(len) != 0 {
                len += 1;
            }
            let slice = std::slice::from_raw_parts(path_ptr, len);
            let path_str = String::from_utf16_lossy(slice);
            CoTaskMemFree(path_ptr as _);
            Some(PathBuf::from(path_str))
        }
    }

    #[cfg(not(windows))]
    pub fn lookup_folder_path(_sid_string: &str) -> Option<PathBuf> {
        None
    }

    /// Checks whether the profile exists for the current Windows user.
    pub fn exists(identity: &LabIdentity) -> Result<bool, NativeRuntimeError> {
        let sid = Self::derive_sid(identity)?;
        if let Some(folder) = Self::lookup_folder_path(&sid) {
            Ok(folder.exists())
        } else {
            Ok(false)
        }
    }

    /// Creates or opens an existing AppContainer profile without administrative elevation.
    /// Follows strict least-privilege: ZERO capabilities requested.
    #[cfg(windows)]
    pub fn create_or_open(identity: &LabIdentity) -> Result<Self, NativeRuntimeError> {
        use windows_sys::Win32::Foundation::S_OK;
        use windows_sys::Win32::Security::FreeSid;
        use windows_sys::Win32::Security::Isolation::CreateAppContainerProfile;

        let wide_name = to_wide(identity.profile_name());
        let wide_display = to_wide(identity.display_name());
        let wide_desc = to_wide(identity.description());

        let mut psid: *mut core::ffi::c_void = std::ptr::null_mut();

        // Pass null capabilities and 0 count -> Least Privilege (Rule 23)
        let hr = unsafe {
            CreateAppContainerProfile(
                wide_name.as_ptr(),
                wide_display.as_ptr(),
                wide_desc.as_ptr(),
                std::ptr::null(),
                0,
                &mut psid,
            )
        };

        const HRESULT_ALREADY_EXISTS: i32 = -2147024713; // 0x800700B7: HRESULT_FROM_WIN32(ERROR_ALREADY_EXISTS)

        if !psid.is_null() {
            unsafe { FreeSid(psid) };
        }

        if hr != S_OK && hr != HRESULT_ALREADY_EXISTS {
            return Err(NativeRuntimeError::ProfileCreationFailed {
                lab_id: identity.id().to_string(),
                os_code: hr as u32,
                message: format!(
                    "CreateAppContainerProfile failed with HRESULT 0x{:08X}",
                    hr as u32
                ),
            });
        }

        let sid_string = Self::derive_sid(identity)?;

        let folder_path = Self::lookup_folder_path(&sid_string);

        Ok(Self {
            identity: identity.clone(),
            sid_string,
            folder_path,
        })
    }

    #[cfg(not(windows))]
    pub fn create_or_open(identity: &LabIdentity) -> Result<Self, NativeRuntimeError> {
        let sid = Self::derive_sid(identity)?;
        Ok(Self {
            identity: identity.clone(),
            sid_string: sid,
            folder_path: None,
        })
    }

    /// Deletes the AppContainer profile and removes its per-user security identifier and metadata.
    #[cfg(windows)]
    pub fn delete(identity: &LabIdentity) -> Result<(), NativeRuntimeError> {
        use windows_sys::Win32::Foundation::S_OK;
        use windows_sys::Win32::Security::Isolation::DeleteAppContainerProfile;

        let wide_name = to_wide(identity.profile_name());
        let hr = unsafe { DeleteAppContainerProfile(wide_name.as_ptr()) };

        const HRESULT_FILE_NOT_FOUND: i32 = -2147024894; // 0x80070002
        const HRESULT_ELEMENT_NOT_FOUND: i32 = -2147023728; // 0x80070490

        if hr == S_OK || hr == HRESULT_FILE_NOT_FOUND || hr == HRESULT_ELEMENT_NOT_FOUND {
            Ok(())
        } else {
            Err(NativeRuntimeError::ProfileCleanupFailed {
                lab_id: identity.id().to_string(),
                os_code: hr as u32,
                message: format!(
                    "DeleteAppContainerProfile failed with HRESULT 0x{:08X}",
                    hr as u32
                ),
            })
        }
    }

    #[cfg(not(windows))]
    pub fn delete(_identity: &LabIdentity) -> Result<(), NativeRuntimeError> {
        Ok(())
    }
}
