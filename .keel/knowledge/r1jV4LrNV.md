---
source_type: Voyage
source: epics/1vwq96cpt/voyages/1vwq9Zf67/KNOWLEDGE.md
scope: null
source_story_id: null
---

### r1jV4LrNV: Keep Lifecycle Command Handlers As Thin Adapters

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Refactoring voyage and epic lifecycle commands to align with application-service orchestration boundaries |
| **Insight** | Moving orchestration into a dedicated application service lets command modules stay stable adapters while preserving behavior through existing command tests |
| **Suggested Action** | Add use-case methods first, then delegate command `run` entrypoints to those methods and update cross-command callsites to service APIs |
| **Applies To** | src/application/*.rs; src/commands/voyage/*.rs; src/commands/epic/*.rs |
| **Linked Knowledge IDs** | 1vyDuwgcV |
| **Observed At** |  |
| **Score** | 0.82 |
| **Confidence** | 0.88 |
| **Applied** | 1vwqCejs5 |
