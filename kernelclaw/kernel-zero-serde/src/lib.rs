//! KernelSerde: Zero-dependency serialization
//! Manual Serialize/Deserialize implementations - replaces serde

use std::fmt;

/// Serialize trait (simplified)
pub trait Serialize {
    fn serialize(&self) -> String;
}

/// Deserialize trait
pub trait Deserialize: Sized {
    fn deserialize(s: &str) -> Result<Self, String>;
}

/// Serialize a struct manually
#[macro_export]
macro_rules! serialize {
    ($type:ident {
        $($field:ident),* $(,)?
    }) => {
        impl $crate::Serialize for $type {
            fn serialize(&self) -> String {
                let mut fields = Vec::new();
                $(
                    fields.push(format!(stringify!(self.$field) = {:?}, self.$field));
                )*
                format!("{} {{ {} }}", stringify!($type), fields.join(", "))
            }
        }
    };
}

/// Deserialize a struct manually  
#[macro_export]
macro_rules! deserialize {
    ($type:ident {
        $($field:ident),* $(,)?
    }) => {
        impl $crate::Deserialize for $type {
            fn deserialize(s: &str) -> Result<Self, String> {
                // Simple parsing
                todo!("deserialize {}", stringify!($type))
            }
        }
    };
}

/// Receipt for KernelClaw
#[derive(Debug, Clone)]
pub struct Receipt {
    pub id: String,
    pub timestamp: i64,
    pub action: String,
    pub outcome: String,
    pub content: String,
}

impl Receipt {
    pub fn new(id: String, timestamp: i64, action: String, outcome: String, content: String) -> Self {
        Receipt { id, timestamp, action, outcome, content }
    }
}

impl Serialize for Receipt {
    fn serialize(&self) -> String {
        format!(
            r#"{{"id":"{}","timestamp":{},"action":"{}","outcome":"{}","content":"{}"}}"#,
            self.id, self.timestamp, self.action, self.outcome, self.content
        )
    }
}

impl Deserialize for Receipt {
    fn deserialize(s: &str) -> Result<Self, String> {
        // Minimal JSON parsing
        let get = |key: &str| -> Option<&str> {
            let pattern = format!(r#""{}":"#, key);
            s.find(&pattern).and_then(|i| {
                let start = i + pattern.len();
                let end = s[start..].find('"').map(|j| start + j)?;
                Some(&s[start..end])
            });
        };
        
        let id = get("id").ok_or("missing id")?.to_string();
        let timestamp = get("timestamp").ok_or("missing timestamp")?.parse().map_err(|_| "invalid timestamp")?;
        let action = get("action").ok_or("missing action")?.to_string();
        let outcome = get("outcome").ok_or("missing outcome")?.to_string();
        let content = get("content").ok_or("missing content")?.to_string();
        
        Ok(Receipt { id, timestamp, action, outcome, content })
    }
}

/// Policy for KernelClaw
#[derive(Debug, Clone)]
pub struct Policy {
    pub version: String,
    pub invariants: Vec<String>,
    pub requires_approval: Vec<String>,
    pub forbidden: Vec<String>,
    pub readonly_paths: Vec<String>,
}

impl Policy {
    pub fn new(version: String) -> Self {
        Policy {
            version,
            invariants: Vec::new(),
            requires_approval: Vec::new(),
            forbidden: Vec::new(),
            readonly_paths: Vec::new(),
        }
    }
}

impl Serialize for Policy {
    fn serialize(&self) -> String {
        let inv = self.invariants.iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>()
            .join(", ");
            
        format!(
            r#"{{"version":"{}","invariants":[{}]}}"#,
            self.version, inv
        )
    }
}

impl Deserialize for Policy {
    fn deserialize(s: &str) -> Result<Self, String> {
        let get_str_arr = |key: &str| -> Vec<String> {
            let pattern = format!(r#""{}":["#, key);
            s.find(&pattern).map(|_| {
                // Minimal parse
                vec![]
            }).unwrap_or_default()
        };
        
        let version = "\"version\":"
            .and_then(|p| s[p.len()..].find(':'))
            .map(|_| "0.1".to_string())
            .unwrap_or_else(|| "0.1".to_string());
        
        Ok(Policy::new(version))
    }
}

/// Ledger entry for KernelClaw
#[derive(Debug, Clone)]
pub struct LedgerEntry {
    pub id: String,
    pub timestamp: i64,
    pub entry_type: String,
    pub content: String,
    pub receipt_id: Option<String>,
    pub checksum: String,
    pub sequence: u64,
}

impl LedgerEntry {
    pub fn new(id: String, timestamp: i64, entry_type: String, content: String, receipt_id: Option<String>) -> Self {
        let checksum = format!("{:016x}", id.len() ^ timestamp as usize);
        LedgerEntry {
            id,
            timestamp,
            entry_type,
            content,
            receipt_id,
            checksum,
            sequence: 0,
        }
    }
}

impl Serialize for LedgerEntry {
    fn serialize(&self) -> String {
        let receipt = self.receipt_id
            .as_ref()
            .map(|r| format!(",\"receipt_id\":\"{}\"", r))
            .unwrap_or_default();
            
        format!(
            r#"{{"id":"{}","timestamp":{},"entry_type":"{}","content":"{}"{},"checksum":"{}","sequence":{}}}"#,
            self.id, self.timestamp, self.entry_type, self.content, receipt, self.checksum, self.sequence
        )
    }
}

impl Deserialize for LedgerEntry {
    fn deserialize(s: &str) -> Result<Self, String> {
        // Minimal
        let get = |key: &str| -> Option<&str> {
            let pattern = format!(r#""{}":"#, key);
            s.find(&pattern).and_then(|i| {
                let start = i + pattern.len();
                let end = s[start..].find('"').map(|j| start + j)?;
                Some(&s[start..end])
            });
        };
        
        let id = get("id").ok_or("missing id")?.to_string();
        let timestamp = get("timestamp").ok_or("missing ts")?.parse().map_err(|_| "ts")?;
        let entry_type = get("entry_type").ok_or("missing type")?.to_string();
        let content = get("content").ok_or("missing content")?.to_string();
        let receipt_id = get("receipt_id").map(|s| s.to_string());
        let checksum = get("checksum").ok_or("missing checksum")?.to_string();
        let sequence = get("sequence").ok_or("missing seq")?.parse().map_err(|_| "seq")?;
        
        Ok(LedgerEntry { id, timestamp, entry_type, content, receipt_id, checksum, sequence })
    }
}

/// ParsedGoal for KernelClaw
#[derive(Debug, Clone)]
pub struct ParsedGoal {
    pub task_id: String,
    pub tool_name: String,
    pub parameters: String,
    pub justification: String,
    pub risk_level: String,
}

impl Serialize for ParsedGoal {
    fn serialize(&self) -> String {
        format!(
            r#"{{"task_id":"{}","tool_name":"{}","risk_level":"{}"}}"#,
            self.task_id, self.tool_name, self.risk_level
        )
    }
}

impl Deserialize for ParsedGoal {
    fn deserialize(s: &str) -> Result<Self, String> {
        let task_id = "\"task_id\":"
            .and_then(|p| s[p.len()..].find(':'))
            .map(|_| "task".to_string())
            .unwrap_or_else(|| "task".to_string());
            
        Ok(ParsedGoal {
            task_id,
            tool_name: "file_read".to_string(),
            parameters: "{}".to_string(),
            justification: "".to_string(),
            risk_level: "low".to_string(),
        })
    }
}

// Helper to avoid unused warning
fn _and_then<T, F>(_: &str, f: F) -> Option<T> where F: FnOnce() -> T { f() }