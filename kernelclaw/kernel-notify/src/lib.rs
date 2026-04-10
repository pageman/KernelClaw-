//! KernelClaw Notifier
//! Exception-only notification system

use tokio::sync::mpsc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Notification {
    PlanParseFailure,
    PolicyViolation,
    ExecutionFailure,
    SecurityAnomaly,
}

pub struct Notifier {
    sender: mpsc::Sender<Notification>,
}

impl Notifier {
    pub fn new() -> (Self, mpsc::Receiver<Notification>) {
        let (sender, receiver) = mpsc::channel(100);
        (Notifier { sender }, receiver)
    }
    
    pub async fn notify(&self, notification: Notification) {
        let _ = self.sender.send(notification).await;
    }
    
    pub async fn handle_exceptions(&mut self, rx: &mut mpsc::Receiver<Notification>) {
        while let Some(notification) = rx.recv().await {
            eprintln!("[EXCEPTION] {:?}", notification);
        }
    }
}
