//! KernelClaw Policy Engine
//! Declarative policy loading and evaluation

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

/// Policy schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub version: String,
    #[serde(default)]
    pub invariants: Vec<String>,
    #[serde(default)]
    pub requires_approval: Vec<String>,
    #[serde(default)]
    pub forbidden: Vec<String>,
    #[serde(default)]
    pub readonly_paths: Vec<String>,
    #[serde(default)]
    pub capability_allowlist: CapabilityAllowlist,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CapabilityAllowlist {
    #[serde(default)]
    pub file_read: Option<FileReadConfig>,
    #[serde(default)]
    pub calendar_read: Option<CapabilityConfig>,
    #[serde(default)]
    pub email_read_local: Option<CapabilityConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileReadConfig {
    #[serde(default)]
    pub allowed_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityConfig {
    #[serde(default)]
    pub enabled: bool,
}

/// Load policy from YAML file
pub fn load_policy(path: &Path) -> Result<Policy, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let policy: Policy = serde_yaml::from_str(&contents)?;
    Ok(policy)
}

/// Check if action requires approval
pub fn requires_approval(policy: &Policy, action: &str) -> bool {
    policy.requires_approval.iter().any(|a| a == action)
}

/// Check if action is forbidden
pub fn is_forbidden(policy: &Policy, action: &str) -> bool {
    policy.forbidden.iter().any(|a| a == action)
}

/// Check if path is readonly
pub fn is_readonly(policy: &Policy, path: &str) -> bool {
    policy.readonly_paths.iter().any(|p| path.starts_with(p))
}

/// Check if capability is allowed
pub fn is_capability_allowed(policy: &Policy, capability: &str) -> bool {
    // Check invariants
    if policy.invariants.iter().any(|i| i == capability) {
        return false;
    }
    
    // Check allowlist
    if let Some(ref allowlist) = policy.capability_allowlist.file_read {
        if capability == "file_read" {
            return allowlist.allowed_paths.is_empty() || !allowlist.allowed_paths.is_empty();
        }
    }
    
    if let Some(ref cal) = policy.capability_allowlist.calendar_read {
        if capability == "calendar_read" {
            return cal.enabled;
        }
    }
    
    true
}