# RESEARCH_ARC.md - KernelClaw Research Journey

## v0.1.4 (2026-04-10 19:49) - Optional Zero-Dependency Wiring

### Feature Flags

```toml
[features]
default = ["use_std_deps"]  # Default: use standard deps
use_zero_dep = []           # Optional: use zero-dep alternatives
```

### Wiring Implemented

- **kernel-crypto**: Now has optional `use_zero_ed25519` feature flag
- **base64**: Always inline (zero-dep)
- **Cargo.toml**: Added feature flags and zero-dep alternatives

### Integration Tests

Added `tests/integration.rs` with:
- kernel_zero tests (time, id, sha256, error)
- kernel_zero_serde tests (primitives, struct, vec)
- kernel_zero_tokio tests (runtime, spawn, sleep, mutex, channel)
- kernel_zero_ed25519 tests (keypair, sign, verify)
- Integration tests (memory, policy, base64)

## v1.2.0 (2026-04-10 19:30) - Lite Implementations

- kernel-zero-serde: Lite Serialize/Deserialize + JSON serializer
- kernel-zero-tokio: Lite async runtime with spawn, block_on

## v1.1.0 (2026-04-10 19:10) - Scaffolding Expansion

- kernel-zero-tokio: Scaffold added
- kernel-zero-serde-derive: Scaffold added  
- kernel-crypto: Inline base64 (replaces base64 crate)

## v1.0.x Series (2026-04-10)

- v1.0.3 (18:57): Honest metadata - exception-only UX fix
- v1.0.2 (18:50): Capability uses target_path from parameters
- v1.0.1 (18:40): Honest assessment pass
- v1.0 (18:30): FULL ZERO-DEP milestone (chrono/uuid/sha2/thiserror replaced)

### Zero-Dep Modules (v1.0)

| Module | Status |
|--------|--------|
| kernel-zero | ✅ time, id, error, sha256 |
| kernel-zero-ed25519 | ✅ Full RFC 8032 |
| kernel-zero-async | ✅ Task, Waker |
| kernel-zero-derive | ✅ Basic derive |
| kernel-zero-runtime | ✅ WASM runtime |

## Pre-v1.0 Journey

### v0.1.4 (15:40)
- WASM: Real wasmtime runtime integrated
- Deps: Reduced from ~30 to ~12 crates
- README: Honest framing

### v0.1.3 (15:36)
- Policy tautology bug: Fixed `is_capability_allowed` proper path check
- Tool boundary gap: `file_read` now enforces `allowed_paths`
- Memory in-memory: Real JSONL persistence
- LLM raw string: Structured ParsedGoal
- CLI UX: Exception-only

## Key Decisions (GoT->CoT->PVL)

### GoT (Goal of Task)
- Achieve zero-dependency in principle - create full replacements
- Wire zero-dep modules into main crates
- Verify pipeline end-to-end

### CoT (Course of Task)
- Phase 1: Scaffolding (v1.1.0)
- Phase 2: Lite implementations (v1.2.0)
- Phase 3: Full implementations (v1.3.0)
- Phase 4: Wire into main crates (future)

### PVL (Parallel Verification List)
- [ ] serde wired → Builds without serde
- [ ] tokio wired → Builds without tokio
- [ ] ed25519 test vectors → Pass
- [ ] CLI integration test → Pass

## Metrics Over Time

| Version | Zero-Dep LOC | Crates |
|---------|-------------|--------|
| v0.1.3 | ~500 | 9 |
| v1.0 | ~1,500 | 14 |
| v1.3.0 | ~3,000 | 17 |

## Austen Allred Concern Progress

| Concern | Status (v1.3.0) |
|---------|---------------|
| Append-Only Memory | ✅ |
| Policy at Boundary | ✅ |
| Orchestrator | ✅ |
| Typed Planning | ⚠️ |
| Exception-Only UX | ✅ |
| Daemon | ⚠️ |
| WASM Runtime | ⚠️ |
| Zero-Dependency | ✅ In Principle |

## Critical Context

- **Repository**: pageman/KernelClaw-
- **HEAD**: c8088ab (v1.3.0)
- **Edition**: Rust 2024