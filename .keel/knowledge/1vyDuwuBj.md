---
source_type: Story
source: stories/1vxppkN0w/REFLECT.md
scope: 1vxYzSury/1vxpomgnN
source_story_id: 1vxppkN0w
created_at: 2026-03-04T10:15:05
---

### 1vyDuwuBj: Planning Show Parsing Needs Scaffold Filters

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Extracting PRD summaries from partially authored templates |
| **Insight** | Requirement parsing must explicitly ignore scaffold rows like `TODO`/template defaults or placeholder mode appears complete when it is not. |
| **Suggested Action** | Keep placeholder filters and add fixture tests that assert empty summaries on scaffold-only PRDs. |
| **Applies To** | `src/cli/commands/management/epic/show.rs`, planning projection parsers |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-04T18:10:15+00:00 |
| **Score** | 0.83 |
| **Confidence** | 0.92 |
| **Applied** | yes |
