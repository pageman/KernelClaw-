# METADATA_ANALYSIS.md - KernelClaw v0.2.1 (Honest Assessment)

## Repository Overview

- **Repository**: pageman/KernelClaw-
- **HEAD**: 931c1aa (v0.2.1)
- **Version**: 0.2.1
- **Edition**: 2024
- **Status**: VSIK + Knowledge Graph (Honest about gaps)

## Crate Inventory

### Main Crates (9 crates)
kernel-cli, kernel-core, kernel-crypto, kernel-daemon, kernel-exec, kernel-llm, kernel-memory, kernel-notify, kernel-policy

### Zero-Dependency Modules (11 crates)
kernel-zero, kernel-zero-ed25519, kernel-zero-serde, kernel-zero-tokio, kernel-zero-json, kernel-zero-yaml, kernel-zero-dirs, kernel-zero-runtime, kernel-zero-async, kernel-zero-derive, kernel-zero-serde-derive

### Tools
tools/graph-viz.html - Three.js visualization

## External Dependencies

| Dependency | Status | Replacement |
|------------|--------|-------------|
| serde | ✅ Optional | kernel-zero-serde |
| serde_json | ✅ Optional | kernel-zero-json |
| serde_yaml | ✅ Optional | kernel-zero-yaml |
| tokio | ✅ Optional | kernel-zero-tokio |
| ed25519-dalek | ✅ Optional | kernel-zero-ed25519 |
| sha2 | ✅ Optional | kernel-zero::sha256 |
| uuid | ✅ Optional | kernel-zero::id |
| chrono | ✅ Optional | kernel_zero::time |
| thiserror | ✅ Optional | kernel-zero::error |
| dirs | ✅ Optional | kernel-zero-dirs |

## External CDN Dependencies

| Resource | Location | Notes |
|----------|----------|-------|
| Three.js | cdnjs.cloudflare.com | Used in tools/graph-viz.html only |

Note: Three.js is used ONLY in the optional visualization tool, NOT in the core kernel. It's loaded from CDN, not in Cargo.toml.

## Implementation Status (v0.2.1)

| Concern | Status | Notes |
|---------|--------|-------|
| Append-Only Memory | ✅ Working | Real JSONL with checksums |
| Policy at Tool Boundary | ✅ Working | allowed_paths enforced |
| Orchestrator Pipeline | ✅ Working | Full pipeline |
| Self-Improvement (VSIK) | ✅ Working | Proposal → Review → Approve |
| Knowledge Graph | ✅ Working | Relational model |
| Graph Visualization | ⚠️ CDN | Three.js from cdnjs |
| Typed Planning | ⚠️ Heuristic | Rule-based |
| Zero-Dependency | ⚠️ Optional | Feature flags available |

## Honest Assessment

### Working
- Append-only JSONL memory
- Policy enforcement
- VSIK loop (proposals)
- Knowledge Graph
- Basic CLI
- Basic daemon

### Partial/Gaps
- Typed planning is rule-based (not model-backed)
- Exception-only UX partial
- Three.js visualization uses CDN (not zero-dep for web)
- WASM not wired

## GoT→CoT→PVL

### GoT
- Full zero-dep kernel
- VSIK + Knowledge Graph working

### CoT
- v0.1.x: Zero-dep foundation
- v0.2.0: VSIK MVP
- v0.2.1: Knowledge Graph

### PVL
| Check | Status |
|-------|--------|
| Zero-dep core | ✅ |
| VSIK loop | ✅ |
| Knowledge Graph | ✅ |
| Visualization | ⚠️ CDN |

## Version History

| Version | Date | Changes |
|---------|------|---------|
| v0.2.1 | 2026-04-10 | Knowledge Graph + Three.js |
| v0.2.0 | 2026-04-10 | VSIK MVP |
| v0.1.7 | 2026-04-10 | MIT License |

## Recommended Next Steps

1. Wire WASM execution
2. Make typed planning model-backed
3. Optional: Inline Three.js or use canvas/svg for zero-dep viz