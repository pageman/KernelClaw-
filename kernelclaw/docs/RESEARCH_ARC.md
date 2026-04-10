# KernelClaw - Research Journey

## Problem Space

KernelClaw emerged from a specific problem: **autonomous agents that cannot be trusted to execute actions safely**. 

Existing agent frameworks suffer from:
- No capability gating - any tool can be called
- No audit trail - what happened and why is opaque  
- No policy enforcement - constraints exist but aren't enforced
- Chatty UX - endless "thinking..." messages on happy path

## The Hypothesis (Goals)

We aimed to build an agent kernel with:
1. **Zero-dependency static binary** - Minimal attack surface
2. **Append-only signed memory** - Tamper-proof audit ledger
3. **Capability-based execution** - Policy gates at tool boundary
4. **Exception-only UX** - Silent on success, noisy on failure

## Journey Log

### v0.1.0 - MVP Release
- Basic modular workspace structure
- Policy loading from YAML
- Receipt signing - **This actually works**

### v0.1.1 - Audit Pass
- README with honest assessment
- Research arc documentation

### v0.1.2-0.1.5 - Various Attempts
- Fixed policy tautology bug
- Added zero-dep modules
- Reduced dependencies
- Added WASM scaffold

### v0.1.6 - Honest Assessment (Critical)
- Language was ahead of implementation
- Major gaps identified in audit

### v0.1.7 - Full Pipeline Fix (Current)

**ALL RECOMMENDATIONS FIXED:**

| Gap | Fix Applied |
|-----|-------------|
| **Memory durability** | ✅ JSONL with SHA256 checksums |
| **Policy enforcement** | ✅ ToolPolicy wired to file_read() |
| **Goal parsing** | ✅ ParsedGoal validation wired |
| **Orchestrator** | ✅ Full pipeline: parse→validate→execute→receipt→record |
| **CLI Run** | ✅ Real execution, not stub |
| **CLI Receipts** | ✅ Lists from ledger |

## Honest Final Assessment

### Working (v0.1.7)
- Ed25519 signing/verification - ✅ REAL
- Policy YAML loading - ✅ WORKING  
- JSONL persistent ledger with checksums - ✅ WORKING
- Policy enforced at tool boundary - ✅ WORKING
- Full orchestrator pipeline - ✅ WORKING
- CLI with real execution - ✅ WORKING

### Remaining
- WASM sandbox - NOT active in path
- Daemon mode - NOT implemented
- Zero-dependency - Using standard crates

## Key Technical Fixes (v0.1.7)

### 1. Memory Durability
```rust
// BEFORE (in-memory lost on restart):
entries: Mutex<Vec<LedgerEntry>>

// AFTER (DURABLE JSONL):
pub fn append(&self, entry_type, content, receipt_id) -> Result<String, String> {
    // Write to JSONL file with sequence
    // Compute SHA256 checksum
    // Verify on read
}
```

### 2. Policy at Tool Boundary
```rust
// BEFORE (just checks ".." then reads):
pub fn file_read(path: &str) -> Result<String, String> {
    if path.contains("..") { return Err(..); }
    fs::read_to_string(p)
}

// AFTER (enforces policy):
pub fn file_read(path: &str, policy: &ToolPolicy) {
    if !is_path_allowed(path, policy) { return Err(...); }
    // Policy allowed_paths ARE enforced!
}
```

### 3. Full Orchestrator Pipeline
```rust
// NEW: Full pipeline
pub fn execute_goal(&mut self, raw_goal: &str) -> ExecutionReceipt {
    // 1. Parse via LLM → ParsedGoal
    // 2. Validate policy
    // 3. Execute via capability-gated executor  
    // 4. Sign receipt
    // 5. Append to durable ledger
}
```

## One Sentence Assessment

> "KernelClaw moved from being a 'persuasive repo-shaped argument' to an actual prototype with working pipeline after v0.1.7."

## References

- Austen Allred: https://x.com/Austen
- Original desiderata: https://x.com/Austen/status/2042444789891654076
- RFC 8032 (Ed25519)
- RFC 6234 (SHA-256)