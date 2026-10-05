use std::collections::HashMap;

pub const MAX_HEADERS_SIZE: usize = 16 * 1024; // 16 KB
pub const MAX_BODY_SIZE: usize = 2 * 1024 * 1024; // 2 MB
pub const MAX_RESPONSE_SIZE: usize = 10 * 1024 * 1024; // 10 MB
pub const MAX_QUERY_LEN: usize = 2048;

#[derive(Debug, PartialEq, Eq)]
pub enum HttpParseError {
    Incomplete,
    HeadersTooLarge,
    PayloadTooLarge,
    InvalidRequestLine,
    MethodNotAllowed(String),
    InvalidUri(String),
    PathTraversal(String),
    InvalidRoute(String),
    InvalidHost(String),
    MissingHost,
    InvalidHeader,
}

#[derive(Debug, Clone)]
pub struct ParsedRequest {
    pub method: String,
    pub full_path: String,
    pub session_token: String,
    pub subpath: String,
    pub query: Option<String>,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

pub struct HttpResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn new(status_code: u16, status_text: &str, body: Vec<u8>) -> Self {
        Self {
            status_code,
            status_text: status_text.to_string(),
            headers: Vec::new(),
            body,
        }
    }

    pub fn text(status_code: u16, status_text: &str, msg: &str) -> Self {
        let mut resp = Self::new(status_code, status_text, msg.as_bytes().to_vec());
        resp.headers.push((
            "Content-Type".to_string(),
            "text/plain; charset=utf-8".to_string(),
        ));
        resp
    }

    pub fn json(status_code: u16, status_text: &str, json_str: &str) -> Self {
        let mut resp = Self::new(status_code, status_text, json_str.as_bytes().to_vec());
        resp.headers.push((
            "Content-Type".to_string(),
            "application/json; charset=utf-8".to_string(),
        ));
        resp
    }

    /// Serializes the response to standard HTTP/1.1 wire format with security headers.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend(format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text).as_bytes());

        // Default security & transport headers
        out.extend(b"Connection: close\r\n");
        out.extend(b"X-Content-Type-Options: nosniff\r\n");
        out.extend(b"X-Frame-Options: SAMEORIGIN\r\n");
        out.extend(b"Cache-Control: no-store\r\n");
        out.extend(format!("Content-Length: {}\r\n", self.body.len()).as_bytes());

        for (k, v) in &self.headers {
            let k_lower = k.to_lowercase();
            // Filter out hop-by-hop headers
            if k_lower != "connection"
                && k_lower != "content-length"
                && k_lower != "transfer-encoding"
                && k_lower != "upgrade"
            {
                out.extend(format!("{}: {}\r\n", k, v).as_bytes());
            }
        }

        out.extend(b"\r\n");
        out.extend(&self.body);
        out
    }
}

