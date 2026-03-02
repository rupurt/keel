# Hard Schema Migration and Compatibility Cleanup - Software Requirements Specification

> Ship a hard migration to canonical states and fields, then remove compatibility paths and stale terminology.

**Epic:** [1vuz8K4NM](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

- Deliver a hard migration command that rewrites legacy board schema values to canonical forms.
- Remove runtime compatibility aliases/deserializers after migration is available.
- Normalize epic completion date field naming to `completed_at`.
- Clean CLI/docs/tests terminology to canonical state names.
- Ensure newly scaffolded epic/voyage planning artifacts are doctor-clean for timestamp and placeholder hygiene.
- Out of scope: queue-policy behavior design and transition-enforcement architecture.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| Boards can be migrated in-place with deterministic text transforms. | Internal | Requires staged export/import migration tooling. |
| Canonical state names are finalized (`needs-human-verification`, `in-progress`, `strategic/tactical`). | Product Decision | Migration command requires versioned mapping support. |
| Users accept a hard cutover and must run migration before new binaries. | User Decision 1 | Legacy parsing paths would need to stay and dilute cleanup goals. |

## Constraints

- Migration command must be idempotent.
- Post-migration runtime should reject legacy schema values rather than silently mapping them.
- Date fields for epic completion must be consistent with model expectations (`completed_at`).

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | Implement a hard migration command that rewrites legacy story/voyage/epic states and legacy date field keys to canonical schema values. | User Decision 1 | Migration integration tests on fixture boards. |
| SRS-02 | Remove legacy state/status deserializer aliases from canonical model/state-machine types after migration support is in place. | User Decision 1 | Unit tests confirming legacy values are rejected. |
| SRS-03 | Remove compatibility aliases (`Stage`, `Status`) and update code to canonical state types directly. | Refactor Program | Compile-time migration and unit tests. |
| SRS-04 | Normalize epic completion field usage to `completed_at` across command handlers, loader/model expectations, and doctor checks. | Architecture Review | Command + doctor tests for epic completion/reopen paths. |
| SRS-05 | Remove legacy terminology from CLI help, docs, and tests so canonical wording is the only supported language. | User Decision 1 | Grep-based checks and updated snapshot tests. |
| SRS-06 | Ensure newly scaffolded epic/voyage artifacts emit datetime `created_at` values and avoid default TODO placeholders that violate doctor structure/content checks. | Architecture Review | Command scaffolding tests + doctor regression checks on fresh artifacts. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-NFR-01 | Migration execution must be deterministic and safe to rerun without further changes. | Reliability | Idempotency tests over migrated fixtures. |
| SRS-NFR-02 | Migration failures must provide explicit file paths and actionable remediation guidance. | Operability | Error-message snapshot tests. |
| SRS-NFR-03 | Post-cutover code should minimize compatibility branches to reduce maintenance burden. | Maintainability | Static analysis and code review checklist. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
