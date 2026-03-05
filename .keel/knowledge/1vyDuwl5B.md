---
source_type: Story
source: stories/1vwqCfHz7/REFLECT.md
scope: 1vwq96cpt/1vwq9rycE
source_story_id: 1vwqCfHz7
created_at: 2026-03-02T08:14:31
---

### 1vyDuwl5B: Canonical read models remove adapter drift

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | When both flow rendering and diagnostics need the same capacity semantics |
| **Insight** | Duplicated DTOs and charge enums across adapters force conversion shims and create drift risk in UI logic. |
| **Suggested Action** | Keep one projection type in `read_model` and make interface modules thin adapters over that projection. |
| **Applies To** | `src/read_model/capacity.rs`, `src/flow/capacity.rs`, `src/commands/diagnostics/capacity.rs`, `src/flow/display.rs` |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-02T16:14:00+00:00 |
| **Score** | 0.89 |
| **Confidence** | 0.96 |
| **Applied** | story `1vwqCfHz7` |
