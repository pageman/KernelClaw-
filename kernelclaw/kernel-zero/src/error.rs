//! KernelClaw Zero-Dep - Error handling
//! Replaces thiserror with zero external dependencies

use std::error::Error as StdError;
use std::fmt;

/// Base error enum - all KernelClaw errors
#[derive(Debug)]
pub enum Error {
    // Policy errors
    PolicyLoad(String),
    PolicyInvalid(String),
    
    // Crypto errors  
    CryptoSign(String),
    CryptoVerify(String),
    
    // Memory/ledger errors
    LedgerRead(String),
    LedgerWrite(String),
    LedgerCorrupt(String),
    
    // Execution errors
    ExecDenied(String),
    ExecFailed(String),
    ExecToolNotFound(String),
    
    // LLM errors
    LlmParse(String),
    LlmValidation(String),
    LlmNetwork(String),
    
    // IO errors (wrapped std)
    Io(std::io::Error),
    
    // Parse errors
    Parse(String),
}

impl Error {
    pub fn policy_load(s: impl Into<String>) -> Self {
        Error::PolicyLoad(s.into())
    }
    
    pub fn policy_invalid(s: impl Into<String>) -> Self {
        Error::PolicyInvalid(s.into())
    }
    
    pub fn crypto_sign(s: impl Into<String>) -> Self {
        Error::CryptoSign(s.into())
    }
    
    pub fn crypto_verify(s: impl Into<String>) -> Self {
        Error::CryptoVerify(s.into())
    }
    
    pub fn ledger_read(s: impl Into<String>) -> Self {
        Error::LedgerRead(s.into())
    }
    
    pub fn ledger_write(s: impl Into<String>) -> Self {
        Error::LedgerWrite(s.into())
    }
    
    pub fn ledger_corrupt(s: impl Into<String>) -> Self {
        Error::LedgerCorrupt(s.into())
    }
    
    pub fn exec_denied(s: impl Into<String>) -> Self {
        Error::ExecDenied(s.into())
    }
    
    pub fn exec_failed(s: impl Into<String>) -> Self {
        Error::ExecFailed(s.into())
    }
    
    pub fn exec_tool(s: impl Into<String>) -> Self {
        Error::ExecToolNotFound(s.into())
    }
    
    pub fn llm_parse(s: impl Into<String>) -> Self {
        Error::LlmParse(s.into())
    }
    
    pub fn llm_validation(s: impl Into<String>) -> Self {
        Error::LlmValidation(s.into())
    }
    
    pub fn llm_network(s: impl Into<String>) -> Self {
        Error::LlmNetwork(s.into())
    }
    
    pub fn parse(s: impl Into<String>) -> Self {
        Error::Parse(s.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::PolicyLoad(s) => write!(f, "Policy load error: {}", s),
            Error::PolicyInvalid(s) => write!(f, "Policy invalid: {}", s),
            Error::CryptoSign(s) => write!(f, "Crypto sign error: {}", s),
            Error::CryptoVerify(s) => write!(f, "Crypto verify error: {}", s),
            Error::LedgerRead(s) => write!(f, "Ledger read error: {}", s),
            Error::LedgerWrite(s) => write!(f, "Ledger write error: {}", s),
            Error::LedgerCorrupt(s) => write!(f, "Ledger corrupt: {}", s),
            Error::ExecDenied(s) => write!(f, "Execution denied: {}", s),
            Error::ExecFailed(s) => write!(f, "Execution failed: {}", s),
            Error::ExecToolNotFound(s) => write!(f, "Tool not found: {}", s),
            Error::LlmParse(s) => write!(f, "LLM parse error: {}", s),
            Error::LlmValidation(s) => write!(f, "LLM validation error: {}", s),
            Error::LlmNetwork(s) => write!(f, "LLM network error: {}", s),
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Parse(s) => write!(f, "Parse error: {}", s),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<Error> for std::io::Error {
    fn from(e: Error) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    }
}