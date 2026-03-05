---
source_type: Story
source: stories/1vx8V5uUT/REFLECT.md
scope: 1vwq96cpt/1vx8TLqpp
source_story_id: 1vx8V5uUT
---

### 1vyDuwLDX: Path-Wide Module Moves Need Import Rewrite First

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Relocating top-level module families (`commands`, `flow`, `next`) to a new root (`cli`) while preserving behavior. |
| **Insight** | Bulk file moves are low-risk only when import rewrites and architecture path fixtures are updated in the same slice; otherwise compile passes but contract tests drift. |
| **Suggested Action** | For physical normalization stories, perform move + import rewrite + fixture path updates atomically before running full test and doctor checks. |
| **Applies To** | src/main.rs, src/cli/**, src/architecture_contract_tests.rs |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-02T19:15:47+00:00 |
| **Score** | 0.84 |
| **Confidence** | 0.92 |
| **Applied** |  |
