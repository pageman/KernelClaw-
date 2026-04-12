# KernelClaw - Agent Kernel

**Status**: v0.2.1 - VSIK + Knowledge Graph

> "Verifiable Self-Improving Kernel with relational knowledge and visualization."

## About - The Austen Allred Concern

KernelClaw is an attempt to implement the agent kernel from:
https://x.com/Austen/status/2042444789891654076

## Quick Start

```bash
# Build
cargo build

# Initialize
cargo run -- init

# Check status
cargo run -- status

# Run a goal
cargo run -- run "Write a hello world program"

# List receipts
cargo run -- receipts
```

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

## Architecture

```
┌─────────────────────────────────────┐
│           kernel-cli               │
│   (init, run, status, proposals)   │
└──────┬──────────┬────────────┬──────┘
       │          │            │
       ▼          ▼            ▼
   kernel-   kernel-    kernel-llm
    core     exec        (parsing)
       │          │            
       ▼          ▼            
   kernel-   kernel-    kernel-  
   policy   memory      daemon
       │          │            │
       ▼          ▼            ▼
   ┌─────────────────────────────────┐
   │    Zero-Dep Modules (11)        │
   │ kernel-zero, kernel-zero-*, etc│
   └─────────────────────────────────┘
```

## CLI Commands

### Basic Operations
```bash
# Initialize kernel
cargo run -- init

# Run a goal
cargo run -- run "Your goal here"

# Check system status
cargo run -- status

# List receipts
cargo run -- receipts
```

### VSIK Commands
```bash
# List improvement proposals
cargo run -- proposals list

# Show proposal details
cargo run -- proposals show <proposal_id>

# Approve and apply proposal
cargo run -- proposals approve <proposal_id>

# Reject proposal
cargo run -- proposals reject <proposal_id>
```

### Daemon
```bash
# Start daemon
cargo run -- daemon
```

## VSIK - Verifiable Self-Improving Kernel

### How It Works

1. **Failure occurs** → Orchestrator detects failure point
2. **Distillation** → Graph-aware proposal with related nodes
3. **Storage** → Proposal stored in ledger with checksum
4. **Review** → `kernelclaw proposals list` / `show <id>`
5. **Approval** → `kernelclaw proposals approve <id>`
6. **Activation** → Changes applied, activation receipt signed

### Proposal Structure

```rust
pub struct ImprovementProposal {
    pub id: String,           // prop_{timestamp}
    pub failed_goal: String,
    pub failure_point: String,
    pub probable_cause: String,
    pub candidate_safeguard: String,
    pub proposed_changes: Vec<ProposedChange>,
    pub status: ProposalStatus,  // Pending, Approved, Rejected
}
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
use kernel_core::graph::{KnowledgeGraph, Node, Edge, NodeType};

// Create graph
let mut graph = KnowledgeGraph::new();

// Add nodes
graph.add_node(Node::new("tool_file_read", NodeType::Tool, "file_read"));

// Add edges
graph.add_edge(Edge::new("tool_file_read", "cap_readonly", "requires"));

// Find related nodes
let related = graph.find_related("goal_123");

// Find connected to failure
let connections = graph.find_connected_to_failure("permission_denied");
```

### Graph Visualization

Open `tools/graph-viz.html` in a browser:

**Note**: Uses Three.js from CDN (cdnjs.cloudflare.com). This is an optional visualization tool, not part of the core kernel.

## Zero-Dependency Options

### Enable Zero-Dependency Mode

```toml
# Cargo.toml
[features]
default = ["use_std_deps"]
use_zero_dep = []
```

Or build with:
```bash
cargo build --features use_zero_dep
```

### Available Zero-Dep Modules

| Module | Replaces | LOC |
|--------|---------|-----|
| kernel-zero | chrono, uuid, thiserror, sha256 | ~800 |
| kernel-zero-ed25519 | ed25519-dalek | ~500 |
| kernel-zero-serde | serde | ~700 |
| kernel-zero-tokio | tokio | ~700 |
| kernel-zero-json | serde_json | ~10KB |
| kernel-zero-yaml | serde_yaml | ~5KB |
| kernel-zero-dirs | dirs | ~8.5KB |

## Policy Configuration

Edit `policy.yaml`:

```yaml
capabilities:
  - name: file_read
    allowed_paths:
      - /workspace/*

tools:
  - name: file_read
    capability: file_read
```

## Version History

| Version | Date | Changes |
|---------|------|---------|
| v0.2.1 | 2026-04-12 | Knowledge Graph + Improvement Report |
| v0.2.0 | 2026-04-10 | VSIK MVP |
| v0.1.7 | 2026-04-10 | MIT License |
| v0.1.6 | 2026-04-10 | Honest assessment |

## Documentation

- `README.md` - This file
- `docs/METADATA_ANALYSIS.md` - Full metadata analysis
- `docs/RESEARCH_ARC.md` - Research journey
- `docs/IMPROVEMENT_REPORT.md` - Improvement recommendations

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