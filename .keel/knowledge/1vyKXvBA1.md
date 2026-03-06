---
source_type: Story
source: stories/1vyGZgiTK/REFLECT.md
scope: 1vyFgR2MA/1vyFn0OuN
source_story_id: 1vyGZgiTK
created_at: 2026-03-05T18:04:07
---

### 1vyKXvBA1: Lineage Surfaces Need IDs And Prose Together

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Rendering planning lineage for human review while keeping canonical IDs visible for machine-checkable contracts |
| **Insight** | Planning surfaces become much more reviewable when each lineage row carries the canonical ID, the authored prose, and the parent/child disposition context together; token-only output hides meaning, while prose-only output hides the contract. |
| **Suggested Action** | For future lineage read models, project one canonical row format that combines identifiers with authored descriptions and relation context before the CLI renderer touches it. |
| **Applies To** | `src/read_model/planning_show.rs`, `src/cli/commands/management/epic/show.rs`, `src/cli/commands/management/voyage/show.rs` |
| **Linked Knowledge IDs** | 1vyJXGpcM |
| **Observed At** | 2026-03-06T02:04:15+00:00 |
| **Score** | 0.79 |
| **Confidence** | 0.90 |
| **Applied** | yes |
