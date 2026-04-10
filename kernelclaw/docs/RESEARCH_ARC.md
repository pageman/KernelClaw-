# RESEARCH_ARC.md - KernelClaw Research Journey

## v0.2.1 (2026-04-10 23:04) - Honest Assessment

### External Dependencies Analysis

#### Rust Dependencies (in Cargo.toml)
All 10 Rust dependencies are optional with zero-dep replacements:
- serde → kernel-zero-serde
- serde_json → kernel-zero-json
- serde_yaml → kernel-zero-yaml
- tokio → kernel-zero-tokio
- ed25519-dalek → kernel-zero-ed25519
- sha2 → kernel-zero
- uuid → kernel-zero
- chrono → kernel-zero
- thiserror → kernel-zero
- dirs → kernel-zero-dirs

#### CDN Dependencies (NOT in Cargo.toml)
| Resource | File | Status |
|----------|------|---------|
| Three.js | tools/graph-viz.html | CDN only |

Note: Three.js is used ONLY in optional visualization, NOT in core kernel.

## GoT→CoT→PVL Pipeline

### GoT (Goal of Task)
- Full zero-dependency kernel
- VSIK with Knowledge Graph
- Honest about remaining gaps

### CoT (Course of Task)
- v0.1.x: Zero-dep foundation ✅
- v0.2.0: VSIK MVP ✅
- v0.2.1: Knowledge Graph + Viz ⚠️ CDN

### PVL (Verification)

| Check | Status |
|-------|--------|
| Rust zero-dep | ✅ All optional |
| VSIK loop | ✅ Working |
| Knowledge Graph | ✅ Working |
| Visualization | ⚠️ CDN |

## Honest Verdict

- **Core kernel**: Full zero-dep achievable
- **Visualization**: Uses Three.js from CDN (not zero-dep, but optional tool)

## Critical Context

- **Repository**: pageman/KernelClaw-
- **HEAD**: 931c1aa
- **Status**: VSIK + Knowledge Graph with honest gap assessment