use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::http::{HttpResponse, ParsedRequest};
use super::server::{BrokerError, LabRequestHandler};

/// Protocol message format exchanged between Broker Host and Sandboxed Lab over stdio.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StdioMessage {
    #[serde(rename = "READY")]
    Ready { lab_id: String, version: String },
    #[serde(rename = "HTTP_REQUEST")]
    HttpRequest {
        id: u64,
        method: String,
        path: String,
        headers: HashMap<String, String>,
        body: String,
    },
    #[serde(rename = "HTTP_RESPONSE")]
    HttpResponse {
        id: u64,
        status: u16,
        headers: HashMap<String, String>,
        body: String,
    },
    #[serde(rename = "HEALTH")]
    Health { id: u64 },
    #[serde(rename = "STATUS")]
    Status {
        id: u64,
        status: String,
        lab_id: String,
    },
    #[serde(rename = "STOP")]
    Stop,
    #[serde(rename = "CLEAN_EXIT")]
    CleanExit,
}

/// Bidirectional stdio communication channel to a sandboxed lab process.
pub struct StdioLabChannel {
    stdin: Arc<Mutex<Option<File>>>,
    stdout: Arc<Mutex<Option<BufReader<File>>>>,
    counter: AtomicU64,
}

impl StdioLabChannel {
    pub fn new(stdin_file: File, stdout_file: File) -> Self {
        Self {
            stdin: Arc::new(Mutex::new(Some(stdin_file))),
            stdout: Arc::new(Mutex::new(Some(BufReader::new(stdout_file)))),
            counter: AtomicU64::new(1),
        }
    }

    /// Sends a STOP command to the child process and closes stdin.
    pub fn send_stop(&self) -> Result<(), BrokerError> {
        let stop_msg = StdioMessage::Stop;
        let mut json =
            serde_json::to_string(&stop_msg).map_err(|e| BrokerError::Internal(e.to_string()))?;
        json.push('\n');

        let mut stdin_lock = self.stdin.lock().unwrap();
        if let Some(ref mut stdin) = *stdin_lock {
            let _ = stdin.write_all(json.as_bytes());
            let _ = stdin.flush();
        }
        // Drop stdin to signal EOF to child
        *stdin_lock = None;
        Ok(())
    }

    /// Reads initial READY message from lab startup.
    pub fn wait_for_ready(&self, timeout: Duration) -> Result<String, BrokerError> {
        let mut stdout_lock = self.stdout.lock().unwrap();
        if let Some(ref mut reader) = *stdout_lock {
            let start = std::time::Instant::now();
            let mut line = String::new();
            while start.elapsed() < timeout {
                line.clear();
                match reader.read_line(&mut line) {
                    Ok(0) => {
                        return Err(BrokerError::LabError(
                            "Child closed stdout before READY".to_string(),
                        ))
                    }
                    Ok(_) => {
                        let trimmed = line.trim();
                        if trimmed.is_empty() {
                            continue;
                        }
                        if let Ok(StdioMessage::Ready { lab_id, version: _ }) =
                            serde_json::from_str(trimmed)
                        {
                            return Ok(lab_id);
                        }
                    }
                    Err(e) => return Err(BrokerError::LabError(e.to_string())),
                }
            }
            Err(BrokerError::LabTimeout)
        } else {
            Err(BrokerError::LabNotRunning("unknown".to_string()))
        }
    }
}

