# KernelClaw - Agent Kernel

**Status**: v0.1.8 - Daemon + WASM integrated

## About - The Austen Allred Concern

KernelClaw responds to Austen Allred's "Agent Desiderata":
https://x.com/Austen/status/2042444789891654076

### The 4-Point Desiderata

1. **Zero-dependency** - Minimal runtime deps
2. **Append-only signed memory** - Durable audit trail
3. **Capability-based execution** - Policy-gated tools
4. **Exception-only UX** - Silent success, noisy failure

## Implementation Status (v0.1.8)

| Concern | Status | Notes |
|---------|--------|-------|
| Crypto Receipts | ✅ Working | Ed25519 signing |
| Append-Only Memory | ✅ Working | JSONL with checksums |
| Policy Enforcement | ✅ Working | allowed_paths enforced |
| Orchestrator Pipeline | ✅ Working | Full pipeline wired |
| Typed Goal Planning | ✅ Working | ParsedGoal validated |
| Exception-Only UX | ✅ Working | Errors to stderr |
| **Daemon Mode** | ✅ **NEW** | Unix socket listener |
| **WASM Runtime** | ✅ **NEW** | Integrated (stub execution) |

## What's Implemented

- **kernel-memory**: JSONL with SHA256 checksums (DURABLE)
- **kernel-exec**: Policy enforcement at tool boundary
- **kernel-core**: Full orchestrator pipeline
- **kernel-llm**: ParsedGoal validation wired
- **kernel-cli**: Real execution + receipt listing
- **kernel-daemon**: Unix socket server (NEW!)
- **kernel-exec/wasm**: WASM runtime integration (NEW!)

## Architecture (v0.1.8)

```
kernel-core/       - Orchestrator (FULL pipeline!)
kernel-exec/       - Capability-gated + WASM runtime
kernel-policy/     - YAML policy
kernel-memory/     - JSONL with checksums (DURABLE)
kernel-crypto/     - Ed25519 signing
kernel-llm/        - Typed goal parsing
kernel-cli/        - Real execution
kernel-daemon/    - Unix socket server (NEW!)
```

## Quick Start

```bash
cargo build --release
./target/release/kernelclaw init
./target/release kernelclaw status
./target/release kernelclaw run "read /tmp/test"
./target/release kernelclaw receipts
```

## Version

v0.1.8 - Daemon + WASM integrated

License: MIT