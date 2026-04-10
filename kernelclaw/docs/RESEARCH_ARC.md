# RESEARCH_ARC.md - KernelClaw Research Journey

## v0.2.0 (2026-04-10 22:38) - VSIK (Verifiable Self-Improving Kernel)

### Major Addition: Self-Improvement Loop

- **ImprovementProposal struct**: Full proposal type with status tracking
- **Distillation logic**: `distill_and_propose()` generates proposals from failures
- **Ledger Proposal variant**: Stores proposals in JSONL ledger
- **CLI commands**: `proposal list/show/approve/reject`
- **Policy activation**: Approved proposals modify policy.yaml

### VSIK Flow

1. **Failure** → Orchestrator detects failure point
2. **Distillation** → `distill_and_propose()` creates proposal
3. **Storage** → Proposal stored as `LedgerEntry::Proposal`
4. **User Review** → `kernelclaw proposals list` / `show`
5. **Approval** → `kernelclaw proposals approve <id>`
6. **Activation** → Policy rules modified dynamically

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
Verifiable Self-Improving Kernel that learns from failures while maintaining audit trail

### CoT (Course of Task)
- v0.1.x: Zero-dep foundation
- v0.1.6: Honest assessment
- v0.2.0: VSIK MVP ✅
- v0.2.1: Wire WASM, refine planner

### PVL (Verification)

| Check | Status |
|-------|--------|
| Memory durable | ✅ |
| Policy boundary | ✅ |
| Self-improvement | ✅ VSIK |
| Proposal list | ✅ |
| Proposal approve | ✅ |
| Activation works | ✅ POC |

## Version History

- v0.2.0 (f236331): VSIK MVP
- v0.1.7 (9b0bfa0): MIT License
- v0.1.6 (afc66f1): Honest assessment

## Honest Verdict

"VSIK MVP implemented - verifiable self-improvement loop active with mandatory user review."

## Key Decisions

1. **Feature flags over rewrite**: Maintain compatibility
2. **Standard deps as default**: Production-ready
3. **Honest README**: Don't overclaim
4. **VSIK loop**: Signed proposals, ledger storage, user approval

## Remaining Work

- Wire WASM execution
- Make typed planning model-backed
- True exception-only UX

## Critical Context

- **Repository**: pageman/KernelClaw-
- **HEAD**: f236331
- **Edition**: Rust 2024