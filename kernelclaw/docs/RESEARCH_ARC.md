# KernelClaw - Research Journey

## Problem Space

KernelClaw emerged from Austen Allred's "Agent Desiderata": building trustworthy autonomous agents.

## Journey Log

### v1.0.3 - Honest Assessment (Current)

**Fixes applied:**
- Capability uses actual target path (not tool name)
- Policy wired to executor
- CLI prints on explicit request only (Status, Receipts, Run result)

### v1.0.2 - Post-Blocker Fix
- Fixed capability path issue
- Honest README

### v1.0.1 - Honest Pass
- CLI Run actually executes
- Policy wired to executor

### v1.0 - Zero-Dep Milestone
- Zero-dep modules created

## Current State (v1.0.3)

### Working
- Memory: JSONL with checksums
- Policy: enforced at boundary
- Orchestrator: full pipeline
- Execution: real via orchestrator

### Not Working (Acknowledged)
- Daemon mode
- WASM active path
- Full zero-dependency

## One Sentence Assessment

> "KernelClaw is an honest prototype with real enforcement at key boundaries."

## References

- Austen Allred: https://x.com/Austen
- Original: https://x.com/Austen/status/2042444789891654076