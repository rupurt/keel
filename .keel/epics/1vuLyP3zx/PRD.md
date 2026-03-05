# validation-unification - Product Requirements

> Make doctor checks the single source of truth for all domain invariants and transition gates.

## Problem Statement

Validation logic is currently fragmented across `src/invariants.rs`, `src/state_machine/gating.rs`, `src/state_machine/validation.rs`, and various `src/commands/diagnostics/doctor/checks/` modules. This redundancy leads to:
1. **Drift**: A check might pass in `doctor` but fail in a command gate (or vice versa).
2. **Maintenance Overhead**: Changes to domain rules must be implemented in multiple places.
3. **Inconsistency**: Different parts of the system report errors using different types (`GateProblem` vs `Problem`).

## Goals & Objectives

Unify the validation architecture to ensure that every domain rule is defined exactly once and enforced everywhere.

| Goal | Success Metric | Target |
|------|----------------|--------|
| DRY Validation | Zero duplicated logic between doctor and gates | 0 duplications |
| Unified Reporting | Single `Problem` type used throughout the system | 1 type |
| Robust Gating | All transitions guarded by the same checks reported by doctor | 100% coverage |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Developer | Extending or maintaining Keel | A clear, single place to add or modify domain rules |
| Agent | Implementing features via Keel | Consistent feedback between `doctor` and command results |

## Scope

### In Scope

- Unification of `GateProblem` and `Problem` types.
- Centralization of all checks into the `doctor/checks` modules.
- Refactoring `gating.rs` to delegate to these shared check modules.
- Updating `story submit`, `voyage start`, and other gated commands to use unified checks.
- Ensuring `doctor` automatically runs all registered domain checks.

### Out of Scope

- Implementing new domain rules (focus is on refactoring existing ones).
- Performance optimization of checks (unless regressions occur).

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-01 | Unified Problem Type | must | Ensure consistent reporting across all tools |
| FR-02 | Shared Check Modules | must | Eliminate logic duplication and drift |
| FR-03 | Command Gate Integration | must | Ensure gates and doctor use the same logic |
| FR-04 | Auto-fix Support | should | Maintain current fixing capabilities in unified system |
| FR-05 | Evidence Chain Unification | should | Consolidate SRS/Evidence checks from submit.rs into doctor |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| NFR-01 | Pure Domain Logic | must | Domain checks should be testable without complex IO mocks |
| NFR-02 | Error Actionability | must | All unified problems must provide clear paths to resolution |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

- Add unit and integration tests that assert `doctor` and transition gates emit the same `Problem` set for equivalent board states.
- Add regression tests for `story submit`, `voyage start`, and other gate paths that now delegate to `doctor/checks`.
- Run `just keel doctor` and command-level lifecycle tests in CI to prevent gate/reporting drift.

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Existing checks are correct | We may codify incorrect behavior | Review checks during migration |
| All gates can be mapped to doctor checks | Some runtime-only checks might remain split | Audit all `evaluate_*` functions |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| How to handle IO-heavy vs Pure checks? | Engineering | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] `GateProblem` is removed/merged into `Problem`
- [ ] `src/state_machine/gating.rs` uses `doctor/checks` modules
- [ ] `keel doctor` output matches the blocking logic of `story submit`
- [ ] `invariants.rs` is minimized to core data extraction (parsers)
<!-- END SUCCESS_CRITERIA -->

