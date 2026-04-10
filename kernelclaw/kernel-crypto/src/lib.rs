//! KernelClaw Crypto - Ed25519 signing with optional zero-dep

/// Use zero-dep Ed25519 implementation if available
#[cfg(feature = "use_zero_ed25519")]
pub use kernel_zero_ed25519::signing::{generate_keypair, create_receipt, verify_receipt, SigningKeyPair};
/// Use standard ed25519-dalek otherwise
#[cfg(not(feature = "use_zero_ed25519"))]
pub use ed25519_dalek::{SigningKeyPair, Signature, Signer, Verifier};

#[cfg(not(feature = "use_zero_ed25519"))]
use ed25519_dalek::{SigningKeypair, Signer, Verifier};
use serde::{Deserialize, Serialize};

/// Receipt for execution records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub id: String,
    pub timestamp: i64,
    pub action: String,
    pub outcome: String,
    pub content: String,
    pub signature: String,
}

/// Generate a new keypair
#[cfg(not(feature = "use_zero_ed25519"))]
pub fn generate_keypair() -> SigningKeyPair {
    let mut csprng = rand::thread_rng();
    SigningKeypair::generate(&mut csprng)
}

/// Create a signed receipt
#[cfg(not(feature = "use_zero_ed25519"))]
pub fn create_receipt(
    id: &str,
    action: &str,
    content: &str,
    outcome: &str,
    kp: &SigningKeyPair,
) -> Result<Receipt, String> {
    let payload = format!("{}:{}:{}:{}:{}", id, action, content, outcome, kp.verifying_key());
    let signature = kp.sign(payload.as_bytes());
    
    Ok(Receipt {
        id: id.to_string(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
        action: action.to_string(),
        outcome: outcome.to_string(),
        content: content.to_string(),
        signature: base64_encode(&signature.to_bytes()),
    })
}

/// Verify a receipt
#[cfg(not(feature = "use_zero_ed25519"))]
pub fn verify_receipt(receipt: &Receipt, pk: &VerifyingKey) -> bool {
    let payload = format!("{}:{}:{}:{}:{}", receipt.id, receipt.action, receipt.content, receipt.outcome, pk);
    let sig_bytes = base64_decode(&receipt.signature).ok()?;
    let signature = Signature::from_bytes(&sig_bytes);
    pk.verify(payload.as_bytes(), &signature).is_ok()
}

/// Simple base64 encoding (inline, replaces base64 crate)
pub fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        
        result.push(ALPHABET[(b0 >> 2) as usize] as char);
        result.push(ALPHABET[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        
        if chunk.len() > 1 {
            result.push(ALPHABET[(((b1 & 0x0F) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            result.push('=');
        }
        
        if chunk.len() > 2 {
            result.push(ALPHABET[(b2 & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    
    result
}

/// Simple base64 decoding (inline)
pub fn base64_decode(data: &str) -> Result<Vec<u8>, String> {
    const DECODE: [u8; 256] = {
        let mut arr = [0u8; 256];
        let mut i = 0u8;
        while i < 26 {
            arr[(b'A' + i) as usize] = i;
            arr[(b'a' + i) as usize] = i + 26;
            arr[(b'0' + i) as usize] = i + 52;
            i += 1;
        }
        arr[(b'+') as usize] = 62;
        arr[(b'/') as usize] = 63;
        arr
    };
    
    let data = data.as_bytes();
    let mut result = Vec::new();
    
    let mut i = 0;
    while i < data.len() {
        let mut chunk = [0u8; 4];
        let mut valid = 0;
        
        while valid < 4 && i < data.len() {
            if data[i] != b'=' {
                chunk[valid] = DECODE[data[i] as usize];
                valid += 1;
            }
            i += 1;
        }
        
        if valid > 0 {
            result.push(((chunk[0] << 2) | (chunk[1] >> 4)) as u8);
            if valid > 1 && data.get(i - valid + 2).copied() != Some(&b'=') {
                result.push(((chunk[1] << 4) | (chunk[2] >> 2)) as u8);
            }
            if valid > 2 && data.get(i - valid + 3).copied() != Some(&b'=') {
                result.push(((chunk[2] << 6) | chunk[3]) as u8);
            }
        }
    }
    
    Ok(result)
}