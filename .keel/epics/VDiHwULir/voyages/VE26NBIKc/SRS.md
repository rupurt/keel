# URL-Aware Capture Defaults - SRS

## Summary

Epic: VDiHwULir
Goal: Auto-derive capture defaults from URL location

## Scope

### In Scope

- [SCOPE-01] Auto-default `--class` to `web`, `--retrieved-at` to today, and `--provider` to `manual:<domain>` when `--location` is a URL.
- [SCOPE-02] Allow explicit flags to override auto-derived defaults.

### Out of Scope

- [SCOPE-03] HTTP fetching or page content extraction.
- [SCOPE-04] Auto-deriving subjective fields (authority, freshness, notes).

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | When `--location` starts with `http://` or `https://` and `--class` is omitted, the class must default to `web`. | SCOPE-01 | FR-01 | test |
| SRS-02 | When `--location` is a URL and `--retrieved-at` is omitted, the retrieval date must default to today. | SCOPE-01 | FR-02 | test |
| SRS-03 | When `--location` is a URL and `--provider` is omitted, the provider must default to `manual:<domain>` extracted from the URL host. | SCOPE-01 | FR-03 | test |
| SRS-04 | Explicit `--class`, `--retrieved-at`, and `--provider` flags must override URL-derived defaults. | SCOPE-02 | FR-04 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | URL detection must use string prefix matching only (no HTTP client or URL parsing crate). | SCOPE-01 | NFR-01 | test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
