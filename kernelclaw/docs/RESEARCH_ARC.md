# RESEARCH_ARC.md - KernelClaw Research Journey

## v0.2.1 (2026-04-10 22:54) - VSIK + Knowledge Graph Complete

### Major Additions

- **Knowledge Graph module** (`kernel-core/graph.rs`)
  - Node, Edge, GraphPatch types
  - KnowledgeGraph with HashMap backend
  - find_related(), find_connected_to_failure()
  - generate_graph_aware_proposal()

- **Three.js Visualization** (`tools/graph-viz.html`)
  - Force-directed layout
  - Color-coded nodes by type
  - Interactive (click, drag)
  - Keyboard controls

### VSIK + Knowledge Graph Flow

1. **Failure** → Orchestrator detects failure point
2. **Graph Extraction** → Find related nodes in knowledge graph
3. **Graph Patch** → Generate subgraph (3-10 nodes/edges)
4. **Proposal** → Wrap in ImprovementProposal, sign
5. **Review** → User reviews via CLI or visualization
6. **Approval** → Merge into graph, apply changes

## Zero-Dependency Modules (11 Total)

| Module | Status |
|--------|--------|
| kernel-zero | ✅ |
| kernel-zero-ed25519 | ✅ |
| kernel-zero-serde | ✅ |
| kernel-zero-tokio | ✅ |
| kernel-zero-json | ✅ |
| kernel-zero-yaml | ✅ |
| kernel-zero-dirs | ✅ |
| kernel-zero-runtime | ⚠️ Stub |
| kernel-zero-async | ✅ |
| kernel-zero-derive | ✅ |
| kernel-zero-serde-derive | ✅ |

## GoT→CoT→PVL Pipeline

### GoT (Goal of Task)
Verifiable Self-Improving Kernel with relational knowledge

### CoT (Course of Task)
- v0.1.x: Zero-dep foundation
- v0.1.6: Honest assessment
- v0.2.0: VSIK MVP
- v0.2.1: Knowledge Graph + visualization ✅

### PVL (Verification)

| Check | Status |
|-------|--------|
| Memory durable | ✅ |
| Policy boundary | ✅ |
| Graph model | ✅ |
| VSIK proposals | ✅ |
| Graph visualization | ✅ |

## Version History

- v0.2.1 (36c40b2): Knowledge Graph + Three.js
- v0.2.0 (7531cbe): VSIK MVP
- v0.2.0 (0d28f62): Super-exhaustive metadata
- v0.1.7 (9b0bfa0): MIT License
- v0.1.6 (afc66f1): Honest assessment

## Honest Verdict

"VSIK with Knowledge Graph - verifiable self-improvement with relational model and visualization."

## Key Milestones

1. ✅ Append-only JSONL memory with checksums
2. ✅ Policy enforcement at tool boundary
3. ✅ VSIK loop (proposal → review → approve)
4. ✅ Knowledge Graph with relational model
5. ✅ Three.js visualization for graph exploration
6. ✅ Zero-dep alternatives available

## Remaining Work

- Wire WASM execution
- Make typed planning model-backed
- Add daemon /graph/export endpoint

## Critical Context

- **Repository**: pageman/KernelClaw-
- **HEAD**: 36c40b2
- **Edition**: Rust 2024