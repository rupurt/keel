# Accelerated Bearing Source Capture - Product Requirements

## Problem Statement

Capturing evidence sources via `keel bearing research` requires 8 mandatory flags even when the location is a URL that carries implicit metadata. Operators must manually specify `--class web`, `--retrieved-at <today>`, and `--provider manual:<domain>` every time, creating friction that slows research workflows.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Reduce required flags for URL-based evidence capture by auto-deriving class, retrieval date, and provenance from the URL. | Number of required flags when capturing a URL source | 4 (down from 8) |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Explorer | Collects evidence sources during bearing research | Faster capture with fewer mandatory flags for URL-based sources |

## Scope

### In Scope

- [SCOPE-01] Auto-default `--class` to `web`, `--retrieved-at` to today, and `--provider` to `manual:<domain>` when `--location` is a URL.
- [SCOPE-02] Allow explicit flags to override auto-derived defaults.

### Out of Scope

- [SCOPE-03] HTTP fetching or page content extraction (no new dependencies).
- [SCOPE-04] Auto-deriving `--observed-at`, `--authority`, `--freshness`, or `--notes` (subjective/temporal fields remain manual).

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | When `--location` starts with `http://` or `https://`, `--class` must default to `web` if not explicitly provided. | GOAL-01 | must | URL sources are almost always web class. |
| FR-02 | When `--location` is a URL, `--retrieved-at` must default to today's date if not explicitly provided. | GOAL-01 | must | Retrieval date for a URL is always "now". |
| FR-03 | When `--location` is a URL, `--provider` must default to `manual:<domain>` extracted from the URL if not explicitly provided. | GOAL-01 | must | Domain is the natural provenance identifier for web sources. |
| FR-04 | Explicit `--class`, `--retrieved-at`, and `--provider` flags must override the URL-derived defaults. | GOAL-01 | must | Operators must retain full control. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | URL detection must not add external dependencies (no HTTP client). | GOAL-01 | must | Keeps the tool lightweight and offline-capable. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| URL defaults | Cargo test: capture with URL location, verify class/retrieved-at/provider auto-derived | Story-level test |
| Override behavior | Cargo test: capture with URL + explicit flags, verify explicit values win | Story-level test |
| Non-URL passthrough | Cargo test: capture with non-URL location, verify all flags still required | Story-level test |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| URL locations always start with `http://` or `https://` | Some URLs may be missed | Validate against existing evidence source records |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Should `file://` URLs also trigger defaults? | Planner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] `keel bearing research <id> --location https://example.com --observed-at 2026-03-16 --authority high --freshness high --notes "note"` succeeds without --class, --retrieved-at, or --provider.
<!-- END SUCCESS_CRITERIA -->
