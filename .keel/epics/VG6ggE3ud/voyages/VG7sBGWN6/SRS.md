# Stabilize Mission Request Command Semantics - SRS

## Summary

Epic: VG6ggE3ud
Goal: Stabilize the mission request command contract for automation callers

## Scope

### In Scope

- [SCOPE-01] Define the canonical stdin/stdout request envelope for `template`, `parse`, `validate`, `draft`, `apply`, and `ack`.
- [SCOPE-02] Define deterministic machine-facing success, validation-failure, and execution-failure semantics for the command family.
- [SCOPE-03] Define the minimum provider metadata and mission fields automation callers must supply or can derive.

### Out of Scope

- [SCOPE-90] Keeper-side polling or provider webhook execution.
- [SCOPE-91] GitHub-specific revision tracking and acknowledgement policy.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | `keel mission request parse` and `validate` must consume a canonical mission request envelope that can be passed over stdin or loaded from a file without provider-specific fields becoming required. | SCOPE-01 | FR-01 | manual |
| SRS-02 | The command family must define stable machine-facing results for success, validation failure, and command failure, including which conditions are recoverable by the caller. | SCOPE-02 | FR-01 | manual |
| SRS-03 | The contract must define which fields are required from callers, which can be derived from provider metadata, and which are optional hints for downstream planning behavior. | SCOPE-03 | FR-01 | manual |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | The command contract must be deterministic and replayable so the same mission request payload produces the same semantic result across automation callers. | SCOPE-02 | NFR-01 | manual |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
