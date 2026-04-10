//! KernelClaw Executor
//! Capability-based execution with WASM sandbox

pub mod tools;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::fs;

#[derive(Error, Debug)]
pub enum ExecError {
    #[error("Capability denied: {0}")]
    CapabilityDenied(String),
    #[error("Sandbox error: {0}")]
    Sandbox(String),
    #[error("Execution error: {0}")]
    Execution(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Capability {
    FileRead(String),
    FileWrite(String),
    ShellExec(String),
    CalendarRead,
    WasmTool(String),
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
}

pub struct Executor;

impl Executor {
    pub fn new() -> Self { Self }
    
    pub fn check_capabilities(&self, caps: &[Capability]) -> Result<(), ExecError> {
        for cap in caps {
            match cap {
                Capability::ShellExec(_) => return Err(ExecError::CapabilityDenied("ShellExec not allowed in v0.1".to_string())),
                Capability::FileWrite(_) => return Err(ExecError::CapabilityDenied("FileWrite not allowed in v0.1".to_string())),
                _ => {}
            }
        }
        Ok(())
    }
    
    pub fn execute(&self, req: &ExecRequest) -> Result<ExecResult, ExecError> {
        self.check_capabilities(&req.capabilities)?;
        
        match req.tool_name.as_str() {
            "file_read" => {
                if let Some(path) = req.params.get("path").and_then(|v| v.as_str()) {
                    match tools::file_read(path) {
                        Ok(content) => Ok(ExecResult { success: true, output: content, error: None }),
                        Err(e) => Ok(ExecResult { success: false, output: String::new(), error: Some(e) }),
                    }
                } else {
                    Ok(ExecResult { success: false, output: String::new(), error: Some("Missing path param".to_string()) })
                }
            }
            "echo" => {
                let input = req.params.get("input").and_then(|v| v.as_str()).unwrap_or("");
                Ok(ExecResult { success: true, output: tools::echo_tool(input), error: None })
            }
            "calendar_summary" => {
                Ok(ExecResult { success: true, output: tools::calendar_summary(), error: None })
            }
            "file_read_dir" => {
                if let Some(path) = req.params.get("path").and_then(|v| v.as_str()) {
                    match tools::file_read_dir(path) {
                        Ok(entries) => Ok(ExecResult { success: true, output: entries.join("\n"), error: None }),
                        Err(e) => Ok(ExecResult { success: false, output: String::new(), error: Some(e) }),
                    }
                } else {
                    Ok(ExecResult { success: false, output: String::new(), error: Some("Missing path param".to_string()) })
                }
            }
            _ => Err(ExecError::Execution(format!("Unknown tool: {}", req.tool_name))),
        }
    }
}
