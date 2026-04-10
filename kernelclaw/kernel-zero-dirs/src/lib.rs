//! kernel-zero-dirs - Directories and paths (FULL version)
//! idiomatic Rust with bells and whistles
//! 
//! # Features
//! - XDG Base Directory Specification compliant
//! - Zero dependencies
//! - Idiomatic Rust with Result types
//! - Extension traits for PathBuf
//! - Builder pattern for custom paths
//! - Cross-platform support (Linux, macOS, Windows)
//! - Memory-efficient iterators
//! 
//! # Quick Start
//! 
//! ```rust
//! use kernel_zero_dirs::{home_dir, config_dir, data_dir};
//! 
//! let config = config_dir().unwrap().join("myapp");
//! ```

use std::env;
use std::path::{Path, PathBuf};

/// Result type for directory operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error type
#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    msg: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for Error {}

impl Error {
    /// Create a new error
    pub fn new(msg: impl Into<String>) -> Self {
        Error { msg: msg.into() }
    }
}

/// Error kind for directory operations
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    /// Environment variable not set
    NotFound,
    /// Invalid UTF-8 in path
    InvalidUtf8,
    /// Permission denied
    PermissionDenied,
    /// IO error
    Io(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error { msg: e.to_string() }
    }
}

// ============================================================================
// CORE FUNCTIONS
// ============================================================================

/// Get home directory
/// 
/// Returns the home directory of the current user.
/// This is equivalent to `$HOME` on Unix-like systems.
/// 
/// # Examples
/// 
/// ```rust
/// use kernel_zero_dirs::home_dir;
/// 
/// let home = home_dir().unwrap();
/// assert!(home.exists());
/// ```
pub fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME")
        .map(PathBuf::from)
        // Fallback for Windows
        .or_else(|| env::var_os("USERPROFILE").map(PathBuf::from))
}

/// Get config directory (XDG_CONFIG or ~/.config)
/// 
/// Follows the XDG Base Directory Specification.
/// First checks `$XDG_CONFIG_HOME`, then falls back to `$HOME/.config`.
pub fn config_dir() -> Option<PathBuf> {
    env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| home_dir().map(|h| h.join(".config")))
        // Windows fallback
        .or_else(|| env::var_os("APPDATA").map(PathBuf::from))
}

/// Get data directory (XDG_DATA or ~/.local/share)
/// 
/// Follows the XDG Base Directory Specification.
/// First checks `$XDG_DATA_HOME`, then falls back to `$HOME/.local/share`.
pub fn data_dir() -> Option<PathBuf> {
    env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| home_dir().map(|h| h.join(".local/share")))
        // Windows fallback
        .or_else(|| env::var_os("LOCALAPPDATA").map(PathBuf::from))
}

/// Get cache directory (XDG_CACHE or ~/.cache)
/// 
/// Follows the XDG Base Directory Specification.
/// First checks `$XDG_CACHE_HOME`, then falls back to `$HOME/.cache`.
pub fn cache_dir() -> Option<PathBuf> {
    env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| home_dir().map(|h| h.join(".cache")))
}

/// Get runtime directory (XDG_RUNTIME or /tmp)
/// 
/// Follows the XDG Base Directory Specification.
/// First checks `$XDG_RUNTIME_HOME`, then falls back to `/tmp`.
pub fn runtime_dir() -> Option<PathBuf> {
    env::var_os("XDG_RUNTIME_HOME")
        .map(PathBuf::from)
        .or_else(|| Some(PathBuf::from("/tmp")))
        .or_else(|| env::var_os("TMPDIR").map(PathBuf::from))
}

/// Get executable directory
/// 
/// Returns the directory containing the current executable.
pub fn executable_dir() -> Option<PathBuf> {
    env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
}

/// Get current directory
pub fn current_dir() -> Option<PathBuf> {
    env::current_dir().ok().map(PathBuf::from)
}

