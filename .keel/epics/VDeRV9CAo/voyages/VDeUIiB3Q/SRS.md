# Explicit Lifecycle Reactors - SRS

## Summary

This voyage establishes the first explicit reactor seam inside Keel. It replaces
the bespoke process-manager branching logic with named reactor units for the
existing lifecycle automations, makes the voyage-completion event path explicit,
and documents that reactors live in the application layer. Shared
simulation-context and temporal read-model refactors are deferred to later
voyages.

## Scope

### In Scope

- [SCOPE-02] Explicit reactor pipeline for process-manager event-to-action planning

### Out of Scope

- [SCOPE-01] Shared simulation context carrying board state and injected reference time
- [SCOPE-03] Routine due-state and scheduled-routine projection integration with the shared simulation context
- [SCOPE-04] Replace DDD and hexagonal architecture with a new top-level doctrine
- [SCOPE-05] Introduce a continuous event loop, daemon, or ECS-style entity model
- [SCOPE-06] Rename stable user-facing CLI vocabulary around game terminology

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Introduce explicit reactor contracts and planner wiring so process-manager lifecycle reactions are expressed as named units instead of one bespoke match tree. | SCOPE-02 | FR-02 | unit test |
| SRS-02 | Preserve the existing story-started and story-accepted lifecycle automations while moving them onto explicit reactors. | SCOPE-02 | FR-02 | unit test |
| SRS-03 | Emit and consume voyage completion through an explicit event path that preserves current epic-finalization behavior. | SCOPE-02 | FR-02 | integration test |
| SRS-04 | Document reactor ownership and preservation rules so the architecture explicitly keeps reactors in the application layer without changing CLI behavior. | SCOPE-02 | FR-04 | llm-judge |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Reactors remain application-layer orchestration units and do not move transition authority into domain, read-model, or CLI code. | SCOPE-02 | NFR-01 | architecture test |
| SRS-NFR-02 | Reactor planning order remains deterministic for identical board and event inputs. | SCOPE-02 | NFR-03 | unit test |
| SRS-NFR-03 | The refactor leaves one canonical process-manager reaction path with legacy planner branching removed. | SCOPE-02 | NFR-03 | unit test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
