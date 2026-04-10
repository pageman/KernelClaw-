# METADATA_ANALYSIS.md - KernelClaw v0.1.6 (Honest)

## Repository Overview

- **Repository**: pageman/KernelClaw-
- **HEAD**: afc66f1 (v0.1.6 - Honest Assessment)
- **Version**: 0.1.6
- **Edition**: 2024
- **Status**: Partially credible prototype (not hardened proof)

## Crate Inventory

### Main Crates (9 crates)

| Crate | Purpose | Dependencies |
|-------|---------|--------------|
| kernel-cli | CLI entry | tokio, dirs |
| kernel-core | Orchestration | tokio, serde, serde_yaml |
| kernel-crypto | Signing | thiserror |
| kernel-daemon | Unix socket | dirs |
| kernel-exec | Execution | tokio, serde, serde_json |
| kernel-llm | Ollama client | tokio |
| kernel-memory | JSONL ledger | kernel-zero |
| kernel-notify | Notifications | - |
| kernel-policy | Policy engine | serde_yaml |

### Zero-Dependency Modules (11 crates)

| Crate | LOC | Replaces | Status |
|------|-----|----------|--------|
| kernel-zero | ~800 | chrono, uuid, thiserror | ✅ Working |
| kernel-zero-ed25519 | ~500 | ed25519-dalek | ✅ Working |
| kernel-zero-serde | ~700 | serde | ✅ Working |
| kernel-zero-tokio | ~700 | tokio | ✅ Working |
| kernel-zero-json | ~10KB | serde_json | ✅ Working |
| kernel-zero-yaml | ~5KB | serde_yaml | ✅ Working |
| kernel-zero-dirs | ~8.5KB | dirs | ✅ Working |
| kernel-zero-runtime | ~2KB | - | ⚠️ Stub |
| kernel-zero-async | 250 | - | ✅ Working |
| kernel-zero-derive | 250 | - | ✅ Working |
| kernel-zero-serde-derive | 100 | - | ✅ Working |

## Implementation Status (v0.1.6)

| Concern | Status | Notes |
|---------|--------|-------|
| Append-Only Memory | ✅ Working | Real JSONL with checksums |
| Policy at Tool Boundary | ✅ Working | allowed_paths enforced |
| Orchestrator Pipeline | ✅ Working | Full pipeline with policy |
| Typed Planning | ⚠️ Heuristic | Rule-based, not model-backed |
| Exception-Only UX | ⚠️ Partial | Some prints on success |
| Daemon | ⚠️ Basic | Unix socket only |
| WASM Runtime | ⚠️ Stub | Not wired in execution path |
| Zero-Dependency | ⚠️ Optional | Feature flags available |

## External Dependencies

| Dependency | Status | Replacement |
|------------|--------|-------------|
| serde | Optional | kernel-zero-serde |
| serde_json | Optional | kernel-zero-json |
| serde_yaml | Optional | kernel-zero-yaml |
| tokio | Optional | kernel-zero-tokio |
| dirs | Optional | kernel-zero-dirs |
| thiserror | Optional | kernel-zero (error) |
| uuid | Optional | kernel-zero (id) |
| chrono | Optional | kernel-zero (time) |

## Honest Assessment

### What's Actually Working

- Real durable append-only memory (JSONL with checksums)
- Policy enforcement at file tool boundary
- Full orchestration pipeline
- Unix socket daemon (basic)
- Ed25519 signing

### What's Partial/Stub

- Typed planning: Rule-based heuristic (not model-backed)
- WASM: Runtime exists but not in execution path
- Exception-only UX: Some commands still print on success
- Zero-dep: Feature flags available but not wired

### Known Issues

1. README claims still slightly outrun code
2. Typed planning is heuristic, not AI-backed
3. WASM execution path not wired
4. Some UX prints on success rather than silent

## GoT→CoT→PVL

### Goal of Task (GoT)
- Make fully working kernel implementation
- Fix remaining blockers

### Course of Task (CoT)
- Phase 1: Honest assessment (done)
- Phase 2: Fix policy wiring
- Phase 3: Wire WASM execution
- Phase 4: True exception-only UX

### Parallel Verification List

| Item | Status |
|------|--------|
| Memory durable | ✅ |
| Policy enforced | ✅ |
| Daemon basic | ✅ |
| WASM wired | ❌ Not done |
| Typed planner | ❌ Heuristic |
| Exception-only | ❌ Partial |

## Recommended Next Steps

### Priority 1 (Critical)
1. Wire WASM execution in kernel-exec
2. Make parse_goal use actual LLM (not rule-based)
3. True exception-only UX

### Priority 2 (Important)
4. Wire zero-dep modules as default
5. Add integration tests

### Priority 3 (Nice to have)
6. Add more daemon commands
7. Polish error messages

## Version History

| Version | Date | Changes |
|---------|------|---------|
| v0.1.6 | 2026-04-10 | Honest assessment |
| v0.1.5 | 2026-04-10 | JSON + YAML zero-dep |
| v0.1.4 | 2026-04-10 | First zero-dep modules |
| v1.3.0 | 2026-04-10 | Full implementations |

## Metrics

- **Total LOC**: ~30,000
- **Zero-dep LOC**: ~25,000
- **Crates**: 20
- **Truth**: Partially credible prototype

## Critical Context

- **Repo URL**: https://github.com/pageman/KernelClaw-
- **Branch**: master
- **Honest Verdict**: Partially credible prototype kernel, but not hardened proof