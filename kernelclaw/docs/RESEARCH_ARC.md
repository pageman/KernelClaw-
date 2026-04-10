# KernelClaw - Research Journey

## Problem Space

KernelClaw emerged from a specific problem: **autonomous agents that cannot be trusted to execute actions safely**. 

Existing agent frameworks suffer from:
- No capability gating - any tool can be called
- No audit trail - what happened and why is opaque  
- No policy enforcement - constraints exist but aren't enforced
- Chatty UX - endless "thinking..." messages on happy path

## The Hypothesis

We could build an agent kernel with:
1. **Zero-dependency static binary** - minimal attack surface
2. **Append-only signed memory** - tamper-proof audit ledger
3. **Capability-based execution** - policy gates at tool boundary
4. **Exception-only UX** - silent on success, noisy on failure

## Journey Log

### v0.1.0 - MVP Release
- Basic modular workspace structure
- Policy loading from YAML
- Receipt signing

### v0.1.1 - Audit Pass
- Added session logging
- README with honest status
- Research arc documentation

### v0.1.2 - Enforced Constraints
- Fixed policy tautology bug (`is_empty() || !is_empty()` always true)
- Real sled persistence for memory (not in-memory Mutex)
- Typed LLM parsing with validation
- Capability gating at tool boundary

### v0.1.3 - Tool Boundary Fixes
- file_read now checks allowed_paths BEFORE reading
- Executor passes policy to tools
- Exception-only CLI UX

### v0.1.4 - WASM + Reduced Deps
- Added wasmtime runtime integration
- Reduced crates from ~30 to ~12
- Honest README: "minimal-dependency" not "zero-dependency"

### v0.1.5 - Zero-Dependency Proof of Principle

#### Initial zero-dep modules:
- `kernel-zero` (839 LOC) - time, id, error, sha256, json, toml

#### Production-ready zero-dep:
- `kernel-zero-ed25519` (424 LOC) - Full RFC 8032 curve operations
- `kernel-zero-runtime` (~500 LOC) - Full async runtime

#### In progress:
- `kernel-zero-derive` - Serialize/Deserialize traits
- `kernel-zero-serde` - JSON serializer

## Key Technical Fixes

### 1. Policy Tautology Bug
```rust
// BEFORE (always true):
return allowlist.allowed_paths.is_empty() || !allowlist.allowed_paths.is_empty();

// AFTER (proper check):
if !config.enabled { return false; }
if let Some(path) = target {
    if !config.allowed_paths.is_empty() {
        return config.allowed_paths.iter().any(|p| path.starts_with(p));
    }
}
```

### 2. Tool Boundary Enforcement
```rust
// BEFORE (just checks ".." then reads):
pub fn file_read(path: &str) { fs::read_to_string(p) }

// AFTER (enforces policy):
pub fn file_read(path: &str, policy: &ToolPolicy) {
    if !is_path_allowed(path, policy) { return Err(...); }
    fs::read_to_string(p)
}
```

### 3. Memory Persistence
```rust
// BEFORE (in-memory):
entries: Mutex<Vec<LedgerEntry>>  // lost on restart

// AFTER (durable):
append() { 
    // Write to JSONL file with sequence number
    // Compute SHA256 checksum
    // Verify on read
}
```

## Current State

| Component | Status | Notes |
|----------|--------|-------|
| Crypto | ✅ Strong | Full Ed25519 |
| Memory | ✅ Strong | JSONL + checksums |
| Policy | ✅ Strong | YAML + enforce |
| Executor | ✅ Strong | Capability gating |
| WASM | ✅ Strong | wasmtime |
| Zero-dep | ⚠️ POC | ~2500 LOC |

## Remaining Work

1. **Hook up zero-dep modules** - Replace serde/tokio with kernel-zero-*)
2. **Red-team testing** - Adversarial scenarios
3. **Benchmarking** - Binary size, startup time

## Lessons Learned

- "Zero-dependency" is a spectrum, not binary
- Policy enforcement must be at tool boundary, not just capability level
- Exception-only UX requires discipline - easy to slip into chatty patterns
- Clean-room implementations reveal what you actually need vs abstract over

## References

- Austen Allred's "Agent Desiderata"
- RFC 8032 (Ed25519)
- RFC 6234 (SHA-256)