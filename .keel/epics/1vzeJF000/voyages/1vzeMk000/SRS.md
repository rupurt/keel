# Domain Foundation - Software Requirements Specification

> Mission model, state machine, frontmatter, loader, and Board integration

**Epic:** [1vzeJF000](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

### In Scope

- [SCOPE-01] Mission domain model struct with YAML frontmatter schema
- [SCOPE-02] Mission state machine (Defining, Active, Achieved, Verified, Paused, Abandoned)
- [SCOPE-03] Mission directory structure (.keel/missions/<id>/) with README.md
- [SCOPE-04] CHARTER.md scaffold with Goals table, Constraints, and Halting Rules
- [SCOPE-05] LOG.md scaffold for decision journal entries
- [SCOPE-06] Board loader integration — load missions from disk into Board struct
- [SCOPE-07] Mission frontmatter parsing with strict datetime validation

### Out of Scope

- [SCOPE-90] CLI commands (V2)
- [SCOPE-91] Doctor checks and lineage (V3)
- [SCOPE-92] Flow integration and `keel next` awareness (V4)

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| Existing entity patterns (frontmatter, state machine, loader) are sufficient for Mission | Assumption | May need new infrastructure patterns |
| Single active mission per board is adequate for v1 | Assumption | Would need multi-mission scheduling earlier |

## Constraints

- Mission must follow existing entity conventions (directory-based ID authority, YAML frontmatter, Entity trait)
- State machine transitions must be validated at the type level like other entities

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Define MissionFrontmatter struct with id, title, status, created_at, updated_at, activated_at, achieved_at, verified_at fields | SCOPE-01 | FR-01 | unit test |
| SRS-02 | Define MissionStatus enum with Defining, Active, Achieved, Verified, Paused, Abandoned variants | SCOPE-02 | FR-02 | unit test |
| SRS-03 | Implement typed state machine transitions: activate (Defining→Active), achieve (Active→Achieved), verify (Achieved→Verified), pause (Active→Paused), resume (Paused→Active), abandon (Active/Paused→Abandoned) | SCOPE-02 | FR-02 | unit test |
| SRS-04 | Define Mission struct implementing Entity trait with frontmatter, path, has_charter, has_log fields | SCOPE-01 | FR-01 | unit test |
| SRS-05 | Create .keel/missions/<id>/ directory structure with README.md, CHARTER.md scaffold, and LOG.md scaffold | SCOPE-03, SCOPE-04, SCOPE-05 | FR-04 | unit test |
| SRS-06 | Add missions: HashMap<String, Mission> to Board struct | SCOPE-06 | FR-01 | unit test |
| SRS-07 | Implement load_missions() in loader that discovers and parses .keel/missions/*/README.md | SCOPE-06 | FR-01 | unit test |
| SRS-08 | CHARTER.md scaffold includes Goals table (MG-XX ID, Description, Verification columns), Constraints section, and Halting Rules section | SCOPE-04 | FR-03 | unit test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Mission loading must not regress board load time by more than 10% on boards with zero missions | SCOPE-06 | NFR-02 | benchmark |
| SRS-NFR-02 | Mission state machine transitions must be deterministic across repeated invocations | SCOPE-02 | NFR-01 | unit test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
