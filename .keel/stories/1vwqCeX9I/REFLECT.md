---
created_at: 2026-03-02T09:23:09
---

# Reflection - Extract Template Rendering Service

## Knowledge

<!--
Institutional memory is mandatory. Capture what you learned during implementation.
Format:
### L001: Title
| Field | Value |
|-------|-------|
| **Category** | code/testing/process/architecture |
| **Context** | describe when this applies |
| **Insight** | the fundamental discovery |
| **Suggested Action** | what to do next time |
| **Applies To** | file patterns or components |
| **Observed At** | RFC3339 timestamp (e.g. 2026-02-22T12:00:00Z) |
| **Score** | 0.0-1.0 (impact significance) |
| **Confidence** | 0.0-1.0 (insight quality) |
| **Applied** | |
-->

### 1vyDuwrqB: Shared template rendering reduces cross-command coupling

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Multiple creation paths (story, epic, voyage, bearing, ADR, transitions) performing placeholder substitution |
| **Insight** | Keeping placeholder substitution in command-local helpers increases coupling and makes cross-command refactors noisier than necessary. |
| **Suggested Action** | Route all template substitution through `infrastructure::template_rendering::render` and enforce usage with architecture contract tests. |
| **Applies To** | `src/infrastructure/template_rendering.rs`, `src/commands/*/new.rs`, `src/commands/story/reflect.rs`, `src/transitions/bearing_engine.rs` |
| **Observed At** | 2026-03-02T17:22:22Z |
| **Score** | 0.88 |
| **Confidence** | 0.96 |
| **Applied** | story `1vwqCeX9I` |

## Observations

The migration was low risk because template rendering had a tiny, deterministic surface and could be moved without changing frontmatter mutation behavior.
Adding a contract test for creation paths prevented regression back to `story::new::render_template` and gave concrete verification for the AC.
