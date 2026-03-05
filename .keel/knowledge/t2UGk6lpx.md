---
source_type: Voyage
source: epics/1vwq96cpt/voyages/1vwq9Zf67/KNOWLEDGE.md
scope: null
source_story_id: null
---

### t2UGk6lpx: Event-First Cross-Aggregate Orchestration Preserves Boundaries

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Story start/accept and voyage completion previously embedded cross-aggregate calls inside lifecycle services. |
| **Insight** | Emitting explicit domain events and routing follow-on actions through a process manager keeps use cases focused while preserving existing behavior. |
| **Suggested Action** | Keep cross-aggregate progression in process managers and add event/action tests whenever new lifecycle automation is introduced. |
| **Applies To** | src/application/story_lifecycle.rs, src/application/voyage_epic_lifecycle.rs, src/application/process_manager.rs |
| **Linked Knowledge IDs** | 1vyDuwiVQ |
| **Observed At** |  |
| **Score** | 0.84 |
| **Confidence** | 0.92 |
| **Applied** | Introduced DomainEvent + DomainProcessManager and rewired lifecycle services to emit events. |
