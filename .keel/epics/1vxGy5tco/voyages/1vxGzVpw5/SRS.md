# Doctor And Transition Hard Enforcement - Software Requirements Specification

> Enforce unresolved scaffold/default text as hard failures in doctor and lifecycle transitions for planning coherence.

**Epic:** [1vxGy5tco](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

This voyage introduces hard validation behavior for unresolved scaffold/default content.
It covers doctor severity changes, stage-aware story/reflection coherency checks, and lifecycle transition gate enforcement for story submit/accept.
Validation of generated report artifacts is explicitly excluded for this phase.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| Existing placeholder detection utility can be extended without breaking current parsing. | code | May require new structural utilities and broader refactor. |
| Story lifecycle gates already centralize submit/accept legality checks. | architecture | Enforcement may need duplicated checks in command handlers. |
| Team accepts hard-cutover behavior for validation outcomes. | process | Rollout may stall until compatibility strategy is revisited. |

## Constraints

- Unresolved scaffold/default text must be treated as `error` for covered checks.
- Reflection/story completeness enforcement applies only at terminal workflow points (`needs-human-verification`, `done`, submit, accept).
- Planning-doc-focused validation scope excludes generated report artifacts for now.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | `keel doctor` must classify unresolved scaffold/default text in covered planning/coherency docs as errors. | FR-05 | doctor check unit tests + integration run |
| SRS-02 | `keel doctor` must fail terminal stories (`needs-human-verification` or `done`) that retain default story scaffold text. | FR-07 | story doctor tests with stage fixtures |
| SRS-03 | `keel doctor` must fail terminal stories that retain default reflection scaffold text. | FR-07 | reflection doctor tests with stage fixtures |
| SRS-04 | Story submit and accept transition gates must block unresolved scaffold/default text in story README and REFLECT artifacts. | FR-06 | transition gate tests + command tests |
| SRS-05 | Generated report artifacts must remain outside this voyage's unresolved-scaffold validation scope. | FR-05 | negative tests confirming no checks on excluded files |
| SRS-06 | Regression tests must assert hard-cutover behavior and remove legacy warning-oriented expectations. | FR-06, NFR-02 | test suite updates + passing CI checks |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-NFR-01 | Error messages must identify failing artifact and relevant unresolved pattern. | NFR-03 | assertion checks on message content |
| SRS-NFR-02 | Check behavior must be deterministic across doctor and transition-gate execution paths. | NFR-02 | shared helper tests + parity tests |
| SRS-NFR-03 | No compatibility fallback paths may downgrade error behavior to warnings. | NFR-01 | code inspection + regression tests |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
