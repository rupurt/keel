---
source_type: Voyage
source: epics/1vwq96cpt/voyages/1vwq9RqCe/KNOWLEDGE.md
created_at: 2026-03-02T10:07:49
---

### rbij9ueSM: Shared template rendering reduces cross-command coupling

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Multiple creation paths (story, epic, voyage, bearing, ADR, transitions) performing placeholder substitution |
| **Insight** | Keeping placeholder substitution in command-local helpers increases coupling and makes cross-command refactors noisier than necessary. |
| **Suggested Action** | Route all template substitution through `infrastructure::template_rendering::render` and enforce usage with architecture contract tests. |
| **Applies To** | `src/infrastructure/template_rendering.rs`, `src/commands/*/new.rs`, `src/commands/story/reflect.rs`, `src/transitions/bearing_engine.rs` |
| **Linked Knowledge IDs** | 1vyDuwrqB |
| **Observed At** |  |
| **Score** | 0.88 |
| **Confidence** | 0.96 |
| **Applied** | story `1vwqCeX9I` |
