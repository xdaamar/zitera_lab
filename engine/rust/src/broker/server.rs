use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use super::http::{
    parse_http_request, sanitize_lab_response_headers, HttpParseError, HttpResponse, ParsedRequest,
    MAX_BODY_SIZE, MAX_HEADERS_SIZE,
};
use super::session::{BrokerSessionManager, SessionError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrokerError {
    SessionNotFound,
    SessionTerminated,
    SessionExpired,
    InvalidToken,
    LabNotRunning(String),
    LabTimeout,
    LabError(String),
    Internal(String),
}

impl std::fmt::Display for BrokerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BrokerError::SessionNotFound => write!(f, "Session not found"),
            BrokerError::SessionTerminated => write!(f, "Lab session is terminated"),
            BrokerError::SessionExpired => write!(f, "Lab session has expired"),
            BrokerError::InvalidToken => write!(f, "Invalid session token"),
            BrokerError::LabNotRunning(id) => write!(f, "Lab {} is not running", id),
            BrokerError::LabTimeout => write!(f, "Lab response timed out"),
            BrokerError::LabError(e) => write!(f, "Lab error: {}", e),
            BrokerError::Internal(e) => write!(f, "Internal broker error: {}", e),
        }
    }
}

impl std::error::Error for BrokerError {}

/// Handler trait for lab request execution (e.g. stdio process or mock).
pub trait LabRequestHandler: Send + Sync {
    fn handle_request(
        &self,
        lab_id: &str,
        req: &ParsedRequest,
    ) -> Result<HttpResponse, BrokerError>;
}

/// A thread-safe, bounded localhost HTTP broker for lab sandboxes.
pub struct BrokerServer {
    port: u16,
    session_manager: Arc<BrokerSessionManager>,
    handler: Arc<dyn LabRequestHandler>,
    is_running: Arc<AtomicBool>,
    thread_handle: Option<std::thread::JoinHandle<()>>,
}

impl BrokerServer {
    /// Starts a new broker bound exclusively to `127.0.0.1` on a dynamic OS-assigned port.
    pub fn start(
        session_manager: Arc<BrokerSessionManager>,
        handler: Arc<dyn LabRequestHandler>,
    ) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let port = listener.local_addr()?.port();

        let is_running = Arc::new(AtomicBool::new(true));
        let is_running_clone = Arc::clone(&is_running);
        let session_mgr_clone = Arc::clone(&session_manager);
        let handler_clone = Arc::clone(&handler);

        // Non-blocking accept loop with small timeout
        listener.set_nonblocking(true)?;

        let thread_handle = std::thread::spawn(move || {
            while is_running_clone.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((stream, _peer_addr)) => {
                        let sm = Arc::clone(&session_mgr_clone);
                        let h = Arc::clone(&handler_clone);
                        std::thread::spawn(move || {
                            handle_client_connection(stream, sm, h);
                        });
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(Duration::from_millis(10));
                    }
                    Err(_) => {
                        if !is_running_clone.load(Ordering::SeqCst) {
                            break;
                        }
                    }
                }
            }
        });

        Ok(Self {
            port,
            session_manager,
            handler,
            is_running,
            thread_handle: Some(thread_handle),
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn session_manager(&self) -> &Arc<BrokerSessionManager> {
        &self.session_manager
    }

    pub fn handler(&self) -> &Arc<dyn LabRequestHandler> {
        &self.handler
    }

    /// Computes the entrypoint URL for a lab session in the format:
    /// `http://127.0.0.1:<port>/session/<token>/`
    pub fn entry_url(&self, session_token: &str) -> String {
        format!("http://127.0.0.1:{}/session/{}/", self.port, session_token)
    }

    pub fn stop(&mut self) {
        if self.is_running.swap(false, Ordering::SeqCst) {
            if let Some(handle) = self.thread_handle.take() {
                let _ = handle.join();
            }
        }
    }
}

impl Drop for BrokerServer {
    fn drop(&mut self) {
        self.stop();
    }
}

fn handle_client_connection(
    mut stream: TcpStream,
    session_manager: Arc<BrokerSessionManager>,
    handler: Arc<dyn LabRequestHandler>,
) {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(5)));

    let max_read = MAX_HEADERS_SIZE + MAX_BODY_SIZE;
    let mut buffer = Vec::new();
    let mut temp = [0u8; 4096];

    loop {
        match stream.read(&mut temp) {
            Ok(0) => break,
            Ok(n) => {
                buffer.extend_from_slice(&temp[..n]);
                if buffer.len() > max_read {
                    let resp =
                        HttpResponse::text(413, "Payload Too Large", "Request size exceeds limit");
                    let _ = stream.write_all(&resp.to_bytes());
                    return;
                }
                // Try parsing or check if header complete
                match parse_http_request(&buffer) {
                    Ok(parsed) => {
                        // Successfully parsed full request
                        let resp = process_parsed_request(parsed, &session_manager, &handler);
                        let _ = stream.write_all(&resp.to_bytes());
                        let _ = stream.flush();
                        return;
                    }
                    Err(HttpParseError::Incomplete) => {
                        // Continue reading
                        continue;
                    }
                    Err(e) => {
                        let resp = map_parse_error_to_response(e);
                        let _ = stream.write_all(&resp.to_bytes());
                        let _ = stream.flush();
                        return;
                    }
                }
            }
            Err(e)
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut =>
            {
                let resp = HttpResponse::text(408, "Request Timeout", "Read timed out");
                let _ = stream.write_all(&resp.to_bytes());
                return;
            }
            Err(_) => return,
        }
    }

    // If stream closed before request completed
    if !buffer.is_empty() {
        if let Err(e) = parse_http_request(&buffer) {
            let resp = map_parse_error_to_response(e);
            let _ = stream.write_all(&resp.to_bytes());
        }
    }
}

