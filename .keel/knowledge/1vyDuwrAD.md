---
source_type: Story
source: stories/1vxppk4Oj/REFLECT.md
scope: 1vxYzSury/1vxpomgnN
source_story_id: 1vxppk4Oj
---

### 1vyDuwrAD: Centralized show projections reduce drift

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Multiple show commands were independently parsing PRD/SRS/story evidence with diverging placeholder and ordering rules. |
| **Insight** | A shared read-model projection layer stabilizes data contracts, keeps ordering deterministic, and lets renderers remain thin. |
| **Suggested Action** | Add new planning/read surfaces by extending `read_model::planning_show` first, then adapt renderer output only. |
| **Applies To** | `src/read_model/planning_show.rs`, `src/cli/commands/management/*/show.rs` |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-04T19:47:27+00:00 |
| **Score** | 0.84 |
| **Confidence** | 0.89 |
| **Applied** | yes |