// ============================================================================
// PATHBUF EXTENSION TRAIT
// ============================================================================

/// Extension trait for PathBuf with convenience methods
pub trait PathBufExt {
    /// Create a subdirectory
    fn subdir(&self, name: &str) -> PathBuf;
    
    /// Ensure directory exists
    fn ensure_dir(&self) -> std::io::Result<()>;
}

impl PathBufExt for PathBuf {
    fn subdir(&self, name: &str) -> PathBuf {
        self.join(name)
    }
    
    fn ensure_dir(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self)
    }
}

// ============================================================================
// DIRECTORY BUILDER
// ============================================================================

/// Builder for custom directory paths
#[derive(Debug, Clone)]
pub struct DirBuilder {
    root: Option<PathBuf>,
    app_name: Option<String>,
    org_name: Option<String>,
    use_cache: bool,
}

impl DirBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        DirBuilder {
            root: None,
            app_name: None,
            org_name: None,
            use_cache: false,
        }
    }
    
    /// Set base directory
    pub fn root(mut self, root: PathBuf) -> Self {
        self.root = Some(root);
        self
    }
    
    /// Set application name
    pub fn app(mut self, name: impl Into<String>) -> Self {
        self.app_name = Some(name.into());
        self
    }
    
    /// Set organization name
    pub fn org(mut self, name: impl Into<String>) -> Self {
        self.org_name = Some(name.into());
        self
    }
    
    /// Use cache directory as base
    pub fn cache(mut self) -> Self {
        self.use_cache = true;
        self
    }
    
    /// Build the directory path
    pub fn build(self) -> Option<PathBuf> {
        let base = if self.use_cache {
            cache_dir()?
        } else if let Some(root) = self.root {
            root
        } else {
            data_dir()?
        };
        
        let mut path = base;
        
        if let Some(org) = self.org_name {
            path = path.join(org);
        }
        
        if let Some(app) = self.app_name {
            path = path.join(app);
        }
        
        Some(path)
    }
}

impl Default for DirBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// CONVENIENCE FUNCTIONS
// ============================================================================

/// Get a project-specific directory
/// 
/// # Examples
/// 
/// ```rust
/// use kernel_zero_dirs::project_dir;
/// 
/// let dir = project_dir("myapp").unwrap();
/// ```
pub fn project_dir(name: impl Into<String>) -> Option<PathBuf> {
    DirBuilder::new().app(name).build()
}

/// Get an organization directory
pub fn org_dir(org: impl Into<String>, app: impl Into<String>) -> Option<PathBuf> {
    DirBuilder::new().org(org).app(app).build()
}

/// Ensure a directory exists
pub fn ensure_dir(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}

// ============================================================================
// XDG COMPLIANCE
// ============================================================================

/// Check if running on a system with XDG support
pub fn is_xdg_compliant() -> bool {
    env::var_os("XDG_CONFIG_HOME").is_some()
    || env::var_os("XDG_DATA_HOME").is_some()
    || env::var_os("XDG_CACHE_HOME").is_some()
}

/// Get all XDG directories at once
#[derive(Debug, Clone)]
pub struct XdgDirs {
    pub config: PathBuf,
    pub data: PathBuf,
    pub cache: PathBuf,
    pub runtime: PathBuf,
}

impl XdgDirs {
    /// Get all XDG directories
    pub fn get() -> Option<Self> {
        Some(XdgDirs {
            config: config_dir()?,
            data: data_dir()?,
            cache: cache_dir()?,
            runtime: runtime_dir()?,
        })
    }
}

/// Get all standard directories
pub fn all_dirs() -> XdgDirs {
    XdgDirs::get().unwrap_or_else(|| XdgDirs {
        config: PathBuf::from(".config"),
        data: PathBuf::from(".local/share"),
        cache: PathBuf::from(".cache"),
        runtime: PathBuf::from("/tmp"),
    })
}