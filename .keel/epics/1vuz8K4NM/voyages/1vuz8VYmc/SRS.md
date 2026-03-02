# Policy Module and Queue Semantics - Software Requirements Specification

> Create one queue-policy source for thresholds, flow derivation, and human/agent next prioritization.

**Epic:** [1vuz8K4NM](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

- Deliver a single policy module that defines queue thresholds, priority rules, and actor boundaries used by `keel next` and `keel flow`.
- Refactor `next` and `flow` paths to consume policy helpers rather than hardcoded values.
- Align architecture and command documentation with the canonical policy values.
- Out of scope: removing legacy schema compatibility (covered by voyage `1vuz8jNo3`).
- Out of scope: disabling auto-start of planned voyages from `keel story start` (explicitly retained).

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| Existing `next` and `flow` modules remain the primary queue surfaces. | Internal | Refactor scope expands to additional command paths. |
| Actor-role filtering remains optional and must continue to work with policy decisions. | Internal | Policy API requires extra role-aware branches. |
| Voyage auto-start behavior from `story start` remains required. | Product Decision | Queue semantics may conflict with expected workflow behavior. |

## Constraints

- Human-mode `keel next` must never return implementation work (`Work`) decisions.
- Queue thresholds and queue-priority rules must be declared in one module and imported by consumers.
- Decision ordering must remain deterministic for identical board state.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | Provide a `queue_policy` module with canonical threshold constants, actor queue categories, and flow-state derivation helpers. | Architecture Review | Unit tests for exported constants/helpers. |
| SRS-02 | `src/next/algorithm.rs` must consume policy APIs for threshold checks and priority ordering with no inline threshold literals. | User Direction | Unit and integration tests for `keel next`. |
| SRS-03 | Human-mode `keel next` must emit only human-queue actions (`decision`, `accept`, `research`, `needs-stories`, `needs-planning`, `blocked`, `empty`). | User Decision 2 | Integration tests for human-mode decision set. |
| SRS-04 | `src/flow/*` state derivation and bottleneck summaries must consume the same policy values used by `next`. | Architecture Review | Flow and bottleneck test assertions over shared policy values. |
| SRS-05 | Architecture and command docs must describe the canonical policy thresholds and queue boundaries implemented in code. | User Direction | Documentation review and consistency checks. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-NFR-01 | Policy helpers must be deterministic and side-effect free. | Quality | Pure unit tests with repeatable outputs. |
| SRS-NFR-02 | Agent-mode behavior must remain functionally equivalent aside from threshold alignment. | Quality | Regression tests for agent `next` decisions. |
| SRS-NFR-03 | Policy values must be discoverable in one file without cross-module constant hunting. | Maintainability | Code review checklist and module-level docs. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