impl LabRequestHandler for StdioLabChannel {
    fn handle_request(
        &self,
        lab_id: &str,
        req: &ParsedRequest,
    ) -> Result<HttpResponse, BrokerError> {
        let req_id = self.counter.fetch_add(1, Ordering::SeqCst);

        let subpath = if req.subpath.is_empty() {
            "/".to_string()
        } else {
            req.subpath.clone()
        };

        let msg = StdioMessage::HttpRequest {
            id: req_id,
            method: req.method.clone(),
            path: subpath,
            headers: req.headers.clone(),
            body: String::from_utf8_lossy(&req.body).to_string(),
        };

        let mut json =
            serde_json::to_string(&msg).map_err(|e| BrokerError::Internal(e.to_string()))?;
        json.push('\n');

        // 1. Write to stdin
        {
            let mut stdin_lock = self.stdin.lock().unwrap();
            let stdin = stdin_lock
                .as_mut()
                .ok_or_else(|| BrokerError::LabNotRunning(lab_id.to_string()))?;

            stdin.write_all(json.as_bytes()).map_err(|e| {
                BrokerError::LabError(format!("Failed to write to lab stdin: {}", e))
            })?;
            stdin
                .flush()
                .map_err(|e| BrokerError::LabError(format!("Failed to flush lab stdin: {}", e)))?;
        }

        // 2. Read response from stdout
        {
            let mut stdout_lock = self.stdout.lock().unwrap();
            let reader = stdout_lock
                .as_mut()
                .ok_or_else(|| BrokerError::LabNotRunning(lab_id.to_string()))?;

            let mut line = String::new();
            let mut attempts = 0;
            while attempts < 10 {
                line.clear();
                match reader.read_line(&mut line) {
                    Ok(0) => {
                        return Err(BrokerError::LabError(
                            "Child closed stdout unexpectedly".to_string(),
                        ))
                    }
                    Ok(_) => {
                        let trimmed = line.trim();
                        if trimmed.is_empty() {
                            attempts += 1;
                            continue;
                        }
                        match serde_json::from_str::<StdioMessage>(trimmed) {
                            Ok(StdioMessage::HttpResponse {
                                id,
                                status,
                                headers,
                                body,
                            }) => {
                                if id == req_id {
                                    let mut resp =
                                        HttpResponse::new(status, "OK", body.into_bytes());
                                    for (k, v) in headers {
                                        resp.headers.push((k, v));
                                    }
                                    return Ok(resp);
                                }
                            }
                            Ok(_) => {
                                // Ignore non-matching message (e.g. heartbeat or status)
                                attempts += 1;
                            }
                            Err(_) => {
                                attempts += 1;
                            }
                        }
                    }
                    Err(e) => {
                        return Err(BrokerError::LabError(format!(
                            "Failed to read stdout: {}",
                            e
                        )))
                    }
                }
            }
            Err(BrokerError::LabTimeout)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use std::net::TcpStream;
    use std::path::PathBuf;
    use std::time::Duration;

    fn send_broker_http(port: u16, raw_req: &str) -> String {
        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port))
            .expect("Failed to connect to broker on 127.0.0.1");
        stream
            .set_read_timeout(Some(Duration::from_secs(4)))
            .unwrap();
        stream
            .set_write_timeout(Some(Duration::from_secs(4)))
            .unwrap();

        stream.write_all(raw_req.as_bytes()).unwrap();
        stream.flush().unwrap();

        let mut resp = Vec::new();
        let _ = stream.read_to_end(&mut resp);
        String::from_utf8_lossy(&resp).to_string()
    }

