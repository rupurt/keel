---
source_type: Story
source: stories/1vx8UtmC9/REFLECT.md
scope: 1vwq96cpt/1vx8TLqpp
source_story_id: 1vx8UtmC9
created_at: 2026-03-02T12:03:53
---

### 1vyDuw8wW: Enforce Root Layout With Contracts

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Large module migrations where old root files can remain importable after moves |
| **Insight** | Physical moves alone are not stable; contract tests must also assert forbidden `main.rs` module declarations and removed root file paths |
| **Suggested Action** | Pair every structural move with architecture contracts that check both declaration edges and on-disk paths |
| **Applies To** | src/main.rs, src/architecture_contract_tests.rs, src/**/mod.rs |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-02T12:15:00+00:00 |
| **Score** | 0.89 |
| **Confidence** | 0.92 |
| **Applied** | Added normalized-root and legacy-path assertions for all migrated root modules |
