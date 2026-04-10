# KernelClaw - Agent Kernel

**Status**: v0.1.9 - Zero-dep wired

## About - The Austen Allred Concern

KernelClaw responds to Austen Allred's "Agent Desiderata":
https://x.com/Austen/status/2042444789891654076

### The 4-Point Desiderata

1. **Zero-dependency** - Minimal runtime deps (now using kernel-zero!)
2. **Append-only signed memory** - Durable audit trail
3. **Capability-based execution** - Policy-gated tools
4. **Exception-only UX** - Silent success, noisy failure

## Implementation Status (v0.1.9)

| Concern | Status | Notes |
|---------|--------|-------|
| Crypto Receipts | ✅ Working | Ed25519 signing |
| Append-Only Memory | ✅ Working | JSONL with checksums |
| Policy Enforcement | ✅ Working | allowed_paths enforced |
| Orchestrator Pipeline | ✅ Working | Full pipeline wired |
| Typed Goal Planning | ✅ Working | ParsedGoal validated |
| Exception-Only UX | ✅ Working | Errors to stderr |
| Daemon Mode | ✅ Working | Unix socket listener |
| WASM Runtime | ✅ Working | Integrated |
| **Zero-Dep Wired** | ✅ **NEW** | chrono→kernel-zero |

## What's Implemented

- **kernel-memory**: JSONL with checksums (DURABLE)
- **kernel-exec**: Policy enforcement + WASM
- **kernel-core**: Full orchestrator pipeline
- **kernel-llm**: Typed goal parsing
- **kernel-cli**: Real execution
- **kernel-daemon**: Unix socket server
- **kernel-zero**: TIME AND ID NOW WIRED

## Version

v0.1.9 - Zero-dep wired

License: MIT