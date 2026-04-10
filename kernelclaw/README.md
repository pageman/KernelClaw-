# KernelClaw

A production-oriented, single-binary Rust daemon implementing the KernelClaw MVH specification.

## Core Primitives

1. **Typed Goal Interpreter** - Parses user goals into structured execution plans
2. **Append-Only Signed Memory Ledger** - Immutable audit trail with cryptographic receipts
3. **Capability-Based Executor** - Policy-gated execution with WASM sandbox isolation
4. **Exception-Only Notifier** - Silent success, noisy failures

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
└──────────────────────────┬──────────────────────────────────┘
                           │
    ┌──────────┬───────────┼───────────┬──────────┐
    ▼          ▼           ▼           ▼          ▼
┌───────┐ ┌────────┐ ┌────────┐ ┌─────────┐ ┌──────────┐
│memory │ │crypto  │ │exec    │ │  llm    │ │ notify   │
│ledger │ │receipts│ │sandbox │ │ adapter │ │ notifier │
└───────┘ └────────┘ └────────┘ └─────────┘ └──────────┘
```

## Build

```bash
# Development
cargo build

# Release (static binary)
cargo build --release --target x86_64-unknown-linux-musl
```

## Status

See EXECUTION_STATUS.md for current phase progress.

## License

MIT OR Apache-2.0