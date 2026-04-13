//! KernelClaw Policy Engine - ENFORCED constraints
//! Unified policy used by both kernel-exec and kernel-core

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

/// Capability definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub allowed: bool,
    #[serde(default)]
    pub allowed_paths: Vec<String>,
    #[serde(default)]
    pub requires_approval: bool,
}

/// Unified Policy - used by kernel-exec
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub version: String,
    #[serde(default)]
    pub capabilities: Vec<Capability>,
    #[serde(default)]
    pub allowed_paths: Vec<String>,
    #[serde(default)]
    pub max_file_size: u64,
}

impl Default for Policy {
    fn default() -> Self {
        Policy {
            version: "0.1.0".to_string(),
            capabilities: vec![
                Capability { name: "file_read".to_string(), allowed: true, allowed_paths: vec![], requires_approval: false },
                Capability { name: "file_write".to_string(), allowed: false, allowed_paths: vec![], requires_approval: true },
                Capability { name: "shell".to_string(), allowed: false, allowed_paths: vec![], requires_approval: true },
                Capability { name: "echo".to_string(), allowed: true, allowed_paths: vec![], requires_approval: false },
                Capability { name: "calendar".to_string(), allowed: false, allowed_paths: vec![], requires_approval: false },
            ],
            allowed_paths: vec!["/tmp/".to_string(), "/var/tmp/".to_string()],
            max_file_size: 1024 * 1024,
        }
    }
}

/// Load policy from YAML
pub fn load_policy(path: &Path) -> Result<Policy, Box<dyn std::error::Error>> {
    if !path.exists() {
        return Ok(Policy::default());
    }
    let contents = fs::read_to_string(path)?;
    let policy: Policy = serde_yaml::from_str(&contents)?;
    Ok(policy)
}

/// Get capability config
pub fn get_capability(policy: &Policy, name: &str) -> Option<&Capability> {
    policy.capabilities.iter().find(|c| c.name == name)
}

/// Check if path is allowed for a capability
pub fn is_path_allowed(policy: &Policy, path: &str, capability: &str) -> bool {
    if policy.allowed_paths.is_empty() {
        return true;
    }
    
    if let Some(cap) = get_capability(policy, capability) {
        if !cap.allowed_paths.is_empty() {
            return cap.allowed_paths.iter().any(|p| path.starts_with(p));
        }
    }
    
    policy.allowed_paths.iter().any(|p| path.starts_with(p))
}

/// Check if capability is allowed
pub fn is_capability_allowed(policy: &Policy, name: &str) -> bool {
    get_capability(policy, name).map(|c| c.allowed).unwrap_or(false)
}

/// Check if capability requires approval
pub fn requires_approval(policy: &Policy, name: &str) -> bool {
    get_capability(policy, name).map(|c| c.requires_approval).unwrap_or(false)
}