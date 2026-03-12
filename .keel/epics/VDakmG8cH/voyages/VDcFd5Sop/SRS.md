# Pulse Automation Execution - SRS

## Summary

This voyage operationalizes recurring work by adding a non-interactive pulse
command, idempotent materialization behavior, and scheduled automation
visibility in flow.

## Scope

### In Scope

- [SCOPE-01] `keel pulse` command surface and automation-cycle output
- [SCOPE-02] Due routine materialization with duplicate prevention
- [SCOPE-03] Scheduled automation lane or equivalent scheduled-capacity flow view

### Out of Scope

- [SCOPE-90] Hosted schedulers or daemonized background services
- [SCOPE-91] Narrative documentation for business automation users

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Provide a `keel pulse` command that runs non-interactively and reports the routines it evaluated or triggered. | SCOPE-01 | FR-01 | integration test |
| SRS-02 | Materialize due routine work exactly once per eligible window and skip duplicate work safely on repeated runs. | SCOPE-02 | FR-02 | integration test |
| SRS-03 | Preserve structured diagnostic state for why pulse ran, skipped, or deferred routine work. | SCOPE-01,SCOPE-02 | FR-02 | integration test |
| SRS-04 | Extend `keel flow` to surface due or upcoming scheduled automation demand. | SCOPE-03 | FR-03 | integration test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Pulse remains idempotent and safe for frequent cron/systemd execution. | SCOPE-02 | NFR-01 | integration test |
| SRS-NFR-02 | Pulse and scheduled-lane output remain structured and observable enough for operators and regression tests. | SCOPE-01,SCOPE-03 | NFR-02 | integration test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
