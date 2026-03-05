---
source_type: Story
source: stories/1vwqCejs5/REFLECT.md
scope: 1vwq96cpt/1vwq9Zf67
source_story_id: 1vwqCejs5
---

### 1vyDuwgcV: Keep Lifecycle Command Handlers As Thin Adapters

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Refactoring voyage and epic lifecycle commands to align with application-service orchestration boundaries |
| **Insight** | Moving orchestration into a dedicated application service lets command modules stay stable adapters while preserving behavior through existing command tests |
| **Suggested Action** | Add use-case methods first, then delegate command `run` entrypoints to those methods and update cross-command callsites to service APIs |
| **Applies To** | src/application/*.rs; src/commands/voyage/*.rs; src/commands/epic/*.rs |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-02T02:02:28+00:00 |
| **Score** | 0.82 |
| **Confidence** | 0.88 |
| **Applied** | 1vwqCejs5 |
