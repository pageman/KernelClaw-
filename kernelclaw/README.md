# KernelClaw - Agent Kernel

A production-oriented agent kernel with enforced constraints.

## Design Thesis

KernelClaw addresses four core concerns in autonomous agent execution:

1. **Zero-Dependency Static Binary** - Minimal external crate dependencies
2. **Append-Only Signed Memory** - Durable ledger with checksum verification  
3. **Capability-Based Execution** - Policy-gated tool execution at tool boundary
4. **Exception-Only UX** - Silent on success, noisy only on failures

## Current Implementation Status

| Concern | Status | Notes |
|---------|--------|-------|
| Crypto Receipts | ✅ Strong | Ed25519 signing, receipt verification |
| Append-Only Memory | ✅ Strong | JSONL persistence, SHA256 checksums |
| Typed Planning | ✅ Strong | Structured ParsedGoal with validation |
| Capability Enforcement | ✅ Strong | Policy at tool boundary with allowlist |
| WASM Isolation | ⚠️ Stub | Scaffold ready, runtime pending |
| Exception-Only UX | ✅ Strong | CLI silent on success |
| Zero-Dependency | ⚠️ Partial | Minimal deps - see Cargo.toml |

## Quick Start

```bash
cargo build --release
./target/release/kernelclaw init
./target/release/kernelclaw status
```

## Architecture

```
kernel-core/     - Orchestration (parse -> validate -> execute -> receipt -> record)
kernel-exec/      - Capability-gated executor with tool boundary enforcement
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
- Network: disabled

Edit `policy.yaml` to adjust.

## Version

v0.1.3 - Strong enforcement pass

License: MIT