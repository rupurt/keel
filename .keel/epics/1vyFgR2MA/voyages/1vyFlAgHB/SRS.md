# Epic Problem Hydration - Software Requirements Specification

> Seed authored problem context from `epic new --problem` and keep goals as PRD-authored content so new epics start with real strategic context instead of scaffold comments.

**Epic:** [1vyFgR2MA](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

In scope:
- Add explicit CLI support for authored epic problem input during `epic new`.
- Replace CLI-owned goal hydration with a problem-only strategic input contract for `epic new`.
- Hydrate authored problem content into fresh epic scaffold surfaces that currently depend on CLI strategic input.
- Keep the `Goals & Objectives` section authored directly in `PRD.md` instead of hydrating it from CLI input.
- Keep fresh epic scaffolds doctor-clean after the new authored inputs are applied.
- Extend template-token contracts to support the new problem-only strategic-input hydration path.

Out of scope:
- Canonical `GOAL-*` IDs and goal-to-requirement linkage validation.
- Scope linkage and scope-drift detection.
- PRD-to-SRS FR/NFR lineage enforcement.
- Historical migration of already-authored epic PRDs.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| `epic new` remains the canonical epic scaffolding path. | Workflow | Additional creation paths would also need the same problem-only contract. |
| Epic creation still benefits from one CLI-owned strategic seed even if goals move fully into authored PRD content. | Product contract | The command could become an empty shell again if it stops seeding any authored context. |
| Template token inventory tests remain the right place to enforce CLI-owned planning tokens. | Tooling | Token ownership drift could reappear in other scaffold paths. |

## Constraints

- Fail fast on missing or empty strategic inputs; do not synthesize fake product narrative when required values are absent.
- Keep the scaffold content concise and editable so planners can refine it immediately after creation.
- Do not couple this voyage to canonical `GOAL-*` IDs; goal structure and goal lineage belong to downstream voyages.
- Fresh epic scaffolds must remain compatible with existing structural and authored-content doctor checks.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | `keel epic new` MUST accept a required `--problem` argument and reject empty problem values at CLI/runtime validation time. | FR-05 | CLI parsing tests + epic creation tests |
| SRS-02 | `keel epic new --problem` MUST hydrate authored narrative content into the PRD `## Problem Statement` section and any epic scaffold summary surface that depends on CLI strategic input. | FR-05 | template rendering tests + scaffold fixture assertions |
| SRS-03 | `keel epic new` MUST stop treating `Goals & Objectives` as CLI-owned scaffold content; fresh PRDs leave that section for direct authoring instead of hydrating a single `--goal` value. | FR-05 | scaffold output tests + PRD content assertions |
| SRS-04 | Template token inventory and rendering paths MUST support the new problem-only strategic-input contract without introducing placeholder or ownership drift. | FR-05 | template token tests + rendering regression tests |
| SRS-05 | Freshly scaffolded epic artifacts MUST remain doctor-clean and structurally coherent after the problem-only hydration behavior lands. | FR-05 | doctor regression tests on new epic fixtures |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-NFR-01 | Epic scaffolding behavior MUST be deterministic for identical CLI inputs. | NFR-02 | deterministic fixture tests |
| SRS-NFR-02 | The new CLI path MUST fail fast instead of injecting compatibility defaults or fake narrative when required strategic input is missing. | NFR-01 | negative tests for empty/missing input |
| SRS-NFR-03 | Generated problem seed content and revised goal scaffolds MUST remain concise, human-editable, and free of unresolved placeholders. | NFR-03 | string assertions + doctor hygiene tests |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
