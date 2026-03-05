---
source_type: Voyage
source: epics/1vxYzSury/voyages/1vxpomgnN/KNOWLEDGE.md
scope: null
source_story_id: null
---

### 1ejHEDO4x: Planning Show Parsing Needs Scaffold Filters

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Extracting PRD summaries from partially authored templates |
| **Insight** | Requirement parsing must explicitly ignore scaffold rows like `TODO`/template defaults or placeholder mode appears complete when it is not. |
| **Suggested Action** | Keep placeholder filters and add fixture tests that assert empty summaries on scaffold-only PRDs. |
| **Applies To** | `src/cli/commands/management/epic/show.rs`, planning projection parsers |
| **Linked Knowledge IDs** | 1vyDuwuBj |
| **Observed At** |  |
| **Score** | 0.83 |
| **Confidence** | 0.92 |
| **Applied** | yes |
