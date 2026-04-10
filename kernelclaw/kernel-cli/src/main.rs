//! KernelClaw CLI - EXCEPTION-ONLY UX
//! Silent on success, noisy ONLY on failure

use clap::{Parser, Subcommand};
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "kernelclaw")]
#[command(version = "0.1.3")]
#[command(about = "Agent kernel with enforced constraints", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize (silent success)
    Init,
    /// Execute goal (silent success)
    Run { goal: String },
    /// Show status
    Status,
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
            // SILENT - no "Initialized!" message
            ExitCode::SUCCESS
        }
        
        Some(Commands::Status) => {
            // Only print on explicit request
            let home = dirs::home_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join(".kernelclaw");
            
            if !home.exists() {
                eprintln!("[EXCEPTION] Not initialized");
                return ExitCode::FAILURE;
            }
            
            println!("KernelClaw v0.1.3");
            println!("Home: {:?}", home);
            ExitCode::SUCCESS
        }
        
        Some(Commands::Run { goal }) => {
            // Would run full pipeline - silent success
            // v0.1.3: orchestration not wired to CLI yet
            eprintln!("[EXCEPTION] Run requires daemon mode");
            ExitCode::FAILURE
        }
        
        None => {
            println!("KernelClaw v0.1.3");
            ExitCode::SUCCESS
        }
    }
}