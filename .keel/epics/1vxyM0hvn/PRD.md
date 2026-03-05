# Parallel Safety For Next - Product Requirements

> Recommend parallel work with minimum merge-conflict risk using semantic code-structure analysis and pairwise blocker explanations.

## Problem Statement

`keel next --parallel` previously relied on shallow dependency checks and could recommend stories that still collide at implementation time. Without semantic conflict analysis and clear blocker explanations, parallel recommendations created avoidable merge churn, context switching, and reviewer friction.

## Goals & Objectives

| Goal | Success Metric | Target |
|------|----------------|--------|
| Increase parallel recommendation safety | Parallel suggestions avoid high-conflict pairs under representative fixtures | 0 high-confidence conflicting pairs returned |
| Make blocker reasoning explicit | Each excluded pair includes a clear blocker explanation | 100% of blocked pairs rendered with rationale |
| Keep scoring conservative | Only high-confidence low-conflict sets are emitted | No speculative low-confidence recommendations |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Implementer Agent | Pulls parallel work from queue for throughput | Low-conflict recommendations with clear exclusion reasons |
| Human reviewer | Reviews concurrent story delivery quality | Fewer merge collisions and clearer dependency rationale |
| Maintainer | Owns queue policy and diagnostics | Deterministic, testable conflict scoring behavior |

## Scope

### In Scope

- Extract semantic conflict features from code-structure and story metadata signals.
- Compute conservative pairwise conflict scores and confidence thresholds.
- Render blocker and compatibility rationale for selected and excluded story pairs.
- Add doctor/read-model coherence checks that keep conflict metadata and recommendations aligned.

### Out of Scope

- Replacing existing dependency graph semantics outside parallel recommendation paths.
- Probabilistic optimization based on runtime telemetry not available in-board.
- Auto-merging or branch orchestration tooling.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-01 | `next --parallel` must score pairwise story compatibility using semantic conflict features, not only explicit blockers. | must | Reduces hidden collision risk in concurrent execution. |
| FR-02 | Recommendation selection must apply a conservative confidence threshold before returning parallel sets. | must | Prevents risky low-signal suggestions. |
| FR-03 | Output must include pairwise blocker explanations for excluded candidate pairs. | must | Makes recommendations auditable and actionable. |
| FR-04 | Story-level metadata must support explicit blocked-by overrides that feed conflict analysis. | should | Preserves human governance on special-case conflicts. |
| FR-05 | Doctor and projection checks must validate coherence between conflict metadata and parallel recommendation output. | should | Prevents silent drift between data and decisions. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| NFR-01 | Conflict scoring must be deterministic for identical board inputs. | must | Ensures reproducible queue guidance. |
| NFR-02 | Pairwise analysis must remain performant for typical active-queue sizes. | must | Prevents recommendation latency regressions. |
| NFR-03 | Rendering must stay concise while still exposing blocker rationale for decision review. | should | Balances usability and diagnostic depth. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

- Add feature-extraction and pairwise-scoring unit tests that lock deterministic conflict outcomes for fixtures.
- Add command-level tests for `next --parallel` to verify conservative thresholding and blocker explanation rendering.
- Add doctor/read-model coherence tests that fail when conflict metadata and recommendation logic diverge.
- Gate completion with `just keel doctor` and parallel next regression suite in `just test`.

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Available code-structure signals are sufficient to distinguish low- and high-conflict pairs. | Scoring may produce too many false positives or false negatives. | Evaluate fixtures across recently completed parallel stories. |
| Conservative thresholds are preferable to higher throughput with occasional collisions. | Teams may request tunable aggressiveness controls. | Collect qualitative feedback from reviewers and implementers. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Large boards may require optimization if pairwise comparison count grows significantly. | Maintainer | Monitoring |
| Overly strict thresholds could reduce parallel suggestions below useful levels. | Epic owner | Mitigated with fixture calibration |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] `next --parallel` recommendations are based on deterministic semantic conflict scoring with conservative thresholds.
- [ ] Excluded pairings include actionable blocker explanations in command output.
- [ ] Story metadata overrides and doctor coherence checks keep conflict data trustworthy.
- [ ] Regression tests demonstrate lower conflict risk for returned parallel sets in fixture scenarios.
<!-- END SUCCESS_CRITERIA -->

## Voyages

<!-- Implementation breakdown (auto-generated by keel) -->

<!-- BEGIN VOYAGES -->
| Voyage | Status | Description |
|--------|--------|-------------|
| [1vxyMT6nz](voyages/1vxyMT6nz/) | done | Implement semantic conflict detection and conservative parallel recommendation logic. |
<!-- END VOYAGES -->
