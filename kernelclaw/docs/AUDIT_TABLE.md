# KernelClaw - Strict Audit Table (v0.1.7)

## Claim vs Reality (UPDATED)

| Claim in README/About | Current Evidence | Verdict | Priority |
|-----------------------|------------------|---------|----------|
| **Zero-dependency** | Standard crates | ⚠️ PARTIAL | LOW |
| **Append-only memory** | JSONL with SHA256 | ✅ FIXED | DONE |
| **Capability execution** | Native dispatch + policy | ✅ FIXED | DONE |
| **Policy at tool boundary** | file_read() checks allowed_paths | ✅ FIXED | DONE |
| **WASM sandbox** | Not in active path | ❌ STUB | MEDIUM |
| **Typed goal interpreter** | ParsedGoal wired to orchestrator | ✅ FIXED | DONE |
| **Exception-only UX** | CLI errors to stderr | ✅ WORKING | DONE |
| **Full orchestrator** | parse→validate→execute→receipt→record | ✅ FIXED | DONE |
| **Ed25519 signing** | Full implementation | ✅ REAL | - |
| **Policy loading** | YAML→in-memory | ✅ REAL | - |

## v0.1.7 Changes

| Gap | Was | Now |
|-----|-----|-----|
| Memory durability | In-memory Mutex | **JSONL with checksums** |
| Policy enforcement | Not at boundary | **Enforced in file_read()** |
| Goal parsing | Schema unused | **ParsedGoal wired** |
| Orchestrator | Just receipts | **Full pipeline** |
| CLI Run | Stubbed | **Real execution** |
| CLI Receipts | Stubbed | **Lists from ledger** |

## Remaining Gaps (v0.1.7)

| Gap | Status | Notes |
|-----|--------|-------|
| WASM sandbox | Not active | Runtime not integrated |
| Daemon mode | Not implemented | Socket listener stub |

## One Sentence Assessment (v0.1.7)

> "KernelClaw now has working implementations for most core concerns after v0.1.7 pipeline fixes."

---

**Next Steps**: 
- Add WASM runtime to active path
- Implement daemon mode