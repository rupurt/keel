# Unified Transition Enforcement - Software Requirements Specification

> Unify runtime and doctor transition validation behind one gate-driven enforcement path.

**Epic:** [1vuz8K4NM](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

- Deliver one transition-enforcement service used by story/voyage runtime commands and doctor/reporting checks.
- Remove duplicated per-command validation branches where gate evaluators already define behavior.
- Standardize transition error formatting and policy application.
- Out of scope: schema migration and compatibility removal (voyage `1vuz8jNo3`).

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| Existing gate evaluators in `src/state_machine/gating.rs` remain authoritative rule definitions. | Internal | Service must duplicate gate logic and risk drift. |
| Story and voyage transition specs remain the canonical transition map. | Internal | Service contract must expand to infer transitions dynamically. |
| Doctor checks can consume gate outputs with reporting policy. | Internal | Separate doctor-only logic would remain and reduce unification value. |

## Constraints

- Runtime command behavior must remain safe and backward-compatible for valid transitions.
- Doctor must continue surfacing warnings without blocking runtime semantics.
- Error output must stay actionable and entity-scoped.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | Provide a unified transition-enforcement API that combines transition legality checks, gate evaluation, and block-policy handling. | Architecture Review | Unit tests for service behaviors across story/voyage transitions. |
| SRS-02 | Story and voyage runtime commands must route through the unified enforcer rather than hand-rolled validation branches. | Refactor Program | Command integration tests for start/submit/accept/plan/start/done paths. |
| SRS-03 | Doctor transition and completion checks must consume the same gate evaluators with reporting policy instead of duplicated logic. | Refactor Program | Doctor tests validating reported warnings/errors. |
| SRS-04 | Transition error formatting must be centralized so runtime and reporting messages share consistent entity, transition, and problem formatting. | Architecture Review | Snapshot tests for error output. |
| SRS-05 | Regression tests must prove parity between runtime blocking and reporting visibility for representative gate failures. | Quality | End-to-end tests covering strict vs reporting policy. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-NFR-01 | Enforcer APIs must be deterministic and side-effect free before explicit transition execution. | Quality | Pure unit tests and mocked boards. |
| SRS-NFR-02 | Unified enforcement must not increase command latency materially for normal board sizes. | Performance | Benchmark comparison before/after refactor. |
| SRS-NFR-03 | Validation code paths should have one obvious entry point per transition type to reduce maintenance overhead. | Maintainability | Code review checklist and module docs. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
