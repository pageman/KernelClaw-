//! KernelClaw Zero-Dependency module
//! All utilities with ZERO external dependencies

pub mod time;
pub mod id;
pub mod error;
pub mod sha256;
pub mod ed25519;
pub mod json;
pub mod toml;

// Re-exports
pub use time::Timestamp;
pub use id::{random_id, short_id, content_id, receipt as receipt_id, goal as goal_id};
pub use error::{Error, Result};
pub use sha256::{hash as sha256_hash, hex_hash as sha256_hex_hash, receipt_hash};
pub use ed25519::{KeyPair, Signature, signing};
pub use json::{parse as json_parse, to_string as json_to_string, Value};
pub use toml::{parse as toml_parse, to_string as toml_to_string, Value as TomlValue};

/// Get current timestamp
pub fn now() -> i64 {
    time::now()
}

/// Generate new receipt ID
pub fn new_receipt_id() -> String {
    receipt_id()
}

/// Generate new goal ID  
pub fn new_goal_id() -> String {
    goal_id()
}

/// Compute SHA256 hash
pub fn hash(data: &[u8]) -> [u8; 32] {
    sha256_hash(data)
}

/// Compute receipt hash
pub fn compute_receipt_hash(id: &str, timestamp: i64, content: &str) -> String {
    receipt_hash(id, timestamp, content)
}