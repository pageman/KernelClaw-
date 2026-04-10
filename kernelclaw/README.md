# KernelClaw - Agent Kernel

A production-oriented agent kernel with enforced constraints.

## Design Thesis

KernelClaw addresses core concerns in autonomous agent execution:

1. **Minimal-Dependency Static Binary** - Targeted ~10 crates, not 50+
2. **Append-Only Signed Memory** - Durable JSONL ledger with checksums  
3. **Capability-Based Execution** - Policy-gated tool execution at tool boundary
4. **Exception-Only UX** - Silent on success, noisy only on failures

## Implementation Status

| Concern | Status | Notes |
|---------|--------|-------|
| Crypto Receipts | ✅ Strong | Ed25519 signing, receipt verification |
| Append-Only Memory | ✅ Strong | JSONL persistence, SHA256 checksums |
| Typed Planning | ✅ Strong | Structured ParsedGoal with validation |
| Capability Enforcement | ✅ Strong | Policy at tool boundary with allowlist |
| **WASM Isolation** | ✅ Now Real | wasmtime runtime integrated |
| Exception-Only UX | ✅ Strong | CLI silent on success |
| **Minimal Dependencies** | ✅ Improved | ~12 crates (from ~30+) |

## Dependencies (v0.1.4)

```
Core (4):
  - serde (JSON/YAML)
  - tokio (async)
  - ed25519-dalek (crypto)
  - sha2 (hashing)

Utilities (8):
  - serde_json, serde_yaml
  - uuid, chrono
  - thiserror, dirs
  - tokio (sync)
  - rand (seeding)
```

**Less realistic**: "zero-dependency" means custom implementations for everything. We use standard crates.

## Quick Start

```bash
cargo build --release
./target/release/kernelclaw init
./target/release/kernelclaw status
```

## Architecture

```
kernel-core/     - Orchestration (parse -> validate -> execute -> receipt -> record)
kernel-exec/      - Capability-gated executor + WASM runtime
kernel-policy/    - YAML policy with invariants, allowlists
kernel-memory/    - Append-only JSONL with checksums
kernel-crypto/    - Ed25519 signing
kernel-llm/      - Typed goal parsing
kernel-cli/      - Exception-only CLI
```

## Policy

Default policy enforces:
- file_read: only /tmp/, /var/tmp/, ~/Documents/
- file_write: disabled by default  
- shell: disabled
- WASM: enabled (sandboxed)

Edit `policy.yaml` to adjust.

## Version

v0.1.4 - WASM runtime + reduced deps

License: MIT