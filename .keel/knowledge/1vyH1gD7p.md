---
source_type: Story
source: stories/1vyGZEO8S/REFLECT.md
scope: 1vyFgR2MA/1vyFiQPoH
source_story_id: 1vyGZEO8S
created_at: 2026-03-05T15:54:30
---

### 1vyH1gD7p: Preserve Empty Markdown Table Cells In Planning Parsers

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Parsing authored PRD/SRS markdown tables where an empty cell is semantically meaningful |
| **Insight** | Splitting markdown table rows and then filtering empty columns hides missing required values like SRS `Source`, so contract validation must preserve column positions and only trim boundary pipes and whitespace |
| **Suggested Action** | Use a shared row splitter that preserves interior empty cells before resolving header-based column indexes |
| **Applies To** | `src/domain/state_machine/*.rs`, planning document table parsers |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-05T23:54:30+00:00 |
| **Score** | 0.84 |
| **Confidence** | 0.93 |
| **Applied** |  |
