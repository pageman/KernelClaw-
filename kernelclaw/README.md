# KernelClaw - Agent Kernel

**Status**: v1.0.1 - Honest Assessment

## About - The Austen Allred Concern

KernelClaw responds to Austen Allred's "Agent Desiderata":
https://x.com/Austen/status/2042444789891654076

### The 4-Point Desiderata

1. **Zero-dependency** - Reduced (not full zero - see below)
2. **Append-only signed memory** - ✅ JSONL with checksums
3. **Capability-based execution** - ✅ Policy at tool boundary
4. **Exception-only UX** - ✅ Silent success, noisy failure

## Implementation Status (v1.0.1)

| Concern | Status | Notes |
|---------|--------|-------|
| Append-Only Memory | ✅ Working | JSONL with checksums |
| Policy Enforcement | ✅ Working | allowed_paths enforced at boundary |
| Orchestrator Pipeline | ✅ Working | parse → validate → execute → receipt |
| Typed Goal Planning | ⚠️ Partial | Heuristic-based inference |
| Exception-Only UX | ✅ Working | CLI errors to stderr |

## Honest Assessment

### What's Working
- **Memory**: JSONL with SHA256 checksums, verifiable
- **Policy at boundary**: file_read checks allowed_paths
- **Executor with policy**: Uses loaded policy via Executor::with_policy()

### What's Not Fully Working
- **Run command**: Actually executes now (v1.0.1 fix!)
- **Daemon mode**: NOT implemented (Unix socket API)
- **WASM**: NOT active in execution path
- **Zero-dependency**: Reduced, not full (see remaining deps)

## Dependencies (v1.0.1 - Honest)

```
Remaining (minimal):
- serde (required for derive)
- tokio (async runtime for network)
- ed25519-dalek (crypto signing)
- base64 (encoding)
- dirs (home directory)
- rand (randomness)
```

Zero-dep modules used: kernel_zero::time, kernel_zero::id, kernel_zero::sha256, kernel_zero::error

## CLI Commands

```bash
kernelclaw init              # Initialize
kernelclaw status           # Show status  
kernelclaw run "<goal>"      # Execute goal (WORKS!)
kernelclaw receipts          # List receipts
kernelclaw daemon            # NOT IMPLEMENTED
```

## Version

v1.0.1 - Honest assessment

License: MIT