//! KernelClaw Executor - Feature-gated tool execution
//! Tools are optional via Cargo features

mod tools;

use serde::{Deserialize, Serialize};
use kernel_zero::error::Error;

pub use tools::ToolPolicy;

#[derive(Debug)]
pub enum ExecError {
    #[error("Capability DENIED: {0}")]
    Denied(String),
    #[error("Execution failed: {0}")]
    Failed(String),
    #[error("Tool not enabled: {0}")]
    ToolNotEnabled(String),
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
        // 1. Check tool is enabled
        self.check_tool_enabled(&req.tool_name)?;
        
        // 2. Enforce capabilities
        for cap in &req.capabilities {
            enforce_capability(cap, &self.policy)?;
        }
        
        // 3. Convert for tool boundary
        let tool_policy = ToolPolicy::from(&self.policy);
        
        // 4. Execute tools
        match req.tool_name.as_str() {
            "file_read" => {
                #[cfg(feature = "tool-file-read")]
                {
                    if let Some(path) = req.params.get("path").and_then(|v| v.as_str()) {
                        return Ok(execute_file_read(path, &tool_policy));
                    }
                }
                #[cfg(not(feature = "tool-file-read"))]
                return Err(ExecError::ToolNotEnabled("file_read".to_string()));
            }
            "file_read_dir" => {
                #[cfg(feature = "tool-file-read")]
                {
                    if let Some(path) = req.params.get("path").and_then(|v| v.as_str()) {
                        return Ok(execute_file_read_dir(path, &tool_policy));
                    }
                }
                #[cfg(not(feature = "tool-file-read"))]
                return Err(ExecError::ToolNotEnabled("file_read_dir".to_string()));
            }
            "file_write" => {
                #[cfg(feature = "tool-file-write")]
                {
                    if let (Some(path), Some(content)) = (req.params.get("path").and_then(|v| v.as_str()), req.params.get("content").and_then(|v| v.as_str())) {
                        return Ok(execute_file_write(path, content, &tool_policy));
                    }
                }
                #[cfg(not(feature = "tool-file-write"))]
                return Err(ExecError::ToolNotEnabled("file_write".to_string()));
            }
            "file_metadata" => {
                #[cfg(feature = "tool-file-metadata")]
                {
                    if let Some(path) = req.params.get("path").and_then(|v| v.as_str()) {
                        return Ok(execute_file_metadata(path, &tool_policy));
                    }
                }
                #[cfg(not(feature = "tool-file-metadata"))]
                return Err(ExecError::ToolNotEnabled("file_metadata".to_string()));
            }
            "echo" => {
                #[cfg(feature = "tool-echo")]
                {
                    let input = req.params.get("input").and_then(|v| v.as_str()).unwrap_or("");
                    return Ok(ExecResult { success: true, output: tools::echo_tool(input), error: None, sandboxed: false });
                }
                #[cfg(not(feature = "tool-echo"))]
                return Err(ExecError::ToolNotEnabled("echo".to_string()));
            }
            "calendar_summary" => {
                #[cfg(feature = "tool-calendar")]
                return Ok(ExecResult { success: true, output: tools::calendar_summary(), error: None, sandboxed: false });
                #[cfg(not(feature = "tool-calendar"))]
                return Err(ExecError::ToolNotEnabled("calendar_summary".to_string()));
            }
            "health_check" => {
                #[cfg(feature = "tool-health")]
                return Ok(ExecResult { success: true, output: tools::health_check(), error: None, sandboxed: false });
                #[cfg(not(feature = "tool-health"))]
                return Err(ExecError::ToolNotEnabled("health_check".to_string()));
            }
            _ => Err(ExecError::Failed(format!("Unknown tool: {}", req.tool_name))),
        }
    }
    
    /// Check if tool is enabled via feature flags
    fn check_tool_enabled(&self, tool: &str) -> Result<(), ExecError> {
        match tool {
            "file_read" | "file_read_dir" => {
                #[cfg(feature = "tool-file-read")]
                Ok(())
                #[cfg(not(feature = "tool-file-read"))]
                Err(ExecError::ToolNotEnabled(tool.to_string()))
            }
            "file_write" => {
                #[cfg(feature = "tool-file-write")]
                Ok(())
                #[cfg(not(feature = "tool-file-write"))]
                Err(ExecError::ToolNotEnabled(tool.to_string()))
            }
            "file_metadata" => {
                #[cfg(feature = "tool-file-metadata")]
                Ok(())
                #[cfg(not(feature = "tool-file-metadata"))]
                Err(ExecError::ToolNotEnabled(tool.to_string()))
            }
            "echo" => {
                #[cfg(feature = "tool-echo")]
                Ok(())
                #[cfg(not(feature = "tool-echo"))]
                Err(ExecError::ToolNotEnabled(tool.to_string()))
            }
            "calendar_summary" => {
                #[cfg(feature = "tool-calendar")]
                Ok(())
                #[cfg(not(feature = "tool-calendar"))]
                Err(ExecError::ToolNotEnabled(tool.to_string()))
            }
            "health_check" => {
                #[cfg(feature = "tool-health")]
                Ok(())
                #[cfg(not(feature = "tool-health"))]
                Err(ExecError::ToolNotEnabled(tool.to_string()))
            }
            _ => Ok(()), // Unknown tools allowed
        }
    }
}

/// Helper functions that wrap tools with feature checks
fn execute_file_read(path: &str, policy: &ToolPolicy) -> ExecResult {
    match tools::file_read(path, policy) {
        Ok(content) => ExecResult { success: true, output: content, error: None, sandboxed: false },
        Err(e) => ExecResult { success: false, output: String::new(), error: Some(e), sandboxed: false }
    }
}

fn execute_file_read_dir(path: &str, policy: &ToolPolicy) -> ExecResult {
    match tools::file_read_dir(path, policy) {
        Ok(entries) => ExecResult { success: true, output: entries.join("\n"), error: None, sandboxed: false },
        Err(e) => ExecResult { success: false, output: String::new(), error: Some(e), sandboxed: false }
    }
}

fn execute_file_write(path: &str, content: &str, policy: &ToolPolicy) -> ExecResult {
    match tools::file_write(path, content, policy) {
        Ok(_) => ExecResult { success: true, output: format!("Written to {}", path), error: None, sandboxed: false },
        Err(e) => ExecResult { success: false, output: String::new(), error: Some(e), sandboxed: false }
    }
}

fn execute_file_metadata(path: &str, policy: &ToolPolicy) -> ExecResult {
    match tools::file_metadata(path, policy) {
        Ok(info) => ExecResult { success: true, output: info, error: None, sandboxed: false },
        Err(e) => ExecResult { success: false, output: String::new(), error: Some(e), sandboxed: false }
    }
}