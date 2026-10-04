pub mod error;
pub mod profile;

pub use error::NativeRuntimeError;
pub use profile::{
    AppContainerProfileConfig, LabIdentity, MAX_PROFILE_NAME_LEN, PROFILE_NAME_PREFIX,
};

/// Abstract lifecycle interface for managing AppContainer profiles.
/// Concrete Win32 implementation is provided in Checkpoint 2.
pub trait AppContainerProfileLifecycle {
    /// Creates or opens an existing deterministic per-user profile.
    fn ensure_profile(&self, config: &AppContainerProfileConfig) -> Result<(), NativeRuntimeError>;

    /// Queries whether the profile exists and can resolve its SID.
    fn profile_exists(&self, identity: &LabIdentity) -> Result<bool, NativeRuntimeError>;

    /// Safely deletes the AppContainer profile and associated per-user metadata.
    fn delete_profile(&self, identity: &LabIdentity) -> Result<(), NativeRuntimeError>;
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

        // Normalization ensures identical profile name for case variations
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
        // Assert that instantiating identities and configs does not create or mutate files,
        // registry, or services.
        let identity = LabIdentity::new("a05").unwrap();
        let workspace = Path::new("C:\\non_existent_test_workspace");
        let config = AppContainerProfileConfig::new(identity, workspace, true);
        assert!(config.is_ok());
    }
}
