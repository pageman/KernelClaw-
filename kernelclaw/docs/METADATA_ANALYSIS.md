# METADATA_ANALYSIS.md - KernelClaw v0.2.1 (VSIK + Knowledge Graph)

## Repository Overview

- **Repository**: pageman/KernelClaw-
- **HEAD**: 36c40b2 (v0.2.1)
- **Version**: 0.2.1
- **Edition**: 2024
- **Status**: VSIK (Verifiable Self-Improving Kernel) with Knowledge Graph

## Crate Inventory

### Main Crates (9 crates)

| Crate | Purpose | Dependencies |
|-------|---------|--------------|
| kernel-cli | CLI + VSIK commands | tokio, dirs |
| kernel-core | Orchestration + proposals + graph | tokio, serde, serde_yaml |
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
| kernel-zero | ~800 | chrono, uuid, thiserror | ✅ |
| kernel-zero-ed25519 | ~500 | ed25519-dalek | ✅ |
| kernel-zero-serde | ~700 | serde | ✅ |
| kernel-zero-tokio | ~700 | tokio | ✅ |
| kernel-zero-json | ~10KB | serde_json | ✅ |
| kernel-zero-yaml | ~5KB | serde_yaml | ✅ |
| kernel-zero-dirs | ~8.5KB | dirs | ✅ |
| kernel-zero-runtime | ~2KB | - | ⚠️ Stub |
| kernel-zero-async | 250 | - | ✅ |
| kernel-zero-derive | 250 | - | ✅ |
| kernel-zero-serde-derive | 100 | - | ✅ |

### Tools

| Directory | Purpose |
|----------|---------|
| tools/graph-viz.html | Three.js Knowledge Graph visualization |

## Implementation Status (v0.2.1)

| Concern | Status | Notes |
|---------|--------|-------|
| Append-Only Memory | ✅ Working | Real JSONL with checksums |
| Policy at Tool Boundary | ✅ Working | allowed_paths enforced |
| Orchestrator Pipeline | ✅ Working | Full pipeline with policy |
| Self-Improvement (VSIK) | ✅ Working | Proposal → Review → Approve |
| **Knowledge Graph** | ✅ NEW | Relational model + graph-aware proposals |
| **Graph Visualization** | ✅ NEW | Three.js web UI |
| Typed Planning | ⚠️ Heuristic | Rule-based inference |
| Exception-Only UX | ⚠️ Partial | Some prints on success |
| Zero-Dependency | ⚠️ Optional | Feature flags available |

## VSIK + Knowledge Graph Features

### Knowledge Graph

```rust
// Node types
NodeType::Goal, Tool, Capability, Path, FailureType, Skill, 
         UserWorkflow, Proposal, SuccessPattern

// Graph operations
graph.add_node(node);
graph.add_edge(edge);
graph.find_related(node_id);
graph.find_connected_to_failure(failure_type);

// Graph-aware proposal
generate_graph_aware_proposal(failure_point, error, &graph);
```

### VSIK Loop

1. **Failure** → Orchestrator detects failure point
2. **Distillation** → Graph-aware proposal with related nodes
3. **Storage** → Signed proposal in ledger
4. **Review** → `kernelclaw proposals list` / `show`
5. **Approval** → `kernelclaw proposals approve <id>`
6. **Activation** → Changes applied, activation receipt signed

### Graph Visualization

- Three.js force-directed layout
- Color-coded nodes by type
- Click for node details
- Drag to reposition
- Keyboard controls (r, +/-, space)
- Proposal highlighting (yellow glow)

## CLI Commands

```bash
# Goal execution
kernelclaw run "Write a hello world"

# VSIK Proposals
kernelclaw proposals list
kernelclaw proposals show <id>
kernelclaw proposals approve <id>
kernelclaw proposals reject <id>
```

## External Dependencies

| Dependency | Status | Replacement |
|------------|--------|-------------|
| serde | Optional | kernel-zero-serde |
| serde_json | Optional | kernel-zero-json |
| serde_yaml | Optional | kernel-zero-yaml |
| tokio | Optional | kernel-zero-tokio |
| dirs | Optional | kernel-zero-dirs |

## GoT→CoT→PVL

### Goal of Task (GoT)
- Full VSIK with knowledge graph
- Zero-dep philosophy maintained
- Visualization for review

### Course of Task (CoT)
- v0.1.x: Zero-dep foundation
- v0.1.6: Honest assessment
- v0.2.0: VSIK MVP
- v0.2.1: Knowledge Graph + visualization ✅

### PVL (Parallel Verification)

| Check | Status |
|-------|--------|
| Memory durable | ✅ |
| Policy boundary | ✅ |
| Graph model | ✅ |
| VSIK proposals | ✅ |
| Graph visualization | ✅ |
| Zero-dep options | ✅ |

## Recommended Next Steps

### Priority 1
1. Wire WASM execution in kernel-exec
2. Make parse_goal use actual LLM
3. Add /graph/export daemon endpoint

### Priority 2
4. Add entity extraction after failures
5. Wire zero-dep as default
6. Add integration tests for VSIK loop

### Priority 3
7. Enhanced daemon commands
8. Better error messages

## Version History

| Version | Date | Changes |
|---------|------|---------|
| v0.2.1 | 2026-04-10 | Knowledge Graph + Three.js viz |
| v0.2.0 | 2026-04-10 | VSIK MVP |
| v0.1.7 | 2026-04-10 | MIT License |
| v0.1.6 | 2026-04-10 | Honest assessment |

## Metrics

- **Total LOC**: ~35,000
- **Zero-dep LOC**: ~25,000
- **Crates**: 20
- **Tools**: 1 (graph-viz.html)

## Critical Context

- **Repo URL**: https://github.com/pageman/KernelClaw-
- **Branch**: master
- **Status**: VSIK with Knowledge Graph - Verifiable Self-Improving Kernel