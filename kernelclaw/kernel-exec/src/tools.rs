//! KernelClaw Tools - ENFORCED policy at actual tool boundary

use std::fs;
use std::path::Path;

/// Policy for tool execution - ENFORCED at boundary
#[derive(Debug, Clone)]
pub struct ToolPolicy {
    pub allowed_paths: Vec<String>,
    pub max_file_size: u64,
}

impl Default for ToolPolicy {
    fn default() -> Self {
        ToolPolicy {
            allowed_paths: vec!["/tmp/".to_string(), "/var/tmp/".to_string()],
            max_file_size: 1024 * 1024,
        }
    }
}

/// Check if path is in allowed list - ENFORCED
fn is_path_allowed(path: &str, policy: &ToolPolicy) -> bool {
    if policy.allowed_paths.is_empty() {
        return true;
    }
    policy.allowed_paths.iter().any(|p| path.starts_with(p) || path.starts_with(&p.replace("/*", ""))
}

/// file_read tool - WITH policy enforcement at actual tool boundary
pub fn file_read(path: &str, policy: &ToolPolicy) -> Result<String, String> {
    let p = Path::new(path);
    
    // Security: prevent path traversal
    if path.contains("..") {
        return Err("Path traversal not allowed".to_string());
    }
    
    // ENFORCED: Check allowed_paths BEFORE reading
    if !is_path_allowed(path, policy) {
        return Err(format!("Path {} not in allowed list: {:?}", path, policy.allowed_paths));
    }
    
    // Check file exists
    if !p.exists() {
        return Err(format!("File not found: {}", path));
    }
    
    // Check regular file
    if !p.is_file() {
        return Err(format!("Not a file: {}", path));
    }
    
    // Size limit from policy
    let metadata = fs::metadata(p).map_err(|e| e.to_string())?;
    if metadata.len() > policy.max_file_size {
        return Err(format!("File too large (max {} bytes)", policy.max_file_size));
    }
    
    // Read the file
    fs::read_to_string(p)
        .map_err(|e| format!("Read error: {}", e))
}

/// file_read_dir - WITH policy enforcement
pub fn file_read_dir(path: &str, policy: &ToolPolicy) -> Result<Vec<String>, String> {
    let p = Path::new(path);
    
    // Security
    if path.contains("..") {
        return Err("Path traversal not allowed".to_string());
    }
    
    // ENFORCED: Check allowed_paths
    if !is_path_allowed(path, policy) {
        return Err(format!("Path not in allowed list: {:?}", policy.allowed_paths));
    }
    
    if !p.exists() {
        return Err(format!("Directory not found: {}", path));
    }
    
    if !p.is_dir() {
        return Err(format!("Not a directory: {}", path));
    }
    
    let mut entries = Vec::new();
    for entry in fs::read_dir(p).map_err(|e| e.to_string())? {
        if let Ok(e) = entry {
            let name = e.file_name().to_string_lossy().to_string();
            let path_type = if e.path().is_dir() {
                format!("{}/", name)
            } else {
                name
            };
            entries.push(path_type);
        }
    }
    
    entries.sort();
    Ok(entries)
}

/// file_write tool - WITH policy enforcement
pub fn file_write(path: &str, content: &str, policy: &ToolPolicy) -> Result<(), String> {
    let p = Path::new(path);
    
    // Security: prevent path traversal
    if path.contains("..") {
        return Err("Path traversal not allowed".to_string());
    }
    
    // ENFORCED: Check allowed_paths BEFORE writing
    if !is_path_allowed(path, policy) {
        return Err(format!("Path {} not in allowed list: {:?}", path, policy.allowed_paths));
    }
    
    // Check parent directory exists
    if let Some(parent) = p.parent() {
        if !parent.exists() {
            return Err(format!("Parent directory not found: {}", parent.display()));
        }
    }
    
    // Write the file
    fs::write(p, content)
        .map_err(|e| format!("Write error: {}", e))
    
    // Success - no output for writes
    Ok(())
}

/// file_metadata tool
pub fn file_metadata(path: &str, policy: &ToolPolicy) -> Result<String, String> {
    let p = Path::new(path);
    
    // Security
    if path.contains("..") {
        return Err("Path traversal not allowed".to_string());
    }
    
    // Check allowed
    if !is_path_allowed(path, policy) {
        return Err(format!("Path not in allowed list: {:?}", policy.allowed_paths));
    }
    
    if !p.exists() {
        return Err(format!("File not found: {}", path));
    }
    
    let metadata = fs::metadata(p).map_err(|e| e.to_string())?;
    
    let file_type = if p.is_dir() { "directory" } else { "file" };
    let size = metadata.len();
    let modified = metadata.modified()
        .map(|t| format!("{:?}", t))
        .unwrap_or_else(|_| "unknown".to_string());
    
    Ok(format!("{} {} bytes modified: {}", file_type, size, modified))
}

/// echo_tool - No policy needed (safe)
pub fn echo_tool(input: &str) -> String {
    format!("Echo: {}", input)
}

/// calendar_summary - No policy needed
pub fn calendar_summary() -> String {
    "Calendar access not configured in v0.1".to_string()
}

/// health_check - No policy needed
pub fn health_check() -> String {
    "KernelClaw: OK".to_string()
}