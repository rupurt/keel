# CLI Commands - Software Requirements Specification

> Full mission lifecycle commands: new, refine, activate, show, list, pause, achieve, verify, abandon

**Epic:** [1vzeJF000](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

### In Scope

- [SCOPE-01] `keel mission new` command — create mission with README.md, CHARTER.md, LOG.md
- [SCOPE-02] `keel mission refine` command — iterative CHARTER.md goal elicitation loop
- [SCOPE-03] `keel mission activate` command — gate on CHARTER completeness, transition Defining → Active
- [SCOPE-04] `keel mission show` command — display mission state, goals, child entities
- [SCOPE-05] `keel mission list` command — list all missions with status
- [SCOPE-06] `keel mission pause/achieve/verify/abandon` transition commands
- [SCOPE-07] `keel mission log` command — append structured entry to LOG.md
- [SCOPE-08] `keel mission digest` command — compress older LOG.md entries

### Out of Scope

- [SCOPE-90] Doctor checks and lineage validation (V3)
- [SCOPE-91] `keel next` and `keel flow` mission-awareness (V4)

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| V1 domain foundation is complete (Mission struct, state machine, loader) | Dependency | Cannot build CLI without domain layer |
| CHARTER.md goal table format is stable | Assumption | Refine command logic may need updating |

## Constraints

- All commands must support `--json` output for harness consumption
- Transition commands must follow existing guidance pattern (next step / recovery)
- `keel mission refine` must return structured output (question + field) for harness loop

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | `keel mission new "<title>"` creates .keel/missions/<id>/ with README.md (Defining status), CHARTER.md scaffold, and LOG.md scaffold | SCOPE-01 | FR-04 | integration test |
| SRS-02 | `keel mission refine <id>` analyzes CHARTER.md completeness and returns next question or "ready" signal | SCOPE-02 | FR-05 | unit test |
| SRS-03 | `keel mission refine <id> --answer "<text>"` records answer into CHARTER.md and returns next question or "ready" | SCOPE-02 | FR-05 | integration test |
| SRS-04 | `keel mission activate <id>` transitions Defining → Active, gated on CHARTER Goals table having at least one authored MG-XX row | SCOPE-03 | FR-06 | integration test |
| SRS-05 | `keel mission show <id>` displays mission title, status, goals, child epics/bearings, and LOG summary | SCOPE-04 | FR-07 | integration test |
| SRS-06 | `keel mission list` displays all missions with id, title, status, and child count | SCOPE-05 | FR-07 | integration test |
| SRS-07 | `keel mission pause <id>` transitions Active → Paused | SCOPE-06 | FR-08 | unit test |
| SRS-08 | `keel mission achieve <id>` transitions Active → Achieved, gated on all board-verifiable goals being satisfied | SCOPE-06 | FR-08 | integration test |
| SRS-09 | `keel mission verify <id>` transitions Achieved → Verified (terminal) | SCOPE-06 | FR-08 | unit test |
| SRS-10 | `keel mission abandon <id>` transitions Active/Paused → Abandoned (terminal) | SCOPE-06 | FR-08 | unit test |
| SRS-11 | `keel mission log <id> --entry "<text>"` appends timestamped entry to LOG.md | SCOPE-07 | FR-10 | integration test |
| SRS-12 | `keel mission digest <id>` compresses LOG.md entries older than threshold into summary block | SCOPE-08 | FR-11 | integration test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | All commands produce deterministic output for identical inputs | SCOPE-01 | NFR-01 | unit test |
| SRS-NFR-02 | `--json` output follows existing keel JSON conventions (guidance envelope, etc.) | SCOPE-01 | NFR-01 | unit test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
