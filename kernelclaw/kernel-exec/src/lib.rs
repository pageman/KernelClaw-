//! KernelClaw Executor - ENFORCED capability gating at tool boundary
//! Policy enforced at actual tool boundary, not just capability labels

mod tools;

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use tools::{file_read, file_read_dir, echo_tool, calendar_summary, health_check, ToolPolicy};

#[derive(Error, Debug)]
pub enum ExecError {
    #[error("Capability DENIED: {0}")]
    Denied(String),
    #[error("Execution failed: {0}")]
    Failed(String),
}

/// ENFORCED capability model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Capability {
    FileRead(String),
    FileWrite(String),
    ShellExec(String),
    WasmTool(String),
}

/// Policy configuration - loaded from policy.yaml
#[derive(Debug, Clone)]
pub struct Policy {
    pub allow_file_read: bool,
    pub allow_file_write: bool,
    pub allow_shell: bool,
    pub allowed_paths: Vec<String>,
    pub max_file_size: u64,
}

impl Default for Policy {
    fn default() -> Self {
        Policy {
            allow_file_read: true,
            allow_file_write: false,
            allow_shell: false,
            allowed_paths: vec!["/tmp/".to_string(), "/var/tmp/".to_string()],
            max_file_size: 1024 * 1024,
        }
    }
}

impl From<&Policy> for ToolPolicy {
    fn from(p: &Policy) -> Self {
        ToolPolicy {
            allowed_paths: p.allowed_paths.clone(),
            max_file_size: p.max_file_size,
        }
    }
}

/// ENFORCE capability check - ACTUAL enforcement
fn enforce_capability(cap: &Capability, policy: &Policy) -> Result<(), ExecError> {
    match cap {
        Capability::FileRead(path) => {
            if !policy.allow_file_read {
                return Err(ExecError::Denied("file_read disabled".to_string()));
            }
            // Check path at capability level too
            if !policy.allowed_paths.is_empty() {
                let allowed = policy.allowed_paths.iter().any(|p| path.starts_with(p));
                if !allowed {
                    return Err(ExecError::Denied(format!("path {} not in allowlist", path)));
                }
            }
            Ok(())
        }
        Capability::FileWrite(_) => {
            if !policy.allow_file_write {
                return Err(ExecError::Denied("file_write disabled".to_string()));
            }
            Ok(())
        }
        Capability::ShellExec(_) => {
            if !policy.allow_shell {
                return Err(ExecError::Denied("shell disabled".to_string()));
            }
            Ok(())
        }
        Capability::WasmTool(_) => {
            Err(ExecError::Denied("WASM not enabled in v0.1".to_string()))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecRequest {
    pub capabilities: Vec<Capability>,
    pub tool_name: String,
    pub params: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub sandboxed: bool,
}

/// ENFORCED executor - Policy at actual tool boundary
pub struct Executor {
    policy: Policy,
}

impl Executor {
    pub fn new() -> Self {
        Executor { policy: Policy::default() }
    }
    
    pub fn with_policy(policy: Policy) -> Self {
        Executor { policy }
    }
    
    /// Execute with policy enforced at tool boundary
    pub fn execute(&self, req: &ExecRequest) -> Result<ExecResult, ExecError> {
        // 1. ENFORCE capabilities BEFORE execution
        for cap in &req.capabilities {
            enforce_capability(cap, &self.policy)?;
        }
        
        // 2. Convert to ToolPolicy for actual tool boundary enforcement
        let tool_policy = ToolPolicy::from(&self.policy);
        
        // 3. Execute with policy at boundary
        match req.tool_name.as_str() {
            "file_read" => {
                if let Some(path) = req.params.get("path").and_then(|v| v.as_str()) {
                    match tools::file_read(path, &tool_policy) {
                        Ok(content) => Ok(ExecResult { success: true, output: content, error: None, sandboxed: false }),
                        Err(e) => Ok(ExecResult { success: false, output: String::new(), error: Some(e), sandboxed: false })
                    }
                } else {
                    Err(ExecError::Failed("Missing path".to_string()))
                }
            }
            "file_read_dir" => {
                if let Some(path) = req.params.get("path").and_then(|v| v.as_str()) {
                    match tools::file_read_dir(path, &tool_policy) {
                        Ok(entries) => Ok(ExecResult { success: true, output: entries.join("\n"), error: None, sandboxed: false }),
                        Err(e) => Ok(ExecResult { success: false, output: String::new(), error: Some(e), sandboxed: false })
                    }
                } else {
                    Err(ExecError::Failed("Missing path".to_string()))
                }
            }
            "echo" => {
                let input = req.params.get("input").and_then(|v| v.as_str()).unwrap_or("");
                Ok(ExecResult { success: true, output: tools::echo_tool(input), error: None, sandboxed: false })
            }
            "calendar_summary" => {
                Ok(ExecResult { success: true, output: tools::calendar_summary(), error: None, sandboxed: false })
            }
            "health_check" => {
                Ok(ExecResult { success: true, output: tools::health_check(), error: None, sandboxed: false })
            }
            _ => Err(ExecError::Failed(format!("Unknown tool: {}", req.tool_name))),
        }
    }
}