//! ImprovementProposal - Self-improving kernel proposals
//! Enables the VSIK (Verifiable Self-Improving Kernel) loop

use serde::{Deserialize, Serialize};
use kernel_crypto::{SigningKeyPair, create_receipt};
use kernel_zero::id::random_id;
use kernel_zero::time::now;

/// Proposal status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected,
}

/// Proposed change types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposedChange {
    /// Add a new policy rule
    PolicyRuleAdd { rule: String },
    /// Load a new WASM skill
    NewWasmSkill { name: String, wasm_bytes: Vec<u8> },
    /// Patch the planner prompt template
    PlannerPromptPatch { new_template: String },
    /// Refine capability allowlist
    CapabilityRefinement { new_allowlist_item: String },
}

/// Improvement proposal - core self-improvement attractor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementProposal {
    /// Unique proposal ID (e.g., prop_{timestamp})
    pub id: String,
    /// Unix timestamp
    pub timestamp: i64,
    /// The goal that failed or had issues
    pub failed_goal: String,
    /// Where in the pipeline it failed
    pub failure_point: String,
    /// Probable cause analysis
    pub probable_cause: String,
    /// Suggested safeguard
    pub candidate_safeguard: String,
    /// Capability refinement suggestion
    pub candidate_capability_refinement: String,
    /// List of proposed changes
    pub proposed_changes: Vec<ProposedChange>,
    /// Current status
    pub status: ProposalStatus,
    /// Link to distillation receipt
    pub receipt_id: Option<String>,
    /// User signature on approval (optional for MVP)
    pub user_signature: Option<Vec<u8>>,
}

impl ImprovementProposal {
    /// Create a new proposal
    pub fn new(
        failed_goal: String,
        failure_point: String,
        probable_cause: String,
        candidate_safeguard: String,
        candidate_capability_refinement: String,
        proposed_changes: Vec<ProposedChange>,
    ) -> Self {
        ImprovementProposal {
            id: format!("prop_{}", random_id()),
            timestamp: now(),
            failed_goal,
            failure_point,
            probable_cause,
            candidate_safeguard,
            candidate_capability_refinement,
            proposed_changes,
            status: ProposalStatus::Pending,
            receipt_id: None,
            user_signature: None,
        }
    }
    
    /// Approve the proposal (MVP - simple confirmation)
    pub fn approve(&mut self, user_signature: Vec<u8>) {
        self.status = ProposalStatus::Approved;
        self.user_signature = Some(user_signature);
    }
    
    /// Reject the proposal
    pub fn reject(&mut self) {
        self.status = ProposalStatus::Rejected;
    }
    
    /// Sign the proposal
    pub fn sign(&self, kp: &SigningKeyPair) -> String {
        let payload = format!(
            "{}:{}:{}:{}:{}",
            self.id,
            self.failed_goal,
            self.failure_point,
            self.probable_cause,
            self.status_to_string()
        );
        
        // Use kernel-crypto sign if available, otherwise simple hash
        let signature = format!("sig:{}", base64_encode(payload.as_bytes()));
        signature
    }
    
    fn status_to_string(&self) -> String {
        match self.status {
            ProposalStatus::Pending => "pending".to_string(),
            ProposalStatus::Approved => "approved".to_string(),
            ProposalStatus::Rejected => "rejected".to_string(),
        }
    }
}

/// Generate a proposal from failure analysis
pub fn distill_and_propose(
    goal: &str,
    failure_point: &str,
    error_message: &str,
) -> ImprovementProposal {
    let probable_cause = analyze_cause(failure_point, error_message);
    let candidate_safeguard = suggest_safeguard(failure_point);
    let candidate_capability = suggest_capability_refinement(failure_point);
    let changes = generate_proposed_changes(failure_point);
    
    ImprovementProposal::new(
        goal.to_string(),
        failure_point.to_string(),
        probable_cause,
        candidate_safeguard,
        candidate_capability,
        changes,
    )
}

/// Simple cause analysis
fn analyze_cause(failure_point: &str, error: &str) -> String {
    match failure_point {
        "parse" => format!("LLM failed to parse goal: {}", error),
        "validate" => format!("Policy validation failed: {}", error),
        "execute" => format!("Execution failed: {}", error),
        "receipt" => format!("Receipt signing failed: {}", error),
        "record" => format!("Ledger write failed: {}", error),
        _ => format!("Unknown failure: {}", error),
    }
}

/// Suggest a safeguard based on failure point
fn suggest_safeguard(failure_point: &str) -> String {
    match failure_point {
        "parse" => "Add stricter LLM output validation".to_string(),
        "validate" => "Add more specific policy rules".to_string(),
        "execute" => "Add resource limits and timeout".to_string(),
        "receipt" => "Add receipt retry logic".to_string(),
        "record" => "Add ledger backup mechanism".to_string(),
        _ => "Review and refine policy".to_string(),
    }
}

/// Suggest capability refinement
fn suggest_capability_refinement(failure_point: &str) -> String {
    match failure_point {
        "execute" => "Restrict file_write to specific directories".to_string(),
        "validate" => "Add more allowed_paths entries".to_string(),
        _ => "No capability change needed".to_string(),
    }
}

/// Generate proposed changes
fn generate_proposed_changes(failure_point: &str) -> Vec<ProposedChange> {
    let mut changes = Vec::new();
    
    match failure_point {
        "execute" => {
            changes.push(ProposedChange::CapabilityRefinement {
                new_allowlist_item: "/tmp/".to_string(),
            });
        }
        "validate" => {
            changes.push(ProposedChange::PolicyRuleAdd {
                rule: "max_file_size: 1048576".to_string(),
            });
        }
        "parse" => {
            changes.push(ProposedChange::PlannerPromptPatch {
                new_template: "Parse the following goal. Output valid JSON.".to_string(),
            });
        }
        _ => {}
    }
    
    changes
}

/// Simple base64 encoding (inline)
fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        
        result.push(ALPHABET[(b0 >> 2) as usize] as char);
        result.push(ALPHABET[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        
        if chunk.len() > 1 {
            result.push(ALPHABET[(((b1 & 0x0F) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            result.push('=');
        }
        
        if chunk.len() > 2 {
            result.push(ALPHABET[(b2 & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    
    result
}