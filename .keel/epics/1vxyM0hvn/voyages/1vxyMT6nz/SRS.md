# Semantic Conflict Detection For Parallel Next - Software Requirements Specification

> Select low-conflict parallel stories using semantic code-structure analysis, conservative confidence thresholding, and pairwise blocker explanations.

**Epic:** [1vxyM0hvn](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

In scope:
- Semantic pairwise conflict detection for `keel next --parallel`.
- Conservative conflict scoring with confidence and global thresholding.
- Pairwise blocker explanations in terminal and JSON output.
- Optional `blocked_by` metadata override.
- Doctor validation for parallel conflict coherence.

Out of scope:
- Git history or co-change heuristics.
- `keel.toml` profile tuning for this algorithm.
- Automatic code graph generation from external services.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| Existing queue/traceability projections remain available for candidate generation. | Internal | Parallel selection cannot compose with existing readiness logic. |
| Conflict-risk analysis can reuse local repository semantics (paths, modules, scopes) without external APIs. | Technical | Confidence scoring degrades and defaults to blocked more often. |
| Human operators will maintain optional `blocked_by` metadata when used. | Process | Metadata override quality decreases and doctor findings increase. |

## Constraints

- Conservative-by-default: uncertain pair assessments must be blocked.
- Same-file or same-module overlap is not an automatic block; only difficult-to-resolve conflicts should block.
- Recommendations must optimize for minimum conflict risk over throughput.
- Output must show pairwise blockers with concrete reasons.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | Build a semantic feature extractor that computes deterministic pairwise conflict signals from code structure and story metadata. | PRD-01 | cargo test --lib next_parallel_feature_vectors_are_deterministic |
| SRS-02 | Implement conservative pairwise conflict scoring that returns both risk and confidence, downgrading confidence for unresolved architectural uncertainty. | PRD-01 | cargo test --lib next_parallel_pairwise_scoring_is_conservative |
| SRS-03 | Apply one global confidence threshold to parallel eligibility and block uncertain pairs by default. | PRD-02 | cargo test --lib next_parallel_threshold_blocks_uncertain_pairs |
| SRS-04 | Render pairwise blocker explanations and reasons in `keel next --parallel` human and JSON modes. | PRD-02 | cargo test --lib next_parallel_pairwise_blockers_render_consistently |
| SRS-05 | Support optional story `blocked_by` metadata as an explicit override for parallel recommendations. | PRD-03 | cargo test --lib next_parallel_blocked_by_override_enforced |
| SRS-06 | Select actionable parallel stories by minimizing conflict risk while preserving deterministic ordering and clear blocked display. | PRD-01 | cargo test --lib next_parallel_selection_prioritizes_low_risk |
| SRS-07 | Add doctor diagnostics that detect invalid or contradictory parallel blocking signals and require remediation. | PRD-03 | cargo test --lib doctor_parallel_conflict_coherence_checks |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| NFR-01 | Parallel recommendation output is deterministic for identical board and repository state. | NFR-Determinism | cargo test --lib next_parallel_output_is_deterministic |
| NFR-02 | Failure mode is conservative: unresolved semantic context produces blocked pairs with explicit rationale. | NFR-Safety | cargo test --lib next_parallel_unknown_context_is_blocked |
| NFR-03 | Each blocked pair includes at least one reason string describing the risk domain and confidence context. | NFR-Explainability | cargo test --lib next_parallel_blocker_reasons_are_actionable |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
