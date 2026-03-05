# Reflection - Extract Frontmatter Mutation Service

## Knowledge

### 1vyDuwJXq: Declarative Frontmatter Patches Reduce Drift Across Commands

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Multiple commands had bespoke line-replacement logic for status/scope/timestamp edits, increasing drift risk and maintenance overhead. |
| **Insight** | A shared mutation service with `set/remove` operations preserves behavior while eliminating duplicated frontmatter edit loops. |
| **Suggested Action** | Route future frontmatter changes through shared mutation primitives and add service-level tests for insertion/replacement/removal semantics. |
| **Applies To** | src/infrastructure/frontmatter_mutation.rs, src/commands/story/{link,unlink}.rs, src/commands/{adr,bearing}/mod.rs, src/application/voyage_epic_lifecycle.rs |
| **Observed At** | 2026-03-02T15:56:05Z |
| **Score** | 0.82 |
| **Confidence** | 0.9 |
| **Applied** | Migrated status/timestamp/scope mutations to infrastructure::frontmatter_mutation::apply. |

## Observations

Migration was straightforward once mutation semantics were encoded as reusable `set/remove` operations.
The highest-risk area was preserving existing ADR supersede behavior; expressing list updates as explicit field replacement avoided brittle substring replacements.
Full-suite tests provided confidence that existing command behavior stayed intact after centralizing mutation logic.
