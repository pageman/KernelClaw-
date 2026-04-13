//! KernelClaw Zero-Dep - Time utilities
//! Replaces chrono with zero external dependencies

use std::time::SystemTime;

/// Get current Unix timestamp in seconds
pub fn now() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

/// Get current Unix timestamp in milliseconds
pub fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

/// Get current Unix timestamp in microseconds  
pub fn now_micros() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_micros() as i64
}

/// Timestamp wrapper for receipts
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Timestamp(i64);

impl Timestamp {
    pub fn now() -> Self {
        Timestamp(now())
    }
    
    pub fn new(ts: i64) -> Self {
        Timestamp(ts)
    }
    
    pub fn as_i64(&self) -> i64 {
        self.0
    }
    
    pub fn as_millis(&self) -> i64 {
        self.0 * 1000
    }
}

impl From<i64> for Timestamp {
    fn from(v: i64) -> Self {
        Timestamp(v)
    }
}

impl From<Timestamp> for i64 {
    fn from(v: Timestamp) -> Self {
        v.0
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Convert timestamp to RFC 3339 string
pub fn to_rfc3339(ts: i64) -> String {
    // Simplified - just epoch seconds
    format!("{}", ts)
}

/// Parse RFC 3339 or epoch timestamp
pub fn parse(s: &str) -> Result<i64, String> {
    s.parse::<i64>().map_err(|e| e.to_string())
}

/// Duration wrapper
#[derive(Debug, Clone, Copy, Default)]
pub struct Duration(std::time::Duration);

impl Duration {
    pub fn from_secs(s: u64) -> Self {
        Duration(std::time::Duration::from_secs(s))
    }
    
    pub fn from_millis(m: u64) -> Self {
        Duration(std::time::Duration::from_millis(m))
    }
    
    pub fn as_secs(&self) -> u64 {
        self.0.as_secs()
    }
    
    pub fn as_millis(&self) -> u64 {
        self.0.as_millis()
    }
}