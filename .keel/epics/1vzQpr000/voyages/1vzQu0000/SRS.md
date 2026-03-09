# Evidence Capture and Provider Signals - Software Requirements Specification

> Add first-class evidence capture, provider provenance, and configurable research-source weighting for bearing research.

**Epic:** [1vzQpr000](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

In scope:
- [SCOPE-03] Add a canonical evidence contract with source IDs, source classes, provenance, dates, and evidence-quality metadata.
- [SCOPE-04] Support research capture for web, academic/prior-art, social/trend, and manual or internal signals.
- [SCOPE-05] Add configuration for research provider enablement and weighting heuristics that influence evidence ranking and downstream scoring.

Out of scope:
- [SCOPE-01] Redefine bearing artifacts around `BRIEF.md` for framing, `EVIDENCE.md` for cited research capture, and `ASSESSMENT.md` for synthesis and recommendation.
- [SCOPE-02] Hard-cut bearing command and lifecycle language from `survey` semantics to `research` semantics wherever the new workflow applies.
- [SCOPE-06] Update bearing show, file, doctor, and readiness rules to validate and surface the new evidence-backed workflow.
- [SCOPE-07] Evolve assessment and EV scoring to account for evidence breadth, freshness, authority, and contradiction handling.
- [SCOPE-08] Update templates, docs, tests, and fixture boards to the new canonical contract.
- [SCOPE-09] Building authenticated integrations for every possible external provider or paid research API.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| The first implementation can represent provider outputs through deterministic adapters or fixtures before full live integrations are added. | Delivery assumption | The voyage could stall behind auth, quota, or network complexity. |
| `keel.toml` is the right home for provider enablement and weighting controls. | Product assumption | Configuration could splinter into multiple non-canonical override surfaces. |
| Manual or internal evidence must use the same canonical source schema as web or provider-backed research. | Product assumption | Evidence quality and citation handling would diverge by source class. |

## Constraints

- Provider availability, disabled state, and missing credentials must be surfaced explicitly; the system cannot silently replace unavailable research with model-memory summaries.
- Source metadata must stay structured enough for downstream scoring and citations without requiring every provider to supply every optional field on day one.
- This voyage must support the four agreed signal classes: web, academic/prior-art, social/trend, and manual/internal.
- Any provider weighting logic must be deterministic for the same config and inputs.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | `EVIDENCE.md` MUST model canonical source records with stable IDs and metadata including source class, provenance, publication or observation date, retrieval date, and evidence-quality fields such as authority and freshness. | SCOPE-03 | FR-03 | parser tests + template tests + fixture evidence tests |
| SRS-02 | The research workflow MUST capture evidence from web, academic/prior-art, social/trend, and manual/internal inputs through one canonical ingestion path that records provider provenance for each source. | SCOPE-04 | FR-04 | command tests + adapter fixture tests |
| SRS-03 | `keel.toml` MUST expose provider enable/disable state and weighting or ranking heuristics used to order or evaluate evidence sources. | SCOPE-05 | FR-05 | config parse tests + recommendation/order tests |
| SRS-04 | When a provider is unavailable, disabled, or unsupported for the current environment, the research workflow MUST surface that state explicitly in command output and artifact metadata instead of inventing evidence. | SCOPE-04 SCOPE-05 | FR-04 | provider-state tests + CLI output tests |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Equivalent evidence inputs and config MUST produce deterministic source ordering, provider status reporting, and stored evidence metadata. | SCOPE-03 SCOPE-04 SCOPE-05 | NFR-01 | deterministic fixture tests |
| SRS-NFR-02 | Provider failures or gaps MUST stay explicit and MUST NOT be masked by uncited model-generated findings. | SCOPE-04 SCOPE-05 | NFR-03 | failure-mode tests + CLI error tests |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
