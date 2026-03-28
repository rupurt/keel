# First-Class Turn Loop And Scene Contracts - SRS

## Summary

Epic: VF8hiVofm
Goal: Make turn structure and scene semantics explicit projections so visual surfaces render canonical engine state instead of distributed command-local interpretation.

## Scope

### In Scope

- [SCOPE-02] Add a first-class turn-loop projection that reflects the documented phases Orient, Inspect, Pull, Ship, and Close, and expose it through `keel turn`.
- [SCOPE-03] Define central scene contracts that describe scene-capable commands and their canonical signal dependencies.

### Out of Scope

- [SCOPE-05] Reworking unrelated non-scene command output.
- [SCOPE-06] Changing the heartbeat model or workflow topology itself.
- [SCOPE-07] Adding animation or UI affordances outside the CLI/read-model surface.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The turn-loop projection must represent the documented phases and identify the commands that make each phase legible. | SCOPE-02 | FR-03 | automated tests |
| SRS-02 | `keel turn` must render the turn-loop projection in a concise human-readable view and a JSON form suitable for harnesses. | SCOPE-02 | FR-03 | command regression + automated tests |
| SRS-03 | Scene contracts must define which public commands expose `--scene` and which canonical signals each scene depends on. | SCOPE-03 | FR-04 | automated tests |
| SRS-04 | Heartbeat-dependent scenes and health/routing scenes must be mappable through the central scene contracts instead of ad hoc lists. | SCOPE-03 | FR-04 | automated tests |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | The turn and scene projections must remain descriptive and read-only; they must not change existing command behavior beyond surfacing canonical state more explicitly. | SCOPE-02 SCOPE-03 | NFR-02 | code review + tests |
| SRS-NFR-02 | The new turn and scene surfaces must be deterministic and stable enough for drift tests and downstream scripting. | SCOPE-02 SCOPE-03 | NFR-01 | automated tests |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
