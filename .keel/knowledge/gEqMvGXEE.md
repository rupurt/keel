---
source_type: Voyage
source: epics/1vxqMtskC/voyages/1vxvFrNta/KNOWLEDGE.md
scope: null
source_story_id: null
---

### gEqMvGXEE: Prefer direct status flags over aggregated recommendation blocks

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Config introspection commands where automation depends on deterministic machine-readable state |
| **Insight** | A per-technique flag matrix (`detected`, `disabled`, `active`) is a better contract boundary than mixed narrative sections because it cleanly separates inventory from recommendation logic. |
| **Suggested Action** | Keep config/read commands focused on canonical state and move advisory ranking/commentary to dedicated recommend commands. |
| **Applies To** | `src/cli/commands/setup/config.rs`, `src/read_model/verification_techniques.rs` |
| **Linked Knowledge IDs** | 1vyDuwBfG |
| **Observed At** |  |
| **Score** | 0.86 |
| **Confidence** | 0.90 |
| **Applied** | yes |
