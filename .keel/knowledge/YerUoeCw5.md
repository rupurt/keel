---
source_type: Voyage
source: epics/1vwq96cpt/voyages/1vwq9RqCe/KNOWLEDGE.md
scope: null
source_story_id: null
---

### YerUoeCw5: Declarative Frontmatter Patches Reduce Drift Across Commands

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Multiple commands had bespoke line-replacement logic for status/scope/timestamp edits, increasing drift risk and maintenance overhead. |
| **Insight** | A shared mutation service with `set/remove` operations preserves behavior while eliminating duplicated frontmatter edit loops. |
| **Suggested Action** | Route future frontmatter changes through shared mutation primitives and add service-level tests for insertion/replacement/removal semantics. |
| **Applies To** | src/infrastructure/frontmatter_mutation.rs, src/commands/story/{link,unlink}.rs, src/commands/{adr,bearing}/mod.rs, src/application/voyage_epic_lifecycle.rs |
| **Linked Knowledge IDs** | 1vyDuwJXq |
| **Observed At** |  |
| **Score** | 0.82 |
| **Confidence** | 0.90 |
| **Applied** | Migrated status/timestamp/scope mutations to infrastructure::frontmatter_mutation::apply. |
