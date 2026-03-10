# Role Template Injection - Software Requirements Specification

> Scaffold role-specific management and execution templates for harness guidance and context injection.

**Epic:** [VDTpFlMKc](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

### In Scope

- [SCOPE-01] Canonical role templates for `manager/*` and `engineer/*`.
- [SCOPE-02] Deterministic template selection from parsed role taxonomies.
- [SCOPE-03] Injecting the selected role template into actionable `keel next` guidance.

### Out of Scope

- [SCOPE-04] User-authored or runtime-editable role templates.
- [SCOPE-05] Template families beyond the core management and execution roles.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| Harnesses consume `keel next` human or JSON output as the work handoff surface. | Workflow | We would need a dedicated prompt export command instead of guidance injection. |
| `RoleTaxonomy` parsing remains the canonical source of role identity. | Code | Template selection would drift from queue routing and authorization. |

## Constraints

- Reuse the existing actionable guidance contract instead of introducing a second role-template transport.
- Core scope is limited to `manager/*` and `engineer/*` until the base role families are proven stable.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The system must define canonical role templates for `manager/*` and `engineer/*` that include persona, operating priorities, and expected workflow guidance. | SCOPE-01 | FR-05 | unit test |
| SRS-02 | `keel next --role <TAXONOMY>` must attach the resolved role template to actionable human and JSON guidance so harnesses receive context with the work pull. | SCOPE-02, SCOPE-03 | FR-05 | unit test |
| SRS-03 | Unsupported role bases must fail deterministically with an error that lists the supported template families instead of silently falling back. | SCOPE-02 | FR-05 | unit test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Template selection must be deterministic for identical taxonomy inputs and use one canonical lookup path. | SCOPE-02 | NFR-01 | unit test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
