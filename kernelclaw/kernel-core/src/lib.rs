//! KernelClaw Core - FULL orchestration pipeline
//! parse -> validate -> execute -> receipt -> record

use serde::{Deserialize, Serialize};
use kernel_policy::Policy;
use kernel_memory::{MemoryLedger, EntryType};
use kernel_crypto::{SigningKeyPair, create_receipt};
use kernel_exec::Executor;
use kernel_llm::{OllamaClient, ParsedGoal};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: String,
    pub raw: String,
    pub parsed: Option<ParsedGoal>,
    pub status: GoalStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalStatus {
    Pending, Parsing, Planned, Executing, Executed, Completed, Failed,
}

/// Receipt from execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReceipt {
    pub goal_id: String,
    pub tool_name: String,
    pub result: String,
    pub timestamp: i64,
}

/// Main orchestrator - FULL pipeline
pub struct Orchestrator {
    policy: Policy,
    ledger: MemoryLedger,
    executor: Executor,
    keypair: Option<SigningKeyPair>,
    llm: Option<OllamaClient>,
    data_path: std::path::PathBuf,
}

impl Orchestrator {
    pub fn new(policy_path: std::path::PathBuf, data_path: std::path::PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let policy = kernel_policy::load_policy(&policy_path)?;
        let ledger = MemoryLedger::new(data_path.clone());
        
        Ok(Orchestrator {
            policy,
            ledger,
            executor: Executor::new(),
            keypair: None,
            llm: None,
            data_path,
        })
    }
    
    pub fn set_keypair(&mut self, keypair: SigningKeyPair) {
        self.keypair = Some(keypair);
    }
    
    pub fn set_llm(&mut self, client: OllamaClient) {
        self.llm = Some(client);
    }
    
    /// FULL pipeline: parse -> validate -> execute -> receipt -> record
    pub fn execute_goal(&mut self, raw_goal: &str) -> Result<ExecutionReceipt, Box<dyn std::error::Error>> {
        let goal_id = uuid::Uuid::new_v4().to_string();
        
        // Stage 1: PARSE - Typed parsing via LLM
        let parsed = if let Some(ref llm) = self.llm {
            llm.parse_goal(raw_goal)?
        } else {
            return Err("LLM not configured".into());
        };
        
        // Stage 2: VALIDATE - Check policy allows this tool
        if !kernel_policy::is_capability_allowed(&self.policy, &parsed.tool_name, None) {
            return Err(format!("Tool {} not allowed by policy", parsed.tool_name).into());
        }
        
        // Stage 3: EXECUTE - Through capability-gated executor
        let exec_result = self.executor.execute(&kernel_exec::ExecRequest {
            capabilities: vec![kernel_exec::Capability::FileRead(parsed.tool_name.clone())],
            tool_name: parsed.tool_name.clone(),
            params: parsed.parameters.clone(),
        })?;
        
        if !exec_result.success {
            return Err(format!("Execution failed: {:?}", exec_result.error).into());
        }
        
        // Stage 4: RECEIPT - Sign execution record
        let receipt = if let Some(ref kp) = self.keypair {
            create_receipt(raw_goal, "execute_goal", &exec_result.output, "completed", "success", kp)?
        } else {
            return Err("No keypair configured".into());
        };
        
        // Stage 5: RECORD - Append to ledger
        self.ledger.append(
            EntryType::GoalOutcome,
            format!("Goal {} executed: {}", goal_id, parsed.tool_name),
            Some(receipt.id.clone()),
        )?;
        
        Ok(ExecutionReceipt {
            goal_id,
            tool_name: parsed.tool_name,
            result: exec_result.output,
            timestamp: chrono::Utc::now().timestamp(),
        })
    }
    
    pub fn get_ledger(&self) -> &MemoryLedger {
        &self.ledger
    }
}