//! KernelClaw Executor - ENFORCED capability gating at tool boundary
//! Uses kernel_policy::Policy, NOT duplicate

mod tools;

use serde::{Deserialize, Serialize};
use kernel_zero::error::Error;

pub use tools::{file_read, file_read_dir, echo_tool, calendar_summary, health_check, ToolPolicy};

#[derive(Debug)]
pub enum ExecError {
    #[error("Capability DENIED: {0}")]
    Denied(String),
    #[error("Execution failed: {0}")]
    Failed(String),
}

/// Re-export Policy from kernel-policy for unified use
pub use kernel_policy::Policy as KernelPolicy;

/// ENFORCED capability model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Capability {
    FileRead(String),
    FileWrite(String),
    ShellExec(String),
    WasmTool(String),
}

/// Convert kernel_policy::Policy to ToolPolicy
impl From<&KernelPolicy> for ToolPolicy {
    fn from(p: &KernelPolicy) -> Self {
        ToolPolicy {
            allowed_paths: p.allowed_paths.clone(),
            max_file_size: p.max_file_size,
        }
    }
}

/// ENFORCE capability check
fn enforce_capability(cap: &Capability, policy: &KernelPolicy) -> Result<(), ExecError> {
    match cap {
        Capability::FileRead(path) => {
            if !policy.capabilities.iter().any(|c| c.name == "file_read" && c.allowed) {
                return Err(ExecError::Denied("file_read disabled".to_string()));
            }
            if !policy.allowed_paths.is_empty() {
                let allowed = policy.allowed_paths.iter().any(|p| path.starts_with(p));
                if !allowed {
                    return Err(ExecError::Denied(format!("path {} not in allowlist", path)));
                }
            }
            Ok(())
        }
        Capability::FileWrite(_) => {
            if !policy.capabilities.iter().any(|c| c.name == "file_write" && c.allowed) {
                return Err(ExecError::Denied("file_write disabled".to_string()));
            }
            Ok(())
        }
        Capability::ShellExec(_) => {
            if !policy.capabilities.iter().any(|c| c.name == "shell" && c.allowed) {
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

/// ENFORCED executor using kernel_policy
pub struct Executor {
    policy: KernelPolicy,
}

impl Executor {
    pub fn new() -> Self {
        Executor { policy: KernelPolicy::default() }
    }
    
    pub fn with_policy(policy: KernelPolicy) -> Self {
        Executor { policy }
    }
    
    /// Execute with policy enforced
    pub fn execute(&self, req: &ExecRequest) -> Result<ExecResult, ExecError> {
        // 1. Enforce capabilities
        for cap in &req.capabilities {
            enforce_capability(cap, &self.policy)?;
        }
        
        // 2. Convert for tool boundary
        let tool_policy = ToolPolicy::from(&self.policy);
        
        // 3. Execute tools
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
            "file_write" => {
                if let (Some(path), Some(content)) = (req.params.get("path").and_then(|v| v.as_str()), req.params.get("content").and_then(|v| v.as_str())) {
                    match tools::file_write(path, content, &tool_policy) {
                        Ok(_) => Ok(ExecResult { success: true, output: format!("Written to {}", path), error: None, sandboxed: false }),
                        Err(e) => Ok(ExecResult { success: false, output: String::new(), error: Some(e), sandboxed: false })
                    }
                } else {
                    Err(ExecError::Failed("Missing path or content".to_string()))
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