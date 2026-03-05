---
source_type: Voyage
source: epics/1vwq96cpt/voyages/1vwq9rycE/KNOWLEDGE.md
scope: null
source_story_id: null
---

### ZXRUhJpNe: Canonical read models remove adapter drift

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | When both flow rendering and diagnostics need the same capacity semantics |
| **Insight** | Duplicated DTOs and charge enums across adapters force conversion shims and create drift risk in UI logic. |
| **Suggested Action** | Keep one projection type in `read_model` and make interface modules thin adapters over that projection. |
| **Applies To** | `src/read_model/capacity.rs`, `src/flow/capacity.rs`, `src/commands/diagnostics/capacity.rs`, `src/flow/display.rs` |
| **Linked Knowledge IDs** | 1vyDuwl5B |
| **Observed At** |  |
| **Score** | 0.89 |
| **Confidence** | 0.96 |
| **Applied** | story `1vwqCfHz7` |
