---
source_type: Story
source: stories/1vxyMtAbK/REFLECT.md
scope: 1vxyM0hvn/1vxyMT6nz
source_story_id: 1vxyMtAbK
---

### 1vyDuwlIj: Frontmatter Field Additions Need Builder + Literal Sweep

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Adding a new key to `StoryFrontmatter` that is constructed in many tests and read models |
| **Insight** | `#[serde(default)]` handles runtime parsing, but compile-time struct literals and test builders still require explicit wiring or defaults to avoid breakage and hidden drift in fixture generation. |
| **Suggested Action** | When adding frontmatter fields, immediately update `TestStory`, `StoryFactory`, and all explicit `StoryFrontmatter { ... }` literals in one slice before running broader checks. |
| **Applies To** | `src/domain/model/story.rs`, `src/test_helpers.rs`, read-model fixture tests |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-05T02:56:11+00:00 |
| **Score** | 0.81 |
| **Confidence** | 0.92 |
| **Applied** | yes |
