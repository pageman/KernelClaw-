# KernelClaw - Agent Kernel

**Status**: v0.2.1 - VSIK + Knowledge Graph + Zero-Dep Options

> "Verifiable Self-Improving Kernel with relational knowledge and visualization."

## About - The Austen Allred Concern

KernelClaw is an attempt to implement the agent kernel from:
https://x.com/Austen/status/2042444789891654076

It addresses the safety and trust gap in autonomous AI systems through:
- **Append-only signed memory** for audit trails
- **Policy at tool boundary** for capability gating
- **VSIK self-improvement** for autonomous growth
- **Knowledge Graph** for relational reasoning
- **Zero-dep options** for minimal attack surface

## Quick Start

### Build

```bash
cargo build
```

### Initialize Kernel

```bash
cargo run -- init
```

### Run a Goal

```bash
cargo run -- run "Write a hello world program"
```

### Check Status

```bash
cargo run -- status
```

### List Receipts

```bash
cargo run -- receipts
```

## How-To-Use Guide

### Basic Operations

| Command | Description |
|---------|-------------|
| `cargo run -- init` | Initialize kernel in current directory |
| `cargo run -- run "<goal>"` | Execute a natural language goal |
| `cargo run -- status` | Show kernel status and memory |
| `cargo run -- receipts` | List execution receipts |
| `cargo run -- daemon` | Start as Unix socket daemon |

### VSIK Commands

| Command | Description |
|---------|-------------|
| `cargo run -- proposals list` | List improvement proposals |
| `cargo run -- proposals show <id>` | Show proposal details |
| `cargo run -- proposals approve <id>` | Approve and apply proposal |
| `cargo run -- proposals reject <id>` | Reject proposal |

### Configuration

| File | Purpose |
|------|---------|
| `policy.yaml` | Capability and path policies |
| `kernel.json` | Kernel configuration |

## Zero-Dependency Options

### Why Zero-Dep?

Reduces attack surface by eliminating external crate dependencies. All Rust deps have zero-dep alternatives.

### Enable Zero-Dep Mode

```toml
# Cargo.toml
[features]
default = ["use_zero_dep"]
use_std_deps = []
```

Or build with:

```bash
cargo build --release --features use_zero_dep
```

### Available Zero-Dep Modules

| Module | Replaces | LOC | Features |
|--------|---------|-----|---------|
| kernel-zero | chrono, uuid, thiserror, sha256 | ~800 | Time, ID, errors, hash |
| kernel-zero-ed25519 | ed25519-dalek | ~500 | Ed25519 signing |
| kernel-zero-serde | serde | ~700 | Serialize/Deserialize |
| kernel-zero-tokio | tokio | ~700 | Async runtime |
| kernel-zero-json | serde_json | ~10KB | JSON parsing |
| kernel-zero-yaml | serde_yaml | ~5KB | YAML parsing |
| kernel-zero-dirs | dirs | ~8.5KB | Directory paths |

### Feature Matrix

| Feature | With Deps | Zero-Dep |
|---------|---------|---------|
| Binary size | ~2MB | ~1.5MB |
| Build time | Fast | Slower |
| Compilation | Standard | More checks |

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

## CLI Commands Reference

### init

Initialize kernel in current directory:

```bash
cargo run -- init
```

Creates:
- `.kernel/` - Kernel directory
- `policy.yaml` - Default policy
- `kernel.json` - Configuration

### run

Execute a goal:

```bash
cargo run -- run "Read the README.md file"
```

The orchestrator:
1. Parses goal using LLM (if available)
2. Validates against policy
3. Executes tool
4. Signs receipt
5. Records to ledger
6. Updates Knowledge Graph

### status

Show system status:

```bash
cargo run -- status
```

Output:
- Memory ledger location
- Receipt count
- Policy status
- Graph statistics

### receipts

List execution receipts:

```bash
cargo run -- receipts
```

### proposals

VSIK improvement proposals:

```bash
# List all proposals
cargo run -- proposals list

# Show proposal
cargo run -- proposals show prop_1234567890

# Approve proposal
cargo run -- proposals approve prop_1234567890

# Reject proposal
cargo run -- proposals reject prop_1234567890
```

### daemon

Start as Unix socket server:

```bash
cargo run -- daemon
```

Listens on `/tmp/kernelclaw.sock` by default.

## Implementation Status

| Concern | Status | Notes |
|---------|--------|-------|
| Append-Only Memory | ✅ Working | Real JSONL with SHA256 checksums |
| Policy at Tool Boundary | ✅ Working | allowed_paths enforced |
| Orchestrator Pipeline | ✅ Working | Full pipeline with policy |
| Self-Improvement (VSIK) | ✅ Working | Proposal → Review → Approve |
| Knowledge Graph | ✅ Working | Relational model + graph-aware proposals |
| Graph Visualization | ⚠️ CDN | Three.js from cdnjs (optional) |
| Zero-Dependency | ✅ Core | All Rust deps have alternatives |

## Policy Configuration

### Default policy.yaml

```yaml
capabilities:
  - name: file_read
    allowed_paths:
      - /workspace/*
      - /tmp/*

  - name: file_write
    allowed_paths:
      - /workspace/*
    requires_approval: true

tools:
  - name: file_read
    capability: file_read
    description: Read file contents

  - name: file_write
    capability: file_write
    description: Write file contents
```

### Policy Rules

- `allowed_paths` - Glob patterns for allowed paths
- `requires_approval` - Require VSIK approval before execution
- `rate_limit` - Maximum requests per minute

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

### Node Types (9)

| Type | Description |
|-----|-------------|
| Goal | Attempted goals |
| Tool | Available tools |
| Capability | Permissions |
| Path | File paths |
| FailureType | Failure categories |
| Skill | WASM skills |
| UserWorkflow | Workflow patterns |
| Proposal | Improvement proposals |
| SuccessPattern | Successful patterns |

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

> **Note**: Uses Three.js from CDN (cdnjs.cloudflare.com). This is an optional visualization tool, not part of the core kernel.

## External LLM Tools

### ollama-bridge

Optional Ollama-compatible LLM proxy for parsing:

```bash
# Start bridge
./tools/start-bridge.sh 11434

# Bridge listens on port 11434
# Provides /api/generate endpoint
```

### Usage with KernelClaw

Configure `kernel.json`:

```json
{
  "llm": {
    "provider": "ollama",
    "endpoint": "http://localhost:11434"
  }
}
```

## Version History

| Version | Date | Changes |
|---------|------|---------|
| v0.2.1-patch | 2026-04-13 | Robustness fixes + tools |
| v0.2.1 | 2026-04-12 | Knowledge Graph |
| v0.2.0 | 2026-04-10 | VSIK MVP |
| v0.1.7 | 2026-04-10 | MIT License |

## Documentation

| File | Description |
|------|-------------|
| `README.md` | This file |
| `docs/METADATA_ANALYSIS.md` | Full metadata analysis |
| `docs/RESEARCH_ARC.md` | Research journey |
| `docs/IMPROVEMENT_REPORT.md` | Improvement recommendations |

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