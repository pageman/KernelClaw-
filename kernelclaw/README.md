# KernelClaw - Agent Kernel

**Status**: v0.2.1 - VSIK + Knowledge Graph

> "Verifiable Self-Improving Kernel with relational knowledge and visualization."

## About - The Austen Allred Concern

KernelClaw is an attempt to implement the agent kernel from:
https://x.com/Austen/status/2042444789891654076

## Implementation Status (v0.2.1)

| Concern | Status | Notes |
|---------|--------|-------|
| Append-Only Memory | ✅ Working | Real JSONL with checksums |
| Policy at Tool Boundary | ✅ Working | allowed_paths enforced |
| Orchestrator Pipeline | ✅ Working | Full pipeline with policy |
| Self-Improvement (VSIK) | ✅ Working | Proposal → Review → Approve |
| Knowledge Graph | ✅ Working | Relational model + graph-aware proposals |
| Graph Visualization | ⚠️ CDN | Three.js from cdnjs (optional tool) |
| Typed Planning | ⚠️ Heuristic | Rule-based inference |
| Zero-Dependency | ✅ Core | All Rust deps have zero-dep alternatives |

## Quick Start

```bash
# Build
cargo build

# Initialize
cargo run -- init

# Run a goal
cargo run -- run "Write a hello world program"

# List receipts
cargo run -- receipts

# VSIK: List proposals
cargo run -- proposals list
```

## VSIK - Verifiable Self-Improving Kernel

### How It Works

1. **Failure occurs** → Orchestrator detects failure point
2. **Distillation** → Graph-aware proposal with related nodes
3. **Storage** → Proposal stored in ledger with checksum
4. **Review** → `kernelclaw proposals list` / `show <id>`
5. **Approval** → `kernelclaw proposals approve <id>`
6. **Activation** → Changes applied, activation receipt signed

### Proposal Commands

```bash
# List all proposals
kernelclaw proposals list

# Show proposal details
kernelclaw proposals show prop_abc123

# Approve and apply changes
kernelclaw proposals approve prop_abc123

# Reject proposal
kernelclaw proposals reject prop_abc123
```

## Knowledge Graph

### Node Types

- `Goal` - Attempted goals
- `Tool` - Available tools
- `Capability` - Permissions
- `Path` - File paths
- `FailureType` - Failure categories
- `Skill` - WASM skills
- `UserWorkflow` - Workflow patterns
- `Proposal` - Improvement proposals
- `SuccessPattern` - Successful patterns

### Graph Operations

```rust
use kernel_core::graph::{KnowledgeGraph, Node, Edge};

// Create graph
let mut graph = KnowledgeGraph::new();

// Add nodes
graph.add_node(Node::new("tool_file_read", NodeType::Tool, "file_read tool"));

// Add edges
graph.add_edge(Edge::new("tool_file_read", "cap_readonly", "requires"));

// Find related nodes
let related = graph.find_related("goal_123");

// Find connected to failure
let connections = graph.find_connected_to_failure("permission_denied");
```

### Graph Visualization

Open `tools/graph-viz.html` in a browser to see your knowledge graph:

**Note**: The visualization uses Three.js from CDN (cdnjs.cloudflare.com). This is an optional tool, not part of the core kernel. The core kernel has zero external Rust dependencies when using `--features use_zero_dep`.

```
# In browser
open tools/graph-viz.html

# Controls:
# - Click node for details
# - Drag to move nodes
# - r = reset camera
# - + / - = zoom
# - Space = relayout
```

## Architecture

```
kernel-cli          # CLI + VSIK + Graph commands
kernel-core       # Orchestration + proposals + graph
kernel-crypto     # Ed25519 signing
kernel-daemon    # Unix socket server
kernel-exec      # Tool execution
kernel-llm       # Ollama client
kernel-memory    # JSONL ledger
kernel-notify   # Notifications
kernel-policy   # YAML policy
```

## Zero-Dependency

**Core kernel (Rust)**: Full zero-dep achievable via `--features use_zero_dep`

| Dependency | Replacement |
|------------|-------------|
| serde | kernel-zero-serde |
| serde_json | kernel-zero-json |
| serde_yaml | kernel-zero-yaml |
| tokio | kernel-zero-tokio |
| dirs | kernel-zero-dirs |

**Note**: The optional visualization tool (tools/graph-viz.html) uses Three.js from CDN. This is NOT part of the core kernel.

## Dependencies

Using standard deps (default). Zero-dep available via feature flag.

## Version History

| Version | Date | Changes |
|---------|------|---------|
| v0.2.1 | 2026-04-10 | Knowledge Graph + Three.js viz |
| v0.2.0 | 2026-04-10 | VSIK MVP |
| v0.1.7 | 2026-04-10 | MIT License |
| v0.1.6 | 2026-04-10 | Honest assessment |

## Citation

```@software{KernelClaw,
  author = {Paul "The Pageman" Pajo},
  title = {KernelClaw - Agent Kernel},
  url = {https://github.com/pageman/KernelClaw-},
  email = {pageman@gmail.com},
  year = {2026},
  version = {0.2.1}
}
```

## License

MIT OR Apache-2.0