fn process_parsed_request(
    parsed: ParsedRequest,
    session_manager: &BrokerSessionManager,
    handler: &Arc<dyn LabRequestHandler>,
) -> HttpResponse {
    // 1. Validate Session Token
    let session = match session_manager.validate_session(&parsed.session_token) {
        Ok(s) => s,
        Err(SessionError::InvalidToken) => {
            return HttpResponse::text(400, "Bad Request", "Malformed session token");
        }
        Err(SessionError::SessionNotFound) => {
            return HttpResponse::text(404, "Not Found", "Session not found");
        }
        Err(SessionError::Terminated) => {
            return HttpResponse::text(410, "Gone", "Lab session has been terminated");
        }
        Err(SessionError::Expired) => {
            return HttpResponse::text(401, "Unauthorized", "Lab session has expired");
        }
    };

    // 2. Dispatch to Lab Request Handler
    match handler.handle_request(&session.lab_id, &parsed) {
        Ok(mut resp) => {
            resp.headers = sanitize_lab_response_headers(resp.headers, &parsed.session_token);
            resp
        }
        Err(BrokerError::LabTimeout) => HttpResponse::text(
            504,
            "Gateway Timeout",
            "Lab process did not respond in time",
        ),
        Err(BrokerError::LabNotRunning(id)) => HttpResponse::text(
            503,
            "Service Unavailable",
            &format!("Lab {} is not running", id),
        ),
        Err(BrokerError::LabError(err)) => {
            HttpResponse::text(502, "Bad Gateway", &format!("Lab returned error: {}", err))
        }
        Err(e) => HttpResponse::text(
            500,
            "Internal Server Error",
            &format!("Broker error: {}", e),
        ),
    }
}

fn map_parse_error_to_response(err: HttpParseError) -> HttpResponse {
    match err {
        HttpParseError::Incomplete => {
            HttpResponse::text(400, "Bad Request", "Incomplete HTTP request")
        }
        HttpParseError::HeadersTooLarge => HttpResponse::text(
            431,
            "Request Header Fields Too Large",
            "Headers exceed 16KB limit",
        ),
        HttpParseError::PayloadTooLarge => {
            HttpResponse::text(413, "Payload Too Large", "Body exceeds 2MB limit")
        }
        HttpParseError::InvalidRequestLine => {
            HttpResponse::text(400, "Bad Request", "Malformed HTTP request line")
        }
        HttpParseError::MethodNotAllowed(m) => HttpResponse::text(
            405,
            "Method Not Allowed",
            &format!("Method '{}' not supported", m),
        ),
        HttpParseError::InvalidUri(u) => HttpResponse::text(
            400,
            "Bad Request",
            &format!("Invalid URI (forward proxying forbidden): {}", u),
        ),
        HttpParseError::PathTraversal(p) => HttpResponse::text(
            400,
            "Bad Request",
            &format!("Path traversal forbidden: {}", p),
        ),
        HttpParseError::InvalidRoute(r) => {
            HttpResponse::text(404, "Not Found", &format!("Route not found: {}", r))
        }
        HttpParseError::InvalidHost(h) => {
            HttpResponse::text(400, "Bad Request", &format!("Untrusted Host header: {}", h))
        }
        HttpParseError::MissingHost => HttpResponse::text(
            400,
            "Bad Request",
            "Missing Host header in HTTP/1.1 request",
        ),
        HttpParseError::InvalidHeader => {
            HttpResponse::text(400, "Bad Request", "Malformed HTTP header")
        }
    }
}
