//! KernelClaw CLI Entry Point
//! Single-binary Rust daemon

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "kernelclaw")]
#[command(version = "0.1.0")]
#[command(about = "KernelClaw - Production-oriented agent kernel", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize KernelClaw environment
    Init {
        /// Override default config path
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
    /// Start daemon mode
    Daemon {
        /// Socket path (default: ~/.kernelclaw/socket)
        #[arg(short, long)]
        socket: Option<PathBuf>,
    },
    /// Run a single goal
    Run {
        /// Goal to execute
        #[arg(short, long)]
        goal: String,
    },
    /// Show status
    Status,
    /// List receipts
    Receipts {
        /// Number of recent receipts to show
        #[arg(short, long, default_value = "10")]
        count: usize,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { config } => {
            println!("Initializing KernelClaw...");
            init_kernelclaw(config).await?;
        }
        Commands::Daemon { socket } => {
            println!("Starting KernelClaw daemon...");
            start_daemon(socket).await?;
        }
        Commands::Run { goal } => {
            println!("Executing goal: {}", goal);
            run_goal(goal).await?;
        }
        Commands::Status => {
            show_status().await?;
        }
        Commands::Receipts { count } => {
            list_receipts(count).await?;
        }
    }

    Ok(())
}

async fn init_kernelclaw(_config: Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let kc_dir = home.join(".kernelclaw");
    
    println!("Creating KernelClaw home: {:?}", kc_dir);
    
    // Create directories
    std::fs::create_dir_all(kc_dir.join("keys"))?;
    std::fs::create_dir_all(kc_dir.join("receipts"))?;
    std::fs::create_dir_all(kc_dir.join("data"))?;
    
    // Create default policy
    let policy_path = kc_dir.join("policy.yaml");
    if !policy_path.exists() {
        std::fs::write(&policy_path, r#"# KernelClaw Default Policy
version: "0.1.0"
invariants:
  - no_network_access
requires_approval:
  - file_write
  - shell_exec
forbidden: []
readonly_paths: []
capability_allowlist:
  file_read:
    enabled: true
"#)?;
    }
    
    println!("✓ KernelClaw initialized at {:?}", kc_dir);
    Ok(())
}

async fn start_daemon(_socket: Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Daemon mode not yet implemented - see Phase 8");
    Ok(())
}

async fn run_goal(_goal: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Goal execution not yet implemented - see Phase 5-7");
    Ok(())
}

async fn show_status() -> Result<(), Box<dyn std::error::Error>> {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let kc_dir = home.join(".kernelclaw");
    
    println!("KernelClaw Status:");
    println!("  Home: {:?}", kc_dir);
    println!("  Daemon: Not running");
    println!("  Policy: {}", if kc_dir.join("policy.yaml").exists() { "OK" } else { "Not found" });
    println!("  Keys: {}", if kc_dir.join("keys").exists() { "OK" } else { "Not initialized" });
    
    Ok(())
}

async fn list_receipts(_count: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("Receipts listing not yet implemented - see Phase 8");
    Ok(())
}
