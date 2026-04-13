//! KernelClaw LLM - Ollama HTTP client for structured parsing
//! Calls POST http://localhost:11434/api/generate with format: json

use serde::{Deserialize, Serialize};
use kernel_zero::error::Error;

#[derive(Debug)]
pub enum LlmError {
    #[error("Parse: {0}")]
    Parse(String),
    #[error("Validation: {0}")]
    Validation(String),
    #[error("Network: {0}")]
    Network(String),
    #[error("Config: {0}")]
    Config(String),
}

/// STRUCTURED goal output
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

/// Tool to capability mapping - FIXED
fn tool_to_capabilities(tool: &str) -> Vec<String> {
    match tool {
        "file_read" => vec!["file_read".to_string()],
        "file_read_dir" => vec!["file_read".to_string()],
        "file_write" => vec!["file_write".to_string()],
        "echo" => vec!["echo".to_string()],
        "calendar_summary" => vec!["calendar".to_string()],
        "file_metadata" => vec!["file_read".to_string()],
        "health_check" => vec!["health".to_string()],
        _ => vec!["file_read".to_string()],
    }
}

/// Valid tools list
const VALID_TOOLS: &[&str] = &[
    "file_read", 
    "file_read_dir", 
    "file_write",
    "echo", 
    "calendar_summary", 
    "file_metadata", 
    "health_check"
];
const VALID_RISK: &[&str] = &["low", "medium", "high"];

impl ParsedGoal {
    /// Validate parsed goal
    pub fn validate(&self) -> Result<(), LlmError> {
        if self.task_id.to_lowercase() == "rejected" {
            return Err(LlmError::Validation("Goal rejected".to_string()));
        }
        if !VALID_TOOLS.contains(&self.tool_name.as_str()) {
            return Err(LlmError::Validation(format!("Invalid tool: {}", self.tool_name)));
        }
        if !VALID_RISK.contains(&self.risk_level.as_str()) {
            return Err(LlmError::Validation(format!("Invalid risk: {}", self.risk_level)));
        }
        if self.risk_level == "high" && self.justification.is_empty() {
            return Err(LlmError::Validation("High risk requires justification".to_string()));
        }
        Ok(())
    }
}

/// Configuration for LLM
#[derive(Debug, Clone)]
pub struct LlmConfig {
    pub endpoint: String,
    pub model: String,
    pub system_prompt: String,
}

impl Default for LlmConfig {
    fn default() -> Self {
        // Read from environment or use defaults
        Self {
            endpoint: std::env::var("KERNELCLAW_OLLAMA_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:11434".to_string()),
            model: std::env::var("KERNELCLAW_MODEL")
                .unwrap_or_else(|_| "gemma4:e2b".to_string()),
            system_prompt: KERNELCLAW_SYSTEM.to_string(),
        }
    }
}

/// The KernelClaw system prompt for Ollama
const KERNELCLAW_SYSTEM: &str = r#"You are a goal parser for KernelClaw, an agent kernel.
Given a natural-language goal, respond with ONLY a valid JSON object (no markdown fences).
Available tools: file_read, file_read_dir, file_write, echo, calendar_summary, file_metadata, health_check
Risk levels: low, medium, high

Output JSON with these exact fields:
- task_id: unique goal identifier
- tool_name: ONE of the available tools
- parameters: JSON object with tool-specific parameters
- justification: ONE sentence explaining why this tool
- risk_level: low/medium/high
- required_capabilities: capabilities needed for this tool (derived from tool_name)
- expected_output_type: "text" or "json"

IMPORTANT: required_capabilities must match the tool:
- file_read, file_read_dir, file_metadata -> ["file_read"]
- file_write -> ["file_write"]
- echo, calendar_summary, health_check -> ["echo"]

Respond with ONLY JSON, no markdown fences."#;

/// LLM Client with real HTTP calls
pub struct OllamaClient {
    config: LlmConfig,
    http_client: Option<reqwest::Client>,
}

impl OllamaClient {
    pub fn new(endpoint: String, model: String) -> Self {
        let config = LlmConfig {
            endpoint,
            model,
            system_prompt: KERNELCLAW_SYSTEM.to_string(),
        };
        OllamaClient {
            config,
            http_client: None,
        }
    }
    
    pub fn from_config(config: LlmConfig) -> Self {
        OllamaClient {
            config,
            http_client: None,
        }
    }
    
    pub fn local() -> Self {
        Self::default()
    }
    
    /// Parse goal via Ollama HTTP API
    pub fn parse_goal(&self, goal: &str) -> Result<ParsedGoal, LlmError> {
        // Try HTTP call first
        if let Ok(parsed) = self.parse_via_http(goal) {
            return parsed;
        }
        
        // Fallback to rule-based parsing
        self.parse_fallback(goal)
    }
    
    /// Real HTTP call to Ollama /api/generate
    fn parse_via_http(&self, goal: &str) -> Result<ParsedGoal, LlmError> {
        let client = match &self.http_client {
            Some(c) => c,
            None => {
                let c = reqwest::Client::new().map_err(|e| LlmError::Network(e.to_string()))?;
                &self.http_client.insert(c)
            }
        };
        
        let request = serde_json::json!({
            "model": self.config.model,
            "prompt": goal,
            "system": self.config.system_prompt,
            "format": "json",
            "stream": false,
        });
        
        // Make HTTP POST request
        let response = client
            .post(&format!("{}/api/generate", self.config.endpoint))
            .json(&request)
            .send()
            .map_err(|e| LlmError::Network(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(LlmError::Network(format!("Ollama returned {}", response.status())));
        }
        
        let json: serde_json::Value = response
            .json()
            .map_err(|e| LlmError::Parse(e.to_string()))?;
        
        let content = json.get("response")
            .and_then(|v| v.as_str())
            .ok_or_else(|| LlmError::Parse("No response".to_string()))?;
        
        // Parse JSON from response
        let parsed: ParsedGoal = serde_json::from_str(content)
            .map_err(|e| LlmError::Parse(e.to_string()))?;
        
        // FIXED: Derive capabilities from tool, not hardcoded
        let caps = tool_to_capabilities(&parsed.tool_name);
        let parsed = ParsedGoal {
            required_capabilities: caps,
            ..parsed
        };
        
        parsed.validate()?;
        Ok(parsed)
    }
    
    /// Fallback rule-based parsing
    fn parse_fallback(&self, goal: &str) -> Result<ParsedGoal, LlmError> {
        let tool_name = self.infer_tool(goal);
        let risk_level = self.infer_risk(goal);
        let caps = tool_to_capabilities(&tool_name);
        
        let parsed = ParsedGoal {
            task_id: format!("goal_{}", kernel_zero::time::now()),
            tool_name,
            parameters: serde_json::json!({ "input": goal }),
            justification: format!("Parsed from: {}", goal),
            risk_level,
            required_capabilities: caps,
            expected_output_type: "text".to_string(),
        };
        
        parsed.validate()?;
        Ok(parsed)
    }
    
    fn infer_tool(&self, goal: &str) -> String {
        let lower = goal.to_lowercase();
        if lower.contains("write") || lower.contains("create") {
            "file_write".to_string()
        } else if lower.contains("list") || lower.contains("dir") {
            "file_read_dir".to_string()
        } else if lower.contains("metadata") || lower.contains("info") {
            "file_metadata".to_string()
        } else if lower.contains("calendar") {
            "calendar_summary".to_string()
        } else if lower.contains("health") || lower.contains("status") {
            "health_check".to_string()
        } else if lower.contains("echo") || lower.contains("test") {
            "echo".to_string()
        } else {
            "file_read".to_string()
        }
    }
    
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
}