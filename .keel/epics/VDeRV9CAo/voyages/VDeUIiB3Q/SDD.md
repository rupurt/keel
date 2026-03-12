# Explicit Lifecycle Reactors - Software Design Description

> Replace the branching process manager with explicit lifecycle reactors while preserving current CLI behavior.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage extracts the process-manager seam into explicit lifecycle reactors.
The goal is not to redesign runtime semantics; it is to make the existing
cross-aggregate automations legible, testable, and easier to extend.

## Context & Boundaries

The work stays inside the existing layer model. The domain continues to own
entities, transitions, and gating. The application layer owns orchestration and
reactor execution. CLI command surfaces remain unchanged, and read models are
not touched in this slice.

```
domain event
    |
    v
reactor dispatcher
    |
    +--> story-started reactor ----> start voyage action
    |
    +--> story-accepted reactor ---> complete voyage action
    |
    +--> voyage-completed reactor -> finalize epic action
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `DomainEvent` stream | Internal | Reactor inputs for cross-aggregate lifecycle automation | Existing application event types |
| Process manager actions | Internal | Persisted lifecycle follow-up actions | Existing `ProcessAction` set |
| Story and voyage lifecycle services | Internal | Emit and trigger the lifecycle transitions reactors respond to | Existing application services |
| Architecture contract tests | Internal | Guard application-layer ownership and dependency boundaries | Existing test suite |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Reactor ownership | Keep reactors in application orchestration | Cross-aggregate reactions are not domain entities |
| Existing semantics | Preserve current start/complete/finalize behavior exactly while changing internal structure | The slice is architectural, not behavioral |
| Deferred work | Leave simulation-context and read-model unification for later voyages | Avoids mixing multiple refactor lines in the first slice |

## Architecture

The design introduces a reactor dispatcher that delegates each supported
lifecycle event to one or more reactor units. Each reactor inspects board state
and event data, then returns planned process actions. The process manager
remains the coordinator that executes those actions through the existing
executor interface.

## Components

| Component | Responsibility |
|-----------|----------------|
| Reactor dispatcher | Route each supported domain event to the relevant reactors |
| Story-started reactor | Preserve voyage auto-start behavior |
| Story-accepted reactor | Preserve voyage auto-complete behavior |
| Voyage-completed reactor | Preserve epic-finalization behavior and remove placeholder ambiguity |
| Process-manager coordinator | Execute planned actions through the existing executor interface |

## Interfaces

| Interface | Input | Output |
|-----------|-------|--------|
| Reactor planning API | Board snapshot + domain event | Zero or more `ProcessAction`s |
| Process-manager dispatcher | Domain event | Ordered set of planned actions |
| Lifecycle emission path | Story/voyage lifecycle transition | Domain event consumed by reactors |

## Data Flow

1. A lifecycle service or command completes an existing transition and emits a `DomainEvent`.
2. The process manager loads board state and dispatches the event through explicit reactors.
3. Matching reactors return ordered `ProcessAction`s for the existing automation behaviors.
4. The process manager executes those actions through the existing executor.
5. Architecture docs and tests record that this orchestration remains in the application layer.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Reactor has no applicable scope or entity | Reactor returns no actions | No-op with existing behavior preserved | Covered by unit tests on event planning |
| Event emission path is incomplete | Integration test shows voyage completion never reaches reactor dispatcher | Wire the lifecycle service or command through the shared event path | Keep the event path explicit and tested end-to-end |
| Refactor accidentally duplicates planner logic | Process-manager tests still pass through multiple paths or branch copies remain | Remove the legacy branch tree and keep one dispatcher path | Use regression tests to guard the canonical path |
