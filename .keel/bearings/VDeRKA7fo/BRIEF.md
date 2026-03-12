# Simulation Kernel Architecture Research — Brief

## Hypothesis

Keel should formalize a small simulation kernel beneath its existing DDD and hexagonal architecture. If we introduce a deterministic board pulse, explicit reactors for cross-aggregate responses, and shared projections for read paths, we can make the system easier to evolve without turning it into a different product or runtime model.

## Problem Space

Keel already behaves like a deterministic simulation in several places, but the architecture does not name that pattern directly. Cross-aggregate reactions live in one process manager, temporal reasoning lives in isolated read models, and scheduling decisions re-derive board state in multiple places. That makes the architecture harder to explain and raises the risk of either duplicating logic or overcorrecting into a game-engine rewrite.

## Context

The current architecture already has strong boundaries: the domain owns entities and gates, the application layer owns orchestration, infrastructure owns IO, and read models own projections. The research question is whether a simulation vocabulary can clarify the internals of those layers without changing the outer contract.

## Objectives

- Define the minimal simulation concepts Keel actually needs.
- Identify which current modules should absorb those concepts first.
- Produce a recommendation that preserves existing architecture boundaries and CLI behavior.

## Scope

### In Scope

- The current process manager and domain-event flow.
- Temporal evaluation in routine due-state and scheduled routine projections.
- Shared projection opportunities between `keel flow`, `keel next`, and mission steering.

### Out of Scope

- Replacing DDD or hexagonal architecture with a new top-level doctrine.
- Introducing a continuous background game loop, daemon, or ECS rewrite.
- Renaming stable user-facing CLI concepts around game terminology.

## Research Questions

- What is the smallest useful “pulse” or reference-time abstraction for Keel?
- Which cross-aggregate behaviors should become first-class reactors?
- Where would a shared projection pipeline reduce duplicated board scanning without obscuring the domain model?

## Success Criteria

- [ ] The research names a minimal simulation vocabulary that fits the existing architecture.
- [ ] The recommendation identifies concrete target modules for the first refactor slices.
- [ ] The result explains what should not be formalized so the implementation stays incremental.

## Open Questions

- Should `keel next` consume a shared board pulse directly or only downstream projections?
- How far should event publication go before it becomes unnecessary ceremony?
