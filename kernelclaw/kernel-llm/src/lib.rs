//! KernelClaw LLM - STRONGLY TYPED goal interpreter
//! Returns parsed/validated struct, NOT raw string

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LlmError {
    #[error("Parse: {0}")]
    Parse(String),
    #[error("Validation: {0}")]
    Validation(String),
    #[error("Rejected: {0}")]
    Rejected(String),
}

/// STRUCTURED goal output - NOT just string
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedGoal {
    pub task_id: String,
    pub tool_name: String,
    pub parameters: serde_json::Value,
    pub justification: String,
    pub risk_level: String,
    pub required_capabilities: Vec<String>,
    pub expected_output_type: String,
}

/// Valid tools list - enforced
const VALID_TOOLS: &[&str] = &["file_read", "file_read_dir", "echo", "calendar_summary", "file_metadata", "health_check"];
const VALID_RISK: &[&str] = &["low", "medium", "high"];

impl ParsedGoal {
    /// STRICT validation - REQUIRED
    pub fn validate(&self) -> Result<(), LlmError> {
        // Check for rejection
        if self.task_id.to_lowercase() == "rejected" {
            return Err(LlmError::Rejected(self.justification.clone()));
        }
        
        // Validate tool_name - MUST be in allowed list
        if !VALID_TOOLS.contains(&self.tool_name.as_str()) {
            return Err(LlmError::Validation(format!("Invalid tool: {}", self.tool_name)));
        }
        
        // Validate risk_level
        if !VALID_RISK.contains(&self.risk_level.as_str()) {
            return Err(LlmError::Validation(format!("Invalid risk: {}", self.risk_level)));
        }
        
        // High risk requires justification
        if self.risk_level == "high" && self.justification.is_empty() {
            return Err(LlmError::Validation("High risk requires justification".to_string()));
        }
        
        Ok(())
    }
}

/// LLM Client interface
pub struct OllamaClient {
    model: String,
    // endpoint would be used with real HTTP client
}

impl OllamaClient {
    pub fn new(model: String) -> Self {
        OllamaClient { model }
    }
    
    /// Parse and validate goal - returns STRUCTURED output
    pub fn parse_goal(&self, goal: &str) -> Result<ParsedGoal, LlmError> {
        // NOTE: In production, this calls Ollama with structured output
        // For v0.1.3, we simulate the structure that would be parsed
        let parsed = ParsedGoal {
            task_id: format!("goal_{}", uuid::Uuid::new_v4()),
            tool_name: self.infer_tool(goal),
            parameters: serde_json::json!({ "input": goal }),
            justification: "Auto-generated from goal text".to_string(),
            risk_level: "low".to_string(),
            required_capabilities: vec!["file_read".to_string()],
            expected_output_type: "text".to_string(),
        };
        
        // VALIDATE - REQUIRED
        parsed.validate()?;
        
        Ok(parsed)
    }
    
    /// Simple tool inference
    fn infer_tool(&self, goal: &str) -> String {
        let lower = goal.to_lowercase();
        if lower.contains("read") || lower.contains("file") || lower.contains("list") {
            "file_read".to_string()
        } else if lower.contains("echo") || lower.contains("test") {
            "echo".to_string()
        } else {
            "file_read".to_string() // default
        }
    }
    
    pub fn local() -> Self {
        Self::new("llama3.2:3b".to_string())
    }
}