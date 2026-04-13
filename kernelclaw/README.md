# KernelClaw - Agent Kernel

**Status**: v0.2.1 - VSIK + Knowledge Graph + Real LLM Integration

> "Verifiable Self-Improving Kernel with relational knowledge and Ollama-powered parsing."

## About - The Austen Allred Concern

KernelClaw is an attempt to implement the agent kernel from:
https://x.com/Austen/status/2042444789891654076

It addresses the safety and trust gap in autonomous AI systems through:
- **Append-only signed memory** for audit trails
- **Policy at tool boundary** for capability gating
- **VSIK self-improvement** for autonomous growth
- **Knowledge Graph** for relational reasoning
- **Real LLM integration** for goal parsing

## Quick Start

### Build

```bash
cargo build
```

### Configure LLM (Optional)

```bash
export KERNELCLAW_OLLAMA_ENDPOINT=http://localhost:11434
export KERNELCLAW_MODEL=gemma4:e2b
```

### Initialize Kernel

```bash
cargo run -- init
```

### Run a Goal

```bash
cargo run -- run "Read the README.md file"
```

## How-To-Use Guide

### Configuration

| Environment Variable | Default | Description |
|---------------------|----------|-------------|
| `KERNELCLAW_OLLAMA_ENDPOINT` | `http://localhost:11434` | Ollama API endpoint |
| `KERNELCLAW_MODEL` | `gemma4:e2b` | Default model |

### CLI Commands

| Command | Description |
|---------|-------------|
| `cargo run -- init` | Initialize kernel |
| `cargo run -- run "<goal>"` | Execute a goal |
| `cargo run -- status` | Show system status |
| `cargo run -- receipts` | List execution receipts |
| `cargo run -- proposals list` | List VSIK proposals |
| `cargo run -- proposals approve <id>` | Approve proposal |

### Tool-to-Capability Mapping

The **FIXED** mapping derives capabilities from the selected tool:

| Tool | Required Capabilities | Risk |
|------|----------------------|------|
| `file_read`, `file_read_dir`, `file_metadata` | `["file_read"]` | Low |
| `file_write` | `["file_write"]` | High |
| `echo`, `calendar_summary`, `health_check` | `["echo"]` | Low |

## Zero-Dependency Options

### Enable Zero-Dep Mode

```bash
cargo build --release --features use_zero_dep
```

### Available Zero-Dep Modules

| Module | Replaces | Features |
|--------|----------|----------|
| kernel-zero | chrono, uuid, thiserror, sha256 | Time, ID, errors, hash |
| kernel-zero-ed25519 | ed25519-dalek | Ed25519 signing |
| kernel-zero-json | serde_json | JSON parsing |
| kernel-zero-yaml | serde_yaml | YAML parsing |
| kernel-zero-dirs | dirs | Directory paths |

## Architecture

```
┌─────────────────────────────────────┐
│           kernel-cli               │
│   (init, run, status, proposals)   │
└──────┬──────────┬────────────┬──────┘
       │          │            │
       ▼          ▼            ▼
   kernel-   kernel-    kernel-llm
    core     exec        (HTTP)
       │          │            
       ▼          ▼            
   kernel-   kernel-    kernel-  
   policy   memory      daemon
       │          │            │
       ▼          ▼            ▼
   ┌─────────────────────────────────┐
   │    Zero-Dep Modules (11)        │
   └─────────────────────────────────┘
```

## Implementation Status

| Concern | Status | Notes |
|---------|--------|-------|
| Append-Only Memory | ✅ Working | JSONL + SHA256 |
| Policy at Tool Boundary | ✅ Working | Unified kernel_policy |
| Orchestrator Pipeline | ✅ Working | Full pipeline |
| Self-Improvement (VSIK) | ✅ Working | Proposal → Review → Approve |
| Knowledge Graph | ✅ Working | 9 node types |
| LLM Parsing | ✅ Working | Real HTTP to Ollama |
| Zero-Dependency | ✅ Core | All Rust deps have alternatives |

## LLM Integration

### How It Works

1. **Goal received** → Send to Ollama `/api/generate`
2. **LLM parses** → Returns structured JSON (ParsedGoal)
3. **Validate** → Check tool, risk, capabilities
4. **Execute** → Policy check → Tool boundary → Execute
5. **Receipt** → Sign and record to ledger

### System Prompt

The KernelClaw system prompt instructs Ollama to return exactly:

```json
{
  "task_id": "goal_123",
  "tool_name": "file_read",
  "parameters": {"path": "/workspace/README.md"},
  "justification": "Reading the README to understand the project",
  "risk_level": "low",
  "required_capabilities": ["file_read"],
  "expected_output_type": "text"
}
```

### Configuration

```bash
# Required for local Ollama
export KERNELCLAW_OLLAMA_ENDPOINT=http://localhost:11434
export KERNELCLAW_MODEL=gemma4:e2b  # Safe for 8GB Mac

# Or use a different model
export KERNELCLAW_MODEL=llama3.2:3b
```

## Policy Configuration

### Default Policy

```yaml
version: "0.1.0"
capabilities:
  - name: file_read
    allowed: true
    allowed_paths:
      - /tmp/
      - /workspace/
  - name: file_write
    allowed: false
    requires_approval: true
  - name: shell
    allowed: false
allowed_paths:
  - /tmp/
  - /workspace/
max_file_size: 1048576
```

## VSIK - Verifiable Self-Improving Kernel

### How It Works

1. **Failure occurs** → Orchestrator detects failure
2. **Distillation** → Graph-aware proposal
3. **Storage** → Proposal in ledger with checksum
4. **Review** → `proposals list` / `show <id>`
5. **Approval** → `proposals approve <id>`
6. **Activation** → Changes applied, receipt signed

## Knowledge Graph

### Node Types (9)

| Type | Description |
|------|-------------|
| Goal | Attempted goals |
| Tool | Available tools |
| Capability | Permissions |
| Path | File paths |
| FailureType | Failure categories |
| Skill | WASM skills |
| UserWorkflow | Workflow patterns |
| Proposal | Improvement proposals |
| SuccessPattern | Successful patterns |

## Version History

| Version | Date | Changes |
|---------|------|---------|
| v0.2.1-llm | 2026-04-13 | Real LLM HTTP + unified policy |
| v0.2.1-patch | 2026-04-13 | Robustness fixes |
| v0.2.1 | 2026-04-12 | Knowledge Graph |
| v0.2.0 | 2026-04-10 | VSIK MVP |
| v0.1.7 | 2026-04-10 | MIT License |

## Documentation

| File | Description |
|------|-------------|
| `README.md` | This file |
| `docs/METADATA_ANALYSIS.md` | Full metadata analysis |
| `docs/RESEARCH_ARC.md` | Research journey |

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