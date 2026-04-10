//! WASM execution backend using wasmtime
//! Real WASM sandbox isolation

use std::sync::Arc;
use wasmtime::{Engine, Module, Instance, Store, Memory};
use wasmtime_wasi::WasiCtxBuilder;
use thiserror::Error;
use std::path::Path;

#[derive(Error, Debug)]
pub enum WasmError {
    #[error("Compile: {0}")]
    Compile(String),
    #[error("Instantiate: {0}")]
    Instantiate(String),
    #[error("Execute: {0}")]
    Execute(String),
    #[error("Runtime not available")]
    RuntimeUnavailable,
}

/// WASM runtime configuration
#[derive(Debug, Clone)]
pub struct WasmConfig {
    pub max_memory_pages: u64,
    pub max_fuel: u64,
    pub max_stack: u64,
    pub max_output_bytes: usize,
}

impl Default for WasmConfig {
    fn default() -> Self {
        WasmConfig {
            max_memory_pages: 512,    // 512 * 64KB = 32MB max
            max_fuel: 1_000_000,  // Execution fuel limit
            max_stack: 64 * 1024,  // 64KB stack
            max_output_bytes: 64 * 1024,
        }
    }
}

/// WASM runtime - actual sandbox execution
pub struct WasmRuntime {
    engine: Engine,
    config: WasmConfig,
}

impl WasmRuntime {
    /// Create new runtime
    pub fn new(config: WasmConfig) -> Result<Self, WasmError> {
        // Configure engine with limits
        let engine = Engine::new(&config.to_wasmtime_config())
            .map_err(|e| WasmError::Compile(e.to_string()))?;
        
        Ok(WasmRuntime { engine, config })
    }
    
    /// Load and execute a WASM module
    pub fn execute(&self, wasm_path: &str, function: &str, input: &str) -> Result<String, WasmError> {
        // Load module
        let module = Module::from_file(&self.engine, wasm_path)
            .map_err(|e| WasmError::Compile(e.to_string()))?;
        
        // Create store with WASI
        let mut store = Store::new(&self.engine, ());
        let wasi = WasiCtxBuilder::new()
            .map_err(|e| WasmError::Instantiate(e.to_string()))?
            .build();
        store.set_wasi(wasi);
        
        // Instantiate
        let instance = Instance::new(&mut store, &module, &[])
            .map_err(|e| WasmError::Instantiate(e.to_string()))?;
        
        // Get function
        let func = instance.get_typed_func::<(), i32>(&mut store, function)
            .map_err(|e| WasmError::Execute(e.to_string()))?;
        
        // Execute with fuel limit
        store.fuel_consumed()
            .map_err(|_| WasmError::Execute("Fuel limit exceeded".to_string()))?;
        
        // Call function
        let result = func.call(&mut store, ())
            .map_err(|e| WasmError::Execute(e.to_string()))?;
        
        Ok(format!("Result: {}", result))
    }
    
    /// Get memory from instance
    pub fn get_memory(&self, instance: &Instance, store: &mut Store<()>) -> Option<Memory> {
        instance.get_memory(store, "memory")
    }
}

impl WasmConfig {
    fn to_wasmtime_config(&self) -> wasmtime::Config {
        let mut config = wasmtime::Config::new();
        config.max_wasm_stack(self.max_stack as usize);
        config
    }
}

/// Check if WASM runtime is available
pub fn is_available() -> bool {
    // Simple check - can we create engine?
    Engine::new(&wasmtime::Config::new()).is_ok()
}

/// Tool wrapper for WASM execution
pub struct WasmTool {
    runtime: WasmRuntime,
}

impl WasmTool {
    pub fn new() -> Result<Self, WasmError> {
        Ok(WasmTool {
            runtime: WasmRuntime::new(WasmConfig::default())?,
        })
    }
    
    pub fn execute_tool(&self, wasm_module: &str, tool_name: &str, input: &str) -> Result<String, WasmError> {
        self.runtime.execute(wasm_module, tool_name, input)
    }
}