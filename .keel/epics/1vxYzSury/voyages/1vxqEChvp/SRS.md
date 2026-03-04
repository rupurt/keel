# Icebox-First Story Intake - Software Requirements Specification

> Make story creation default to icebox so planning work never starts as active execution.

**Epic:** [1vxYzSury](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

In scope:
- Make `keel story new` create stories in `icebox` stage by default.
- Ensure generated story frontmatter/status reflects `icebox` immediately.
- Preserve clear transition path from `icebox` to execution (`thaw`/`start`) without doctor regressions.

Out of scope:
- Bulk migration of historical story stages.
- Changes to acceptance criteria verification behavior.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| Story template and lifecycle transitions remain the canonical entry path for new stories. | Internal contract | Multiple creation paths could drift stage defaults. |
| Doctor coherence checks continue to treat draft/planned intake discipline as a hard requirement. | Validation | Policy change may not remove the original planning friction. |

## Constraints

- Hard cutover: no dual default stage behavior.
- Existing stage transitions must remain valid and deterministic.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | `keel story new` MUST create all new stories with `status: icebox` by default, regardless of scope. | FR-01 | command unit tests + fixture checks |
| SRS-02 | Story creation output/guidance MUST direct users to thaw/start steps so the execution path remains explicit after icebox creation. | FR-01 | guidance output tests |
| SRS-03 | Story creation + linking in planned voyages MUST no longer produce immediate doctor coherence errors caused by default stage selection. | FR-01 | doctor regression test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-NFR-01 | Stage default behavior MUST be enforced in one canonical implementation path (no legacy fallbacks). | NFR-01 | architecture/behavior regression tests |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
