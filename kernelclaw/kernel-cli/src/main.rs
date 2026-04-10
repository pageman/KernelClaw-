//! KernelClaw CLI - EXCEPTION-ONLY UX
//! Silent on success, noisy ONLY on failure

use clap::{Parser, Subcommand};
use std::process::ExitCode;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "kernelclaw")]
#[command(version = "1.0.1")]
#[command(about = "Agent kernel prototype", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize
    Init,
    /// Execute goal (WORKS in v1.0.1!)
    Run { goal: String },
    /// Show status
    Status,
    /// List receipts (WORKS!)
    Receipts { count: Option<usize> },
    /// Start daemon (NOT IMPLEMENTED)
    Daemon,
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
            println!("KernelClaw v1.0.1");
            println!("Home: {:?}", home);
            
            // Count receipts
            if let Ok(entries) = std::fs::read_dir(home.join("receipts")) {
                let count = entries.count();
                println!("Receipts: {}", count);
            }
            
            ExitCode::SUCCESS
        }
        
        Some(Commands::Run { goal }) => {
            // v1.0.1: ACTUALLY executes the goal!
            match run_goal(&goal) {
                Ok(receipt) => {
                    // Silent success - just print the result
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
            eprintln!("[NOTIMPL] Daemon mode - use Unix socket API");
            ExitCode::FAILURE
        }
        
        None => {
            println!("KernelClaw v1.0.1 - use 'init', 'status', 'run <goal>', or 'receipts'");
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

// v1.0.1: ACTUAL execution via orchestrator
fn run_goal(goal: &str) -> Result<kernel_core::ExecutionReceipt, String> {
    let home = kernelclaw_home();
    let policy_path = PathBuf::from("policy.yaml");
    let data_path = home.join("data");
    
    // Load policy from file or use default
    let policy = if policy_path.exists() {
        kernel_policy::load_policy(&policy_path).map_err(|e| e.to_string())?
    } else {
        kernel_policy::Policy::default()
    };
    
    // Initialize ledger with loaded policy (FIXED: now uses policy!)
    let ledger = kernel_memory::MemoryLedger::new(data_path);
    
    // Initialize executor WITH POLICY (FIXED!)
    let executor = kernel_exec::Executor::with_policy(policy.clone());
    
    // Generate keypair for receipts
    let keypair = kernel_crypto::generate_keypair();
    
    // Create orchestrator with loaded policy
    let mut orchestrator = kernel_core::Orchestrator::new_with_components(
        policy,
        ledger,
        executor,
        keypair,
    );
    
    // Execute full pipeline
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