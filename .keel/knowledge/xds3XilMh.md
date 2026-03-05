---
source_type: Voyage
source: epics/1vwq96cpt/voyages/1vwq9RqCe/KNOWLEDGE.md
scope: null
source_story_id: null
---

### xds3XilMh: Ports Should Mirror Aggregate Boundaries

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Defining persistence abstractions before filesystem adapters are extracted from command modules |
| **Insight** | Repository ports are easier to evolve when contracts are grouped by aggregate boundary (story, voyage, epic, bearing, adr) plus one board snapshot port for orchestration use cases. |
| **Suggested Action** | Keep port interfaces in the application layer and defer adapter wiring to subsequent stories to minimize behavior risk during migration. |
| **Applies To** | src/application/ports.rs, upcoming infrastructure adapter stories |
| **Linked Knowledge IDs** | 1vyDuwLCH |
| **Observed At** |  |
| **Score** | 0.82 |
| **Confidence** | 0.88 |
| **Applied** | yes |
