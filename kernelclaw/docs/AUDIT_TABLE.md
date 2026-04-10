# KernelClaw - Strict Audit Table

## v0.1.6 - Claim vs Reality vs Priority

| Claim in README/About | Current Evidence | Verdict | Priority |
|-----------------------|------------------|---------|----------|
| **Zero-dependency static binary** | 30+ crates in Cargo.toml | ❌ FALSE | LOW |
| **Append-only signed memory** | Mutex<Vec<LedgerEntry>> in-memory | ❌ FALSE | HIGH |
| **Append-only sled database** | No sled usage in kernel-memory | ❌ FALSE | HIGH |
| **Capability-based execution** | Native dispatch, no sandbox | ❌ FALSE | HIGH |
| **Policy enforced at tool boundary** | file_read doesn't check allowed_paths | ❌ FALSE | HIGH |
| **WASM sandbox isolation** | wasmtime not in executor path | ❌ FALSE | MEDIUM |
| **Typed goal interpreter** | Schema defined, not wired to orchestration | ❌ FALSE | HIGH |
| **Exception-only UX** | CLI prints "[STUB]" on happy path | ⚠️ PARTIAL | MEDIUM |
| **Full orchestrator pipeline** | Just creates receipts, no real execution | ❌ FALSE | HIGH |
| **Daemon mode implemented** | CLI says "[NOTIMPL]" | ❌ FALSE | MEDIUM |
| Ed25519 signing/verification | Full implementation in kernel-crypto | ✅ REAL | - |
| Policy YAML loading | Working in kernel-policy | ✅ REAL | - |
| CLI exception-only (errors) | Errors to stderr | ✅ REAL | - |

---

## Gaps Ranked by Priority

### P0 - Must Fix (Breaks Trust)

| Gap | Evidence | Fix |
|-----|-----------|-----|
| **Memory not durable** | `entries: Mutex<Vec<>>` | Add JSONL persistence |
| **Policy not enforced** | file_read doesn't check allowed_paths | Wire ToolPolicy to file_read |
| **Goal not parsed** | LLM returns raw string | Wire ParsedGoal to orchestration |
| **No real execution** | execute_goal() just creates receipt | Full pipeline implementation |

### P1 - Should Fix (Breaking Claims)

| Gap | Evidence | Fix |
|-----|-----------|-----|
| **WASM not active** | wasmtime in Cargo.toml but unused | Add wasm path or remove claim |
| **Policy docs overclaim** | "ENFORCED" in YAML but not enforced | Update YAML language |
| **README overclaim** | "Strong" status markers | Fix to "Partial" or "Stub" |

### P2 - Nice to Have

| Gap | Evidence | Fix |
|-----|-----------|-----|
| **Zero-dependency** | 30+ crates | Use kernel-zero-* modules or update claim |
| **Daemon not implemented** | CLI marker | Implement or remove command |
| **Receipt listing** | CLI marker | Implement |

---

## Honest Summary

**Working**: Crypto signing, Policy loading, CLI error handling
**Stubbed**: Goal execution, Daemon, Receipt listing
**Missing**: Memory durability, Policy enforcement at boundary, Real pipeline

### One Sentence Assessment

> "KernelClaw is currently a persuasive repo-shaped argument for Austen's kernel, not yet Austen's kernel proven in code."

---

## Recommended Next Steps (Priority Order)

1. **WIRE UP kernel-zero-time** - Replace chrono dep
2. **Add JSONL persistence** - Fix memory gap  
3. **Wire ToolPolicy to file_read()** - Fix policy gap
4. **Wire ParsedGoal to execute_goal()** - Fix goal gap
5. **Full pipeline in orchestrator** - Fix execution gap
6. **Update all claims in README** - Match reality

After P0 fixes: Re-audit and update README claims.