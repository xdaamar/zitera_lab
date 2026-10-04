pub mod error;
pub mod job;
pub mod probe;
pub mod process;
pub mod profile;

pub use error::NativeRuntimeError;
pub use job::{JobLimits, JobObject};
pub use process::{
    run_sandboxed, run_sandboxed_with_job, spawn_sandboxed_process, ProcessOutput,
    SandboxedProcessConfig, SandboxedProcessHandle,
};
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
    use std::collections::HashMap;
    use std::fs;
    use std::path::{Path, PathBuf};

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

    #[test]
    #[cfg(windows)]
    fn test_sandboxed_probe_launch_and_token_identity() {
        let identity = LabIdentity::new("PROBE_TOKEN_TEST").unwrap();
        let _ = AppContainerProfile::create_or_open(&identity);

        let probe_exe = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join("zitera-engine.exe");

        let mut env_map = HashMap::new();
        env_map.insert("ZITERA_ALLOWED_KEY".to_string(), "ALLOWED_VALUE".to_string());

        let config = SandboxedProcessConfig {
            executable: probe_exe,
            arguments: vec!["sandbox-probe".to_string(), "--inspect-token".to_string()],
            working_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            environment: env_map,
        };

        let output = run_sandboxed(&identity, &config)
            .expect("Launching process under AppContainer MUST succeed");

        assert_eq!(
            output.exit_code, 0,
            "Probe must exit with 0 on token inspection, stderr: {}",
            output.stderr
        );
        assert!(
            output.stdout.contains("IS_APPCONTAINER: true"),
            "stdout must confirm IS_APPCONTAINER: true, got:\n{}",
            output.stdout
        );
        assert!(
            output.stdout.contains("ELEVATED: false"),
            "AppContainer process must NOT possess elevation: {}",
            output.stdout
        );
        assert!(
            output.stdout.contains("APPCONTAINER_SID: S-1-15-2-"),
            "AppContainer SID must be reported in token output: {}",
            output.stdout
        );

        let _ = AppContainerProfile::delete(&identity);
    }

    #[test]
    #[cfg(windows)]
    fn test_environment_sanitization_and_secret_isolation() {
        let identity = LabIdentity::new("PROBE_ENV_TEST").unwrap();
        let _ = AppContainerProfile::create_or_open(&identity);

        let probe_exe = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join("zitera-engine.exe");

        // Explicitly set an ambient host secret that must NOT leak into the sandbox
        std::env::set_var("GITHUB_TOKEN", "ghp_leaked_super_secret_host_token_999");
        std::env::set_var("AWS_SECRET_ACCESS_KEY", "AKIA_LEAKED_HOST_SECRET_KEY");

        // Pass only an explicitly allowlisted variable to the sandbox
        let mut clean_env = HashMap::new();
        clean_env.insert("ZITERA_LAB_TEST_VAR".to_string(), "SAFE_VALUE_123".to_string());

        let config_allow = SandboxedProcessConfig {
            executable: probe_exe.clone(),
            arguments: vec![
                "sandbox-probe".to_string(),
                "--check-env".to_string(),
                "ZITERA_LAB_TEST_VAR".to_string(),
            ],
            working_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            environment: clean_env.clone(),
        };

        let out_allow = run_sandboxed(&identity, &config_allow).unwrap();
        assert_eq!(out_allow.exit_code, 0);
        assert!(
            out_allow.stdout.contains("ENV_PRESENT: ZITERA_LAB_TEST_VAR=SAFE_VALUE_123"),
            "Explicitly allowlisted environment variable must be present"
        );

        // Verify that unallowed host secret is absent inside the sandboxed process
        let config_secret = SandboxedProcessConfig {
            executable: probe_exe,
            arguments: vec![
                "sandbox-probe".to_string(),
                "--check-env".to_string(),
                "GITHUB_TOKEN".to_string(),
            ],
            working_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            environment: clean_env,
        };

        let out_secret = run_sandboxed(&identity, &config_secret).unwrap();
        assert_eq!(out_secret.exit_code, 0);
        assert!(
            out_secret.stdout.contains("ENV_ABSENT: GITHUB_TOKEN"),
            "Host secret GITHUB_TOKEN must NOT leak into sandboxed environment: {}",
            out_secret.stdout
        );

        let _ = AppContainerProfile::delete(&identity);
    }

    #[test]
    #[cfg(windows)]
    fn test_filesystem_isolation_boundary() {
        let identity = LabIdentity::new("PROBE_FS_TEST").unwrap();
        let _profile = AppContainerProfile::create_or_open(&identity)
            .expect("Creating profile for filesystem test must succeed");

        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();

        let probe_exe = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join("zitera-engine.exe");

        let test_base = repo_root.join("dev_internal").join("test_artifacts").join("sandbox_probe");
        let allowed_dir = test_base.join("allowed_dir");

        let _ = fs::create_dir_all(&allowed_dir);

        // Grant AppContainer access (*S-1-15-2-1 ALL APPLICATION PACKAGES) to allowed_dir
        let _ = std::process::Command::new("icacls")
            .arg(&allowed_dir)
            .arg("/grant")
            .arg("*S-1-15-2-1:(OI)(CI)F")
            .arg("/T")
            .output();

        // Prepare test files
        let allowed_file = allowed_dir.join("disposable_allowed.txt");
        let _ = fs::write(&allowed_file, "ZITERA_ALLOWED_PAYLOAD");

        // 1. Read allowed file inside AppContainer profile storage
        let config_read_allowed = SandboxedProcessConfig {
            executable: probe_exe.clone(),
            arguments: vec![
                "sandbox-probe".to_string(),
                "--read-file".to_string(),
                allowed_file.to_string_lossy().to_string(),
            ],
            working_dir: allowed_dir.clone(),
            environment: HashMap::new(),
        };
        let out_read_allowed = run_sandboxed(&identity, &config_read_allowed).unwrap();
        assert_eq!(out_read_allowed.exit_code, 0);
        assert!(out_read_allowed.stdout.contains("ALLOWED_READ: ZITERA_ALLOWED_PAYLOAD"));

        // 2. Write allowed file inside AppContainer profile storage
        let new_file = allowed_dir.join("child_created.txt");
        let config_write_allowed = SandboxedProcessConfig {
            executable: probe_exe.clone(),
            arguments: vec![
                "sandbox-probe".to_string(),
                "--write-file".to_string(),
                new_file.to_string_lossy().to_string(),
                "CHILD_CONTENT_OK".to_string(),
            ],
            working_dir: allowed_dir.clone(),
            environment: HashMap::new(),
        };
        let out_write_allowed = run_sandboxed(&identity, &config_write_allowed).unwrap();
        assert_eq!(out_write_allowed.exit_code, 0);
        assert!(out_write_allowed.stdout.contains("ALLOWED_WRITE"));
        assert_eq!(fs::read_to_string(&new_file).unwrap(), "CHILD_CONTENT_OK");

        // 3. Attempt to read protected host security file (MUST BE DENIED by Windows OS security)
        let config_read_denied = SandboxedProcessConfig {
            executable: probe_exe.clone(),
            arguments: vec![
                "sandbox-probe".to_string(),
                "--read-file".to_string(),
                "C:\\Windows\\System32\\config\\SAM".to_string(),
            ],
            working_dir: allowed_dir.clone(),
            environment: HashMap::new(),
        };
        let out_read_denied = run_sandboxed(&identity, &config_read_denied).unwrap();
        assert_eq!(
            out_read_denied.exit_code, 2,
            "Reading host system security database must be denied by OS, got: {:?}",
            out_read_denied
        );
        assert!(out_read_denied.stdout.contains("DENIED_READ"));

        // 4. Attempt to write to Windows system directory (MUST BE DENIED by OS)
        let config_write_system = SandboxedProcessConfig {
            executable: probe_exe,
            arguments: vec![
                "sandbox-probe".to_string(),
                "--write-file".to_string(),
                "C:\\Windows\\zitera_malicious_probe.txt".to_string(),
                "FAIL".to_string(),
            ],
            working_dir: allowed_dir,
            environment: HashMap::new(),
        };
        let out_write_system = run_sandboxed(&identity, &config_write_system).unwrap();
        assert_eq!(out_write_system.exit_code, 2, "Writing to C:\\Windows must be denied by OS");
        assert!(out_write_system.stdout.contains("DENIED_WRITE"));

        // Clean up
        let _ = fs::remove_dir_all(&test_base);
        let _ = AppContainerProfile::delete(&identity);
    }

    #[test]
    #[cfg(windows)]
    fn test_job_object_containment_and_kill_on_close() {
        let identity = LabIdentity::new("JOB_KILL_TEST").unwrap();
        let _ = AppContainerProfile::create_or_open(&identity);

        let job = JobObject::create(None).expect("Creating JobObject must succeed");
        let mut limits = JobLimits::default();
        limits.kill_on_job_close = true;
        job.set_limits(&limits).unwrap();

        let probe_exe = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join("zitera-engine.exe");

        let config = SandboxedProcessConfig {
            executable: probe_exe,
            arguments: vec![
                "sandbox-probe".to_string(),
                "--sleep-ms".to_string(),
                "15000".to_string(),
            ],
            working_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            environment: HashMap::new(),
        };

        let handle = spawn_sandboxed_process(&identity, &config, Some(&job))
            .expect("Spawning sandboxed process into JobObject must succeed");

        // Verify process is registered in the job
        let pids = job.query_process_ids().unwrap();
        assert!(pids.contains(&handle.pid), "Job must track the spawned process PID");

        // Terminate JobObject explicitly with code 42
        job.terminate(42).unwrap();

        // Process must have been terminated with exit code 42
        let output = handle.wait().unwrap();
        assert_eq!(output.exit_code, 42);

        let _ = AppContainerProfile::delete(&identity);
    }

    #[test]
    #[cfg(windows)]
    fn test_job_object_process_tree_containment() {
        let identity = LabIdentity::new("JOB_TREE_TEST").unwrap();
        let _ = AppContainerProfile::create_or_open(&identity);

        let job = JobObject::create(None).expect("Creating JobObject must succeed");
        let limits = JobLimits::default();
        job.set_limits(&limits).unwrap();

        let probe_exe = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join("zitera-engine.exe");

        // Parent process spawns a child that sleeps
        let config = SandboxedProcessConfig {
            executable: probe_exe.clone(),
            arguments: vec![
                "sandbox-probe".to_string(),
                "--spawn-child".to_string(),
                probe_exe.to_string_lossy().to_string(),
                "sandbox-probe".to_string(),
                "--sleep-ms".to_string(),
                "15000".to_string(),
            ],
            working_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            environment: HashMap::new(),
        };

        let handle = spawn_sandboxed_process(&identity, &config, Some(&job)).unwrap();

        // Give child a moment to spawn
        std::thread::sleep(std::time::Duration::from_millis(500));

        // Terminate entire job tree
        job.terminate(99).unwrap();

        let output = handle.wait().unwrap();
        assert_eq!(output.exit_code, 99);

        let _ = AppContainerProfile::delete(&identity);
    }

    #[test]
    #[cfg(windows)]
    fn test_job_object_resource_limits_and_abnormal_exit_recovery() {
        let identity = LabIdentity::new("JOB_RECOVERY").unwrap();
        let _ = AppContainerProfile::create_or_open(&identity);

        let probe_exe = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join("zitera-engine.exe");

        // 1. Crash/Abnormal exit code verification
        let config_crash = SandboxedProcessConfig {
            executable: probe_exe.clone(),
            arguments: vec![
                "sandbox-probe".to_string(),
                "--exit-with-code".to_string(),
                "137".to_string(),
            ],
            working_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            environment: HashMap::new(),
        };
        let out_crash = run_sandboxed(&identity, &config_crash).unwrap();
        assert_eq!(out_crash.exit_code, 137);

        // 2. Immediate clean recovery: next run still succeeds normally
        let config_recovery = SandboxedProcessConfig {
            executable: probe_exe,
            arguments: vec![
                "sandbox-probe".to_string(),
                "--exit-with-code".to_string(),
                "0".to_string(),
            ],
            working_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            environment: HashMap::new(),
        };
        let out_recovery = run_sandboxed(&identity, &config_recovery).unwrap();
        assert_eq!(out_recovery.exit_code, 0);

        let _ = AppContainerProfile::delete(&identity);
    }

    #[test]
    #[cfg(windows)]
    fn test_network_isolation_and_loopback_boundaries() {
        let identity = LabIdentity::new("NET_ISOLATION").unwrap();
        let _ = AppContainerProfile::create_or_open(&identity);

        let probe_exe = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join("zitera-engine.exe");

        // 1. Internet connection attempt (1.1.1.1:80)
        let config_internet = SandboxedProcessConfig {
            executable: probe_exe.clone(),
            arguments: vec![
                "sandbox-probe".to_string(),
                "--connect-network".to_string(),
                "1.1.1.1".to_string(),
                "80".to_string(),
            ],
            working_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            environment: HashMap::new(),
        };
        let out_internet = run_sandboxed(&identity, &config_internet).unwrap();
        assert_eq!(out_internet.exit_code, 2, "Expected outbound internet to be blocked");
        assert!(out_internet.stdout.contains("NETWORK_BLOCKED"));

        // 2. LAN connection attempt (192.168.1.1:80)
        let config_lan = SandboxedProcessConfig {
            executable: probe_exe.clone(),
            arguments: vec![
                "sandbox-probe".to_string(),
                "--connect-network".to_string(),
                "192.168.1.1".to_string(),
                "80".to_string(),
            ],
            working_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            environment: HashMap::new(),
        };
        let out_lan = run_sandboxed(&identity, &config_lan).unwrap();
        assert_eq!(out_lan.exit_code, 2, "Expected outbound LAN to be blocked");
        assert!(out_lan.stdout.contains("NETWORK_BLOCKED"));

        let _ = AppContainerProfile::delete(&identity);
    }







    #[test]
    #[cfg(windows)]
    fn test_lifecycle_repeated_cycles_no_leaks() {
        let identity = LabIdentity::new("CYCLE_10X").unwrap();
        let probe_exe = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join("zitera-engine.exe");

        let config = SandboxedProcessConfig {
            executable: probe_exe,
            arguments: vec![
                "sandbox-probe".to_string(),
                "--exit-with-code".to_string(),
                "0".to_string(),
            ],
            working_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            environment: HashMap::new(),
        };

        for i in 1..=10 {
            let profile = AppContainerProfile::create_or_open(&identity)
                .unwrap_or_else(|e| panic!("Cycle {} create failed: {:?}", i, e));
            assert!(profile.sid_string.starts_with("S-1-15-2-"));

            let output = run_sandboxed(&identity, &config)
                .unwrap_or_else(|e| panic!("Cycle {} run failed: {:?}", i, e));
            assert_eq!(output.exit_code, 0);

            AppContainerProfile::delete(&identity)
                .unwrap_or_else(|e| panic!("Cycle {} delete failed: {:?}", i, e));
        }
    }

    #[test]
    #[cfg(windows)]
    fn test_native_runtime_performance_and_latency() {
        use std::time::Instant;

        let identity = LabIdentity::new("PERF_BENCH").unwrap();
        let probe_exe = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("debug")
            .join("zitera-engine.exe");

        // 1. Measure Profile Creation Latency
        let t0 = Instant::now();
        let _ = AppContainerProfile::create_or_open(&identity).unwrap();
        let profile_setup_us = t0.elapsed().as_micros();

        // 2. Measure Job Object Setup Latency
        let t1 = Instant::now();
        let job = JobObject::create(Some("PERF_JOB")).unwrap();
        let mut limits = JobLimits::default();
        limits.kill_on_job_close = true;
        limits.active_process_limit = Some(10);
        job.set_limits(&limits).unwrap();
        let job_setup_us = t1.elapsed().as_micros();

        // 3. Measure Sandboxed Process Spawn Latency
        let config = SandboxedProcessConfig {
            executable: probe_exe,
            arguments: vec![
                "sandbox-probe".to_string(),
                "--exit-with-code".to_string(),
                "0".to_string(),
            ],
            working_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            environment: HashMap::new(),
        };

        let t2 = Instant::now();
        let output = run_sandboxed(&identity, &config).unwrap();
        let process_launch_and_exec_ms = t2.elapsed().as_millis();
        assert_eq!(output.exit_code, 0);

        // 4. Measure Teardown Latency
        let t3 = Instant::now();
        drop(job);
        AppContainerProfile::delete(&identity).unwrap();
        let teardown_us = t3.elapsed().as_micros();

        println!(
            "PERFORMANCE_METRICS: profile_setup={}us, job_setup={}us, process_exec={}ms, teardown={}us",
            profile_setup_us, job_setup_us, process_launch_and_exec_ms, teardown_us
        );

        assert!(process_launch_and_exec_ms < 1000);
    }
}
