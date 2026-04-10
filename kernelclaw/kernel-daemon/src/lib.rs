//! KernelClaw Daemon - Unix socket server
//! Quick win implementation

use std::io::{Read, Write};
use std::os::unix::net::UnixListener;
use std::path::PathBuf;
use std::thread;
use std::sync::Arc;
use std::collections::HashMap;

/// Daemon configuration
pub structDaemonConfig {
    socket_path: PathBuf,
    max_pending: usize,
    timeout_ms: u64,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        DaemonConfig {
            socket_path: dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".kernelclaw/kernel.sock"),
            max_pending: 10,
            timeout_ms: 30000,
        }
    }
}

/// Protocol messages
#[derive(Debug, Clone)]
pub enum Message {
    Execute { goal: String },
    Receipts { count: usize },
    Status,
    Shutdown,
}

impl Message {
    pub fn parse(data: &str) -> Option<Message> {
        let data = data.trim();
        if data.starts_with("EXEC:") {
            Some(Message::Execute { goal: data[5..].to_string() })
        } else if data.starts_with("RECEIPTS:") {
            let count = data[9..].parse().unwrap_or(10);
            Some(Message::Receipts { count })
        } else if data == "STATUS" {
            Some(Message::Status)
        } else if data == "SHUTDOWN" {
            Some(Message::Shutdown)
        } else {
            None
        }
    }
    
    pub fn response(&self) -> String {
        match self {
            Message::Execute { goal } => format!("EXECUTING: {}\n", goal),
            Message::Receipts { count } => format!("RECEIPTS: {} pending\n", count),
            Message::Status => "STATUS: OK\n".to_string(),
            Message::Shutdown => "SHUTDOWN: OK\n".to_string(),
        }
    }
}

/// Start daemon with socket
pub fn start_daemon(config: DaemonConfig) -> Result<(), String> {
    // Create socket directory
    if let Some(parent) = config.socket_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    
    // Remove existing socket
    if config.socket_path.exists() {
        std::fs::remove_file(&config.socket_path).map_err(|e| e.to_string())?;
    }
    
    // Create listener
    let listener = UnixListener::bind(&config.socket_path)
        .map_err(|e| format!("Failed to bind socket: {}", e))?;
    
    println!("KernelClaw daemon listening on {:?}", config.socket_path);
    
    // Accept connections
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0u8; 4096];
                if let Ok(size) = stream.read(&mut buffer) {
                    let data = String::from_utf8_lossy(&buffer[..size]);
                    if let Some(msg) = Message::parse(&data) {
                        let response = msg.response();
                        let _ = stream.write_all(response.as_bytes());
                    }
                }
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }
    
    Ok(())
}

/// Client for daemon
pub struct DaemonClient {
    socket_path: PathBuf,
}

impl DaemonClient {
    pub fn new(socket_path: PathBuf) -> Self {
        DaemonClient { socket_path }
    }
    
    pub fn execute(&self, goal: &str) -> Result<String, String> {
        let mut stream = std::os::unix::net::UnixStream::connect(&self.socket_path)
            .map_err(|e| e.to_string())?;
        
        let request = format!("EXEC:{}\n", goal);
        stream.write_all(request.as_bytes()).map_err(|e| e.to_string())?;
        
        let mut response = String::new();
        stream.read_to_string(&mut response).map_err(|e| e.to_string())?;
        
        Ok(response)
    }
    
    pub fn status(&self) -> Result<String, String> {
        let mut stream = std::os::unix::net::UnixStream::connect(&self.socket_path)
            .map_err(|e| e.to_string())?;
        
        stream.write_all(b"STATUS\n").map_err(|e| e.to_string())?;
        
        let mut response = String::new();
        stream.read_to_string(&mut response).map_err(|e| e.to_string())?;
        
        Ok(response)
    }
}