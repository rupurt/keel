# Core Changes - Software Requirements Specification

> GOAL-02, GOAL-01

**Epic:** [VDTpFlMKc](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

### In Scope

- [SCOPE-01] Role taxonomy parsing and matching.
- [SCOPE-02] Updating `keel next` to use `--role`.
- [SCOPE-03] Updating `keel flow` terminology.
- [SCOPE-04] Authorizing `keel story accept` based on role.

### Out of Scope

- [SCOPE-05] Dynamic role creation.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| vibes repository taxonomy parser | Code | We have to write the parser contract from scratch. |

## Constraints

- Hard cutover applies: legacy `--agent` and `--human` flags must not remain as runtime aliases.
- Queue routing must continue to use the existing two-lane pull model while renaming the lanes.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The system must provide a parser for role taxonomies (e.g. `role/specialization:tags`). | SCOPE-01 | FR-01 | unit test |
| SRS-02 | `keel next` must accept a `--role` flag and reject `--human` and `--agent` with explicit conflict guidance. | SCOPE-02 | FR-01 | unit test |
| SRS-03 | `keel next` must map `manager/*` roles to the Management queue and `engineer/*` to Execution. | SCOPE-02 | FR-04 | unit test |
| SRS-04 | `keel flow`, command help, and regression docs must display "Management Queue" instead of "Human Queue" and "Execution Queue" instead of "Agent Queue". | SCOPE-03 | FR-02 | unit test |
| SRS-05 | `keel story accept` must require `--role <TAXONOMY>` and enforce a `manager/*` role for stories containing manual verification criteria. | SCOPE-04 | FR-03 | unit test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Role parsing and queue-routing decisions must be deterministic for the same taxonomy input. | SCOPE-01 | NFR-01 | unit test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
