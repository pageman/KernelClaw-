//! KernelClaw Policy Engine - ENFORCED constraints
//! Policy checked at load time and at tool boundary

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

/// Policy schema - ENFORCED
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub version: String,
    #[serde(default)]
    pub invariants: Vec<Invariant>,
    #[serde(default)]
    pub requires_approval: Vec<String>,
    #[serde(default)]
    pub forbidden: Vec<String>,
    #[serde(default)]
    pub readonly_paths: Vec<String>,
    #[serde(default)]
    pub capability_allowlist: CapabilityAllowlist,
}

/// Invariant - MUST NEVER be violated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invariant {
    pub name: String,
    pub description: String,
}

/// Capability allowlist - ENFORCED
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CapabilityAllowlist {
    #[serde(default)]
    pub file_read: Option<FileReadConfig>,
    #[serde(default)]
    pub file_write: Option<CapabilityConfig>,
    #[serde(default)]
    pub shell: Option<CapabilityConfig>,
    #[serde(default)]
    pub calendar: Option<CapabilityConfig>,
    #[serde(default)]
    pub wasm: Option<CapabilityConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileReadConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub allowed_paths: Vec<String>,
    #[serde(default)]
    pub max_file_size_bytes: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityConfig {
    #[serde(default)]
    pub enabled: bool,
}

/// Load policy from YAML - ENFORCED at load time
pub fn load_policy(path: &Path) -> Result<Policy, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let policy: Policy = serde_yaml::from_str(&contents)?;
    
    // ENFORCE: Validate at load time
    validate_policy(&policy)?;
    
    Ok(policy)
}

/// Validate policy - fail fast if invalid
fn validate_policy(policy: &Policy) -> Result<(), Box<dyn std::error::Error>> {
    if policy.invariants.is_empty() {
        return Err("Policy must have at least one invariant".into());
    }
    Ok(())
}

/// Check if action REQUIRES approval - ENFORCED
pub fn requires_approval(policy: &Policy, action: &str) -> bool {
    policy.requires_approval.iter().any(|a| a.eq_ignore_ascii_case(action))
}

/// Check if action is FORBIDDEN - ENFORCED
pub fn is_forbidden(policy: &Policy, action: &str) -> bool {
    if policy.forbidden.iter().any(|a| a.eq_ignore_ascii_case(action)) {
        return true;
    }
    // Check invariants
    for inv in &policy.invariants {
        if action.to_lowercase().contains(&inv.name.to_lowercase()) {
            return true;
        }
    }
    false
}

/// Check if path is READONLY - ENFORCED
pub fn is_readonly(policy: &Policy, path: &str) -> bool {
    for readonly in &policy.readonly_paths {
        if path.starts_with(readonly) {
            return true;
        }
    }
    false
}

/// Check if capability is ALLOWED - ENFORCED
/// FIXED: Was tautology `is_empty() || !is_empty()` which always returns true
pub fn is_capability_allowed(policy: &Policy, capability: &str, target: Option<&str>) -> bool {
    // 1. Check invariants - violate = deny
    for inv in &policy.invariants {
        if capability.to_lowercase().contains(&inv.name.to_lowercase()) {
            return false;
        }
    }
    
    // 2. Check forbidden
    if is_forbidden(policy, capability) {
        return false;
    }
    
    // 3. Check capability allowlist - ACTUAL ENFORCEMENT
    match capability {
        "file_read" => {
            if let Some(ref config) = policy.capability_allowlist.file_read {
                if !config.enabled {
                    return false;
                }
                // FIXED: Proper check - if paths defined, check target
                if let Some(path) = target {
                    if !config.allowed_paths.is_empty() {
                        return config.allowed_paths.iter().any(|p| path.starts_with(p));
                    }
                }
                return true;
            }
            true
        }
        "file_write" => {
            if let Some(ref config) = policy.capability_allowlist.file_write {
                return config.enabled;
            }
            false // Default deny
        }
        "shell" => {
            if let Some(ref config) = policy.capability_allowlist.shell {
                return config.enabled;
            }
            false
        }
        "calendar" => {
            if let Some(ref config) = policy.capability_allowlist.calendar {
                return config.enabled;
            }
            false
        }
        "wasm" => {
            if let Some(ref config) = policy.capability_allowlist.wasm {
                return config.enabled;
            }
            false
        }
        _ => false,
    }
}

/// Get capability decision with reason
pub fn get_capability_decision(policy: &Policy, capability: &str, target: Option<&str>) -> (bool, String) {
    if is_capability_allowed(policy, capability, target) {
        (true, "Allowed".to_string())
    } else if requires_approval(policy, capability) {
        (false, "Requires approval".to_string())
    } else {
        (false, "Forbidden or not in allowlist".to_string())
    }
}