# RESEARCH_ARC.md - KernelClaw Research Journey

## v0.1.6 (2026-04-10 21:19) - Honest Assessment

### Top Blockers (Fresh Pass)

1. README claims outrun code
2. Orchestrator uses target_path (fixed earlier)
3. CLI loads policy with with_policy (fixed earlier)
4. Typed planning is still rule-based
5. WASM is explicitly disabled
6. Exception-only UX partial
7. Workspace consistent (kernel-daemon exists)

### What's Fixed

- Real durable JSONL memory (not in-memory)
- Policy at file tool boundary enforced
- Orchestrator properly wired
- README now honest about gaps

## Zero-Dependency Modules (11 Total)

| Module | Status | Notes |
|--------|--------|-------|
| kernel-zero | ✅ Working | time, id, error, sha256 |
| kernel-zero-ed25519 | ✅ Working | RFC 8032 |
| kernel-zero-serde | ✅ Working | Serialize/Deserialize |
| kernel-zero-tokio | ✅ Working | Async runtime |
| kernel-zero-json | ✅ Working | JSON parsing |
| kernel-zero-yaml | ✅ Working | YAML parsing |
| kernel-zero-dirs | ✅ Working | XDG dirs |
| kernel-zero-runtime | ⚠️ Stub | Not wired |
| kernel-zero-async | ✅ Working |
| kernel-zero-derive | ✅ Working |
| kernel-zero-serde-derive | ✅ Working |

## Version History

- v0.1.6 (afc66f1): Honest assessment
- v0.1.5 (3f94471): JSON + YAML zero-dep
- v0.1.4 (a0ec984): Optional zero-dep wiring
- v1.3.0 (c8088ab): Full implementations

## GoT→CoT→PVL Pipeline

### GoT (Goal of Task)
Achieve hardened kernel implementation that addresses Austen's concerns

### CoT (Course of Task)
- Phase 1: Create zero-dep modules ✅
- Phase 2: Add honest assessment ⚠️
- Phase 3: Wire WASM execution ❌
- Phase 4: Make typed planning model-backed ❌

### PVL (Parallel Verification)

| Check | Status |
|-------|--------|
| Memory durable | ✅ |
| Policy boundary | ✅ |
| Daemon basic | ✅ |
| WASM wired | ❌ |
| Typed planner | ❌ |
| Exception-only | ❌ |

## Remaining Work

### Must Fix
1. Wire WASM runtime into execution path
2. Replace rule-based parse_goal with LLM call
3. Make UX truly exception-only

### Should Fix
4. Wire zero-dep as default
5. Add integration tests

### Nice to Have
6. Enhanced daemon commands
7. Better error messages

## Honest Verdict

"Partially credible prototype kernel, but not hardened proof that Austen's kernel has been built."

## Key Decisions Made

1. **Feature flags over complete rewrite**: Maintain compatibility
2. **Standard deps as default**: Production-ready
3. **Honest README**: Don't overclaim

## Critical Context

- **Repository**: pageman/KernelClaw-
- **HEAD**: afc66f1
- **Edition**: Rust 2024