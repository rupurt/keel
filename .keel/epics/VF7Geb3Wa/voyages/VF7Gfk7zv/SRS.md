# Introduce Derived Heartbeat Surface And Flow Fallback - SRS

## Summary

Epic: VF7Geb3Wa
Goal: Add a derived heartbeat projection and command, then cut flow over to it with a temporary compatibility fallback while the file-backed path still exists.

## Scope

### In Scope

- [SCOPE-01] Add a derived heartbeat projection that reports latest activity timestamp and signal source from repository state.
- [SCOPE-02] Expose the projection through a new `keel heartbeat` command for operators.
- [SCOPE-03] Cut `keel flow --scene` over to the derived heartbeat signal while keeping the file-backed path only as a bounded compatibility fallback.
- [SCOPE-04] Add regression coverage for derived, fallback, and idle/unplugged heartbeat scenarios.

### Out of Scope

- [SCOPE-05] Removing `.keel/heartbeat` from loader, cache, graph, hooks, or docs.
- [SCOPE-06] Rewriting `keel poke` or pre-commit hook behavior beyond the minimum compatibility needed for pass 1.
- [SCOPE-07] Broader changes to delivery heuristics unrelated to heartbeat-derived energization.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The derived heartbeat projection must compute the latest available activity timestamp from dirty tracked files first and otherwise from reachable commit activity, while exposing which source won. | SCOPE-01 | FR-01 | automated tests |
| SRS-02 | `keel heartbeat` must render the derived heartbeat timestamp, age, and source clearly enough for an operator to understand why the board is energized or idle. | SCOPE-02 | FR-02 | command regression + manual review |
| SRS-03 | `keel flow --scene` must use the derived heartbeat as its primary energization input and only use the legacy file-backed heartbeat when the derived signal is unavailable during pass 1. | SCOPE-03 | FR-03 | flow tests + CLI proof |
| SRS-04 | Pass 1 must include regression coverage for energized, unplugged, and compatibility-fallback scenarios so the later file removal has a stable safety net. | SCOPE-04 | FR-03 | automated tests |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | The derived heartbeat must remain deterministic and platform-stable without surfacing inode-level implementation details as a user-facing contract. | SCOPE-01 SCOPE-02 | NFR-01 | code review + tests |
| SRS-NFR-02 | The compatibility fallback must be isolated enough that pass 2 can delete it without changing the primary heartbeat projection API or command shape. | SCOPE-03 SCOPE-04 | NFR-02 | code review + tests |
| SRS-NFR-03 | Relevant fmt, clippy, and test suites must pass after the pass 1 cutover. | SCOPE-01 SCOPE-02 SCOPE-03 SCOPE-04 | NFR-03 | fmt + clippy + tests |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