    #[test]
    #[cfg(windows)]
    fn test_sandboxed_mock_lab_broker_e2e() {
        use crate::broker::server::BrokerServer;
        use crate::broker::session::BrokerSessionManager;
        use crate::native_runtime::job::{JobLimits, JobObject};
        use crate::native_runtime::process::{spawn_sandboxed_process, SandboxedProcessConfig};
        use crate::native_runtime::profile::AppContainerProfile;
        use crate::native_runtime::LabIdentity;

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

        if !probe_exe.exists() {
            eprintln!("Skipping test: zitera-engine.exe not yet built");
            return;
        }

        let identity = LabIdentity::new("MOCK_BROKER_LAB").unwrap();
        let _ = AppContainerProfile::create_or_open(&identity).unwrap();

        let job = JobObject::create(Some("MOCK_BROKER_JOB")).unwrap();
        let limits = JobLimits {
            kill_on_job_close: true,
            active_process_limit: Some(10),
            ..Default::default()
        };
        job.set_limits(&limits).unwrap();

        let config = SandboxedProcessConfig {
            executable: probe_exe,
            arguments: vec!["sandbox-probe".to_string(), "--mock-lab".to_string()],
            working_dir: repo_root,
            environment: std::collections::HashMap::new(),
        };

        // 1. Spawn Sandboxed Lab Process inside AppContainer + Job Object
        let mut handle = spawn_sandboxed_process(&identity, &config, Some(&job))
            .expect("spawn_sandboxed_process must succeed");

        assert!(handle.pid > 0, "Spawned lab must have valid PID");

        // 2. Extract stdio file streams and wrap in StdioLabChannel
        let (stdin_opt, stdout_opt) = handle.take_io();
        assert!(stdin_opt.is_some(), "Must have captured stdin");
        assert!(stdout_opt.is_some(), "Must have captured stdout");

        let channel = Arc::new(StdioLabChannel::new(
            stdin_opt.unwrap(),
            stdout_opt.unwrap(),
        ));

        // 3. Verify Lab Startup READY handshake over stdio
        let lab_id = channel
            .wait_for_ready(Duration::from_secs(5))
            .expect("Lab must report READY");
        assert_eq!(lab_id, "MOCK_LAB");

        // 4. Start Broker Server connected to the sandboxed stdio channel
        let session_mgr = Arc::new(BrokerSessionManager::new(Duration::from_secs(60)));
        let session = session_mgr.create_session("MOCK_LAB", None);

        let mut broker = BrokerServer::start(
            Arc::clone(&session_mgr),
            Arc::clone(&channel) as Arc<dyn LabRequestHandler>,
        )
        .expect("BrokerServer::start must succeed");
        let port = broker.port();

        // 5. Test GET / -> "ZITERA MOCK LAB"
        let req_root = format!(
            "GET /session/{}/ HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
            session.session_id, port
        );
        let resp_root = send_broker_http(port, &req_root);
        assert!(
            resp_root.starts_with("HTTP/1.1 200 OK\r\n"),
            "Got: {}",
            resp_root
        );
        assert!(resp_root.contains("ZITERA MOCK LAB"));
        assert!(resp_root.contains("X-Content-Type-Options: nosniff"));

        // 6. Test GET /status -> JSON status
        let req_status = format!(
            "GET /session/{}/status HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
            session.session_id, port
        );
        let resp_status = send_broker_http(port, &req_status);
        assert!(
            resp_status.starts_with("HTTP/1.1 200 OK\r\n"),
            "Got: {}",
            resp_status
        );
        assert!(resp_status.contains("appcontainer_sandboxed"));
        assert!(resp_status.contains("MOCK_LAB"));

        // 7. Test POST /echo -> Controlled payload reflection
        let echo_payload = "TEST_PAYLOAD_ECHO_123";
        let req_echo =
            format!(
            "POST /session/{}/echo HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nContent-Length: {}\r\n\r\n{}",
            session.session_id, port, echo_payload.len(), echo_payload
        );
        let resp_echo = send_broker_http(port, &req_echo);
        assert!(
            resp_echo.starts_with("HTTP/1.1 200 OK\r\n"),
            "Got: {}",
            resp_echo
        );
        assert!(resp_echo.contains(echo_payload));

        // 8. Prove Security Boundary: Browser cannot SSRF external targets through broker
        let req_ssrf = format!(
            "GET http://evil.example/session/{}/ HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
            session.session_id, port
        );
        let resp_ssrf = send_broker_http(port, &req_ssrf);
        assert!(resp_ssrf.starts_with("HTTP/1.1 400 Bad Request"));

        // 9. Send STOP command and verify clean exit
        channel.send_stop().expect("send_stop must succeed");
        let output = handle.wait().expect("handle.wait must succeed");
        assert_eq!(
            output.exit_code, 0,
            "Mock lab must exit with code 0 on STOP"
        );

        // 10. Invalidate session and verify broker rejects subsequent requests
        session_mgr.invalidate_session(&session.session_id);
        let resp_after_stop = send_broker_http(port, &req_root);
        assert!(resp_after_stop.starts_with("HTTP/1.1 410 Gone"));

        broker.stop();
        let _ = AppContainerProfile::delete(&identity);
    }

