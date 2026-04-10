//! KernelClaw Cryptography Module
//! Key generation, signing, and receipt management

use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use std::path::PathBuf;
use std::fs;

/// Signing key wrapper
#[derive(Debug, Clone)]
pub struct SigningKeyPair {
    pub signing: SigningKey,
    pub verifying: VerifyingKey,
}

impl From<(SigningKey, VerifyingKey)> for SigningKeyPair {
    fn from((signing, verifying): (SigningKey, VerifyingKey)) -> Self {
        SigningKeyPair { signing, verifying }
    }
}

/// Generate a new keypair
pub fn generate_keypair() -> SigningKeyPair {
    let signing = SigningKey::generate(&mut OsRng);
    let verifying = signing.verifying_key();
    SigningKeyPair { signing, verifying }
}

/// Load keypair from file
pub fn load_keypair(path: &PathBuf) -> Result<SigningKeyPair, std::io::Error> {
    let bytes = fs::read(path)?;
    if bytes.len() != 32 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid key length"));
    }
    let key: [u8; 32] = bytes.try_into().unwrap();
    let signing = SigningKey::from_bytes(&key);
    let verifying = signing.verifying_key();
    Ok(SigningKeyPair { signing, verifying })
}

/// Save keypair to file
pub fn save_keypair(path: &PathBuf, keypair: &SigningKeyPair) -> Result<(), std::io::Error> {
    fs::write(path, keypair.signing.to_bytes())
}

/// Sign data and return signature
pub fn sign(data: &[u8], keypair: &SigningKeyPair) -> Vec<u8> {
    use ed25519_dalek::Signer;
    let signature = keypair.signing.sign(data);
    signature.to_bytes().to_vec()
}

/// Verify signature
pub fn verify(data: &[u8], signature: &[u8], key: &VerifyingKey) -> bool {
    use ed25519_dalek::Verifier;
    if signature.len() != 64 { return false; }
    let sig_arr: [u8; 64] = match signature.try_into() { Ok(s) => s, Err(_) => return false, };
    let sig = ed25519_dalek::Signature::from_bytes(&sig_arr);
    key.verify(data, &sig).is_ok()
}

/// Compute SHA256 hash
pub fn hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Receipt structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub id: String,
    pub timestamp: i64,
    pub goal: String,
    pub action: String,
    pub input_summary: String,
    pub result_summary: String,
    pub status: String,
    pub signature: String,
    pub receipt_hash: String,
}

/// Create and sign a receipt
pub fn create_receipt(
    goal: &str,
    action: &str,
    input_summary: &str,
    result_summary: &str,
    status: &str,
    keypair: &SigningKeyPair,
) -> Receipt {
    let timestamp = chrono::Utc::now().timestamp();
    let id = uuid::Uuid::new_v4().to_string();
    
    let payload = format!("{}|{}|{}|{}|{}|{}", id, timestamp, goal, action, input_summary, result_summary);
    let signature_bytes = sign(payload.as_bytes(), keypair);
    let signature = BASE64.encode(&signature_bytes);
    let receipt_hash = BASE64.encode(&hash(payload.as_bytes()));
    
    Receipt { id, timestamp: timestamp, goal: goal.to_string(), action: action.to_string(),
        input_summary: input_summary.to_string(), result_summary: result_summary.to_string(),
        status: status.to_string(), signature, receipt_hash }
}

/// Verify a receipt
pub fn verify_receipt(receipt: &Receipt, key: &VerifyingKey) -> bool {
    let payload = format!("{}|{}|{}|{}|{}|{}", receipt.id, receipt.timestamp, 
        receipt.goal, receipt.action, receipt.input_summary, receipt.result_summary);
    let signature_bytes = match BASE64.decode(&receipt.signature) { Ok(s) => s, Err(_) => return false };
    verify(payload.as_bytes(), &signature_bytes, key)
}