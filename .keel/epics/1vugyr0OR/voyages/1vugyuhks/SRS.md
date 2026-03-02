# Rich Evidence Capture - Software Requirements Specification

> Streamline the evidence recording process with editor integration and multi-modal attachments.

**Epic:** [1vugyr0OR](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

### In Scope
*   `$EDITOR` integration for manual evidence notes.
*   First-class support for `vhs` terminal recording.
*   Automated `llm-judge` transcript capture.
*   Rich provenance tracking (Git SHA, timestamps, environment) for all artifacts.

### Out of Scope
*   Live video streaming (offline recording only).

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | Editor integration for evidence messages | FR-04 | CLI demo |
| SRS-02 | VHS recording integration (`record --vhs`) | FR-02 | Generated GIF in EVIDENCE/ |
| SRS-03 | LLM-Judge transcript capture (`record --judge`) | FR-02 | Signed transcript in EVIDENCE/ |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-NFR-01 | Tamper-proof evidence (hashes in manifest) | NFR-01 | keel doctor validation |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
