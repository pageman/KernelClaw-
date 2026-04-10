//! KernelClaw LLM - TYPED goal parser with validation
//! Schema enforced, NOT just raw string

use serde::{Deserialize, Serialize};
use kernel_zero::error::Error;

##[derive(Debug)]
pub enum LlmError {
    #[error("Parse: {0}")]
    Parse(String),
    #[error("Validation: {0}")]
    Validation(String),
    #[error("Network: {0}")]
    Network(String),
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
            return Err(LlmError::Validation("Goal rejected".to_string()));
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

/// LLM Client - with wired parsing
pub struct OllamaClient {
    endpoint: String,
    model: String,
}

impl OllamaClient {
    pub fn new(endpoint: String, model: String) -> Self {
        OllamaClient { endpoint, model }
    }
    
    /// Parse goal with STRUCTURED validation - NOT just raw string
    pub fn parse_goal(&self, goal: &str) -> Result<ParsedGoal, LlmError> {
        // In production: call Ollama with structured output
        // For v0.1.6: Use rule-based parsing
        
        let tool_name = self.infer_tool(goal);
        let risk_level = self.infer_risk(goal);
        
        let parsed = ParsedGoal {
            task_id: format!("goal_{}", random_id()),
            tool_name,
            parameters: serde_json::json!({ "input": goal }),
            justification: format!("Auto-parsed from: {}", goal),
            risk_level,
            required_capabilities: vec!["file_read".to_string()],
            expected_output_type: "text".to_string(),
        };
        
        // VALIDATE - REQUIRED, NOT optional
        parsed.validate()?;
        
        Ok(parsed)
    }
    
    /// Simple tool inference
    fn infer_tool(&self, goal: &str) -> String {
        let lower = goal.to_lowercase();
        if lower.contains("read") || lower.contains("file") {
            "file_read".to_string()
        } else if lower.contains("list") || lower.contains("dir") {
            "file_read_dir".to_string()
        } else if lower.contains("echo") || lower.contains("test") {
            "echo".to_string()
        } else {
            "file_read".to_string()
        }
    }
    
    /// Simple risk inference
    fn infer_risk(&self, goal: &str) -> String {
        let lower = goal.to_lowercase();
        if lower.contains("write") || lower.contains("delete") || lower.contains("sudo") {
            "high".to_string()
        } else if lower.contains("read") {
            "low".to_string()
        } else {
            "medium".to_string()
        }
    }
    
    pub fn local() -> Self {
        Self::new("http://localhost:11434".to_string(), "llama3.2:3b".to_string())
    }
}