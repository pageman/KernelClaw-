# KernelClaw - Agent Kernel

**Status**: v0.1.7 - Working prototype with full pipeline

## About - The Austen Allred Concern

KernelClaw responds to Austen Allred's "Agent Desiderata":
https://x.com/Austen/status/2042444789891654076

### The 4-Point Desiderata (Goals)

1. **Zero-dependency static binary** - Goal: minimal deps
2. **Append-only signed memory** - Goal: durable audit trail
3. **Capability-based execution** - Goal: policy-gated tools
4. **Exception-only UX** - Goal: silent success

## Implementation Status (v0.1.7)

| Concern | Status | Notes |
|---------|--------|-------|
| Crypto Receipts | ✅ Working | Ed25519 signing, verification |
| Append-Only Memory | ✅ Working | JSONL with SHA256 checksums |
| Policy Enforcement | ✅ Working | allowed_paths enforced at tool boundary |
| Orchestrator Pipeline | ✅ Working | parse → validate → execute → receipt → record |
| Typed Goal Planning | ✅ Working | ParsedGoal with validation |
| Exception-Only UX | ✅ Working | CLI errors to stderr |

## What's Implemented

- **kernel-memory**: DURABLE JSONL ledger with checksums (not in-memory)
- **kernel-exec**: Policy enforcement at file_read() boundary
- **kernel-core**: Full orchestrator pipeline wired
- **kernel-llm**: ParsedGoal validation (wired to orchestration)
- **kernel-cli**: Real execution, receipt listing

## What's Not Implemented

- WASM sandbox (not in active path)
- Daemon mode

## Architecture (v0.1.7)

```
kernel-core/       - Orchestrator (FULL pipeline!)
kernel-exec/       - Capability-gated (policy enforced)
kernel-policy/     - YAML policy
kernel-memory/     - JSONL with checksums (DURABLE)
kernel-crypto/     - Ed25519 signing
kernel-llm/        - ParsedGoal validation
kernel-cli/        - Real execution
```

## Quick Start

```bash
cargo build --release
./target/release/kernelclaw init
./target/release/kernelclaw status
./target/release kernelclaw run "read /tmp/test"
./target/release kernelclaw receipts
```

## Version

v0.1.7 - Full pipeline working

License: MIT