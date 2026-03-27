# Downstream Adoption And Upgrade Docs - SRS

## Summary

Epic: VF2RJfiKo
Goal: Add formal docs that show downstream repositories how to adopt Keel's agent contract and how to upgrade and sync upstream guidance without losing project-specific adaptations.

## Scope

### In Scope

- [SCOPE-01] Add a workflow docs page that explains the role of `AGENTS.md` and `INSTRUCTIONS.md` in downstream repositories using Keel as the project-management engine.
- [SCOPE-02] Show the concrete adaptation seams between upstream Keel instructions and the downstream `port` repository.
- [SCOPE-03] Add a workflow docs page that explains how to upgrade Keel and sync upstream instruction changes safely.
- [SCOPE-04] Update navigation and supporting cross-links so the new pages are discoverable from the current docs IA.

### Out of Scope

- [SCOPE-05] Automated file sync tooling or CLI features for downstream instruction upgrades.
- [SCOPE-06] New runtime behavior in Keel itself.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The docs site must include a workflow page that explains how `AGENTS.md` and `INSTRUCTIONS.md` operate as the downstream agent contract when a repository adopts Keel as its project-management engine. | SCOPE-01 | FR-01 | manual |
| SRS-02 | That page must describe which parts of the upstream contract remain canonical and which parts are adapted downstream, using `port` as the concrete example. | SCOPE-01 SCOPE-02 | FR-02 | manual |
| SRS-03 | The docs site must include a workflow page that explains how to upgrade Keel and sync upstream instruction changes while preserving repo-specific wrappers, proof contracts, and validation surfaces. | SCOPE-03 | FR-03 | manual |
| SRS-04 | The new workflow material must be integrated into the current public docs navigation and linked from at least one adjacent onboarding or workflow page. | SCOPE-04 | FR-01 | manual |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | The new docs pages must reuse the existing product-style docs visual language and components so they read as part of the public docs site rather than as a raw internal appendix. | SCOPE-01 SCOPE-03 SCOPE-04 | NFR-01 | manual |
| SRS-NFR-02 | The guidance must stay vendor-neutral by focusing on repository contracts, command surfaces, and upgrade steps rather than a single model or harness provider. | SCOPE-01 SCOPE-02 SCOPE-03 | NFR-02 | manual |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
