---
source_type: Story
source: stories/1vwqCfma0/REFLECT.md
scope: 1vwq96cpt/1vwq9rycE
source_story_id: 1vwqCfma0
created_at: 2026-03-02T10:34:57
---

### 1vyDuwCgL: Interface Adapters Should Delegate Instead Of Recompute

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | When both flow rendering and diagnostics commands need the same projection outputs |
| **Insight** | Duplicated adapter-level projection/load/render paths drift quickly and should be collapsed behind a single interface that consumes canonical read-model DTOs |
| **Suggested Action** | Keep one shared capacity interface and enforce delegation from command modules through architecture contracts |
| **Applies To** | `src/commands/diagnostics/capacity.rs`, `src/flow/capacity.rs`, `src/architecture_contract_tests.rs` |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-02T18:35:00+00:00 |
| **Score** | 0.83 |
| **Confidence** | 0.90 |
| **Applied** | Delegated diagnostics capacity command to `flow::capacity` and added explicit contract test for shared interface usage |
