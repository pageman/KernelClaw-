//! KernelClaw Memory Ledger - DURABLE Append-Only with JSONL
//! REPLACES in-memory Mutex with persistent JSONL

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use std::sync::Mutex;
use kernel_zero::id::random_id;
use kernel_zero::time::now as utc_now;
use kernel_zero::sha256::Sha256;

/// Ledger entry type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EntryType {
    Fact, Event, GoalOutcome, ReceiptRef, Summary, Exception, Proposal,
}

/// Ledger entry with checksum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub id: String,
    pub timestamp: i64,
    pub entry_type: EntryType,
    pub content: String,
    pub receipt_id: Option<String>,
    pub checksum: String,
    pub sequence: u64,
}

/// DURABLE Append-Only Memory Ledger
/// Replaces in-memory Mutex with JSONL files
pub struct MemoryLedger {
    path: PathBuf,
    sequence: Mutex<u64>,
}

impl MemoryLedger {
    /// Create new durable ledger
    pub fn new(path: PathBuf) -> Self {
        fs::create_dir_all(&path).ok();
        
        // Find highest sequence
        let mut max_seq = 0u64;
        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.ends_with(".jsonl") {
                    if let Ok(seq) = name[..name.len()-6].parse::<u64>() {
                        if seq > max_seq { max_seq = seq; }
                    }
                }
            }
        }
        
        MemoryLedger {
            path,
            sequence: Mutex::new(max_seq + 1),
        }
    }
    
    /// Compute SHA256 checksum for integrity verification
    fn compute_checksum(entry: &LedgerEntry) -> String {
        let mut hasher = kernel_zero::sha256::Sha256::new();
        hasher.update(entry.id.as_bytes());
        hasher.update(entry.timestamp.to_be_bytes());
        hasher.update(entry.content.as_bytes());
        if let Some(ref rid) = entry.receipt_id {
            hasher.update(rid.as_bytes());
        }
        format!("{:x}", hasher.finalize())
    }
    
    /// Append entry - DURABLE, NOT in-memory
    pub fn append(&self, entry_type: EntryType, content: String, receipt_id: Option<String>) -> Result<String, String> {
        let sequence = {
            let mut s = self.sequence.lock().unwrap();
            let next = *s;
            *s += 1;
            next
        };
        
        let entry = LedgerEntry {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now().timestamp(),
            entry_type,
            content,
            receipt_id,
            checksum: String::new(), // compute below
            sequence,
        };
        
        // Compute checksum BEFORE storing
        let checksum = Self::compute_checksum(&entry);
        let entry = LedgerEntry { checksum: checksum.clone(), ..entry };
        
        // Serialize to JSON
        let json = serde_json::to_string(&entry).map_err(|e| e.to_string())?;
        
        // Write to JSONL file (append-only)
        let file_path = self.path.join(format!("{:010}.jsonl", sequence));
        fs::write(&file_path, json).map_err(|e| e.to_string())?;
        
        // Verify by reading back
        let verify = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
        if verify != json {
            return Err("Write verification failed".to_string());
        }
        
        Ok(entry.id)
    }
    
    /// Get all entries in sequence order - DURABLE
    pub fn get_all(&self) -> Result<Vec<LedgerEntry>, String> {
        let mut entries = Vec::new();
        
        let read_dir = fs::read_dir(&self.path).map_err(|e| e.to_string())?;
        for entry in read_dir.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".jsonl") {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    if let Ok(ledger_entry) = serde_json::from_str::<LedgerEntry>(&content) {
                        entries.push(ledger_entry);
                    }
                }
            }
        }
        
        entries.sort_by(|a, b| a.sequence.cmp(&b.sequence));
        Ok(entries)
    }
    
    /// Get entry by ID - DURABLE
    pub fn get(&self, id: &str) -> Result<Option<LedgerEntry>, String> {
        for entry in self.get_all()? {
            if entry.id == id {
                return Ok(Some(entry));
            }
        }
        Ok(None)
    }
    
    /// Verify checksum integrity
    pub fn verify(&self, id: &str) -> Result<bool, String> {
        if let Some(entry) = self.get(id)? {
            let computed = Self::compute_checksum(&entry);
            return Ok(computed == entry.checksum);
        }
        Ok(false)
    }
    
    /// Get count
    pub fn len(&self) -> usize {
        self.get_all().map(|e| e.len()).unwrap_or(0)
    }
    
    /// Get entries by type
    pub fn query_by_type(&self, entry_type: EntryType) -> Result<Vec<LedgerEntry>, String> {
        Ok(self.get_all()?
            .into_iter()
            .filter(|e| e.entry_type == entry_type)
            .collect())
    }
}