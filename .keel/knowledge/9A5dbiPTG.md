---
source_type: Voyage
source: epics/1vxqMtskC/voyages/1vxqN5jnA/KNOWLEDGE.md
scope: null
source_story_id: null
---

### 9A5dbiPTG: Advisory parser keeps keel.toml resilient

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Technique overrides need richer schema while core config loading should not fail when optional override blocks are malformed. |
| **Insight** | Parsing overrides from raw TOML with per-field diagnostics allows invalid entries to be ignored safely without blocking normal command behavior. |
| **Suggested Action** | Keep optional/advanced config surfaces advisory by default, then merge validated entries into canonical models with explicit diagnostics. |
| **Applies To** | `src/read_model/verification_techniques.rs` |
| **Linked Knowledge IDs** | 1vyDuwSon |
| **Observed At** |  |
| **Score** | 0.82 |
| **Confidence** | 0.88 |
| **Applied** | yes |
