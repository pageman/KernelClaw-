# KernelClaw - Agent Kernel

**Status**: v0.1.4 - Optional Zero-Dependency Wiring

## About - The Austen Allred Concern

KernelClaw responds to Austen Allred's "Agent Desiderata":
https://x.com/Austen/status/2042444789891654076

## Implementation Status (v0.1.4)

| Concern | Status | Notes |
|---------|--------|-------|
| Append-Only Memory | ✅ Real | JSONL with checksums |
| Policy at Tool Boundary | ✅ Working | allowed_paths enforced |
| Orchestrator Pipeline | ✅ Working | Full pipeline |
| Typed Planning | ⚠️ Heuristic | Rule-based inference |
| Exception-Only UX | ✅ Working | Prints on explicit request |
| Daemon | ⚠️ Stub | Unix socket, not wired |
| WASM Runtime | ⚠️ Integrated | Runtime exists, not active |
| Zero-Dependency | ⚠️ Optional | Feature flags available |

## Quick Start

```bash
# Build
cargo build

# Run CLI
cargo run -- init
cargo run -- status
cargo run -- run "Write a hello world program"
```

## Zero-Dependency Options

### Feature Flags

```toml
# Cargo.toml
[features]
default = ["use_std_deps"]  # Uses standard deps (default)
# use_zero_dep = []         # Uncomment to use zero-dep alternatives
```

### Default (Standard Dependencies)

```toml
[dependencies]
serde = "1"
serde_json = "1"
serde_yaml = "0.9"
tokio = { version = "1", features = ["rt", "sync"] }
ed25519-dalek = "2"
sha2 = "0.10"
```

### Zero-Dependency Mode

Available zero-dep modules:

| Module | Replaces | LOC |
|--------|---------|-----|
| kernel-zero | chrono, uuid | ~800 |
| kernel-zero-ed25519 | ed25519-dalek | ~500 |
| kernel-zero-serde | serde | ~700 |
| kernel-zero-tokio | tokio | ~700 |

### Enabling Zero-Dependency

```toml
# In your Cargo.toml
[features]
default = []
use_zero_dep = ["kernel-zero-serde", "kernel-zero-tokio", "kernel-zero-ed25519"]

[dependencies]
kernel-zero = { path = "kernel-zero" }
kernel-zero-serde = { path = "kernel-zero-serde" }
kernel-zero-tokio = { path = "kernel-zero-tokio" }
kernel-zero-ed25519 = { path = "kernel-zero-ed25519" }
```

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

## Pipeline Flow

```
Goal → Parse (kernel-llm) → Validate (kernel-policy) → Execute (kernel-exec) → Receipt (kernel-crypto) → Record (kernel-memory)
```

## Policy Configuration

Edit `policy.yaml`:

```yaml
capabilities:
  - name: file_read
    allowed_paths:
      - /workspace/*
  - name: file_write
    allowed_paths:
      - /workspace/*

tools:
  - name: file_read
    capability: file_read
  - name: file_write
    capability: file_write
```

## Zero-Dependency Modules

### kernel-zero

```rust
use kernel_zero::time::now;
use kernel_zero::id::random_id;
use kernel_zero::sha256::Sha256;

// Get current timestamp
let ts = now();

// Generate random ID
let id = random_id();

// Hash data
let mut hasher = Sha256::new();
hasher.update(b"data");
let hash = hasher.finalize();
```

### kernel-zero-serde

```rust
use kernel_zero_serde::{Serialize, Deserialize, to_json, JsonSerializer};

// Define a serializable struct
#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    value: i32,
}

// Serialize to JSON
let config = Config { name: "test".into(), value: 42 };
let json = to_json(&config).unwrap();

// Deserialize from JSON
let parsed: Config = from_json(&json).unwrap();
```

### kernel-zero-tokio

```rust
use kernel_zero_tokio::{Runtime, spawn, block_on, sleep};

// Create runtime
let rt = Runtime::new();

// Spawn a task
let handle = rt.spawn(async {
    sleep(Duration::from_millis(100)).await;
    "done"
});

// Block on result
let result = rt.block_on(async {
    handle.await;
    "completed"
});
```

### kernel-zero-ed25519

```rust
use kernel_zero_ed25519::signing::{generate_keypair, sign, verify};

// Generate keypair
let kp = generate_keypair();

// Sign message
let message = b"Hello, World!";
let signature = sign(message, &kp);

// Verify
let is_valid = verify(message, &signature, &kp.verifying_key);
assert!(is_valid);
```

## Testing

```bash
# Run all tests
cargo test

# Run with zero-dep features
cargo test --features use_zero_dep

# Run integration tests
cargo test --test integration
```

## Version History

| Version | Date | Changes |
|---------|------|---------|
| v0.1.4 | 2026-04-10 | Optional zero-dep wiring |
| v1.3.0 | 2026-04-10 | Full lite serde + tokio |
| v1.2.0 | 2026-04-10 | Lite implementations |
| v1.1.0 | 2026-04-10 | Scaffolding |
| v1.0.3 | 2026-04-10 | Honest assessment |

## Crate Inventory

- **Total crates**: 17 (9 main + 8 zero-dep)
- **Zero-dep LOC**: ~4,000+
- **Edition**: Rust 2024

## License

MIT OR Apache-2.0