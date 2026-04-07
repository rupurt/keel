# Normalize GitHub Issues Into Mission Requests - SRS

## Summary

Epic: VG73ONWxt
Goal: Define the first GitHub issue ingestion slice that detects formal mission requests, normalizes them, and invokes native Keel commands.

## Scope

### In Scope

- [SCOPE-01] Define the GitHub-first activation, normalization, and acknowledgement flow that turns a formal issue into a canonical mission request and invokes the matching Keel command surface.

### Out of Scope

- [SCOPE-02] Additional providers, non-GitHub transport adapters, or connector-specific branching beyond the first GitHub issue path.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The voyage SHALL define the GitHub activation rule based on the formal issue-title prefix and the structured mission request body template. | SCOPE-01 | FR-01 | manual |
| SRS-02 | The voyage SHALL define how Keeper normalizes issue title, body, provider identity, and revision metadata into the canonical mission request envelope before invoking Keel. | SCOPE-01 | FR-01 | manual |
| SRS-03 | The voyage SHALL define the invocation and acknowledgement boundary between Keeper provider ingress and the native `keel mission request` commands. | SCOPE-01 | FR-01 | manual |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | The GitHub ingestion flow SHALL preserve deterministic replay inputs for retries, edits, and acknowledgement decisions. | SCOPE-01 | NFR-01 | manual |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
