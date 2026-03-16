# Research-Preserving Bearing Promotion - Product Requirements

## Problem Statement

When `keel bearing lay` promotes a bearing to an epic, research artifacts (EVIDENCE.md source records, BRIEF.md open questions) are silently dropped. The generated PRD receives generic boilerplate for goals, scope, and risks while the operator's curated evidence provenance and unresolved questions vanish. This forces manual restatement during voyage planning and breaks the evidence chain from research to delivery.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Preserve evidence source records in the generated PRD so epic planners retain citation provenance. | PRD contains source table from EVIDENCE.md after `bearing lay` | 100% of laid bearings with evidence |
| GOAL-02 | Carry open questions and risks from bearing BRIEF into PRD Open Questions & Risks table. | PRD risks section populated from BRIEF open questions | 100% of laid bearings with open questions |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Mission Steward | Plans voyages and stories after bearing promotion | Needs evidence provenance and open risks visible in the epic PRD without revisiting bearing directories |
| Operator | Implements stories under the epic | Needs source citations traceable from story-level work back to original research |

## Scope

### In Scope

- [SCOPE-01] Include EVIDENCE.md source table in generated PRD as a "Research Provenance" section.
- [SCOPE-02] Populate PRD Open Questions & Risks table from BRIEF.md open questions.

### Out of Scope

- [SCOPE-03] Restructuring assessment analysis extraction (already works).
- [SCOPE-04] Generating richer goals/requirements from bearing content (future work).
- [SCOPE-05] Backward-compatible migration of existing PRDs.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | `bearing lay` must include the EVIDENCE.md source table in the generated PRD under a "Research Provenance" section. | GOAL-01 | must | Without sources in the PRD, evidence provenance is severed at promotion. |
| FR-02 | `bearing lay` must extract open questions from BRIEF.md and populate the PRD Open Questions & Risks table with them. | GOAL-02 | must | Open questions represent unresolved risks that must survive into epic planning. |
| FR-03 | When EVIDENCE.md or open questions are absent, the corresponding PRD sections must degrade gracefully to the existing boilerplate. | GOAL-01, GOAL-02 | must | Bearings without evidence or open questions must still lay cleanly. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Generated PRD must pass `keel doctor` structural validation after `bearing lay`. | GOAL-01, GOAL-02 | must | New sections must not break existing epic validation. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Evidence provenance | Cargo test: lay bearing with EVIDENCE.md, verify PRD contains source table | Story-level test linked to SRS requirement |
| Open questions carry | Cargo test: lay bearing with open questions, verify PRD risks populated | Story-level test linked to SRS requirement |
| Graceful degradation | Cargo test: lay bearing without evidence/questions, verify existing boilerplate | Story-level test linked to SRS requirement |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| EVIDENCE.md source table follows the existing `\| ID \| Class \| ...` format | Parser may not extract sources correctly | Verify against existing evidence fixtures |
| BRIEF.md open questions are bullet-pointed under `## Open Questions` | Extraction regex may miss items | Verify against existing brief fixtures |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Should the source table be copied verbatim or reformatted for PRD style? | Planner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] A bearing with evidence and open questions produces a PRD that contains both the source table and populated risks after `bearing lay`.
- [ ] A bearing without evidence or open questions produces the same PRD as today (no regression).
<!-- END SUCCESS_CRITERIA -->
