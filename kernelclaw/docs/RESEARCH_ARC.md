# KernelClaw - Research Journey (v0.1.8)

## Problem Space

KernelClaw emerged from Austen Allred's "Agent Desiderata": building trustworthy autonomous agents with core safety properties.

## Journey Log

### v0.1.8 - Daemon + WASM (Current)

**NEW FEATURES IMPLEMENTED:**

| Feature | Implementation |
|---------|----------------|
| **Daemon Mode** | ✅ Unix socket server in `kernel-daemon/` |
| **WASM Runtime** | ✅ Integrated into `kernel-exec/wasm_runtime.rs` |

### Previous Fixes (v0.1.7)
- Memory durability → JSONL with checksums
- Policy enforcement → allowed_paths enforced at boundary
- Goal parsing → ParsedGoal validation wired
- Orchestrator → Full pipeline

### v0.1.6 - Honest Assessment
- Language was ahead of implementation
- Critical gaps identified

## Current State (v0.1.8)

### Working
- Ed25519 signing/verification
- Policy YAML loading
- JSONL persistent ledger with checksums
- Policy enforced at tool boundary
- Full orchestrator pipeline
- CLI with real execution
- Daemon mode with Unix socket
- WASM runtime (stub execution)

### Remaining
- WASM actual execution (stubbed, module not compiled)
- Zero-dependency wired to main crates

## Key Technical Achievements

1. **Memory Durability**: JSONL with SHA256 checksums
2. **Policy at Boundary**: file_read() enforces allowed_paths
3. **Full Pipeline**: parse → validate → execute → receipt → record
4. **Daemon**: Unix socket protocol
5. **WASM Integration**: Runtime ready for tool registration

## One Sentence Assessment

> "KernelClaw v0.1.8 has working daemon mode, WASM runtime integration, and full orchestrator pipeline."

## References

- Austen Allred: https://x.com/Austen
- Original desiderata: https://x.com/Austen/status/2042444789891654076
- RFC 8032 (Ed25519)
- RFC 6234 (SHA-256)