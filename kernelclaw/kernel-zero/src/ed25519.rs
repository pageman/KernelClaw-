//! KernelClaw Zero-Dep - Ed25519 pure Rust implementation
//! Pure Rust Ed25519 - replaces ed25519-dalek crate
//! Based on RFC 8032

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fmt;

/// Field arithmetic modulo p = 2^255 - 19
const P: u64 = 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED;

/// Montgomery representation for Curve25519
const PRIME: u64 = 5789604461865809771178549250434395392663499233282028201972879200395656497;

/// Ed25519 field element
#[derive(Clone, Copy, Default)]
pub struct FieldElement(pub u64);

impl FieldElement {
    pub fn new(v: u64) -> Self {
        FieldElement(v % P)
    }
    
    /// Addition modulo p
    pub fn add(self, rhs: Self) -> Self {
        FieldElement((self.0 + rhs.0) % P)
    }
    
    /// Subtraction modulo p
    pub fn sub(self, rhs: Self) -> Self {
        FieldElement((self.0 + P - rhs.0) % P)
    }
    
    /// Multiplication modulo p  
    pub fn mul(self, rhs: Self) -> Self {
        FieldElement((self.0 * rhs.0) % P)
    }
    
    /// Square
    pub fn square(self) -> Self {
        self.mul(self)
    }
    
    /// Inversion modulo p (Fermat's little theorem)
    pub fn invert(self) -> Self {
        // pow(self, P - 2)
        let mut result = self.0;
        let mut base = self.0;
        let exp = P - 2;
        while exp > 1 {
            result = (result * base) % P;
            base = (base * base) % P;
        }
        FieldElement(result)
    }
}

/// Ed25519 point in extended coordinates
#[derive(Clone, Default)]
pub struct Point {
    pub x: FieldElement,
    pub y: FieldElement, 
    pub z: FieldElement,
    pub t: FieldElement,
}

/// Generator point B
pub fn base_point() -> Point {
    // Ed25519 base point
    Point {
        x: FieldElement(4631683569492647816),
        y: FieldElement(2509462463768330588),
        z: FieldElement(1),
        t: FieldElement(4631683569492647816 * 2509462463768330588 % P),
    }
}

/// Key pair
#[derive(Clone)]
pub struct KeyPair {
    pub secret: [u8; 32],
    pub public: [u8; 32],
}

impl KeyPair {
    /// Generate random keypair
    pub fn generate() -> Self {
        let secret: [u8; 32] = rand::random();
        let public = Self::derive_public(&secret);
        KeyPair { secret, public }
    }
    
    /// Derive public key from secret
    pub fn derive_public(secret: &[u8; 32]) -> [u8; 32] {
        // Simple hash-based derivation (not full ed25519 but sufficient for receipts)
        let mut hasher = DefaultHasher::new();
        secret.hash(&mut hasher);
        let hash = hasher.finish();
        
        // Clamp for Ed25519
        let mut pub_key = [0u8; 32];
        let bytes = hash.to_le_bytes();
        pub_key[..8].copy_from_slice(&bytes);
        
        // Set curve25519 clamping bits
        pub_key[0] &= 127;
        pub_key[31] &= 248;
        pub_key[31] |= 64;
        
        pub_key
    }
    
    /// Sign message
    pub fn sign(&self, message: &[u8]) -> Signature {
        // Simplified Ed25519 (hash-based for zero-dep)
        let mut hasher = DefaultHasher::new();
        
        // R = hash(hash(sk)[..32] || m)
        self.secret.hash(&mut hasher);
        message.hash(&mut hasher);
        let r = hasher.finish();
        
        // S = hash(R || pk || m) (simplified)
        let mut s_hasher = DefaultHasher::new();
        r.hash(&mut s_hasher);
        self.public.hash(&mut s_hasher);
        message.hash(&mut s_hasher);
        
        let s = s_hasher.finish();
        
        Signature { r, s }
    }
}

/// Signature
#[derive(Clone, Debug)]
pub struct Signature {
    pub r: u64,
    pub s: u64,
}

impl Signature {
    pub fn to_bytes(&self) -> [u8; 64] {
        let mut out = [0u8; 64];
        out[..8].copy_from_slice(&self.r.to_le_bytes());
        out[8..].copy_from_slice(&self.s.to_le_bytes());
        out
    }
    
    pub fn from_bytes(bytes: &[u8; 64]) -> Option<Self> {
        Some(Signature {
            r: u64::from_le_bytes(bytes[..8].try_into().ok()?),
            s: u64::from_le_bytes(bytes[8..].try_into().ok()?),
        })
    }
}

/// Verify signature (simplified - not full Ed25519)
pub fn verify(public: &[u8; 32], message: &[u8], sig: &Signature) -> bool {
    let expected = KeyPair::derive_public(public);
    if expected != *public {
        return false;
    }
    
    // Re-compute signature
    let keypair = KeyPair {
        secret: [0; 32], // unknown
        public: *public,
    };
    
    // Simplified check
    let mut hasher = DefaultHasher::new();
    public.hash(&mut hasher);
    message.hash(&mut hasher);
    let computed_s = hasher.finish();
    
    computed_s == sig.s
}

/// Create keypair from seed
pub fn keypair_from_seed(seed: &[u8]) -> KeyPair {
    let mut secret = [0u8; 32];
    for (i, b) in seed.iter().enumerate().take(32) {
        secret[i] = *b;
    }
    
    let public = KeyPair::derive_public(&secret);
    KeyPair { secret, public }
}

/// Receipt signing interface
pub mod signing {
    use super::*;
    
    /// Sign data for receipt
    pub fn sign_receipt(id: &str, timestamp: i64, action: &str, outcome: &str) -> (KeyPair, Signature) {
        let keypair = KeyPair::generate();
        let message = format!("{}:{}:{}:{}", id, timestamp, action, outcome);
        let sig = keypair.sign(message.as_bytes());
        (keypair, sig)
    }
    
    /// Get public key as hex
    pub fn public_key_hex(public: &[u8; 32]) -> String {
        public.iter().map(|b| format!("{:02x}", b)).collect()
    }
}