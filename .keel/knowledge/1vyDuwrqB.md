---
source_type: Story
source: stories/1vwqCeX9I/REFLECT.md
scope: 1vwq96cpt/1vwq9RqCe
source_story_id: 1vwqCeX9I
---

### 1vyDuwrqB: Shared template rendering reduces cross-command coupling

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Multiple creation paths (story, epic, voyage, bearing, ADR, transitions) performing placeholder substitution |
| **Insight** | Keeping placeholder substitution in command-local helpers increases coupling and makes cross-command refactors noisier than necessary. |
| **Suggested Action** | Route all template substitution through `infrastructure::template_rendering::render` and enforce usage with architecture contract tests. |
| **Applies To** | `src/infrastructure/template_rendering.rs`, `src/commands/*/new.rs`, `src/commands/story/reflect.rs`, `src/transitions/bearing_engine.rs` |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-02T17:22:22+00:00 |
| **Score** | 0.88 |
| **Confidence** | 0.96 |
| **Applied** | story `1vwqCeX9I` |
