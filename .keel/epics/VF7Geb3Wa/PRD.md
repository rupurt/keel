# Derive Heartbeat From Repository Activity - Product Requirements

## Problem Statement

Keel still treats a synthetic `.keel/heartbeat` file as the pacemaker signal for recent work even though Git state and worktree changes are the real activity source. That file adds ritual, hides the actual governor controls, and makes `flow`, hooks, and docs tell a less coherent story than the engine now needs.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Make repository activity the canonical heartbeat signal. | `keel heartbeat` and `keel flow --scene` share one derived activity source. | Pass 1 landed |
| GOAL-02 | Remove the synthetic heartbeat file from the operator loop. | Hooks, `poke`, and core read models no longer depend on committing `.keel/heartbeat`. | Pass 2 landed |
| GOAL-03 | Keep the pacemaker model legible for downstream adopters. | Foundational and public docs explain the Git/worktree-derived model and upgrade path. | Docs cutover landed |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Operator | Uses `keel flow`, `keel doctor`, and commit hooks to understand whether the board is energized. | A pacemaker signal that reflects real work rather than a synthetic file ritual. |
| Keel Maintainer | Changes core read models, hooks, and diagnostics. | One coherent heartbeat model across CLI, read models, and board policy. |
| Downstream Maintainer | Adapts `AGENTS.md`, `INSTRUCTIONS.md`, and upgrade guidance in another repo. | A documented pacemaker contract that does not require committing `.keel/heartbeat`. |

## Scope

### In Scope

- [SCOPE-01] Introduce a derived heartbeat read model based on Git/worktree activity and expose it through `keel heartbeat`.
- [SCOPE-02] Cut `keel flow --scene` over to the derived heartbeat signal, using the file-backed path only as a bounded compatibility fallback during pass 1.
- [SCOPE-03] Remove file-backed heartbeat loading, hook staging, cache invalidation, and related operator messaging in pass 2.
- [SCOPE-04] Update foundational docs, MDX docs, and downstream upgrade guidance to teach the new pacemaker model.

### Out of Scope

- [SCOPE-05] New live telemetry, daemons, or background monitoring beyond the existing CLI-driven workflow.
- [SCOPE-06] Redesigning the broader mission or delivery heuristics beyond heartbeat-derived energization.
- [SCOPE-07] Vendor-specific AI harness behavior or downstream automation outside the documented Keel contract.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Keel must derive heartbeat activity from repository state, prioritizing dirty tracked worktree files and otherwise using the latest reachable commit activity. | GOAL-01 | must | The heartbeat should describe real work, not a synthetic file write. |
| FR-02 | Keel must expose the derived heartbeat through a dedicated `keel heartbeat` command that surfaces timestamp, source, and current state clearly enough for operators to reason about `flow`. | GOAL-01 GOAL-03 | must | Operators need a first-class way to inspect the signal driving energized versus unplugged behavior. |
| FR-03 | `keel flow --scene` must use the derived heartbeat signal as its primary energization input and only rely on the legacy file path during the bounded compatibility phase. | GOAL-01 | must | The user-facing scene must follow the same model the new command reports. |
| FR-04 | Core board models, hooks, `poke`, diagnostics, and cache/graph surfaces must stop depending on `.keel/heartbeat` by the end of pass 2. | GOAL-02 | must | The migration is incomplete if the file remains the hidden governor. |
| FR-05 | Foundational docs and public docs must describe commit and hook lifecycle as the governing pacemaker controls once the file-backed path is removed. | GOAL-03 | should | Downstream repos need the operational story to stay coherent after the cutover. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | The heartbeat model must remain deterministic across supported platforms and avoid exposing inode-level behavior as a user-facing semantic contract. | GOAL-01 GOAL-03 | must | Platform-stable operator semantics matter more than filesystem trivia. |
| NFR-02 | Pass 1 compatibility logic must be explicit and removable without changing the derived heartbeat API shape. | GOAL-01 GOAL-02 | must | The fallback should not become a permanent second control plane. |
| NFR-03 | Formatting, linting, and regression suites covering heartbeat, flow, hooks, and docs must pass before the epic closes. | GOAL-01 GOAL-02 GOAL-03 | must | The cutover touches both code and operating contract surfaces. |
| NFR-04 | Downstream instructions must be able to adopt the new model without requiring local repo-specific patches beyond documented sync steps. | GOAL-03 | should | The OSS and paid-user path depends on a portable contract. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Derived heartbeat projection | Unit and command regression tests | Pass 1 story evidence |
| Flow cutover | Flow scene tests plus CLI proof | Pass 1 story evidence |
| File-path removal | Doctor, cache, hook, and board-model regression tests | Pass 2 story evidence |
| Documentation cutover | Artifact review and docs build | Pass 2 story evidence |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Dirty tracked files plus HEAD commit time are sufficient to represent "recent work" for the operator loop. | Additional repository signals would be needed and the cutover could stall. | Validate in pass 1 while comparing the derived signal against current `flow` expectations. |
| Downstream repos can absorb the cutover through docs and hook updates without retaining `.keel/heartbeat` for compatibility. | The migration would need a longer-lived compatibility shim. | Validate in pass 2 by updating downstream guidance in the docs. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Should untracked files contribute to the heartbeat, or should the model stay limited to tracked worktree state plus commits? | Epic owner | Open |
| Are there any graph or cache surfaces that need a synthetic pacemaker node even after file-backed heartbeat removal? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] `keel heartbeat` becomes the canonical operator-facing heartbeat surface.
- [ ] `keel flow --scene` follows the derived heartbeat model rather than `.keel/heartbeat` mtime.
- [ ] Hooks, `poke`, diagnostics, and docs no longer require or teach a committed heartbeat file.
<!-- END SUCCESS_CRITERIA -->
