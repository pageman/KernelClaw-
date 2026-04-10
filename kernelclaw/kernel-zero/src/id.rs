//! KernelClaw Zero-Dep - ID generator
//! Replaces uuid with zero external dependencies

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Generate random 16-byte hex string (22 chars base64 or 32 hex)
pub fn random_id() -> String {
    let bytes: [u8; 16] = rand::random();
    hex_encode(&bytes)
}

/// Generate short ID (8 bytes = 16 hex chars)
pub fn short_id() -> String {
    let bytes: [u8; 8] = rand::random();
    hex_encode(&bytes)
}

/// Generate deterministic ID from input (for content-addressing)
pub fn content_id(data: &[u8]) -> String {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

/// Hex encode bytes to lowercase hex string
pub fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Hex decode hex string to bytes
pub fn hex_decode(hex: &str) -> Option<Vec<u8>> {
    if hex.len() % 2 != 0 {
        return None;
    }
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i+2], 16).ok())
        .collect()
}

/// Validate hex string
pub fn is_valid_hex(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_hexdigit())
}

/// ID types for KernelClaw
pub mod id {
    use super::*;
    
    /// Receipt ID format: rcpt_ + 16 hex chars
    pub fn receipt() -> String {
        format!("rcpt_{}", short_id())
    }
    
    /// Goal ID format: goal_ + 16 hex chars  
    pub fn goal() -> String {
        format!("goal_{}", short_id())
    }
    
    /// Ledger entry ID format: entr_ + 16 hex chars
    pub fn entry() -> String {
        format!("entr_{}", short_id())
    }
}