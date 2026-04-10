# KernelClaw - Research Journey (v0.1.9)

## Problem Space

KernelClaw emerged from Austen Allred's "Agent Desiderata": building trustworthy autonomous agents.

## Journey Log

### v0.1.9 - Zero-Dep Wired (Current)

**NEW: Wire kernel-zero to main crates**

| Dependency | Replaced By |
|------------|-------------|
| chrono | kernel_zero::time |
| uuid | kernel_zero::id |

All 4 main crates now use zero-dep time/id functions!

### v0.1.8 - Daemon + WASM
- Daemon mode: Unix socket server
- WASM runtime: Integrated

### v0.1.7 - Full Pipeline Fix
- Memory durability (JSONL)
- Policy enforcement at boundary
- Goal parsing wired
- Full orchestrator

## Current State (v0.1.9)

### Working
- Ed25519 signing/verification
- Policy YAML loading
- JSONL persistent ledger with checksums
- Policy enforced at tool boundary
- Full orchestrator pipeline
- CLI with real execution
- Daemon mode (Unix socket)
- WASM runtime (integrated)
- **Zero-dep time/id WIRED**

### Remaining
- WASM actual execution (stubbed)
- More zero-dep (sha2, thiserror)

## One Sentence Assessment

> "KernelClaw v0.1.9 has kernel-zero wired into main crates, reducing external dependencies."

## References

- Austen Allred: https://x.com/Austen
- Original desiderata: https://x.com/Austen/status/2042444789891654076