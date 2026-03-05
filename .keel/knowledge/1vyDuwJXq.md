---
source_type: Story
source: stories/1vwqCeiHm/REFLECT.md
scope: 1vwq96cpt/1vwq9RqCe
source_story_id: 1vwqCeiHm
created_at: 2026-03-02T07:57:52
---

### 1vyDuwJXq: Declarative Frontmatter Patches Reduce Drift Across Commands

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Multiple commands had bespoke line-replacement logic for status/scope/timestamp edits, increasing drift risk and maintenance overhead. |
| **Insight** | A shared mutation service with `set/remove` operations preserves behavior while eliminating duplicated frontmatter edit loops. |
| **Suggested Action** | Route future frontmatter changes through shared mutation primitives and add service-level tests for insertion/replacement/removal semantics. |
| **Applies To** | src/infrastructure/frontmatter_mutation.rs, src/commands/story/{link,unlink}.rs, src/commands/{adr,bearing}/mod.rs, src/application/voyage_epic_lifecycle.rs |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-02T15:56:05+00:00 |
| **Score** | 0.82 |
| **Confidence** | 0.90 |
| **Applied** | Migrated status/timestamp/scope mutations to infrastructure::frontmatter_mutation::apply. |
