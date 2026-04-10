# KernelClaw - Agent Kernel

**Status**: v0.2.0 - VSIK (Verifiable Self-Improving Kernel)

> "VSIK MVP implemented - verifiable self-improvement loop active with mandatory user review."

## About - The Austen Allred Concern

KernelClaw is an attempt to implement the agent kernel from:
https://x.com/Austen/status/2042444789891654076

## Implementation Status (v0.2.0)

| Concern | Status | Notes |
|---------|--------|-------|
| Append-Only Memory | ✅ Working | Real JSONL with checksums |
| Policy at Tool Boundary | ✅ Working | allowed_paths enforced |
| Orchestrator Pipeline | ✅ Working | Full pipeline with policy |
| Self-Improvement (VSIK) | ✅ NEW | Proposal → Review → Approve |
| Typed Planning | ⚠️ Heuristic | Rule-based inference |
| Exception-Only UX | ⚠️ Partial | Some prints on success |
| Daemon | ⚠️ Basic | Unix socket, limited |
| WASM Runtime | ⚠️ Stub | Not wired in execution |
| Zero-Dependency | ⚠️ Optional | Feature flags available |

## VSIK - Verifiable Self-Improving Kernel

### How It Works

1. **Failure occurs** → Orchestrator detects failure point
2. **Distillation** → `distill_and_propose()` generates ImprovementProposal
3. **Storage** → Proposal stored in ledger with checksum
4. **User Review** → `kernelclaw proposals list` / `show <id>`
5. **Approval** → `kernelclaw proposals approve <id>`
6. **Activation** → Approved changes modify policy.yaml

### Proposal Structure

```rust
pub struct ImprovementProposal {
    pub id: String,                    // prop_{timestamp}
    pub failed_goal: String,
    pub failure_point: String,         // parse, validate, execute, etc.
    pub probable_cause: String,
    pub candidate_safeguard: String,
    pub proposed_changes: Vec<ProposedChange>,
    pub status: ProposalStatus,          // Pending, Approved, Rejected
    pub user_signature: Option<Vec<u8>>,
}
```

### CLI Commands

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

### Proposed Changes

- `PolicyRuleAdd`: Add new policy rule
- `NewWasmSkill`: Load new WASM skill
- `PlannerPromptPatch`: Update planner template
- `CapabilityRefinement`: Add to allowlist

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

# VSIK: List improvement proposals
cargo run -- proposals list
```

## Architecture

```
kernel-cli          # CLI + VSIK commands
kernel-core       # Orchestration + proposals
kernel-crypto     # Ed25519 signing + receipts
kernel-daemon    # Unix socket server
kernel-exec      # Tool execution
kernel-llm       # Ollama client
kernel-memory    # JSONL ledger + Proposal
kernel-notify   # Notifications
kernel-policy   # YAML policy engine
```

## Zero-Dependency Modules (11)

| Module | Replaces |
|--------|----------|
| kernel-zero | chrono, uuid, thiserror |
| kernel-zero-serde | serde |
| kernel-zero-json | serde_json |
| kernel-zero-yaml | serde_yaml |
| kernel-zero-tokio | tokio |
| kernel-zero-dirs | dirs |

Enable: `cargo build --features use_zero_dep`

## Dependencies

Using standard deps (default). Zero-dep available via feature flag.

## Version History

| Version | Date | Changes |
|---------|------|---------|
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
  version = {0.2.0}
}
```

## License

MIT OR Apache-2.0