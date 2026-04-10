# KernelClaw - Research Journey

## Problem Space

KernelClaw emerged from a specific problem: **autonomous agents that cannot be trusted to execute actions safely**. 

Existing agent frameworks suffer from:
- No capability gating - any tool can be called
- No audit trail - what happened and why is opaque  
- No policy enforcement - constraints exist but aren't enforced
- Chatty UX - endless "thinking..." messages on happy path

## The Hypothesis (Goals)

We aimed to build an agent kernel with:
1. **Zero-dependency static binary** - Minimal attack surface
2. **Append-only signed memory** - Tamper-proof audit ledger
3. **Capability-based execution** - Policy gates at tool boundary
4. **Exception-only UX** - Silent on success, noisy on failure

## Journey Log

### v0.1.0 - MVP Release
- Basic modular workspace structure
- Policy loading from YAML
- Receipt signing - **This actually works**

### v0.1.1 - Audit Pass
- README with honest assessment
- Research arc documentation
- Session logging

### v0.1.2 - Enforced Constraints (Attempt 1)
- Fixed policy tautology bug
- Added sled to dependencies (but NOT wired in)
- Typed LLM structures (but NOT wired in)

### v0.1.3 - Tool Boundary (Attempt 2)
- Added policy to file_read() call
- Added ToolPolicy type
- BUT: Still not actually enforced at actual boundary

### v0.1.4 - WASM + Reduced Deps
- Added wasmtime to workspace
- Reduced crates from ~30 to ~12
- README still says "minimal-dependency" not "zero"

### v0.1.5 - Zero-Dep (Proof of Principle)
- kernel-zero-* modules created (~2500 LOC)
- BUT: Not wired into main kernel

### v0.1.6 - Honest Assessment (Current)
- Fixed README to be truly honest
- CLI marks stubs explicitly
- Policy tautology fixed
- Key insight: Language was ahead of implementation

## Honest Final Assessment

### What's Working
- Ed25519 signing/verification (kernel-crypto)
- Policy YAML loading (kernel-policy)
- Basic CLI with exception-only UX

### What's NOT Working
| Gap | Status | Why |
|-----|--------|-----|
| Memory durability | IN-MEMORY | Mutex<Vec> not JSONL |
| Policy enforcement | NOT AT TOOL | file_read doesn't check |
| Goal parsing | NOT WIRED | Schema exists but unused |
| Orchestrator | STUB | Just creates receipts |
| WASM sandbox | NOT ACTIVE | No wasmtime in path |

## Key Lessons

1. **Language gets ahead of implementation**: We wrote "enforced" before enforcing
2. **Dependencies ≠ Implementation**: Having sled in Cargo.toml ≠ using it
3. **Stubs spread**: Easy to add schema, hard to wire to execution
4. **Policy exists ≠ Policy enforced**: Need enforcement at boundary

## References

- Austen Allred: https://x.com/Austen
- Original desiderata: https://x.com/Austen/status/2042444789891654076
- RFC 8032 (Ed25519)
- RFC 6234 (SHA-256)