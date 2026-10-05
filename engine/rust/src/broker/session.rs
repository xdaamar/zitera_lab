use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};

/// Errors relating to broker session validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionError {
    InvalidToken,
    Terminated,
    Expired,
    SessionNotFound,
}

impl std::fmt::Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionError::InvalidToken => write!(f, "Invalid session token format"),
            SessionError::Terminated => write!(f, "Session has been terminated"),
            SessionError::Expired => write!(f, "Session has expired"),
            SessionError::SessionNotFound => write!(f, "Session not found"),
        }
    }
}

impl std::error::Error for SessionError {}

/// Secure random byte generator using Windows BCrypt API on Windows
/// or /dev/urandom on Unix platforms.
#[cfg(windows)]
fn generate_random_bytes(buf: &mut [u8]) -> Result<(), String> {
    use windows_sys::Win32::Security::Cryptography::{
        BCryptGenRandom, BCRYPT_USE_SYSTEM_PREFERRED_RNG,
    };
    let status = unsafe {
        BCryptGenRandom(
            std::ptr::null_mut(),
            buf.as_mut_ptr(),
            buf.len() as u32,
            BCRYPT_USE_SYSTEM_PREFERRED_RNG,
        )
    };
    if status == 0 {
        Ok(())
    } else {
        Err(format!(
            "BCryptGenRandom failed with NTSTATUS 0x{:08X}",
            status
        ))
    }
}

#[cfg(not(windows))]
fn generate_random_bytes(buf: &mut [u8]) -> Result<(), String> {
    use std::fs::File;
    use std::io::Read;
    if let Ok(mut f) = File::open("/dev/urandom") {
        if f.read_exact(buf).is_ok() {
            return Ok(());
        }
    }
    let mut seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    for byte in buf.iter_mut() {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        *byte = (seed >> 24) as u8;
    }
    Ok(())
}

/// Generates a cryptographically random 32-character hexadecimal token (128 bits).
pub fn generate_session_token() -> String {
    let mut bytes = [0u8; 16];
    generate_random_bytes(&mut bytes).expect("RNG generation must succeed");
    let mut s = String::with_capacity(32);
    for b in bytes {
        use std::fmt::Write;
        let _ = write!(s, "{:02x}", b);
    }
    s
}

/// Active lab session descriptor.
#[derive(Debug, Clone)]
pub struct BrokerSession {
    pub session_id: String,
    pub lab_id: String,
    pub created_at: Instant,
    pub expires_at: Instant,
    pub is_active: bool,
}

/// Thread-safe manager for active lab broker sessions.
pub struct BrokerSessionManager {
    sessions: RwLock<HashMap<String, BrokerSession>>,
    default_ttl: Duration,
}

impl BrokerSessionManager {
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            default_ttl,
        }
    }

    /// Creates and registers a new cryptographically opaque session for `lab_id`.
    pub fn create_session(&self, lab_id: &str, ttl: Option<Duration>) -> BrokerSession {
        let token = generate_session_token();
        let now = Instant::now();
        let duration = ttl.unwrap_or(self.default_ttl);
        let session = BrokerSession {
            session_id: token.clone(),
            lab_id: lab_id.to_string(),
            created_at: now,
            expires_at: now + duration,
            is_active: true,
        };

        let mut lock = self.sessions.write().unwrap();
        lock.insert(token, session.clone());
        session
    }

    /// Validates a session token: verifies existence, active status, and expiration.
    pub fn validate_session(&self, token: &str) -> Result<BrokerSession, SessionError> {
        if token.len() != 32 || !token.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(SessionError::InvalidToken);
        }

        let lock = self.sessions.read().unwrap();
        let session = lock.get(token).ok_or(SessionError::SessionNotFound)?;

        if !session.is_active {
            return Err(SessionError::Terminated);
        }

        if Instant::now() > session.expires_at {
            return Err(SessionError::Expired);
        }

        Ok(session.clone())
    }

    /// Invalidates a specific session token immediately.
    pub fn invalidate_session(&self, token: &str) {
        let mut lock = self.sessions.write().unwrap();
        if let Some(session) = lock.get_mut(token) {
            session.is_active = false;
        }
    }

    /// Invalidates all sessions associated with `lab_id` (e.g. when lab stops).
    pub fn invalidate_lab(&self, lab_id: &str) {
        let mut lock = self.sessions.write().unwrap();
        for session in lock.values_mut() {
            if session.lab_id == lab_id {
                session.is_active = false;
            }
        }
    }

    /// Clears and invalidates all active sessions (e.g. on server restart).
    pub fn invalidate_all(&self) {
        let mut lock = self.sessions.write().unwrap();
        for session in lock.values_mut() {
            session.is_active = false;
        }
    }

    /// Returns the number of currently valid active sessions.
    pub fn active_session_count(&self) -> usize {
        let lock = self.sessions.read().unwrap();
        let now = Instant::now();
        lock.values()
            .filter(|s| s.is_active && now <= s.expires_at)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_format_and_entropy() {
        let t1 = generate_session_token();
        let t2 = generate_session_token();

        assert_eq!(t1.len(), 32);
        assert_eq!(t2.len(), 32);
        assert_ne!(t1, t2, "Generated tokens must be distinct");
        assert!(t1.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_session_lifecycle_and_invalidation() {
        let manager = BrokerSessionManager::new(Duration::from_secs(60));
        let session = manager.create_session("A01", None);

        // Valid initially
        let validated = manager.validate_session(&session.session_id).unwrap();
        assert_eq!(validated.lab_id, "A01");
        assert!(validated.is_active);

        // Invalidate by token
        manager.invalidate_session(&session.session_id);
        let err = manager.validate_session(&session.session_id).unwrap_err();
        assert_eq!(err, SessionError::Terminated);

        // Non-existent token
        let fake = "0123456789abcdef0123456789abcdef";
        assert_eq!(
            manager.validate_session(fake).unwrap_err(),
            SessionError::SessionNotFound
        );

        // Malformed token format
        assert_eq!(
            manager.validate_session("short").unwrap_err(),
            SessionError::InvalidToken
        );
        assert_eq!(
            manager
                .validate_session("not-hex-characters-here-0123456789")
                .unwrap_err(),
            SessionError::InvalidToken
        );
    }

    #[test]
    fn test_session_expiration() {
        let manager = BrokerSessionManager::new(Duration::from_millis(10));
        let session = manager.create_session("A06", Some(Duration::from_millis(5)));

        std::thread::sleep(Duration::from_millis(15));
        let err = manager.validate_session(&session.session_id).unwrap_err();
        assert_eq!(err, SessionError::Expired);
    }

    #[test]
    fn test_invalidate_lab_sessions() {
        let manager = BrokerSessionManager::new(Duration::from_secs(60));
        let s1 = manager.create_session("A01", None);
        let s2 = manager.create_session("A01", None);
        let s3 = manager.create_session("A06", None);

        assert_eq!(manager.active_session_count(), 3);

        manager.invalidate_lab("A01");

        assert_eq!(
            manager.validate_session(&s1.session_id).unwrap_err(),
            SessionError::Terminated
        );
        assert_eq!(
            manager.validate_session(&s2.session_id).unwrap_err(),
            SessionError::Terminated
        );
        assert!(manager.validate_session(&s3.session_id).is_ok());
        assert_eq!(manager.active_session_count(), 1);
    }
}
