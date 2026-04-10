//! KernelClaw Crypto - Ed25519 signing with optional zero-dep

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

// ============================================================================
// ZERO-DEPENDENCY ED25519
// ============================================================================

#[cfg(feature = "use_zero_ed25519")]
mod zero_dep {
    use super::*;
    use kernel_zero_ed25519::signing::{generate_keypair as gen_kp, sign, verify};
    
    pub fn generate_keypair() -> super::KeyPair {
        let kp = gen_kp();
        super::KeyPair {
            signing: kp.secret.to_bytes(),
            verifying: kp.verifying_key,
        }
    }
    
    pub fn create_receipt(
        id: &str,
        action: &str,
        content: &str,
        outcome: &str,
        kp: &super::KeyPair,
    ) -> Result<super::Receipt, String> {
        use kernel_zero::sha256::Sha256;
        
        let payload = format!("{}:{}:{}:{}", id, action, content, outcome);
        let signature = sign(payload.as_bytes(), &kernel_zero_ed25519::signing::KeyPair {
            secret: kp.signing,
            public: kp.verifying,
        });
        
        Ok(super::Receipt {
            id: id.to_string(),
            timestamp: kernel_zero::time::now(),
            action: action.to_string(),
            outcome: outcome.to_string(),
            content: content.to_string(),
            signature: super::base64_encode(&signature),
        })
    }
    
    pub fn verify_receipt(receipt: &super::Receipt, pk: &[u8; 32]) -> bool {
        use kernel_zero_ed25519::signing::verify;
        
        let payload = format!("{}:{}:{}:{}", receipt.id, receipt.action, receipt.content, receipt.outcome);
        let sig_bytes = match super::base64_decode(&receipt.signature) {
            Ok(s) if s.len() == 64 => {
                let mut arr = [0u8; 64];
                arr.copy_from_slice(&s);
                arr
            }
            _ => return false,
        };
        
        verify(payload.as_bytes(), &sig_bytes, pk)
    }
    
    /// KeyPair for zero-dep
    pub struct KeyPair {
        pub signing: [u8; 32],
        pub verifying: [u8; 32],
    }
}

// ============================================================================
// STANDARD ED25519-DALEK
// ============================================================================

#[cfg(not(feature = "use_zero_ed25519"))]
mod std_dep {
    use super::*;
    use ed25519_dalek::{SigningKey, VerifyingKey, Signer, Verifier};
    
    pub struct KeyPair {
        pub signing: SigningKey,
        pub verifying: VerifyingKey,
    }
    
    impl From<(SigningKey, VerifyingKey)> for KeyPair {
        fn from((signing, verifying): (SigningKey, VerifyingKey)) -> Self {
            KeyPair { signing, verifying }
        }
    }
    
    pub fn generate_keypair() -> KeyPair {
        use rand::rngs::OsRng;
        let signing = SigningKey::generate(&mut OsRng);
        let verifying = signing.verifying_key();
        KeyPair { signing, verifying }
    }
    
    pub fn create_receipt(
        id: &str,
        action: &str,
        content: &str,
        outcome: &str,
        kp: &KeyPair,
    ) -> Result<super::Receipt, String> {
        let payload = format!("{}:{}:{}:{}", id, action, content, outcome);
        let signature = kp.signing.sign(payload.as_bytes());
        
        Ok(super::Receipt {
            id: id.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            action: action.to_string(),
            outcome: outcome.to_string(),
            content: content.to_string(),
            signature: super::base64_encode(&signature.to_bytes()),
        })
    }
    
    pub fn verify_receipt(receipt: &super::Receipt, pk: &VerifyingKey) -> bool {
        let payload = format!("{}:{}:{}:{}", receipt.id, receipt.action, receipt.content, receipt.outcome);
        let sig_bytes = match super::base64_decode(&receipt.signature) {
            Ok(s) => s,
            Err(_) => return false,
        };
        
        if sig_bytes.len() != 64 { return false; }
        let mut arr = [0u8; 64];
        arr.copy_from_slice(&sig_bytes);
        let signature = ed25519_dalek::Signature::from_bytes(&arr);
        
        pk.verify(payload.as_bytes(), &signature).is_ok()
    }
}

// ============================================================================
// PUBLIC API
// ============================================================================

#[cfg(feature = "use_zero_ed25519")]
pub use zero_dep::{KeyPair, generate_keypair, create_receipt, verify_receipt};

#[cfg(not(feature = "use_zero_ed25519"))]
pub use std_dep::{KeyPair, generate_keypair, create_receipt, verify_receipt};

// ============================================================================
// BASE64 (INLINE - ALWAYS ZERO-DEP)
// ============================================================================

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