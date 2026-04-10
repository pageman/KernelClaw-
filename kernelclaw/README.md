# KernelClaw - Agent Kernel

**Status**: v0.1.6 - Working prototype, NOT production-ready

## About - The Austen Allred Concern

KernelClaw is an experiment responding to Austen Allred's "Agent Desiderata" thread:
https://x.com/Austen/status/2042444789891654076

### The 4-Point Desiderata (Goals We Aim For)

1. **Zero-dependency static binary** - Aim for minimal runtime deps
2. **Append-only signed memory** - Goal: durable audit trail
3. **Capability-based execution** - Goal: policy-gated tools
4. **Exception-only UX** - Goal: silent success, noisy failure

**Current Reality**: We are working toward these - NOT yet achieved.

## What IS Implemented

| Concern | Status | Notes |
|---------|--------|-------|
| Crypto Receipts | ✅ Working | Ed25519 signing, verification exists |
| Policy Engine | ✅ Working | YAML loading, structure exists |
| Executor Base | ⚠️ Partial | Native dispatch, not sandboxed yet |

## What's NOT Implemented (Honest Assessment)

- **Append-only Durable Memory**: Currently in-memory only (see `#![stub]` markers)
- **Typed Goal Planning**: LLM schema defined but parsing NOT wired to orchestration
- **WASM Isolation**: Not yet active in execution path
- **Real Policy Enforcement**: allowed_paths exist in YAML but NOT enforced at file_read boundary
- **Full Orchestrator**: execute_goal() still stubbed - generates receipts without real execution
- **Daemon Mode**: Not implemented

The README and documentation describe TARGET state. Implementation follows.

## Dependencies (v0.1.6)

```
Core:
  - serde (required for derive macros)
  - tokio (async runtime)
  
Substitutable (zero-dep POC exists):
  - chrono      → kernel-zero time
  - uuid       → kernel-zero id
  - thiserror  → kernel-zero error
  - sha2       → kernel-zero sha256
  - ed25519    → kernel-zero-ed25519
```

## Honest Architecture

```
kernel-core/       - Orchestrator (PARITAL - stubbed pipeline)
kernel-exec/       - Native executor (NOT sandboxed)
kernel-policy/     - YAML → in-memory (NOT enforced at boundary)
kernel-memory/     - In-memory only (NOT durable)
kernel-crypto/     - Working Ed25519
kernel-llm/        - Schema exists (NOT wired up)
kernel-cli/        - Working CLI
```

## Key Implementation Gaps (v0.1.6)

### 1. Policy at Tool Boundary
```rust
// CURRENT (not enforced):
pub fn file_read(path: &str) -> Result<String, String> {
    if path.contains("..") { return Err(..); }
    fs::read_to_string(p)  // Just reads! No policy check here!
}

// TARGET: Enforce policy at boundary
pub fn file_read(path: &str, policy: &ToolPolicy) {
    if !is_path_allowed(path, policy) { return Err(...); }
}
```

### 2. Memory Durability
```rust
// CURRENT (in-memory):
entries: Mutex<Vec<LedgerEntry>>  // Lost on restart!

// TARGET: JSONL with checksums
```

### 3. Orchestrator Pipeline
```rust
// CURRENT (stub):
pub fn execute_goal(&mut self, raw_goal: &str) -> Receipt {
    // Just creates receipt!
    create_receipt(raw_goal, "execute_goal", ...)
}

// TARGET: Full pipeline
// 1. Parse via LLM → StructuredGoal
// 2. Validate policy
// 3. Execute via capability-gated executor
// 4. Sign receipt
// 5. Append to ledger
```

## Contributing

The repo is organized around addressing each gap. See:
- `kernel-zero-*/` for zero-dependency implementations
- `kernel-*/src/*.rs` for actual execution paths

**No CLA required** - MIT license.

## Version

v0.1.6 - Honest prototype

**License**: MIT