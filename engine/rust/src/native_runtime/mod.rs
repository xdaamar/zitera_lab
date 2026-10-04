pub mod error;
pub mod profile;

pub use error::NativeRuntimeError;
pub use profile::{
    AppContainerProfile, AppContainerProfileConfig, LabIdentity, MAX_PROFILE_NAME_LEN,
    PROFILE_NAME_PREFIX,
};

/// Abstract lifecycle interface for managing AppContainer profiles.
pub trait AppContainerProfileLifecycle {
    /// Creates or opens an existing deterministic per-user profile.
    fn ensure_profile(&self, config: &AppContainerProfileConfig) -> Result<(), NativeRuntimeError>;

    /// Queries whether the profile exists and can resolve its SID.
    fn profile_exists(&self, identity: &LabIdentity) -> Result<bool, NativeRuntimeError>;

    /// Safely deletes the AppContainer profile and associated per-user metadata.
    fn delete_profile(&self, identity: &LabIdentity) -> Result<(), NativeRuntimeError>;
}

/// Concrete Windows implementation of AppContainer lifecycle.
pub struct WindowsAppContainerLifecycle;

impl AppContainerProfileLifecycle for WindowsAppContainerLifecycle {
    fn ensure_profile(&self, config: &AppContainerProfileConfig) -> Result<(), NativeRuntimeError> {
        let _ = AppContainerProfile::create_or_open(&config.identity)?;
        Ok(())
    }

    fn profile_exists(&self, identity: &LabIdentity) -> Result<bool, NativeRuntimeError> {
        AppContainerProfile::exists(identity)
    }

    fn delete_profile(&self, identity: &LabIdentity) -> Result<(), NativeRuntimeError> {
        AppContainerProfile::delete(identity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_valid_lab_identities() {
        let valid_cases = ["a01", "A01", "lab_01", "lab-02", "SEC123"];
        for case in valid_cases {
            let ident = LabIdentity::new(case);
            assert!(ident.is_ok(), "Expected valid LabIdentity for '{}'", case);
            let ident = ident.unwrap();
            assert!(!ident.id().is_empty());
            assert!(ident.profile_name().starts_with(PROFILE_NAME_PREFIX));
        }
    }

    #[test]
    fn test_invalid_lab_identities() {
        let invalid_cases = [
            "",
            "   ",
            "this_id_is_way_too_long_for_a_lab",
            "../traversal",
            "lab/01",
            "lab\\01",
            "lab*foo",
            "lab?bar",
            "lab;echo",
            "lab$var",
            "lab`cmd`",
        ];
        for case in invalid_cases {
            let res = LabIdentity::new(case);
            assert!(
                res.is_err(),
                "Expected validation error for invalid lab ID: '{}'",
                case
            );
        }
    }

    #[test]
    fn test_deterministic_profile_naming() {
        let id1 = LabIdentity::new("a01").unwrap();
        let id2 = LabIdentity::new("A01").unwrap();
        let id3 = LabIdentity::new("a-01").unwrap();

        assert_eq!(id1.profile_name(), id2.profile_name());
        assert_eq!(id1.profile_name(), "ZiteraLab_A01");
        assert_eq!(id3.profile_name(), "ZiteraLab_A_01");

        assert_eq!(id1.display_name(), "ZITERA_LAB A01 Sandbox");
        assert_eq!(
            id1.description(),
            "Per-user isolated AppContainer sandbox for ZITERA_LAB lab A01"
        );
    }

    #[test]
    fn test_profile_name_length_bound() {
        let id = LabIdentity::new("1234567890123456").unwrap();
        assert!(id.profile_name().len() <= MAX_PROFILE_NAME_LEN);
    }

    #[test]
    fn test_path_validation_and_containment() {
        let identity = LabIdentity::new("a01").unwrap();
        let workspace = Path::new("C:\\zitera_lab");

        let config = AppContainerProfileConfig::new(identity.clone(), workspace, false).unwrap();
        assert_eq!(config.lab_root, Path::new("C:\\zitera_lab\\labs\\A01"));
        assert!(!config.enable_network_loopback);
    }

    #[test]
    fn test_no_system_modification_during_foundation_instantiation() {
        let identity = LabIdentity::new("a05").unwrap();
        let workspace = Path::new("C:\\non_existent_test_workspace");
        let config = AppContainerProfileConfig::new(identity, workspace, true);
        assert!(config.is_ok());
    }

    #[test]
    #[cfg(windows)]
    fn test_appcontainer_profile_proof_lifecycle() {
        let identity = LabIdentity::new("FEASIBILITY_P16").unwrap();

        // Ensure clean starting slate
        let _ = AppContainerProfile::delete(&identity);

        // 1. CREATE: Profile created as standard non-elevated user
        let profile = AppContainerProfile::create_or_open(&identity)
            .expect("AppContainer profile creation MUST succeed without Administrator privileges");

        assert!(
            profile.sid_string.starts_with("S-1-15-2-"),
            "AppContainer SID MUST adhere to Windows S-1-15-2- namespace, got: {}",
            profile.sid_string
        );

        // 2. REOPEN: Reopening resolves deterministically to the identical SID
        let reopen = AppContainerProfile::create_or_open(&identity)
            .expect("Reopening existing AppContainer profile MUST succeed deterministically");
        assert_eq!(reopen.sid_string, profile.sid_string);

        // Lookup profile SID independently
        let derived = AppContainerProfile::derive_sid(&identity)
            .expect("DeriveAppContainerSidFromAppContainerName MUST succeed");
        assert!(derived.starts_with("S-1-15-2-"));

        // 3. CLEANUP: Delete profile safely
        let cleanup_res = AppContainerProfile::delete(&identity);
        assert!(cleanup_res.is_ok(), "Profile cleanup MUST succeed");

        // 4. RECREATE: Same lab can recreate safely
        let recreate = AppContainerProfile::create_or_open(&identity)
            .expect("Recreating cleaned AppContainer profile MUST succeed");
        assert_eq!(recreate.sid_string, profile.sid_string);

        // Final cleanup
        let _ = AppContainerProfile::delete(&identity);
    }
}
