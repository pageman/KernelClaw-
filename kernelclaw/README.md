# KernelClaw - Agent Kernel

**Status**: v0.1.4 - Zero-Dependency Optional Wiring

## About - The Austen Allred Concern

KernelClaw responds to Austen Allred's "Agent Desiderata":
https://x.com/Austen/status/2042444789891654076

## Implementation Status (v1.3.0)

| Concern | Status | Notes |
|---------|--------|-------|
| Append-Only Memory | ✅ Real | JSONL with checksums |
| Policy at Tool Boundary | ✅ Working | allowed_paths enforced |
| Orchestrator Pipeline | ✅ Working | Full pipeline |
| Typed Planning | ⚠️ Heuristic | Rule-based inference |
| Exception-Only UX | ✅ Working | Prints on explicit request |
| Daemon | ⚠️ Stub | Unix socket, not wired |
| WASM Runtime | ⚠️ Integrated | Runtime exists, not active |
| Zero-Dependency | ✅ In Principle | Full lite replacements exist |

## Zero-Dependency Modules (v1.3.0)

| Module | LOC | Status |
|--------|-----|--------|
| kernel-zero | 43 | Stable - time, id, error, sha256 |
| kernel-zero-ed25519 | 482 | Full RFC 8032 implementation |
| kernel-zero-async | 253 | Task, Waker, pinned futures |
| kernel-zero-serde | 714 | Full Serialize/Deserialize traits |
| kernel-zero-tokio | 712 | Full async runtime |
| kernel-zero-runtime | 551 | WASM runtime |
| kernel-zero-derive | N/A | Basic derive macros |
| kernel-zero-serde-derive | 97 | Derive macro scaffold |

## What's Working

- **Memory**: JSONL with SHA256 checksums
- **Policy**: loaded and wired to executor
- **Capability**: uses actual target path from parameters
- **Execution**: real goal execution via orchestrator
- **Zero-dep**: Full lite implementations exist for serde and tokio

## What's Not Working (Honest)

- Daemon mode (stub, not wired)
- WASM execution path (runtime exists but not used)
- Zero-dep modules use standard deps (serde, tokio) in main crates

## Version History

| Version | Date | Changes |
|---------|------|---------|
| v0.1.4 | 2026-04-10 | Optional zero-dep wiring, integration tests |
| v1.3.0 | 2026-04-10 | Full lite serde + tokio |
| v1.2.0 | 2026-04-10 | Lite implementations |
| v1.1.0 | 2026-04-10 | Scaffolding expansion |
| v1.0.3 | 2026-04-10 | Honest assessment |

## Feature Flags

```toml
[features]
default = ["use_std_deps"]  # Use standard deps (serde, tokio, ed25519-dalek)
use_zero_dep = []          # Enable zero-dep alternatives
```

When `use_zero_dep` is enabled:
- `serde` → `kernel-zero-serde`
- `tokio` → `kernel-zero-tokio`
- `ed25519-dalek` → `kernel-zero-ed25519`

## Crate Inventory

- **Total crates**: 17 (9 main + 8 zero-dep)
- **Zero-dep LOC**: ~3,000+
- **Edition**: Rust 2024

## License

MIT OR Apache-2.0