/// Parses and validates an incoming raw HTTP request buffer.
pub fn parse_http_request(raw_bytes: &[u8]) -> Result<ParsedRequest, HttpParseError> {
    // 1. Locate headers boundary (\r\n\r\n)
    let header_end = find_header_boundary(raw_bytes)?;
    if header_end > MAX_HEADERS_SIZE {
        return Err(HttpParseError::HeadersTooLarge);
    }

    let header_str = match std::str::from_utf8(&raw_bytes[..header_end]) {
        Ok(s) => s,
        Err(_) => return Err(HttpParseError::InvalidHeader),
    };

    let mut lines = header_str.split("\r\n");
    let request_line = lines.next().ok_or(HttpParseError::InvalidRequestLine)?;

    // 2. Parse Request Line
    let req_parts: Vec<&str> = request_line.split_whitespace().collect();
    if req_parts.len() < 3 {
        return Err(HttpParseError::InvalidRequestLine);
    }

    let method = req_parts[0].to_uppercase();
    let uri = req_parts[1];

    // Method Validation (Section 7)
    match method.as_str() {
        "GET" | "POST" | "HEAD" | "OPTIONS" => {}
        other => return Err(HttpParseError::MethodNotAllowed(other.to_string())),
    }

    // URI & SSRF Protection: Deny absolute URIs or foreign schemes (Section 7, Section 9)
    if !uri.starts_with('/') {
        return Err(HttpParseError::InvalidUri(uri.to_string()));
    }
    if uri.contains("://") || uri.starts_with("//") {
        return Err(HttpParseError::InvalidUri(uri.to_string()));
    }

    // Split path and query
    let (raw_path, raw_query) = match uri.split_once('?') {
        Some((p, q)) => {
            if q.len() > MAX_QUERY_LEN {
                return Err(HttpParseError::InvalidUri(
                    "Query string exceeds maximum length".to_string(),
                ));
            }
            (p, Some(q.to_string()))
        }
        None => (uri, None),
    };

    // Path traversal check
    if raw_path.contains("..")
        || raw_path.contains('\\')
        || raw_path.to_lowercase().contains("%2e%2e")
        || raw_path.to_lowercase().contains("%2f")
        || raw_path.to_lowercase().contains("%5c")
    {
        return Err(HttpParseError::PathTraversal(raw_path.to_string()));
    }

    // 3. Session routing extraction: must match /session/<token>[/<subpath>]
    // Minimum: /session/<32-char-token>
    let parts: Vec<&str> = raw_path.split('/').filter(|s| !s.is_empty()).collect();
    if parts.is_empty() || parts[0] != "session" || parts.len() < 2 {
        return Err(HttpParseError::InvalidRoute(raw_path.to_string()));
    }

    let session_token = parts[1].to_string();
    if session_token.len() != 32 || !session_token.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(HttpParseError::InvalidRoute(
            "Invalid session token format".to_string(),
        ));
    }

    // Build subpath (e.g. "/" or "/api/users")
    let subpath = if parts.len() == 2 {
        if raw_path.ends_with('/') {
            "/".to_string()
        } else {
            "".to_string()
        }
    } else {
        format!("/{}", parts[2..].join("/"))
    };

    // 4. Parse Headers
    let mut headers = HashMap::new();
    let mut content_length: usize = 0;

    for line in lines {
        if line.is_empty() {
            continue;
        }
        if let Some((name, val)) = line.split_once(':') {
            let key = name.trim().to_lowercase();
            let value = val.trim().to_string();

            if key == "content-length" {
                content_length = value
                    .parse::<usize>()
                    .map_err(|_| HttpParseError::InvalidHeader)?;
                if content_length > MAX_BODY_SIZE {
                    return Err(HttpParseError::PayloadTooLarge);
                }
            }
            headers.insert(key, value);
        } else {
            return Err(HttpParseError::InvalidHeader);
        }
    }

    // Host Header Validation (Section 9)
    let host = headers.get("host").ok_or(HttpParseError::MissingHost)?;
    let host_domain = host.split(':').next().unwrap_or("");
    if host_domain != "127.0.0.1" && host_domain != "localhost" {
        return Err(HttpParseError::InvalidHost(host.to_string()));
    }

    // 5. Body Extraction
    let body_start = header_end + 4; // after \r\n\r\n
    let total_available = raw_bytes.len().saturating_sub(body_start);

    if total_available < content_length {
        return Err(HttpParseError::Incomplete);
    }

    let body = raw_bytes[body_start..body_start + content_length].to_vec();

    Ok(ParsedRequest {
        method,
        full_path: raw_path.to_string(),
        session_token,
        subpath,
        query: raw_query,
        headers,
        body,
    })
}

