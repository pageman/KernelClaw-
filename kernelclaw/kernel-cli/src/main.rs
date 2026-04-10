//! KernelClaw CLI - EXCEPTION-ONLY UX
//! Silent on success, noisy ONLY on failure

use clap::{Parser, Subcommand};
use std::process::ExitCode;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "kernelclaw")]
#[command(version = "1.0.3")]
#[command(about = "Agent kernel prototype", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize
    Init,
    /// Execute goal
    Run { goal: String },
    /// Show status
    Status,
    /// List receipts
    Receipts { count: Option<usize> },
    /// Start daemon
    Daemon,
    /// List improvement proposals
    ProposalList,
    /// Show proposal details
    ProposalShow { id: String },
    /// Approve a proposal
    ProposalApprove { id: String },
    /// Reject a proposal
    ProposalReject { id: String },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Init) => {
            init_kernelclaw();
            ExitCode::SUCCESS
        }
        
        Some(Commands::Status) => {
            let home = kernelclaw_home();
            // Only print on explicit request - this is OK
            println!("KernelClaw v1.0.3");
            println!("Home: {:?}", home);
            
            if let Ok(entries) = std::fs::read_dir(home.join("receipts")) {
                println!("Receipts: {}", entries.count());
            }
            
            ExitCode::SUCCESS
        }
        
        Some(Commands::Run { goal }) => {
            // EXCEPTION-ONLY: print only on error
            match run_goal(&goal) {
                Ok(receipt) => {
                    // Silent success - only the result
                    println!("{} -> {}", receipt.tool_name, receipt.result);
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("[EXCEPTION] {}", e);
                    ExitCode::FAILURE
                }
            }
        }
        
        Some(Commands::Receipts { count }) => {
            // EXCEPTION-ONLY
            match list_receipts(count.unwrap_or(10)) {
                Ok(entries) => {
                    for e in entries {
                        println!("{} | {} | {}", e.id, e.entry_type, e.content);
                    }
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("[EXCEPTION] {}", e);
                    ExitCode::FAILURE
                }
            }
        }
        
        Some(Commands::Daemon) => {
            // Start daemon
            let home = kernelclaw_home();
            let socket_path = home.join("kernel.sock");
            
            let config = kernel_daemon::DaemonConfig {
                socket_path: socket_path.clone(),
                max_pending: 10,
                timeout_ms: 30000,
            };
            
            match kernel_daemon::start_daemon(config) {
                Ok(_) => {
                    println!("Daemon started on {:?}", socket_path);
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("Failed to start daemon: {}", e);
                    ExitCode::FAILURE
                }
            }
        }
        
        // === IMPROVEMENT PROPOSAL COMMANDS ===
        
        Some(Commands::ProposalList) => {
            match list_proposals() {
                Ok(proposals) => {
                    if proposals.is_empty() {
                        println!("No proposals found.");
                    } else {
                        for p in proposals {
                            println!("{} - {} - {}", p.0, p.1, p.2);
                        }
                    }
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    ExitCode::FAILURE
                }
            }
        }
        
        Some(Commands::ProposalShow { id }) => {
            match show_proposal(&id) {
                Ok(details) => {
                    println!("{}", details);
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    ExitCode::FAILURE
                }
            }
        }
        
        Some(Commands::ProposalApprove { id }) => {
            match approve_proposal(&id) {
                Ok(_) => {
                    println!("Proposal {} approved and applied.", id);
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    ExitCode::FAILURE
                }
            }
        }
        
        Some(Commands::ProposalReject { id }) => {
            match reject_proposal(&id) {
                Ok(_) => {
                    println!("Proposal {} rejected.", id);
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    ExitCode::FAILURE
                }
            }
        }
        
        None => {
            println!("KernelClaw v1.0.3");
            ExitCode::SUCCESS
        }
    }
}

fn kernelclaw_home() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".kernelclaw")
}

fn init_kernelclaw() {
    let home = kernelclaw_home();
    std::fs::create_dir_all(home.join("keys")).ok();
    std::fs::create_dir_all(home.join("receipts")).ok();
    std::fs::create_dir_all(home.join("data")).ok();
}

fn run_goal(goal: &str) -> Result<kernel_core::ExecutionReceipt, String> {
    let home = kernelclaw_home();
    let policy_path = PathBuf::from("policy.yaml");
    let data_path = home.join("data");
    
    let policy = if policy_path.exists() {
        kernel_policy::load_policy(&policy_path).map_err(|e| e.to_string())?
    } else {
        kernel_policy::Policy::default()
    };
    
    let ledger = kernel_memory::MemoryLedger::new(data_path);
    let executor = kernel_exec::Executor::with_policy(policy.clone());
    let keypair = kernel_crypto::generate_keypair();
    
    let mut orchestrator = kernel_core::Orchestrator::new_with_components(
        policy,
        ledger,
        executor,
        keypair,
    );
    
    orchestrator.execute_goal(goal).map_err(|e| e.to_string())
}

fn list_receipts(count: usize) -> Result<Vec<kernel_memory::LedgerEntry>, String> {
    let home = kernelclaw_home();
    let data_path = home.join("data");
    let ledger = kernel_memory::MemoryLedger::new(data_path);
    let mut entries = ledger.get_all().map_err(|e| e.to_string())?;
    entries.truncate(count);
    Ok(entries)
}