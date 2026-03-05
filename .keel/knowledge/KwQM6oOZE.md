---
source_type: Voyage
source: epics/1vwq96cpt/voyages/1vwq9rycE/KNOWLEDGE.md
created_at: 2026-03-02T10:34:57
---

### KwQM6oOZE: Interface Adapters Should Delegate Instead Of Recompute

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | When both flow rendering and diagnostics commands need the same projection outputs |
| **Insight** | Duplicated adapter-level projection/load/render paths drift quickly and should be collapsed behind a single interface that consumes canonical read-model DTOs |
| **Suggested Action** | Keep one shared capacity interface and enforce delegation from command modules through architecture contracts |
| **Applies To** | `src/commands/diagnostics/capacity.rs`, `src/flow/capacity.rs`, `src/architecture_contract_tests.rs` |
| **Linked Knowledge IDs** | 1vyDuwCgL |
| **Observed At** |  |
| **Score** | 0.83 |
| **Confidence** | 0.90 |
| **Applied** | Delegated diagnostics capacity command to `flow::capacity` and added explicit contract test for shared interface usage |
