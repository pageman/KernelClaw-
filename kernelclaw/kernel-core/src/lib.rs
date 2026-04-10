//! KernelClaw Core - FULL orchestration pipeline
//! parse -> validate -> execute -> receipt -> record

use serde::{Deserialize, Serialize};
use kernel_policy::Policy;
use kernel_memory::{MemoryLedger, EntryType};
use kernel_crypto::{SigningKeyPair, create_receipt};
use kernel_exec::Executor;
use kernel_llm::{OllamaClient, ParsedGoal};

/// Goal status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalStatus {
    Pending, Parsing, Planned, Executing, Executed, Completed, Failed,
}

/// Goal with structured plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: String,
    pub raw: String,
    pub parsed: Option<ParsedGoal>,
    pub status: GoalStatus,
}

/// Execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReceipt {
    pub goal_id: String,
    pub tool_name: String,
    pub result: String,
    pub timestamp: i64,
}

/// Main orchestrator - FULL pipeline with POLICY
pub struct Orchestrator {
    policy: Policy,
    ledger: MemoryLedger,
    executor: Executor,
    keypair: Option<SigningKeyPair>,
    llm: Option<OllamaClient>,
    data_path: std::path::PathBuf,
}

impl Orchestrator {
    /// Create with loaded policy - FIXED to use policy!
    pub fn new(policy_path: std::path::PathBuf, data_path: std::path::PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let policy = kernel_policy::load_policy(&policy_path)?;
        let ledger = MemoryLedger::new(data_path);
        
        // FIXED: Use executor WITH policy!
        let executor = Executor::with_policy(policy.clone());
        
        Ok(Orchestrator {
            policy,
            ledger,
            executor,
            keypair: None,
            llm: None,
            data_path,
        })
    }
    
    /// Create with explicit components - also uses policy
    pub fn new_with_components(
        policy: Policy,
        ledger: MemoryLedger,
        executor: Executor,
        keypair: SigningKeyPair,
    ) -> Self {
        Orchestrator {
            policy,
            ledger,
            executor,
            keypair: Some(keypair),
            llm: None,
            data_path: std::path::PathBuf::new(),
        }
    }
    
    pub fn set_keypair(&mut self, keypair: SigningKeyPair) {
        self.keypair = Some(keypair);
    }
    
    pub fn set_llm(&mut self, client: OllamaClient) {
        self.llm = Some(client);
    }
    
    /// FULL pipeline: parse -> validate -> execute -> receipt -> record
    pub fn execute_goal(&mut self, raw_goal: &str) -> Result<ExecutionReceipt, Box<dyn std::error::Error>> {
        let goal_id = kernel_zero::id::random_id();
        
        // Stage 1: PARSE - Typed parsing via LLM
        let parsed = if let Some(ref llm) = self.llm {
            Some(llm.parse_goal(raw_goal)?)
        } else {
            None
        };
        
        // If parsed, validate and execute with loaded policy
        if let Some(ref parsed_goal) = parsed {
            // Stage 2: VALIDATE - Check policy allows this tool with target path
            let tool_name = &parsed_goal.tool_name;
            
            // FIXED: Check capability with actual target path from parameters
            let target_path = parsed_goal.parameters.get("path")
                .and_then(|v| v.as_str());
            
            if !kernel_policy::is_capability_allowed(&self.policy, tool_name, target_path) {
                return Err(format!("Tool {} not allowed by policy", tool_name).into());
            }
            
            // Stage 3: EXECUTE - Through policy-gated executor (now uses policy!)
            let exec_result = self.executor.execute(&kernel_exec::ExecRequest {
                capabilities: vec![kernel_exec::Capability::FileRead(tool_name.clone())],
                tool_name: tool_name.clone(),
                params: parsed_goal.parameters.clone(),
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
            
            // Stage 5: RECORD - Append to durable ledger
            self.ledger.append(
                EntryType::GoalOutcome,
                format!("Goal {} executed: {}", goal_id, parsed_goal.tool_name),
                Some(receipt.id.clone()),
            )?;
            
            Ok(ExecutionReceipt {
                goal_id,
                tool_name: parsed_goal.tool_name.clone(),
                result: exec_result.output,
                timestamp: kernel_zero::time::now(),
            })
        } else {
            // No LLM - stubbed execution
            let receipt = if let Some(ref kp) = self.keypair {
                create_receipt(raw_goal, "execute_goal", "stubbed_no_llm", "completed", "no_llm", kp)?
            } else {
                return Err("No keypair configured".into());
            };
            
            self.ledger.append(
                EntryType::GoalOutcome,
                format!("Goal {} stubbed (no LLM)", goal_id),
                Some(receipt.id.clone()),
            )?;
            
            Ok(ExecutionReceipt {
                goal_id,
                tool_name: "none".to_string(),
                result: "stubbed_no_llm".to_string(),
                timestamp: kernel_zero::time::now(),
            })
        }
    }
    
    pub fn get_ledger(&self) -> &MemoryLedger {
        &self.ledger
    }
    
    pub fn get_receipts(&self) -> Result<Vec<kernel_memory::LedgerEntry>, String> {
        self.ledger.get_all()
    }
}