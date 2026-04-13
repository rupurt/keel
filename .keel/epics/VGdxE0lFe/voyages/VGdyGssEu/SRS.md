# Specify Managed Foreign Worktree Lifecycle - SRS

## Summary

Epic: VGdxE0lFe
Goal: Define how Keel creates, validates, reuses, and garbage-collects managed foreign worktrees on stack branches for outside reactor execution.

## Scope

### In Scope

- [SCOPE-01] Define the managed worktree requirement for foreign reactor execution.
- [SCOPE-02] Define branch and ownership validation rules for foreign worktrees.
- [SCOPE-03] Define create, reuse, and inspection behavior for open-stack foreign worktrees.
- [SCOPE-04] Define stack-close garbage collection rules.
- [SCOPE-05] Define the minimum command and hook enforcement points needed to reject unsupported foreign execution.

### Out of Scope

- [SCOPE-90] Workspace-wide sandboxing or OS-level file-permission controls.
- [SCOPE-91] General git worktree UX outside Mission Stack foreign execution.
- [SCOPE-92] Rich worktree dashboards or artifact browsers.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Foreign reactor execution in another member repo must require a managed git worktree instead of the member repo's primary checkout. | SCOPE-01 | FR-01 | manual |
| SRS-02 | Managed foreign worktrees must validate `stack/<id>` branch or equivalent approved stack-derived head state before execution begins. | SCOPE-02 | FR-02 | manual |
| SRS-03 | The lifecycle must define create, reuse, and inspection behavior for foreign worktrees while a stack remains open. | SCOPE-03 | FR-03 | manual |
| SRS-04 | The lifecycle must garbage-collect managed foreign worktrees when the stack closes, or report leftovers safely if removal is uncertain. | SCOPE-04 | FR-04 | manual |
| SRS-05 | The design must identify which command and hook boundaries enforce foreign worktree requirements. | SCOPE-05 | FR-05 | manual |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Managed worktree operations must avoid perturbing the member repo's primary checkout. | SCOPE-01 | NFR-01 | manual |
| SRS-NFR-02 | Worktree create and reuse behavior should be idempotent for an open stack member when possible. | SCOPE-03 | NFR-02 | manual |
| SRS-NFR-03 | Cleanup behavior should fail safe by reporting ambiguous leftovers rather than silently deleting uncertain state. | SCOPE-04 | NFR-03 | manual |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
