---
source_type: Voyage
source: epics/1vxyM0hvn/voyages/1vxyMT6nz/KNOWLEDGE.md
created_at: 2026-03-04T19:11:59
---

### sTJiMO70u: Frontmatter Field Additions Need Builder + Literal Sweep

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Adding a new key to `StoryFrontmatter` that is constructed in many tests and read models |
| **Insight** | `#[serde(default)]` handles runtime parsing, but compile-time struct literals and test builders still require explicit wiring or defaults to avoid breakage and hidden drift in fixture generation. |
| **Suggested Action** | When adding frontmatter fields, immediately update `TestStory`, `StoryFactory`, and all explicit `StoryFrontmatter { ... }` literals in one slice before running broader checks. |
| **Applies To** | `src/domain/model/story.rs`, `src/test_helpers.rs`, read-model fixture tests |
| **Linked Knowledge IDs** | 1vyDuwlIj |
| **Observed At** |  |
| **Score** | 0.81 |
| **Confidence** | 0.92 |
| **Applied** | yes |
