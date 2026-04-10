# KernelClaw - Agent Kernel

**Status**: v0.1.6 - Honest Assessment

> "Partially credible prototype kernel, but not hardened proof that Austen's kernel has been built."

## About - The Austen Allred Concern

KernelClaw is an attempt to implement the agent kernel from:
https://x.com/Austen/status/2042444789891654076

## Implementation Status (v0.1.6)

| Concern | Status | Notes |
|---------|--------|-------|
| Append-Only Memory | ✅ Working | Real JSONL with checksums |
| Policy at Tool Boundary | ✅ Working | allowed_paths enforced |
| Orchestrator Pipeline | ✅ Working | Full pipeline with policy |
| Typed Planning | ⚠️ Heuristic | Rule-based inference |
| Exception-Only UX | ⚠️ Partial | Some prints on success |
| Daemon | ⚠️ Basic | Unix socket, limited |
| WASM Runtime | ⚠️ Stub | Not wired in execution |
| Zero-Dependency | ⚠️ Optional | Feature flags available |

## Quick Start

```bash
# Build
cargo build

# Initialize
cargo run -- init

# Check status
cargo run -- status

# Run a goal
cargo run -- run "Write a hello world program"

# List receipts
cargo run -- receipts
```

## Crate Inventory

- **Total crates**: 20 (9 main + 11 zero-dep)
- **LOC**: ~30,000
- **Edition**: Rust 2024

## Architecture

```
kernel-cli          # CLI entry point
kernel-core       # Orchestration pipeline
kernel-crypto     # Ed25519 signing
kernel-daemon    # Unix socket server
kernel-exec      # Tool execution
kernel-llm       # Ollama client
kernel-memory    # JSONL ledger
kernel-notify   # Notifications
kernel-policy   # YAML policy
```

## Policy Configuration

Edit `policy.yaml`:

```yaml
capabilities:
  - name: file_read
    allowed_paths:
      - /workspace/*

tools:
  - name: file_read
    capability: file_read
```

## Zero-Dependency Options

### Feature Flags

```toml
# Cargo.toml
[features]
default = ["use_std_deps"]  # Standard deps
use_zero_dep = []           # Zero-dep alternatives
```

### Available Zero-Dep Modules

| Module | Replaces |
|--------|----------|
| kernel-zero | chrono, uuid, thiserror |
| kernel-zero-serde | serde |
| kernel-zero-json | serde_json |
| kernel-zero-yaml | serde_yaml |
| kernel-zero-tokio | tokio |
| kernel-zero-dirs | dirs |

Enable: `cargo build --features use_zero_dep`

## Testing

```bash
# Run tests
cargo test

# Run with zero-dep
cargo test --features use_zero_dep
```

## Known Gaps (Honest)

1. **Typed planning**: Still rule-based, not model-backed
2. **WASM execution**: Runtime exists but not wired
3. **Exception-only UX**: Some commands still print output
4. **Daemon**: Basic Unix socket only

## Version History

| Version | Date | Status |
|---------|------|--------|
| v0.1.6 | 2026-04-10 | Honest assessment |
| v0.1.5 | 2026-04-10 | Zero-dep JSON/YAML |
| v0.1.4 | 2026-04-10 | First zero-dep modules |

## License

MIT OR Apache-2.0

## Citation

If you use KernelClaw in your research, please cite:

```
@software{KernelClaw,
  author = {Paul "The Pageman" Pajo},
  title = {KernelClaw - Agent Kernel},
  url = {https://github.com/pageman/KernelClaw-},
  email = {pageman@gmail.com},
  year = {2026},
  version = {0.1.7}
}
```