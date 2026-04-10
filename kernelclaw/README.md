# KernelClaw

A production-oriented Rust agent kernel implementing the KernelClaw MVH (Minimum Viable Hero) specification.

## About

This repo is an attempt to respond to Austen Allred's musings at https://x.com/i/status/2042444789891654076

> "Build an AI agent kernel that's:
> - Zero-dependency static binary
> - Append-only signed memory
> - Capability-based execution
> - Exception-only UX"

KernelClaw is a single-binary Rust daemon that provides the four core primitives needed for an AI coding agent:
1. **Typed Goal Interpreter** - Parses user goals into structured execution plans
2. **Append-Only Signed Memory Ledger** - Immutable audit trail with cryptographic receipts
3. **Capability-Based Executor** - Policy-gated execution with WASM sandbox isolation
4. **Exception-Only Notifier** - Silent success, noisy failures

---

## Research Arc

### The Problem Space

In the architecture of Autonomous Coding Agents (ACAs), we face a fundamental tension between capability and control. As agents become more capable, they also become harder to verify, constrain, and audit. Traditional agent implementations suffer from:

- **Opacity**: Internal decisions are opaque and untraceable
- **Mutable History**: Memory can be rewritten, enabling cover-ups
- **Unbounded Capabilities**: Any tool can be invoked without policy enforcement
- **Noisy UX**: Status updates flood the user on every operation

### The Journey

The KernelClaw specification emerged from analyzing failure modes in existing agent systems:

1. **Observation**: Current agents execute without verifiable receipts - if something breaks, there's no audit trail
2. **Insight**: Append-only data structures provideImmutable history - corrections append, never overwrite
3. **Hypothesis**: Signed receipts create verifiable audit trails that enable trust
4. **Implementation**: The four-primitive architecture balances capability with verifiability

### Key Insights

| Insight | Solution |
|---------|----------|
| Decisions are untraceable | Every action gets a signed receipt |
| Memory can be rewritten | Append-only ledger - corrections append, never edit |
| Tools have unbounded access | Capability model + policy engine gates execution |
| Users drown in notifications | Silent success, only exceptions surface |

---

## Quick Start

```bash
# Build
cargo build --release

# Initialize
./target/release/kernelclaw init

# Run a goal
./target/release kernelclaw run --goal "Read the project README"

# Check status
./target/release kernelclaw status
```

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     kernel-cli (kernelclaw)                │
└──────────────────────────┬──────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────┐
│                     kernel-core                              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐          │
│  │ Goal Engine │  │ Orchestrator│  │ Policy Gate │          │
│  └─────────────┘  └─────────────┘  └─────────────┘          │
└───────────��──────────────┬──────────────────────────────────┘
                           │
    ┌──────────┬───────────┼───────────┬──────────┐
    ▼          ▼           ▼           ▼          ▼
┌───────┐ ┌────────┐ ┌────────┐ ┌─────────┐ ┌──────────┐
│memory │ │crypto  │ │exec    │ │  llm    │ │ notify   │
│ledger │ │receipts│ │sandbox │ │ adapter │ │ notifier │
└───────┘ └────────┘ └────────┘ └─────────┘ └──────────┘
```

---

## The KernelClaw Four Primitives

### 1. Typed Goal Interpreter
Parses natural language goals into structured execution plans via local LLM (Ollama).

### 2. Append-Only Signed Memory Ledger
All significant events are recorded to an append-only store with cryptographic receipts.

### 3. Capability-Based Executor
Policy-engine gates determine which tools can execute. WASM sandboxing for untrusted code.

### 4. Exception-Only Notifier
Normal operation produces zero user-facing output. Only failures, violations, and anomalies surface.

---

## Core Features

| Feature | Implementation |
|---------|----------------|
| **Goal Interpretation** | Ollama client with structured output parsing |
| **Signed Receipts** | Ed25519 signatures for all material actions |
| **Policy Engine** | YAML-based declarative policy with allowlists |
| **Memory Ledger** | Append-only sled database with entry types |
| **Tool Execution** | file_read, file_read_dir, echo, calendar_summary |
| **Exception UX** | Silent success, noisy failures |

---

## Repository Structure

```
kernelclaw/
├── kernel-core/          # Orchestration
├── kernel-crypto/        # Signing & receipts
├── kernel-policy/        # Policy engine
├── kernel-memory/       # Append-only ledger
├── kernel-llm/          # LLM adapter
├── kernel-exec/          # Executor + tools
├── kernel-notify/        # Exception notifier
├── kernel-cli/           # CLI entry point
└── policy.yaml          # Default policy
```

---

## Implementation Philosophy

We prioritize:
- **Verifiability**: Every action produces a signed receipt
- **Minimalism**: Single static binary, no external dependencies at runtime
- **Safety**: Policy engine + capability model prevent unbounded execution
- **Auditability**: Append-only ledger enables complete reconstruction

---

## Installation

### From Binary (Recommended)

```bash
# Download latest release
curl -L https://github.com/pageman/KernelClaw-/releases/latest/download/kernelclaw -o kernelclaw
chmod +x kernelclaw
./kernelclaw init
```

### From Source

```bash
git clone https://github.com/pageman/KernelClaw-.git
cd KernelClaw
cargo build --release
./target/release/kernelclaw init
```

---

## Configuration

Default policy lives at `~/.kernelclaw/policy.yaml`:

```yaml
version: "0.1.0"
invariants:
  - no_network_access
requires_approval:
  - file_write
  - shell_exec
capability_allowlist:
  file_read:
    enabled: true
    allowed_paths:
      - ~/Documents/
      - /tmp/
```

---

## Usage

| Command | Description |
|---------|-------------|
| `kernelclaw init` | Initialize KernelClaw home (~/.kernelclaw/) |
| `kernelclaw daemon` | Start daemon mode (Unix socket) |
| `kernelclaw run --goal "..."` | Execute a single goal |
| `kernelclaw status` | Show daemon and policy status |
| `kernelclaw receipts list` | List recent receipts |

---

## Security Model

1. **Capability Model**: Only explicitly allowed capabilities can execute
2. **Policy Engine**: YAML policy gates all execution
3. **Signed Receipts**: All material actions generate Ed25519 signatures
4. **Append-Only Memory**: Historical record can never be modified

---

## Contributing

Contributions welcome! Please read the specification in docs/SPEC.md before submitting.

---

## Citation

If you use KernelClaw in your research, please cite:

```bibtex
@software{KernelClaw2025,
  author = {Paul Pajo},
  title = {KernelClaw: A Production-Oriented Rust Agent Kernel},
  year = {2025},
  url = {https://github.com/pageman/KernelClaw-}
}
```

---

## License

MIT License - see LICENSE file for details.

---

## Related

- [Sutskever 30 Implementations](https://github.com/pageman/sutskever-30-implementations) - 30 foundational deep learning papers in NumPy
- [Capability-Cartography-Layer-3](https://github.com/pageman/Capability-Cartography-Layer-3) - LLM capability assessment framework

---

*Version 0.1.0 - MVP Release*