fn find_header_boundary(raw: &[u8]) -> Result<usize, HttpParseError> {
    for i in 0..raw.len().saturating_sub(3) {
        if raw[i] == b'\r' && raw[i + 1] == b'\n' && raw[i + 2] == b'\r' && raw[i + 3] == b'\n' {
            return Ok(i);
        }
    }
    if raw.len() > MAX_HEADERS_SIZE {
        Err(HttpParseError::HeadersTooLarge)
    } else {
        Err(HttpParseError::Incomplete)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_TOKEN: &str = "0123456789abcdef0123456789abcdef";

    #[test]
    fn test_valid_get_request() {
        let req = format!(
            "GET /session/{}/profile HTTP/1.1\r\nHost: 127.0.0.1:8080\r\nUser-Agent: test\r\n\r\n",
            VALID_TOKEN
        );
        let parsed = parse_http_request(req.as_bytes()).unwrap();
        assert_eq!(parsed.method, "GET");
        assert_eq!(parsed.session_token, VALID_TOKEN);
        assert_eq!(parsed.subpath, "/profile");
        assert_eq!(parsed.headers.get("host").unwrap(), "127.0.0.1:8080");
        assert!(parsed.body.is_empty());
    }

    #[test]
    fn test_valid_post_with_body() {
        let body = b"{\"username\":\"admin\"}";
        let req = format!(
            "POST /session/{}/login HTTP/1.1\r\nHost: localhost:9000\r\nContent-Length: {}\r\nContent-Type: application/json\r\n\r\n",
            VALID_TOKEN,
            body.len()
        );
        let mut full_bytes = req.into_bytes();
        full_bytes.extend(body);

        let parsed = parse_http_request(&full_bytes).unwrap();
        assert_eq!(parsed.method, "POST");
        assert_eq!(parsed.session_token, VALID_TOKEN);
        assert_eq!(parsed.subpath, "/login");
        assert_eq!(parsed.body, body);
    }

    #[test]
    fn test_method_not_allowed() {
        let req = format!(
            "CONNECT /session/{}/proxy HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n",
            VALID_TOKEN
        );
        let err = parse_http_request(req.as_bytes()).unwrap_err();
        assert_eq!(err, HttpParseError::MethodNotAllowed("CONNECT".to_string()));

        let req_trace = format!(
            "TRACE /session/{}/test HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n",
            VALID_TOKEN
        );
        assert_eq!(
            parse_http_request(req_trace.as_bytes()).unwrap_err(),
            HttpParseError::MethodNotAllowed("TRACE".to_string())
        );
    }

    #[test]
    fn test_rejection_of_absolute_uri_and_ssrf() {
        let req1 = "GET http://evil.example/session/123 HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n";
        assert!(matches!(
            parse_http_request(req1.as_bytes()),
            Err(HttpParseError::InvalidUri(_))
        ));

        let req2 = "GET //127.0.0.1:9090/session/123 HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n";
        assert!(matches!(
            parse_http_request(req2.as_bytes()),
            Err(HttpParseError::InvalidUri(_))
        ));
    }

    #[test]
    fn test_path_traversal_rejection() {
        let cases = [
            format!(
                "GET /session/{}/../secret HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n",
                VALID_TOKEN
            ),
            format!(
                "GET /session/{}/a/../../secret HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n",
                VALID_TOKEN
            ),
            format!(
                "GET /session/{}/..\\secret HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n",
                VALID_TOKEN
            ),
            format!(
                "GET /session/{}/%2e%2e/secret HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n",
                VALID_TOKEN
            ),
        ];

        for c in cases {
            assert!(
                matches!(
                    parse_http_request(c.as_bytes()),
                    Err(HttpParseError::PathTraversal(_))
                ),
                "Failed to reject traversal: {}",
                c
            );
        }
    }

    #[test]
    fn test_host_header_enforcement() {
        // Missing host
        let req_no_host = format!("GET /session/{}/ HTTP/1.1\r\n\r\n", VALID_TOKEN);
        assert_eq!(
            parse_http_request(req_no_host.as_bytes()).unwrap_err(),
            HttpParseError::MissingHost
        );

        // Host header spoofing / SSRF attempt
        let req_bad_host = format!(
            "GET /session/{}/ HTTP/1.1\r\nHost: evil.example\r\n\r\n",
            VALID_TOKEN
        );
        assert_eq!(
            parse_http_request(req_bad_host.as_bytes()).unwrap_err(),
            HttpParseError::InvalidHost("evil.example".to_string())
        );
    }

    #[test]
    fn test_oversized_payload_rejection() {
        let huge_len = MAX_BODY_SIZE + 1024;
        let req = format!(
            "POST /session/{}/upload HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Length: {}\r\n\r\n",
            VALID_TOKEN, huge_len
        );
        assert_eq!(
            parse_http_request(req.as_bytes()).unwrap_err(),
            HttpParseError::PayloadTooLarge
        );
    }

    #[test]
    fn test_response_serialization_with_security_headers() {
        let resp = HttpResponse::text(200, "OK", "Hello Zitera");
        let bytes = resp.to_bytes();
        let s = String::from_utf8(bytes).unwrap();

        assert!(s.starts_with("HTTP/1.1 200 OK\r\n"));
        assert!(s.contains("Connection: close\r\n"));
        assert!(s.contains("X-Content-Type-Options: nosniff\r\n"));
        assert!(s.contains("X-Frame-Options: SAMEORIGIN\r\n"));
        assert!(s.contains("Cache-Control: no-store\r\n"));
        assert!(s.contains("Content-Length: 12\r\n"));
        assert!(s.ends_with("\r\n\r\nHello Zitera"));
    }
}
