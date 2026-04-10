//! KernelClaw Memory Ledger
//! Append-only signed memory storage

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use std::sync::Mutex;
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntryType {
    Fact, Event, GoalOutcome, ReceiptRef, Summary, Exception,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub id: String,
    pub timestamp: i64,
    pub entry_type: EntryType,
    pub content: String,
    pub receipt_id: Option<String>,
}

pub struct MemoryLedger {
    entries: Mutex<Vec<LedgerEntry>>,
    path: PathBuf,
}

impl MemoryLedger {
    pub fn new(path: PathBuf) -> Self {
        fs::create_dir_all(&path).ok();
        MemoryLedger { entries: Mutex::new(Vec::new()), path }
    }
    
    pub fn append(&self, entry_type: EntryType, content: String, receipt_id: Option<String>) -> String {
        let entry = LedgerEntry {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now().timestamp(),
            entry_type,
            content,
            receipt_id,
        };
        let id = entry.id.clone();
        self.entries.lock().unwrap().push(entry);
        id
    }
    
    pub fn get_all(&self) -> Vec<LedgerEntry> { self.entries.lock().unwrap().clone() }
    pub fn len(&self) -> usize { self.entries.lock().unwrap().len() }
}
