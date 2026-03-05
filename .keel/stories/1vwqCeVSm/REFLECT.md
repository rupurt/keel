# Reflection - Introduce Domain Events And Process Managers

## Knowledge

### 1vyDuwiVQ: Event-First Cross-Aggregate Orchestration Preserves Boundaries

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Story start/accept and voyage completion previously embedded cross-aggregate calls inside lifecycle services. |
| **Insight** | Emitting explicit domain events and routing follow-on actions through a process manager keeps use cases focused while preserving existing behavior. |
| **Suggested Action** | Keep cross-aggregate progression in process managers and add event/action tests whenever new lifecycle automation is introduced. |
| **Applies To** | src/application/story_lifecycle.rs, src/application/voyage_epic_lifecycle.rs, src/application/process_manager.rs |
| **Observed At** | 2026-03-02T04:31:37Z |
| **Score** | 0.84 |
| **Confidence** | 0.92 |
| **Applied** | Introduced DomainEvent + DomainProcessManager and rewired lifecycle services to emit events. |

## Observations

Process-manager planning tests made the orchestration refactor deterministic and easy to verify without brittle filesystem transition setup.
The main risk was ordering: voyage auto-start must happen before the story transition to satisfy existing voyage start gates.
Running targeted start/accept/voyage tests before the full suite quickly caught and resolved that regression.
