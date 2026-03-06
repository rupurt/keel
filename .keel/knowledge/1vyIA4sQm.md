---
source_type: Story
source: stories/1vyGZEZNc/REFLECT.md
scope: 1vyFgR2MA/1vyFiQPoH
source_story_id: 1vyGZEZNc
created_at: 2026-03-05T16:06:07
---

### 1vyIA4sQm: Scope Planning Diagnostics To Transition-Relevant Voyages

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | when doctor reuses planning-gate lineage checks |
| **Insight** | reporting the exact same lineage rules across the whole board retroactively fails historical `done` voyages that cannot exercise `voyage plan`, so diagnostic scope must match transition reachability unless migration is explicit work |
| **Suggested Action** | apply planning coherence checks to non-terminal voyages by default, and handle historical migrations in separate board-cleanup stories |
| **Applies To** | `src/cli/commands/diagnostics/doctor/checks/*.rs`, planning coherence diagnostics |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-06T00:10:00+00:00 |
| **Score** | 0.88 |
| **Confidence** | 0.95 |
| **Applied** | yes |
