---
source_type: Story
source: stories/1vwqCfPpe/REFLECT.md
scope: 1vwq96cpt/1vwq9Zf67
source_story_id: 1vwqCfPpe
created_at: 2026-03-02T09:33:26
---

### 1vyDuwnad: Enforce policy invariants in application services, not command handlers

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Story and voyage lifecycle commands need consistent enforcement of manual-verification acceptance and requirements-coverage gating. |
| **Insight** | Putting enforcement in application services centralizes lifecycle invariants and avoids drift across multiple command entrypoints. |
| **Suggested Action** | Keep command handlers thin and add service-level tests for every lifecycle policy; use architecture contract tests to block direct transition orchestration from handlers. |
| **Applies To** | `src/application/story_lifecycle.rs`, `src/application/voyage_epic_lifecycle.rs`, `src/commands/{story,voyage,epic}/*.rs`, `src/architecture_contract_tests.rs` |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-02T17:32:48+00:00 |
| **Score** | 0.90 |
| **Confidence** | 0.96 |
| **Applied** | story `1vwqCfPpe` |
