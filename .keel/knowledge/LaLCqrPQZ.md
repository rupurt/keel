---
source_type: Voyage
source: epics/1vwq96cpt/voyages/1vwq9Zf67/KNOWLEDGE.md
scope: null
source_story_id: null
---

### LaLCqrPQZ: Enforce policy invariants in application services, not command handlers

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Story and voyage lifecycle commands need consistent enforcement of manual-verification acceptance and requirements-coverage gating. |
| **Insight** | Putting enforcement in application services centralizes lifecycle invariants and avoids drift across multiple command entrypoints. |
| **Suggested Action** | Keep command handlers thin and add service-level tests for every lifecycle policy; use architecture contract tests to block direct transition orchestration from handlers. |
| **Applies To** | `src/application/story_lifecycle.rs`, `src/application/voyage_epic_lifecycle.rs`, `src/commands/{story,voyage,epic}/*.rs`, `src/architecture_contract_tests.rs` |
| **Linked Knowledge IDs** | 1vyDuwnad |
| **Observed At** |  |
| **Score** | 0.90 |
| **Confidence** | 0.96 |
| **Applied** | story `1vwqCfPpe` |
