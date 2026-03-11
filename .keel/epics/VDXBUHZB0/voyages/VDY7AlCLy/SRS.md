# Storage Backend Configuration - SRS

## Summary

Enable users to configure the preferred storage backend in `keel.toml`. This is a prerequisite for supporting remote server-side boards or alternative local storage formats.

## Scope

### In Scope
- [SCOPE-01] Update `keel.toml` schema to include a `[storage]` section.
- [SCOPE-02] Support `filesystem` as the default backend.
- [SCOPE-03] Implement environment variable overrides for storage configuration.

### Out of Scope
- [SCOPE-04] Implementing the remote HTTP backend (handled in a separate epic).

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Description | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Add `[storage]` section to `keel.toml` with `backend` field. | SCOPE-01 | FR-01 | Unit test |
| SRS-02 | Default to `filesystem` storage if `[storage]` is missing or incomplete. | SCOPE-02 | FR-02 | Unit test |
| SRS-03 | Support `KEEL_STORAGE_BACKEND` environment variable override. | SCOPE-03 | FR-03 | Integration test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Description | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Validation of storage configuration should provide clear error messages for unknown backends. | SCOPE-01 | NFR-01 | Unit test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
