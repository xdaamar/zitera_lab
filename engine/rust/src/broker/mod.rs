pub mod http;
pub mod server;
pub mod session;

pub use http::{
    parse_http_request, HttpParseError, HttpResponse, ParsedRequest, MAX_BODY_SIZE,
    MAX_HEADERS_SIZE, MAX_RESPONSE_SIZE,
};
pub use server::{BrokerError, BrokerServer, LabRequestHandler};
pub use session::{BrokerSession, BrokerSessionManager, SessionError};

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::sync::Arc;
    use std::time::Duration;

    struct TestLabHandler {
        mock_response: String,
    }

    impl LabRequestHandler for TestLabHandler {
        fn handle_request(
            &self,
            lab_id: &str,
            req: &ParsedRequest,
        ) -> Result<HttpResponse, BrokerError> {
            if lab_id == "TIMEOUT_LAB" {
                return Err(BrokerError::LabTimeout);
            }
            if lab_id == "ERROR_LAB" {
                return Err(BrokerError::LabError("Synthetic lab crash".to_string()));
            }

            let resp_body = format!(
                "LAB:{} METHOD:{} SUBPATH:{} BODY:{} MOCK:{}",
                lab_id,
                req.method,
                req.subpath,
                String::from_utf8_lossy(&req.body),
                self.mock_response
            );
            Ok(HttpResponse::text(200, "OK", &resp_body))
        }
    }

    fn setup_test_broker(default_ttl: Duration) -> (BrokerServer, Arc<BrokerSessionManager>) {
        let session_mgr = Arc::new(BrokerSessionManager::new(default_ttl));
        let handler = Arc::new(TestLabHandler {
            mock_response: "PASS".to_string(),
        });
        let server = BrokerServer::start(Arc::clone(&session_mgr), handler)
            .expect("BrokerServer::start must succeed on localhost");
        (server, session_mgr)
    }

    fn send_raw_request(port: u16, raw_req: &str) -> String {
        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port))
            .expect("Failed to connect to broker on 127.0.0.1");
        stream
            .set_read_timeout(Some(Duration::from_secs(3)))
            .unwrap();
        stream
            .set_write_timeout(Some(Duration::from_secs(3)))
            .unwrap();

        stream.write_all(raw_req.as_bytes()).unwrap();
        stream.flush().unwrap();

        let mut resp = Vec::new();
        let _ = stream.read_to_end(&mut resp);
        String::from_utf8_lossy(&resp).to_string()
    }

    #[test]
    fn test_broker_localhost_binding_and_ephemeral_port() {
        let (server, _mgr) = setup_test_broker(Duration::from_secs(60));
        assert!(server.port() > 1024, "Port must be dynamic ephemeral port");

        // Verify connection to 127.0.0.1 succeeds
        let stream = TcpStream::connect(format!("127.0.0.1:{}", server.port()));
        assert!(stream.is_ok(), "Must connect to 127.0.0.1");
    }

    #[test]
    fn test_broker_valid_request_routing_and_security_headers() {
        let (server, session_mgr) = setup_test_broker(Duration::from_secs(60));
        let session = session_mgr.create_session("A01", None);

        let req = format!(
            "GET /session/{}/profile HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
            session.session_id,
            server.port()
        );

        let resp = send_raw_request(server.port(), &req);

        assert!(
            resp.starts_with("HTTP/1.1 200 OK\r\n"),
            "Expected 200 OK, got: {}",
            resp
        );
        assert!(resp.contains("X-Content-Type-Options: nosniff"));
        assert!(resp.contains("X-Frame-Options: SAMEORIGIN"));
        assert!(resp.contains("Cache-Control: no-store"));
        assert!(resp.contains("LAB:A01 METHOD:GET SUBPATH:/profile"));
    }

    #[test]
    fn test_broker_rejects_ssrf_and_open_proxy() {
        let (server, session_mgr) = setup_test_broker(Duration::from_secs(60));
        let session = session_mgr.create_session("A01", None);

        // 1. Foreign URL scheme
        let req1 = format!(
            "GET http://evil.example/session/{}/ HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
            session.session_id,
            server.port()
        );
        let resp1 = send_raw_request(server.port(), &req1);
        assert!(resp1.starts_with("HTTP/1.1 400 Bad Request"));

        // 2. Arbitrary port forward proxy attempt
        let req2 = format!(
            "GET http://127.0.0.1:1337/session/{}/ HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
            session.session_id,
            server.port()
        );
        let resp2 = send_raw_request(server.port(), &req2);
        assert!(resp2.starts_with("HTTP/1.1 400 Bad Request"));

        // 3. Foreign protocol
        let req3 = format!(
            "GET ftp://127.0.0.1/session/{}/ HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
            session.session_id,
            server.port()
        );
        let resp3 = send_raw_request(server.port(), &req3);
        assert!(resp3.starts_with("HTTP/1.1 400 Bad Request"));
    }

    #[test]
    fn test_broker_rejects_path_traversal() {
        let (server, session_mgr) = setup_test_broker(Duration::from_secs(60));
        let session = session_mgr.create_session("A01", None);

        let traversals = [
            format!(
                "GET /session/{}/../secret HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
                session.session_id,
                server.port()
            ),
            format!(
                "GET /session/{}/..\\secret HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
                session.session_id,
                server.port()
            ),
            format!(
                "GET /session/{}/%2e%2e/secret HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
                session.session_id,
                server.port()
            ),
            format!(
                "GET /../../ HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
                server.port()
            ),
        ];

        for req in traversals {
            let resp = send_raw_request(server.port(), &req);
            assert!(
                resp.starts_with("HTTP/1.1 400 Bad Request")
                    || resp.starts_with("HTTP/1.1 404 Not Found"),
                "Traversal request must be denied with 400 or 404. Got: {}",
                resp
            );
        }
    }

    #[test]
    fn test_broker_session_security_lifecycle() {
        let (server, session_mgr) = setup_test_broker(Duration::from_secs(60));
        let session = session_mgr.create_session("A01", None);

        // 1. Wrong / unassigned token
        let wrong_token = "0123456789abcdef0123456789abcdef";
        let req_wrong = format!(
            "GET /session/{}/ HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
            wrong_token,
            server.port()
        );
        let resp_wrong = send_raw_request(server.port(), &req_wrong);
        assert!(resp_wrong.starts_with("HTTP/1.1 404 Not Found"));

        // 2. Terminated session
        session_mgr.invalidate_session(&session.session_id);
        let req_term = format!(
            "GET /session/{}/ HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
            session.session_id,
            server.port()
        );
        let resp_term = send_raw_request(server.port(), &req_term);
        assert!(resp_term.starts_with("HTTP/1.1 410 Gone"));

        // 3. Expired session
        let expired_session = session_mgr.create_session("A06", Some(Duration::from_millis(5)));
        std::thread::sleep(Duration::from_millis(15));
        let req_exp = format!(
            "GET /session/{}/ HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
            expired_session.session_id,
            server.port()
        );
        let resp_exp = send_raw_request(server.port(), &req_exp);
        assert!(resp_exp.starts_with("HTTP/1.1 401 Unauthorized"));
    }

    #[test]
    fn test_broker_host_header_validation() {
        let (server, session_mgr) = setup_test_broker(Duration::from_secs(60));
        let session = session_mgr.create_session("A01", None);

        // Spoofed Host header
        let req_bad_host = format!(
            "GET /session/{}/ HTTP/1.1\r\nHost: evil.attacker.com\r\n\r\n",
            session.session_id
        );
        let resp = send_raw_request(server.port(), &req_bad_host);
        assert!(resp.starts_with("HTTP/1.1 400 Bad Request"));

        // LAN Host header (must be localhost only)
        let req_lan_host = format!(
            "GET /session/{}/ HTTP/1.1\r\nHost: 192.168.1.50\r\n\r\n",
            session.session_id
        );
        let resp_lan = send_raw_request(server.port(), &req_lan_host);
        assert!(resp_lan.starts_with("HTTP/1.1 400 Bad Request"));
    }

    #[test]
    fn test_broker_method_filtering() {
        let (server, session_mgr) = setup_test_broker(Duration::from_secs(60));
        let session = session_mgr.create_session("A01", None);

        // CONNECT
        let req_connect = format!(
            "CONNECT /session/{}/ HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
            session.session_id,
            server.port()
        );
        let resp_connect = send_raw_request(server.port(), &req_connect);
        assert!(resp_connect.starts_with("HTTP/1.1 405 Method Not Allowed"));

        // TRACE
        let req_trace = format!(
            "TRACE /session/{}/ HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
            session.session_id,
            server.port()
        );
        let resp_trace = send_raw_request(server.port(), &req_trace);
        assert!(resp_trace.starts_with("HTTP/1.1 405 Method Not Allowed"));
    }

    #[test]
    fn test_broker_lab_error_and_timeout_recovery() {
        let (server, session_mgr) = setup_test_broker(Duration::from_secs(60));

        let s_timeout = session_mgr.create_session("TIMEOUT_LAB", None);
        let req_timeout = format!(
            "GET /session/{}/ HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
            s_timeout.session_id,
            server.port()
        );
        let resp_to = send_raw_request(server.port(), &req_timeout);
        assert!(resp_to.starts_with("HTTP/1.1 504 Gateway Timeout"));

        let s_err = session_mgr.create_session("ERROR_LAB", None);
        let req_err = format!(
            "GET /session/{}/ HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
            s_err.session_id,
            server.port()
        );
        let resp_e = send_raw_request(server.port(), &req_err);
        assert!(resp_e.starts_with("HTTP/1.1 502 Bad Gateway"));

        // Verify broker recovers and normal request still works
        let s_ok = session_mgr.create_session("A01", None);
        let req_ok = format!(
            "GET /session/{}/check HTTP/1.1\r\nHost: 127.0.0.1:{}\r\n\r\n",
            s_ok.session_id,
            server.port()
        );
        let resp_ok = send_raw_request(server.port(), &req_ok);
        assert!(resp_ok.starts_with("HTTP/1.1 200 OK"));
    }

    #[test]
    fn test_broker_rapid_concurrent_requests() {
        let (server, session_mgr) = setup_test_broker(Duration::from_secs(60));
        let session = session_mgr.create_session("A01", None);

        let mut handles = Vec::new();
        for i in 0..15 {
            let port = server.port();
            let token = session.session_id.clone();
            handles.push(std::thread::spawn(move || {
                let req = format!(
                    "POST /session/{}/item/{} HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nContent-Length: 4\r\n\r\ntest",
                    token, i, port
                );
                send_raw_request(port, &req)
            }));
        }

        for h in handles {
            let resp = h.join().unwrap();
            assert!(
                resp.starts_with("HTTP/1.1 200 OK"),
                "Concurrent request failed: {}",
                resp
            );
        }
    }
}
