use std::fmt;

/// Strongly-typed error domain for the ZITERA 2.0 Native Security Runtime.
/// Conforms to Rule 24: All Windows API failures map to structured, typed errors
/// with internal details preserved, never exposing raw numeric codes without context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeRuntimeError {
    InvalidLabId(String),
    PathTraversalAttempt(String),
    ProfileCreationFailed {
        lab_id: String,
        os_code: u32,
        message: String,
    },
    ProfileLookupFailed {
        lab_id: String,
        os_code: u32,
        message: String,
    },
    ProfileCleanupFailed {
        lab_id: String,
        os_code: u32,
        message: String,
    },
    JobCreationFailed {
        os_code: u32,
        message: String,
    },
    ProcessLaunchFailed {
        os_code: u32,
        message: String,
    },
    SecurityBoundaryViolation(String),
    IoError(String),
}

impl fmt::Display for NativeRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLabId(id) => write!(
                f,
                "Invalid lab identifier '{}'. Must be 1-16 ASCII alphanumeric characters, hyphens, or underscores.",
                id
            ),
            Self::PathTraversalAttempt(path) => write!(
                f,
                "Security violation: path traversal attempt rejected: {}",
                path
            ),
            Self::ProfileCreationFailed { lab_id, os_code, message } => write!(
                f,
                "Failed to create AppContainer profile for lab '{}' (Win32 0x{:08X}): {}",
                lab_id, os_code, message
            ),
            Self::ProfileLookupFailed { lab_id, os_code, message } => write!(
                f,
                "Failed to resolve AppContainer profile SID for lab '{}' (Win32 0x{:08X}): {}",
                lab_id, os_code, message
            ),
            Self::ProfileCleanupFailed { lab_id, os_code, message } => write!(
                f,
                "Failed to clean up AppContainer profile for lab '{}' (Win32 0x{:08X}): {}",
                lab_id, os_code, message
            ),
            Self::JobCreationFailed { os_code, message } => write!(
                f,
                "Failed to create Windows Job Object (Win32 0x{:08X}): {}",
                os_code, message
            ),
            Self::ProcessLaunchFailed { os_code, message } => write!(
                f,
                "Failed to launch sandboxed process under AppContainer (Win32 0x{:08X}): {}",
                os_code, message
            ),
            Self::SecurityBoundaryViolation(reason) => write!(
                f,
                "Security boundary assertion failure: {}",
                reason
            ),
            Self::IoError(msg) => write!(f, "I/O error in native runtime: {}", msg),
        }
    }
}

impl std::error::Error for NativeRuntimeError {}

impl From<std::io::Error> for NativeRuntimeError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err.to_string())
    }
}
