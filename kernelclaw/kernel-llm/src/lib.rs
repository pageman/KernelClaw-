//! KernelClaw LLM Adapter
//! Goal interpretation via local Ollama

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LlmError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Model error: {0}")]
    Model(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedGoal {
    pub task_id: String,
    pub tool_name: String,
    pub parameters: serde_json::Value,
    pub justification: String,
    pub risk_level: String,
    pub required_capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalPlan {
    pub goals: Vec<ParsedGoal>,
    pub uncertainty_notes: Vec<String>,
}

pub struct OllamaClient {
    endpoint: String,
    model: String,
}

impl OllamaClient {
    pub fn new(endpoint: String, model: String) -> Self {
        Self { endpoint, model }
    }
    
    pub async fn generate(&self, prompt: &str) -> Result<String, LlmError> {
        let client = reqwest::Client::new();
        let resp = client.post(&format!("{}/api/generate", self.endpoint))
            .json(&serde_json::json!({
                "model": self.model,
                "prompt": prompt,
                "stream": false
            }))
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;
            
        let json: serde_json::Value = resp.json().await.map_err(|e| LlmError::Parse(e.to_string()))?;
        json.get("response").and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| LlmError::Parse("No response".to_string()))
    }
}
