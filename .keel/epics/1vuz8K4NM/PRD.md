# Flow Coherence Refactor - Product Requirements


## Problem Statement

Keel currently has coherence gaps across planning and execution workflows:

- Queue thresholds and classification logic are duplicated between `next` and `flow`, which allows drift.
- Transition enforcement is split between command handlers and doctor checks, creating inconsistent runtime vs reporting behavior.
- Legacy schema compatibility paths keep old terms and field variants alive, increasing maintenance cost and confusion.
- Newly scaffolded epic/voyage artifacts can fail doctor immediately because of timestamp and placeholder defaults.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | One queue-policy source of truth | Zero duplicated queue-threshold literals in decision paths | 100% migrated |
| GOAL-02 | Coherent human/agent next behavior | Human mode never emits implementation work | 100% test pass |
| GOAL-03 | Unified transition enforcement | Runtime and doctor checks share gate-rule origin | Parity tests passing |
| GOAL-04 | Hard schema cutover | Legacy state/field tokens rejected post-migration | 100% canonical parsing |
| GOAL-05 | Doctor-clean scaffolding defaults | Fresh epic/voyage creation produces no datetime/TODO warnings | 100% on new artifacts |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Human planner | Maintains roadmap, accepts/rejects work, drives research/planning queues | Predictable queue guidance and clear governance |
| Agent implementer | Pulls implementation stories and executes scoped work | Stable transition rules and unambiguous state model |
| Maintainer | Owns CLI behavior, diagnostics, and templates | Low-drift architecture and clear invariants |

## Scope

### In Scope

- [SCOPE-01] Queue policy module and shared thresholds for `next` and `flow`.
- [SCOPE-02] Unified transition enforcement for story/voyage runtime commands and doctor reporting.
- [SCOPE-03] Hard migration path to canonical schema and removal of compatibility aliases.
- [SCOPE-04] Canonical terminology and field-name normalization (`completed_at`, state labels).
- [SCOPE-05] Done-state gating for voyage reporting artifacts.
- [SCOPE-06] Scaffold defaults that are doctor-clean on creation.

### Out of Scope

- [SCOPE-07] New product surfaces unrelated to planning/execution coherence.
- [SCOPE-08] Backward compatibility for legacy board states after hard migration ships.
- [SCOPE-09] Changes to bearing/ADR workflows not required by these coherence goals.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Implement a shared queue policy module consumed by `next` and `flow`. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 GOAL-05 | must | Prevent threshold drift and contradictory queue decisions. |
| FR-02 | Enforce human mode boundary so `keel next` (human mode) cannot return implementation work. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 GOAL-05 | must | Preserve queue intent and operator expectations. |
| FR-03 | Preserve auto-start behavior where `keel story start` starts planned parent voyages. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 GOAL-05 | must | Maintain expected planning-to-execution flow. |
| FR-04 | Route story/voyage transition checks through one enforcement path shared by runtime and doctor policies. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 GOAL-05 | must | Remove duplicated logic and normalize outcomes. |
| FR-05 | Ship hard migration and remove legacy schema compatibility paths post-cutover. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 GOAL-05 | must | Reduce maintenance load and conceptual drift. |
| FR-06 | Normalize epic completion field usage to `completed_at` across command/model/doctor paths. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 GOAL-05 | must | Keep state and data contracts consistent. |
| FR-07 | Generate voyage report/compliance artifacts only when voyage state is `done`. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 GOAL-05 | should | Align artifact lifecycle with transition semantics. |
| FR-08 | Ensure newly scaffolded epic/voyage docs use datetime timestamps and non-placeholder baseline content. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 GOAL-05 | should | Prevent immediate doctor warnings on new artifacts. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Migration behavior must be deterministic and idempotent. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 GOAL-05 | must | Safe re-runs and predictable recovery. |
| NFR-02 | Validation and transition code paths should expose one clear entry point per entity/transition. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 GOAL-05 | must | Improve maintainability and reviewability. |
| NFR-03 | Regression coverage must lock parity between strict runtime blocking and reporting visibility. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 GOAL-05 | must | Prevent silent policy divergence. |
| NFR-04 | Documentation and CLI text must use canonical terminology only. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 GOAL-05 | should | Reduce onboarding and operational confusion. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

- Add regression tests for queue policy parity so `next` and `flow` decisions remain aligned across shared thresholds.
- Add transition-gating parity tests that compare runtime command blocking and doctor output for equivalent fixtures.
- Run hard-migration fixture tests for acceptance and rejection paths, then gate completion on `just keel doctor` and `just test`.

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Teams can run migration before adopting the cutover binary. | Legacy parsing removal would break existing boards. | Provide migration dry-run and fixture tests. |
| Existing gate evaluators remain the authoritative rule source. | Additional refactor is needed before unification can land. | Enforcer integration tests on representative transitions. |
| Queue policy defaults are acceptable for current board scale. | Throughput bottlenecks may require policy tuning. | Boundary-condition regression tests and flow checks. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Rollout sequencing across voyages could cause temporary doc/code mismatch. | Epic owner | Open |
| Hard migration may surface unknown legacy tokens in downstream boards. | Maintainer | Mitigated via explicit migration error reporting |
| Artifact lifecycle gating needs clear behavior for voyage reopen/edit workflows. | Maintainer | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Queue and flow decisions are driven by one policy module with no duplicated thresholds.
- [ ] Human-mode `keel next` never emits implementation work in regression tests.
- [ ] Runtime and doctor transition checks produce coherent, policy-aligned outcomes.
- [ ] Hard migration plus strict canonical parsing pass integration and rejection tests.
- [ ] Freshly scaffolded epic/voyage artifacts pass doctor checks for timestamp and placeholder hygiene.
<!-- END SUCCESS_CRITERIA -->

