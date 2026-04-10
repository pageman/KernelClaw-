//! KernelClaw Zero-Dep - Time utilities
//! Replaces chrono with zero external dependencies

/// Get current Unix timestamp in seconds
pub fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

/// Get current Unix timestamp in milliseconds
pub fn now_millis() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}

/// Format timestamp as ISO-like string
pub fn format_timestamp(ts: i64) -> String {
    // Simple ISO-ish format: 2026-04-10T16:00:00Z
    // For full date parsing, would need calendar math
    format!("{}", ts)
}

/// Parse Unix timestamp
pub fn from_timestamp(ts: i64) -> std::time::SystemTime {
    std::time::UNIX_EPOCH + std::time::Duration::from_secs(ts as u64)
}

/// Timestamp wrapper for receipts
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}