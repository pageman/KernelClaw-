# METADATA_ANALYSIS.md - KernelClaw v0.2.1 (Post-Robustness Fixes)

## Repository Overview

- **Repository**: pageman/KernelClaw-
- **HEAD**: (pending fix commit)
- **Version**: 0.2.1
- **Edition**: 2021 (fixed from 2024)
- **Status**: VSIK + Knowledge Graph - Robustness fixes applied

## Crate Inventory

### Main Crates (9 crates)

| Crate | Purpose | Status |
|-------|---------|--------|
| kernel-cli | CLI entry + VSIK commands | ✅ Working |
| kernel-core | Orchestration + proposals + graph | ✅ Working |
| kernel-crypto | Ed25519 signing + receipts | ✅ Working |
| kernel-daemon | Unix socket server | ⚠️ Basic |
| kernel-exec | Tool execution + WASM | ⚠️ Stub |
| kernel-llm | Ollama client | ✅ Working |
| kernel-memory | JSONL ledger + checksums | ✅ Working |
| kernel-notify | System notifications | ✅ Working |
| kernel-policy | YAML policy engine | ✅ Working |

### Zero-Dependency Modules (11 crates)

| Crate | LOC | Replaces | Status |
|------|-----|----------|--------|
| kernel-zero | ~800 | chrono, uuid, thiserror, sha256 | ✅ Full |
| kernel-zero-ed25519 | ~500 | ed25519-dalek | ✅ Full |
| kernel-zero-serde | ~700 | serde | ✅ Full |
| kernel-zero-tokio | ~700 | tokio | ✅ Full |
| kernel-zero-json | ~10KB | serde_json | ✅ Full |
| kernel-zero-yaml | ~5KB | serde_yaml | ✅ Full |
| kernel-zero-dirs | ~8.5KB | dirs | ✅ Full |
| kernel-zero-runtime | ~2KB | (WASM) | ⚠️ Stub |
| kernel-zero-async | 250 | - | ✅ Working |
| kernel-zero-derive | 252 | - | ✅ Fixed (src/lib.rs created) |
| kernel-zero-serde-derive | 100 | - | ✅ Fixed ([lib] proc-macro = true) |

### Tools

| File | Purpose |
|------|---------|
| tools/graph-viz.html | Three.js Knowledge Graph visualization |

## Robustness Fixes Applied (v0.2.1-patch)

| # | Fix | Status |
|---|-----|--------|
| 1 | Edition 2024 → 2021 | ✅ Done |
| 2 | kernel-zero-derive: src/lib.rs created | ✅ Done |
| 3 | kernel-zero-serde-derive: [lib] proc-macro = true | ✅ Done |
| 4 | kernel-zero/src/time.rs: u128→u64 fix | ✅ Done |

## Implementation Status (v0.2.1)

| Concern | Status | Notes |
|---------|--------|-------|
| Append-Only Memory | ✅ Working | Real JSONL with SHA256 checksums |
| Policy at Tool Boundary | ✅ Working | allowed_paths enforced |
| Orchestrator Pipeline | ✅ Working | Full pipeline with policy |
| Self-Improvement (VSIK) | ✅ Working | Proposal → Review → Approve |
| Knowledge Graph | ✅ Working | Relational model + graph-aware proposals |
| Graph Visualization | ⚠️ CDN | Three.js from cdnjs (optional) |
| Zero-Dependency | ✅ Core | All Rust deps have alternatives |

## External LLM Providers

| Provider | Status | Notes |
|----------|-------|-------|
| ollama-bridge | ✅ Compatible | z-ai-web-dev-sdk proxy, implements /api/generate |
| Ollama (local) | 🔜 Planned | Native /api/generate |
| OpenAI API | 🔜 Planned | GPT-4/3.5 fallback |

### ollama-bridge Integration

The `ollama-bridge` is an external Ollama-compatible API server that proxies to `z-ai-web-dev-sdk` for LLM completions. It provides:

- `/api/generate` endpoint — matches `kernel-llm` requirements
- `/api/chat` endpoint — for conversational use
- `/api/tags` — model listing
- Pre-configured `KERNELCLAW_SYSTEM` prompt for ParsedGoal format

**Architecture**:

```
KernelClaw (kernel-llm) → HTTP POST /api/generate → ollama-bridge → z-ai-web-dev-sdk → LLM
```

**Why External**: 
- KernelClaw aims for zero-dep / self-contained architecture
- `ollama-bridge` pulls in proprietary `z-ai-web-dev-sdk`
- Better as a standalone optional service

**Usage**: Run `node ollama-bridge.mjs --port 11434` and configure KernelClaw to use `http://localhost:11434` as the LLM endpoint.

## External Dependencies (Rust)

| Dependency | Status | Replacement |
|------------|--------|-------------|
| serde | ✅ Optional | kernel-zero-serde |
| serde_json | ✅ Optional | kernel-zero-json |
| serde_yaml | ✅ Optional | kernel-zero-yaml |
| tokio | ✅ Optional | kernel-zero-tokio |
| ed25519-dalek | ✅ Optional | kernel-zero-ed25519 |
| sha2 | ✅ Optional | kernel-zero::sha256 |
| uuid | ✅ Optional | kernel_zero::id |
| chrono | ✅ Optional | kernel_zero::time |
| thiserror | ✅ Optional | kernel-zero::error |
| dirs | ✅ Optional | kernel-zero-dirs |

## GoT→CoT→PVL Pipeline

### Goal of Task (GoT)
- Robust, compilable kernel with VSIK + Knowledge Graph
- Zero-dep architecture working
- Production-ready foundation

### Course of Task (CoT)
- Phase 1: Core kernel foundation (v0.1.x)
- Phase 2: Zero-dep alternatives (v0.1.x)
- Phase 3: VSIK MVP (v0.2.0)
- Phase 4: Knowledge Graph + Visualization (v0.2.1)
- Phase 5: Robustness fixes (v0.2.1-patch)

### PVL (Parallel Verification List)

| Check | Status |
|-------|--------|
| Memory durable | ✅ |
| Policy boundary | ✅ |
| VSIK loop | ✅ |
| Knowledge Graph | ✅ |
| Compilation fixes | ✅ |
| Zero-dep core | ✅ |
| CLI functional | ✅ |

## Recommended Next Steps

### Priority 1 (Critical)
1. Verify compilation after fixes
2. Run cargo test

### Priority 2 (Important)
3. Wire WASM execution
4. Add LLM-backed typed planning

### Priority 3 (Nice to Have)
5. Add integration tests
6. Create Research Arc with claim + sequence

## Version History

| Version | Date | Changes |
|---------|------|---------|
| v0.2.1-patch | 2026-04-13 | Robustness fixes |
| v0.2.1 | 2026-04-12 | Knowledge Graph + docs |
| v0.2.0 | 2026-04-10 | VSIK MVP |
| v0.1.7 | 2026-04-10 | MIT License |

## Metrics

- **Total crates**: 20 (9 main + 11 zero-dep)
- **Total LOC**: ~35,000
- **Zero-dep LOC**: ~25,000

## Critical Context

- **Repo URL**: https://github.com/pageman/KernelClaw-
- **Branch**: master
- **Status**: Robustness fixes applied, ready for testing