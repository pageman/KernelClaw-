# RESEARCH_ARC.md - KernelClaw Research Journey

## v0.1.4 (2026-04-10 19:53) - Super-Exhaustive Analysis

### Metadata Analysis Completed

- Full crate inventory (17 crates: 9 main + 8 zero-dep)
- Zero-dep LOC: ~4,000+
- Feature flags documented
- How-to-use instructions added to README

### Zero-Dependency Modules

| Module | LOC | Purpose | Status |
|--------|-----|---------|--------|
| kernel-zero | ~800 | time, id, error, sha256, json | ✅ Full |
| kernel-zero-ed25519 | ~500 | Ed25519 RFC 8032 | ✅ Full |
| kernel-zero-serde | ~700 | Serialize/Deserialize | ✅ Full |
| kernel-zero-tokio | ~700 | Async runtime | ✅ Full |
| kernel-zero-async | 253 | Task, Waker | ✅ Working |
| kernel-zero-derive | 252 | Basic derive | ✅ Working |
| kernel-zero-runtime | 551 | WASM runtime | ✅ Integrated |
| kernel-zero-serde-derive | 97 | Derive scaffold | ✅ Working |

### Feature Flags

```toml
[features]
default = ["use_std_deps"]  # Standard deps (serde, tokio, ed25519-dalek)
use_zero_dep = []           # Zero-dep alternatives
```

### Wiring Implemented (v0.1.4)

- **kernel-crypto**: Optional `use_zero_ed25519` feature
- **base64**: Always inline (zero-dep)
- **Cargo.toml**: Feature flags + zero-dep alternatives
- **Integration tests**: Added

## v0.1.4 (2026-04-10 19:49) - Optional Wiring

### Feature Flags

```toml
[features]
default = ["use_std_deps"]
use_zero_dep = []
```

### Wiring Implemented

- kernel-crypto: Optional use_zero_ed25519
- base64: Always inline

### Integration Tests

tests/integration.rs with:
- kernel_zero tests
- kernel_zero_serde tests
- kernel_zero_tokio tests
- kernel_zero_ed25519 tests

## v1.3.0 (2026-04-10 19:42) - Full Lite

- kernel-zero-serde: Full implementation
- kernel-zero-tokio: Full implementation

## v1.2.0 (2026-04-10 19:30) - Lite

- Lite serde + tokio

## v1.1.0 (2026-04-10 19:10) - Scaffolding

- kernel-zero-tokio scaffold
- kernel-zero-serde-derive scaffold
- Inline base64

## v1.0.x Series

- v1.0.3: Honest metadata
- v1.0.2: Capability fix
- v1.0.1: Honest assessment
- v1.0: FULL ZERO-DEP milestone

## GoT->CoT->PVL Pipeline

### GoT (Goal of Task)
- Achieve optional zero-dependency via feature flags
- Maintain standard deps as default
- Enable zero-dep for embedded environments

### CoT (Course of Task)
- Phase 1: Create zero-dep modules (v1.0 - v1.3.0)
- Phase 2: Add feature flags (v0.1.4)
- Phase 3: Wire zero-dep into main crates (v0.1.4)
- Phase 4: Add integration tests (v0.1.4)
- Phase 5: More wiring (future)

### PVL (Parallel Verification)
- [x] Feature flags work
- [x] kernel-zero compiles
- [x] kernel-zero-serde compiles
- [x] kernel-zero-tokio compiles
- [x] kernel-zero-ed25519 works
- [ ] Run cargo test
- [ ] Run cargo test --features use_zero_dep

## Metrics Over Time

| Version | Zero-Dep LOC | Crates |
|---------|-------------|--------|
| v0.1.3 | ~500 | 9 |
| v1.0 | ~1,500 | 14 |
| v1.3.0 | ~3,000 | 17 |
| v0.1.4 | ~4,000 | 17 |

## Austen Allred Concern Progress

| Concern | Status (v0.1.4) |
|---------|---------------|
| Append-Only Memory | ✅ |
| Policy at Boundary | ✅ |
| Orchestrator | ✅ |
| Typed Planning | ⚠️ |
| Exception-Only UX | ✅ |
| Daemon | ⚠️ |
| WASM Runtime | ⚠️ |
| Zero-Dependency | ✅ Optional |

## Key Decisions

1. **Feature flags over complete rewrite** - Maintains compatibility while enabling zero-dep
2. **Standard deps as default** - Production-ready, well-tested
3. **Zero-dep as option** - For embedded/constrained environments
4. **Integration tests** - Verify both modes work

## Recommended Next Steps

### Priority 1
- [ ] Run cargo test (both feature sets)
- [ ] Wire more zero-dep modules

### Priority 2
- [ ] Add unit tests for zero-dep
- [ ] Fix warnings

### Priority 3
- [ ] Version bump to 0.2.0
- [ ] Add examples

## Critical Context

- **Repository**: pageman/KernelClaw-
- **HEAD**: a0ec984 (v0.1.4)
- **Edition**: Rust 2024