---
source_type: Voyage
source: epics/1vxYzSury/voyages/1vxpomgnN/KNOWLEDGE.md
created_at: 2026-03-04T11:56:17
---

### PpRjlSIuB: Centralized show projections reduce drift

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Multiple show commands were independently parsing PRD/SRS/story evidence with diverging placeholder and ordering rules. |
| **Insight** | A shared read-model projection layer stabilizes data contracts, keeps ordering deterministic, and lets renderers remain thin. |
| **Suggested Action** | Add new planning/read surfaces by extending `read_model::planning_show` first, then adapt renderer output only. |
| **Applies To** | `src/read_model/planning_show.rs`, `src/cli/commands/management/*/show.rs` |
| **Linked Knowledge IDs** | 1vyDuwrAD |
| **Observed At** |  |
| **Score** | 0.84 |
| **Confidence** | 0.89 |
| **Applied** | yes |
