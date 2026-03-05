---
source_type: Story
source: stories/1vxvIZRXy/REFLECT.md
scope: 1vxqMtskC/1vxvFrNta
source_story_id: 1vxvIZRXy
---

### 1vyDuwBfG: Prefer direct status flags over aggregated recommendation blocks

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Config introspection commands where automation depends on deterministic machine-readable state |
| **Insight** | A per-technique flag matrix (`detected`, `disabled`, `active`) is a better contract boundary than mixed narrative sections because it cleanly separates inventory from recommendation logic. |
| **Suggested Action** | Keep config/read commands focused on canonical state and move advisory ranking/commentary to dedicated recommend commands. |
| **Applies To** | `src/cli/commands/setup/config.rs`, `src/read_model/verification_techniques.rs` |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-04T23:34:00+00:00 |
| **Score** | 0.86 |
| **Confidence** | 0.90 |
| **Applied** | yes |
