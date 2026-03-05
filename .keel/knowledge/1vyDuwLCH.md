---
source_type: Story
source: stories/1vwqCe8MK/REFLECT.md
scope: 1vwq96cpt/1vwq9RqCe
source_story_id: 1vwqCe8MK
created_at: 2026-03-01T16:34:13
---

### 1vyDuwLCH: Ports Should Mirror Aggregate Boundaries

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Defining persistence abstractions before filesystem adapters are extracted from command modules |
| **Insight** | Repository ports are easier to evolve when contracts are grouped by aggregate boundary (story, voyage, epic, bearing, adr) plus one board snapshot port for orchestration use cases. |
| **Suggested Action** | Keep port interfaces in the application layer and defer adapter wiring to subsequent stories to minimize behavior risk during migration. |
| **Applies To** | src/application/ports.rs, upcoming infrastructure adapter stories |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-02T01:29:45+00:00 |
| **Score** | 0.82 |
| **Confidence** | 0.88 |
| **Applied** | yes |
