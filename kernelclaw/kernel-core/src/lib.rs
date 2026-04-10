//! KernelClaw Core
//! Main orchestration and goal handling

use serde::{Deserialize, Serialize};
use kernel_policy::Policy;
use kernel_memory::{MemoryLedger, EntryType};
use kernel_crypto::{SigningKeyPair, create_receipt};
use kernel_exec::Executor;
use kernel_llm::OllamaClient;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: String,
    pub raw: String,
    pub status: GoalStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalStatus {
    Pending, Parsing, Planned, Executing, Completed, Failed,
}

/// Main orchestrator
pub struct Orchestrator {
    pub policy: Policy,
    pub ledger: MemoryLedger,
    pub executor: Executor,
    pub keypair: Option<SigningKeyPair>,
    pub llm: Option<OllamaClient>,
}

impl Orchestrator {
    pub fn new(policy_path: PathBuf, data_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let policy = kernel_policy::load_policy(&policy_path)?;
        let ledger = MemoryLedger::new(data_path);
        
        Ok(Orchestrator {
            policy,
            ledger,
            executor: Executor::new(),
            keypair: None,
            llm: None,
        })
    }
    
    pub fn set_keypair(&mut self, keypair: SigningKeyPair) {
        self.keypair = Some(keypair);
    }
    
    pub fn set_llm(&mut self, client: OllamaClient) {
        self.llm = Some(client);
    }
    
    pub fn execute_goal(&mut self, raw_goal: &str) -> Result<kernel_crypto::Receipt, Box<dyn std::error::Error>> {
        let goal_id = uuid::Uuid::new_v4().to_string();
        
        let receipt = if let Some(ref kp) = self.keypair {
            create_receipt(raw_goal, "execute_goal", raw_goal, "completed", "success", kp)
        } else {
            return Err("No keypair configured".into());
        };
        
        self.ledger.append(
            EntryType::GoalOutcome,
            format!("Goal {} executed", goal_id),
            Some(receipt.id.clone()),
        );
        
        Ok(receipt)
    }
    
    pub fn get_ledger(&self) -> &MemoryLedger {
        &self.ledger
    }
}