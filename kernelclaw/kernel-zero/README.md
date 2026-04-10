# kernel-zero - Zero-Dependency Module

**Status**: Experimental / Proof of Concept

This module provides zero-external-dependency replacements for common functionality. Total: **839 LOC**.

## What This Is

A clean-room implementation of core utilities needed by KernelClaw:
- Time handling
- ID generation  
- Error types
- SHA256 hashing
- Ed25519 signing
- JSON parsing
- TOML parsing

## What This Is NOT

### ⚠️ NOT Production Ready

- **SHA256**: Simplified implementation, not audited for cryptographic use
- **Ed25519**: Hash-based signing, not full RFC 8032 curve operations
- **JSON/TOML**: Minimal coverage, only what KernelClaw needs

### Limitations

| Impl | Limitation |
|------|-----------|
| `sha256.rs` | Fixed seed values, simplified compress |
| `ed25519.rs` | Uses DefaultHasher, not real curve math |
| `json.rs` | Missing: Unicode, floats, dates, deeply nested |
| `toml.rs` | No nested tables, arrays of tables |

## Can Replace

| External Crate | Function | Status |
|---------------|----------|--------|
| chrono | Timestamps | ✅ Works |
| uuid | ID generation | ✅ Works |
| thiserror | Error types | ✅ Works |
| sha2 | SHA256 | ⚠️ Simplified |
| ed25519-dalek | Signing | ⚠️ Hash-based |
| serde_json | JSON | ⚠️ Minimal |
| serde_yaml | TOML | ⚠️ Minimal |

## Cannot Replace (Fundamentally)

| Crate | Why |
|-------|-----|
| **serde** | Requires derive macro system. Would need 2000+ LOC custom derive. |
| **tokio** | Async runtime from scratch is complex. Keep for async. |

## Usage

```rust
use kernel_zero::{now, new_receipt_id, hash, Error};

fn main() -> Result<()> {
    let ts = now();
    let id = new_receipt_id();
    let data = b"test";
    let hash = hash(data);
    Ok(())
}
```

## Honest Metrics

- **LOC**: 839 lines
- **Crates Replaced**: 10 external → 0 external (in kernel-zero)
- **Risk**: Medium (simplified implementations)

## Recommendation

Use for:
- Receipt signing (hash-based OK)
- Internal IDs (no security)
- Time (Unix timestamps, no calendar math)

Do NOT use for:
- Production cryptographic operations
- Security-critical signing
- Complex JSON/TOML parsing

---

**License**: MIT
**Version**: 0.1.0