//! KernelClaw CLI - EXCEPTION-ONLY UX
//! Silent on success, noisy ONLY on failure

use clap::{Parser, Subcommand};
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "kernelclaw")]
#[command(version = "0.1.6")]
#[command(about = "Agent kernel prototype", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize (silent success)
    Init,
    /// Execute goal - STUBBED
    Run { goal: String },
    /// Show status
    Status,
    /// List receipts - STUBBED
    Receipts { count: Option<usize> },
    /// Start daemon - NOT IMPLEMENTED
    Daemon,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Init) => {
            // SILENT SUCCESS - no output
            let home = dirs::home_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join(".kernelclaw");
            
            if let Err(e) = std::fs::create_dir_all(home.join("keys")) { 
                eprintln!("[EXCEPTION] {}", e);
                return ExitCode::FAILURE;
            }
            if let Err(e) = std::fs::create_dir_all(home.join("receipts")) {
                eprintln!("[EXCEPTION] {}", e);
                return ExitCode::FAILURE;
            }
            if let Err(e) = std::fs::create_dir_all(home.join("data")) {
                eprintln!("[EXCEPTION] {}", e);
                return ExitCode::FAILURE;
            }
            // Silent - no "Initialized!" message
            ExitCode::SUCCESS
        }
        
        Some(Commands::Status) => {
            let home = dirs::home_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join(".kernelclaw");
            
            if !home.exists() {
                println!("KernelClaw v0.1.6 - Not initialized");
                return ExitCode::SUCCESS;
            }
            
            println!("KernelClaw v0.1.6");
            println!("Home: {:?}", home);
            
            // Count receipts if available
            if let Ok(entries) = std::fs::read_dir(home.join("receipts")) {
                let count = entries.count();
                println!("Receipts: {}", count);
            }
            
            ExitCode::SUCCESS
        }
        
        Some(Commands::Run { goal }) => {
            // STUBBED: Does not actually execute goal
            // Target: parse via LLM → validate policy → execute → sign receipt → append ledger
            eprintln!("[STUB] Goal execution not implemented");
            eprintln!("  - Would: parse goal via LLM, validate policy, execute, sign receipt");
            ExitCode::FAILURE
        }
        
        Some(Commands::Receipts { count: _ }) => {
            // STUBBED
            eprintln!("[STUB] Receipt listing not implemented");
            ExitCode::FAILURE
        }
        
        Some(Commands::Daemon) => {
            // NOT IMPLEMENTED
            eprintln!("[NOTIMPL] Daemon mode not implemented");
            eprintln!("  - Would require Unix socket listener");
            ExitCode::FAILURE
        }
        
        None => {
            println!("KernelClaw v0.1.6 - use 'init', 'status', or 'run <goal>'");
            ExitCode::SUCCESS
        }
    }
}