    #[test]
    #[cfg(windows)]
    fn test_mock_lab_network_isolation_and_broker_confinement() {
        use crate::broker::server::BrokerServer;
        use crate::broker::session::BrokerSessionManager;
        use crate::native_runtime::process::{run_sandboxed, SandboxedProcessConfig};
        use crate::native_runtime::profile::AppContainerProfile;
        use crate::native_runtime::LabIdentity;

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

        if !probe_exe.exists() {
            return;
        }

        let identity = LabIdentity::new("MOCK_NET_TEST").unwrap();
        let _ = AppContainerProfile::create_or_open(&identity).unwrap();

        // 1. Prove Mock Lab CANNOT access external Internet (1.1.1.1:80)
        let config_internet = SandboxedProcessConfig {
            executable: probe_exe.clone(),
            arguments: vec![
                "sandbox-probe".to_string(),
                "--connect-network".to_string(),
                "1.1.1.1".to_string(),
                "80".to_string(),
            ],
            working_dir: repo_root.clone(),
            environment: std::collections::HashMap::new(),
        };
        let out_internet = run_sandboxed(&identity, &config_internet).unwrap();
        assert_eq!(
            out_internet.exit_code, 2,
            "Sandboxed lab outbound internet must be blocked"
        );
        assert!(out_internet.stdout.contains("NETWORK_BLOCKED"));

        // 2. Prove Mock Lab CANNOT access private LAN (192.168.1.1:80)
        let config_lan = SandboxedProcessConfig {
            executable: probe_exe,
            arguments: vec![
                "sandbox-probe".to_string(),
                "--connect-network".to_string(),
                "192.168.1.1".to_string(),
                "80".to_string(),
            ],
            working_dir: repo_root,
            environment: std::collections::HashMap::new(),
        };
        let out_lan = run_sandboxed(&identity, &config_lan).unwrap();
        assert_eq!(
            out_lan.exit_code, 2,
            "Sandboxed lab outbound LAN must be blocked"
        );
        assert!(out_lan.stdout.contains("NETWORK_BLOCKED"));

        // 3. Prove Browser CANNOT reach arbitrary network target through broker
        let session_mgr = Arc::new(BrokerSessionManager::new(Duration::from_secs(60)));
        let session = session_mgr.create_session("MOCK_LAB", None);
        struct DummyHandler;
        impl LabRequestHandler for DummyHandler {
            fn handle_request(
                &self,
                _id: &str,
                _r: &ParsedRequest,
            ) -> Result<HttpResponse, BrokerError> {
                Ok(HttpResponse::text(200, "OK", "OK"))
            }
        }
        let mut broker =
            BrokerServer::start(Arc::clone(&session_mgr), Arc::new(DummyHandler)).unwrap();
        let port = broker.port();

        // Target attempts
        let targets = [
            format!(
                "GET http://1.1.1.1/session/{}/ HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
                session.session_id, port
            ),
            format!(
                "GET http://192.168.1.1/session/{}/ HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
                session.session_id, port
            ),
            format!(
                "GET http://localhost:8080/session/{}/ HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
                session.session_id, port
            ),
            format!(
                "GET ftp://127.0.0.1/session/{}/ HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
                session.session_id, port
            ),
        ];

        for t in targets {
            let resp = send_broker_http(port, &t);
            assert!(
                resp.starts_with("HTTP/1.1 400 Bad Request"),
                "Must reject foreign target: {}",
                t
            );
        }

        broker.stop();
        let _ = AppContainerProfile::delete(&identity);
    }
}
