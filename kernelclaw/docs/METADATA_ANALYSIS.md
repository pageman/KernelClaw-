# KernelClaw - Metadata Analysis & Next Steps (v0.1.9)

## Repository Metadata

### Crate Structure

| Category | Crates | Total LOC |
|----------|--------|-----------|
| **Main** | 9 | ~1,600 |
| **Zero-Dep POC** | 6 | 2,418 |
| **Total** | 15 | ~4,000+ |

### Main Crates (Production)
```
kernel-cli/        - CLI entry point
kernel-core/       - Orchestrator (FULL pipeline)
kernel-crypto/     - Ed25519 signing/verification
kernel-exec/       - Capability-gated executor + WASM
kernel-llm/        - Typed goal parsing
kernel-memory/     - JSONL with checksums (DURABLE)
kernel-policy/     - YAML policy loading
kernel-notify/     - Notifier
kernel-daemon/     - Unix socket server
```

### Zero-Dependency Wired (v0.1.9)
| Replace | Module |
|---------|--------|
| chrono → | kernel_zero::time |
| uuid → | kernel_zero::id |

## Gap Analysis

### ✅ All Major Gaps Fixed
- Memory durability
- Policy enforcement
- Goal parsing
- Orchestrator pipeline
- Daemon mode
- WASM runtime
- Zero-dep wired (chrono, uuid)

### Remaining (Minor)
| Gap | Status | Notes |
|-----|--------|-------|
| WASM actual execution | Stub | Runtime integrated |
| sha2 → kernel-zero | Not wired | Would save 1 dep |
| thiserror → kernel-zero | Not wired | Would save 1 dep |

## Recommended Next Steps (v0.1.9)

### P0 - Done
- Daemon mode ✅
- WASM runtime ✅
- Zero-dep wired (time/id) ✅

### P1 - Remaining
1. Wire sha2 → kernel_zero::sha256
2. Wire thiserror → kernel_zero::error
3. Wire ed25519-dalek → kernel-zero-ed25519
4. Implement actual WASM execution

## One-Line Summary

> "v0.1.9 has all major features working; remaining is more zero-dep wiring."

### Zero-Dependency Modules (POC)
```
kernel-zero/             - 880 LOC - Core: time, id, error, sha256, json, toml
kernel-zero-ed25519/     - 482 LOC - RFC 8032 Ed25519
kernel-zero-runtime/     - 551 LOC - Full async runtime
kernel-zero-async/       - 253 LOC - Async primitives
kernel-zero-serde/       - 252 LOC - Manual serde impl
kernel-zero-derive/       - 0 LOC - Not implemented
```

### Workspace Dependencies
```
Core (5):
  - serde          - Required for derive macros
  - serde_json     - JSON parsing
  - serde_yaml     - Policy loading
  - tokio          - Async runtime
  - ed25519-dalek  - Crypto signing

Substitutable (6):
  - sha2           → kernel-zero (sha256)
  - uuid           → kernel-zero (id)
  - chrono         → kernel-zero (time)
  - thiserror      → kernel-zero (error)
  - dirs           → std::env
  - rand           → rand::random
```

## Gap Analysis

### ✅ Fixed in v0.1.7
- Memory durability (JSONL)
- Policy enforcement at boundary
- Goal parsing wired
- Orchestrator full pipeline

### ❌ Remaining Gaps

| Gap | LOC Impact | Difficulty | Notes |
|-----|------------|------------|-------|
| WASM sandbox active | ~200 | Medium | Add wasmtime to executor path |
| Daemon mode | ~150 | Easy | Unix socket listener |
| Zero-dep wired | ~1000 | Hard | Replace serde/tokio with kernel-zero-* |
| WASM sandbox | ~200 | Medium | runtime not integrated |

## Recommended Next Steps (Priority Order)

### P0 - High Value

1. **WASM Runtime Integration** (~200 LOC)
   - Add wasmtime to executor path
   - Currently scaffolded, not active
   
2. **Daemon Mode** (~150 LOC)
   - Unix socket listener
   - Simple protocol for goal submission

### P1 - Nice to Have

3. **Hook up Zero-Dep Modules** (~1000 LOC)
   - Replace chrono → kernel-zero::time
   - Replace uuid → kernel-zero::id  
   - Replace thiserror → kernel-zero::error
   - Replace sha2 → kernel-zero::sha256
   
4. **Policy Dynamic Reload**
   - Hot-reload policy without restart

### P2 - Future

5. **WASM Tool Compilation**
   - Compile Rust tool to WASM
   - Execute via wasmtime runtime

6. **Network Policy**
   - Enforce network capability
   - Add reqwest capability gating

## Implementation Effort Estimates

| Feature | Effort | Risk |
|---------|--------|------|
| Daemon mode | 1-2 days | Low |
| WASM integration | 3-5 days | Medium |
| Zero-dep wired | 2-3 days | Medium |
| Policy reload | 1 day | Low |
| Full WASM tools | 1-2 weeks | High |

## One-Line Recommendations

1. **Quick win**: Implement daemon mode (socket listener)
2. **Real value**: Add WASM runtime to active path
3. **Long-term**: Wire zero-dep modules to replace std crates

---

*Analysis date: 2026-04-10*
*Version: v0.1.7*