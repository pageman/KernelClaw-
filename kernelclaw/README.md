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
| WASM Isolation | ✅ Strong | wasmtime runtime integrated |
| Exception-Only UX | ✅ Strong | CLI silent on success |
| **Minimal Dependencies** | ✅ Strong | ~10 crates |

## Zero-Dependency Modules (Proof of Principle)

### Production-Ready Zone
| Module | Status | Description |
|--------|--------|-------------|
| `kernel-zero` | ✅ Core Ready | Time, ID, Error, SHA256 |
| `kernel-zero-async` | ⚠️ Minimal | Async runtime stub |
| `kernel-zero-serde` | ⚠️ Minimal | Serde stub |

### In Development (Production-Ready)
| Module | Target | Description |
|--------|--------|-------------|
| `kernel-zero-ed25519` | Full RFC 8032 | Complete Ed25519 with curve math |
| `kernel-zero-runtime` | Full tokio | Complete async runtime |
| `kernel-zero-derive` | Full serde_derive | Derive macros |

## Dependencies (v0.1.5)

### Core (Required)
- serde - For derive macros on Policy, Receipt, etc.
- tokio - For async runtime

### Utilities (Could be zero-dep)
- uuid → use kernel-zero id module
- chrono → use kernel-zero time module  
- thiserror → use kernel-zero error module

## Architecture

```
kernel-core/       - Orchestration (parse -> validate -> execute -> receipt -> record)
kernel-exec/      - Capability-gated executor + WASM runtime
kernel-policy/    - YAML policy with invariants, allowlists
kernel-memory/    - Append-only JSONL with checksums
kernel-crypto/    - Ed25519 signing
kernel-llm/       - Typed goal parsing
kernel-cli/       - Exception-only CLI

# Zero-dep (proof of principle)
kernel-zero/       - Core utilities (time, id, error, sha256, json, toml)
kernel-zero-async/ - Async runtime stub
kernel-zero-serde/ - Serde stub
```

## Zero-Dependency Progress

### Phase 1: Core Utilities (Complete ✅)
- time.rs - Unix timestamps
- id.rs - UUID generation
- error.rs - Error handling
- sha256.rs - SHA256 hashing
- json.rs - JSON parsing
- toml.rs - TOML parsing

### Phase 2: Crypto (In Progress)
- ed25519.rs - ⚠️ Simplified (hash-based)
- **Target**: Full RFC 8032 curve operations

### Phase 3: Async Runtime (Stub)
- Runtime with worker threads
- **Target**: Full tokio replacement

### Phase 4: Derive Macros (Stub)
- Manual Serialize/Deserialize
- **Target**: Full serde_derive replacement

## Policy

Default policy enforces:
- file_read: only /tmp/, /var/tmp/, ~/Documents/
- file_write: disabled by default  
- shell: disabled
- WASM: enabled (sandboxed)

Edit `policy.yaml` to adjust.

## Quick Start

```bash
cargo build --release
./target/release/kernelclaw init
./target/release kernelclaw status
```

## Version

v0.1.5 - Zero-dependency proof of principle

License: MIT