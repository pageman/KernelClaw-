//! KernelClaw Initial Tools
//! file_read, calendar_read, echo tools

use std::fs;
use std::path::Path;

/// file_read tool - Read files from policy-permitted paths
pub fn file_read(path: &str) -> Result<String, String> {
    let p = Path::new(path);
    
    // Security check - prevent path traversal
    if path.contains("..") {
        return Err("Path traversal not allowed".to_string());
    }
    
    fs::read_to_string(p)
        .map_err(|e| format!("Failed to read: {}", e))
}

/// file_read_dir - List directory contents
pub fn file_read_dir(path: &str) -> Result<Vec<String>, String> {
    let p = Path::new(path);
    
    if path.contains("..") {
        return Err("Path traversal not allowed".to_string());
    }
    
    let mut entries = Vec::new();
    for entry in fs::read_dir(p).map_err(|e| format!("Failed to read dir: {}", e))? {
        if let Ok(entry) = entry {
            entries.push(entry.path().display().to_string());
        }
    }
    Ok(entries)
}

/// echo_tool - Echo back input (test tool)
pub fn echo_tool(input: &str) -> String {
    format!("Echo: {}", input)
}

/// calendar_summary - Mock calendar summary
pub fn calendar_summary() -> String {
    "Calendar access not configured in v0.1".to_string()
}