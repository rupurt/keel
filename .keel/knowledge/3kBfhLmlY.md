---
source_type: Voyage
source: epics/1vwq96cpt/voyages/1vx8TLqpp/KNOWLEDGE.md
created_at: 2026-03-02T12:03:53
---

### 3kBfhLmlY: Enforce Root Layout With Contracts

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Large module migrations where old root files can remain importable after moves |
| **Insight** | Physical moves alone are not stable; contract tests must also assert forbidden `main.rs` module declarations and removed root file paths |
| **Suggested Action** | Pair every structural move with architecture contracts that check both declaration edges and on-disk paths |
| **Applies To** | src/main.rs, src/architecture_contract_tests.rs, src/**/mod.rs |
| **Linked Knowledge IDs** | 1vyDuw8wW |
| **Observed At** |  |
| **Score** | 0.89 |
| **Confidence** | 0.92 |
| **Applied** | Added normalized-root and legacy-path assertions for all migrated root modules |
