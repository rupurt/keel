# Flow Coherence Refactor - Product Requirements

> Unify queue policy, transition enforcement, and canonical schema with hard migration.

## Problem Statement

Keel currently has coherence gaps across planning and execution workflows:

- Queue thresholds and classification logic are duplicated between `next` and `flow`, which allows drift.
- Transition enforcement is split between command handlers and doctor checks, creating inconsistent runtime vs reporting behavior.
- Legacy schema compatibility paths keep old terms and field variants alive, increasing maintenance cost and confusion.
- Newly scaffolded epic/voyage artifacts can fail doctor immediately because of timestamp and placeholder defaults.

## Goals & Objectives

| Goal | Success Metric | Target |
|------|----------------|--------|
| One queue-policy source of truth | Zero duplicated queue-threshold literals in decision paths | 100% migrated |
| Coherent human/agent next behavior | Human mode never emits implementation work | 100% test pass |
| Unified transition enforcement | Runtime and doctor checks share gate-rule origin | Parity tests passing |
| Hard schema cutover | Legacy state/field tokens rejected post-migration | 100% canonical parsing |
| Doctor-clean scaffolding defaults | Fresh epic/voyage creation produces no datetime/TODO warnings | 100% on new artifacts |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Human planner | Maintains roadmap, accepts/rejects work, drives research/planning queues | Predictable queue guidance and clear governance |
| Agent implementer | Pulls implementation stories and executes scoped work | Stable transition rules and unambiguous state model |
| Maintainer | Owns CLI behavior, diagnostics, and templates | Low-drift architecture and clear invariants |

## Scope

### In Scope

- Queue policy module and shared thresholds for `next` and `flow`.
- Unified transition enforcement for story/voyage runtime commands and doctor reporting.
- Hard migration path to canonical schema and removal of compatibility aliases.
- Canonical terminology and field-name normalization (`completed_at`, state labels).
- Done-state gating for voyage reporting artifacts.
- Scaffold defaults that are doctor-clean on creation.

### Out of Scope

- New product surfaces unrelated to planning/execution coherence.
- Backward compatibility for legacy board states after hard migration ships.
- Changes to bearing/ADR workflows not required by these coherence goals.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-01 | Implement a shared queue policy module consumed by `next` and `flow`. | must | Prevent threshold drift and contradictory queue decisions. |
| FR-02 | Enforce human mode boundary so `keel next` (human mode) cannot return implementation work. | must | Preserve queue intent and operator expectations. |
| FR-03 | Preserve auto-start behavior where `keel story start` starts planned parent voyages. | must | Maintain expected planning-to-execution flow. |
| FR-04 | Route story/voyage transition checks through one enforcement path shared by runtime and doctor policies. | must | Remove duplicated logic and normalize outcomes. |
| FR-05 | Ship hard migration and remove legacy schema compatibility paths post-cutover. | must | Reduce maintenance load and conceptual drift. |
| FR-06 | Normalize epic completion field usage to `completed_at` across command/model/doctor paths. | must | Keep state and data contracts consistent. |
| FR-07 | Generate voyage report/compliance artifacts only when voyage state is `done`. | should | Align artifact lifecycle with transition semantics. |
| FR-08 | Ensure newly scaffolded epic/voyage docs use datetime timestamps and non-placeholder baseline content. | should | Prevent immediate doctor warnings on new artifacts. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| NFR-01 | Migration behavior must be deterministic and idempotent. | must | Safe re-runs and predictable recovery. |
| NFR-02 | Validation and transition code paths should expose one clear entry point per entity/transition. | must | Improve maintainability and reviewability. |
| NFR-03 | Regression coverage must lock parity between strict runtime blocking and reporting visibility. | must | Prevent silent policy divergence. |
| NFR-04 | Documentation and CLI text must use canonical terminology only. | should | Reduce onboarding and operational confusion. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

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

## Voyages

<!-- BEGIN VOYAGES -->
| Voyage | Status | Description |
|--------|--------|-------------|
| [Policy Module and Queue Semantics](voyages/1vuz8VYmc/) | draft | Centralize queue policy and align next/flow behavior. |
| [Unified Transition Enforcement](voyages/1vuz8dYT5/) | draft | Unify runtime and reporting transition enforcement paths. |
| [Hard Schema Migration and Compatibility Cleanup](voyages/1vuz8jNo3/) | draft | Cut over to canonical schema and remove compatibility code. |
<!-- END VOYAGES -->
