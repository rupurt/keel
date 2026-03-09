# Evidence-Backed Assessment and Surfaces - Software Requirements Specification

> Make bearing assessment and read surfaces evidence-backed and compute EV scores from evidence quality signals.

**Epic:** [1vzQpr000](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

In scope:
- [SCOPE-06] Update bearing show, file, doctor, and readiness rules to validate and surface the new evidence-backed workflow.
- [SCOPE-07] Evolve assessment and EV scoring to account for evidence breadth, freshness, authority, and contradiction handling.

Out of scope:
- [SCOPE-01] Redefine bearing artifacts around `BRIEF.md` for framing, `EVIDENCE.md` for cited research capture, and `ASSESSMENT.md` for synthesis and recommendation.
- [SCOPE-02] Hard-cut bearing command and lifecycle language from `survey` semantics to `research` semantics wherever the new workflow applies.
- [SCOPE-03] Add a canonical evidence contract with source IDs, source classes, provenance, dates, and evidence-quality metadata.
- [SCOPE-04] Support research capture for web, academic/prior-art, social/trend, and manual or internal signals.
- [SCOPE-05] Add configuration for research provider enablement and weighting heuristics that influence evidence ranking and downstream scoring.
- [SCOPE-08] Update templates, docs, tests, and fixture boards to the new canonical contract.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| The evidence contract and provider metadata from prior voyages are available to scoring and read surfaces through stable parsed structures. | Internal dependency | Surfaces and scoring would need duplicate parsing or placeholder logic. |
| Existing assessment-factor scoring can be extended rather than replaced wholesale. | Internal dependency | EV scoring changes may sprawl into a larger redesign than this voyage intends. |
| Bearing operators still need concise terminal summaries even as evidence provenance becomes richer. | Product assumption | Surfaces could become too dense if every citation detail is rendered at once. |

## Constraints

- Assessment conclusions must cite evidence IDs rather than summarizing unsupported claims.
- EV scoring must stay deterministic and reviewable; heuristic weightings are acceptable, opaque nondeterminism is not.
- Read surfaces need to surface provenance clearly without overwhelming the default terminal flow.
- Readiness and doctor rules must treat missing or weak evidence as explicit contract failures, not soft suggestions.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | `ASSESSMENT.md` and bearing read surfaces MUST require or expose evidence citations so findings, dependencies, alternatives, and recommendations can be traced back to canonical source IDs. | SCOPE-06 | FR-06 | read-model tests + CLI rendering tests |
| SRS-02 | The EV scoring model MUST incorporate evidence-quality signals including breadth, freshness, authority, and contradiction/gap handling alongside authored impact, confidence, effort, and risk factors. | SCOPE-07 | FR-07 | scoring tests + fixture scenario tests |
| SRS-03 | Bearing doctor, readiness, and list/flow surfaces MUST reflect evidence-backed quality gates and score outputs so incomplete or weakly supported research is visible before a bearing is treated as decision-ready. | SCOPE-06 SCOPE-07 | FR-06 | doctor tests + projection tests + CLI snapshot tests |
| SRS-04 | `bearing show` and `bearing file` MUST surface source provenance compactly enough for terminal review, including citation summaries and drill-down access to the underlying evidence document. | SCOPE-06 | FR-06 | CLI rendering tests + VHS proof |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Equivalent evidence inputs and authored assessment factors MUST produce stable EV scores and identical surface ordering. | SCOPE-06 SCOPE-07 | NFR-01 | deterministic score tests + snapshot tests |
| SRS-NFR-02 | Terminal surfaces MUST keep source provenance visible enough for inspection without requiring the operator to open raw files for every review. | SCOPE-06 | NFR-02 | renderer tests + VHS proof |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
