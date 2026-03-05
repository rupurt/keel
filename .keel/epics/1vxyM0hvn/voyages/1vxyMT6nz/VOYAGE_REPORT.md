# VOYAGE REPORT: Semantic Conflict Detection For Parallel Next

## Voyage Metadata
- **ID:** 1vxyMT6nz
- **Epic:** 1vxyM0hvn
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 7/7 stories complete

## Implementation Narrative
### Semantic Conflict Feature Extraction
- **ID:** 1vxyMr3U2
- **Status:** done

#### Summary
Implement deterministic semantic feature extraction for candidate story pairs so `keel next --parallel` can reason about difficult-to-resolve merge conflicts.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Pairwise feature vectors are deterministic for identical board and repository inputs. <!-- verify: cargo test --lib next_parallel_feature_vectors_are_deterministic, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Extractor emits explicit unresolved-context signals when architectural semantics are insufficient. <!-- verify: cargo test --lib next_parallel_feature_vectors_emit_unknown_risk, SRS-01:start:end, proof: ac-2.log-->

#### Implementation Insights
- **L001: Work Item Comparator Is Not Lexical**
  - Insight: `compare_work_item_ids` can order IDs differently from naive lexical sorting (for example `S10` before `S2`)
  - Suggested Action: Use `compare_work_item_ids` for all deterministic work-item ordering and avoid hard-coded lexical expectations in tests
  - Applies To: `src/cli/commands/management/next_support/*`
  - Category: code


#### Verified Evidence
- [ac-1.log](../../../../stories/1vxyMr3U2/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vxyMr3U2/EVIDENCE/ac-2.log)

### Pairwise Blocker Rendering For Parallel Next
- **ID:** 1vxyMsbOj
- **Status:** done

#### Summary
Render pairwise blocker explanations so operators can see exactly which story pairs are blocked and why.

#### Acceptance Criteria
- [x] [SRS-04/AC-01] Human output displays pairwise blocker entries with `story -> blocked_by` and concrete reasons. <!-- verify: cargo test --lib next_parallel_pairwise_blockers_render_human, SRS-04:start:end, proof: ac-1.log-->
- [x] [SRS-04/AC-02] JSON output includes the same pairwise blocker semantics with stable field names. <!-- verify: cargo test --lib next_parallel_pairwise_blockers_render_json, SRS-04:start:end, proof: ac-2.log-->

#### Implementation Insights
- **L001: Keep Blocker Schema Shared Across Human and JSON Paths**
  - Insight: A single blocker model (`story_id`, `blocked_by_story_id`, `reasons`, `confidence`) makes it easy to keep human and JSON outputs in sync
  - Suggested Action: Build future blocker explanations from the same canonical blocker payload and only vary presentation
  - Applies To: `src/cli/commands/management/next.rs`, `src/cli/commands/management/next_support/parallel_threshold.rs`
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vxyMsbOj/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vxyMsbOj/EVIDENCE/ac-2.log)

### Conservative Pairwise Conflict Scoring
- **ID:** 1vxyMsepz
- **Status:** done

#### Summary
Implement conservative scoring that transforms semantic feature vectors into pairwise conflict risk and confidence, biasing toward blocked outcomes when confidence is low.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] Scorer returns pairwise risk and confidence for every evaluated story pair. <!-- verify: cargo test --lib next_parallel_pairwise_scoring_is_conservative, SRS-02:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-02] Unresolved architectural signals reduce confidence and raise conservative conflict risk. <!-- verify: cargo test --lib next_parallel_pairwise_scoring_penalizes_uncertainty, SRS-02:start:end, proof: ac-2.log-->

#### Implementation Insights
- **L001: Unknown Context Should Force Risk Floor**
  - Insight: Unresolved semantic context is easiest to keep safe when scoring applies an explicit risk floor and confidence ceiling instead of only additive penalties
  - Suggested Action: Keep conservative fallback thresholds as first-class scoring invariants and assert them directly in tests
  - Applies To: `src/cli/commands/management/next_support/parallel_*.rs`
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vxyMsepz/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vxyMsepz/EVIDENCE/ac-2.log)

### Parallel Queue Selection With Confidence Threshold
- **ID:** 1vxyMsvug
- **Status:** done

#### Summary
Integrate confidence thresholding into parallel selection so only high-confidence low-conflict candidates are surfaced as actionable work.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] A single global confidence threshold gates pairwise parallel eligibility. <!-- verify: cargo test --lib next_parallel_threshold_blocks_uncertain_pairs, SRS-03:start:end, proof: ac-1.log-->
- [x] [SRS-03/AC-02] Threshold gating blocks uncertain pairs conservatively by default when confidence is unresolved. <!-- verify: cargo test --lib next_parallel_threshold_blocks_uncertain_pairs, SRS-03:start:end, proof: ac-2.log-->

#### Implementation Insights
- **L001: Greedy Threshold Gate Gives Deterministic Safe Subset**
  - Insight: Sorting candidates by canonical work-item comparator before threshold filtering yields deterministic, reproducible safe subsets
  - Suggested Action: Keep canonical ID ordering and missing-pair fallback confidence (`0.0`) as hard invariants in gate logic
  - Applies To: `src/cli/commands/management/next_support/parallel_threshold.rs`
  - Category: code


#### Verified Evidence
- [ac-1.log](../../../../stories/1vxyMsvug/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vxyMsvug/EVIDENCE/ac-2.log)

### Story Blocked By Metadata Override
- **ID:** 1vxyMtAbK
- **Status:** done

#### Summary
Add optional story-level `blocked_by` metadata so planners can explicitly encode parallel constraints regardless of inferred semantic safety.

#### Acceptance Criteria
- [x] [SRS-05/AC-01] Story frontmatter accepts optional `blocked_by` list of story IDs. <!-- verify: cargo test --lib next_parallel_blocked_by_frontmatter_parses, SRS-05:start:end, proof: ac-1.log-->
- [x] [SRS-05/AC-02] `blocked_by` overrides inferred allow decisions and forces pairwise blocking in `next --parallel`. <!-- verify: cargo test --lib next_parallel_blocked_by_override_enforced, SRS-05:start:end, proof: ac-2.log-->

#### Implementation Insights
- **L001: Frontmatter Field Additions Need Builder + Literal Sweep**
  - Insight: `#[serde(default)]` handles runtime parsing, but compile-time struct literals and test builders still require explicit wiring or defaults to avoid breakage and hidden drift in fixture generation.
  - Suggested Action: When adding frontmatter fields, immediately update `TestStory`, `StoryFactory`, and all explicit `StoryFrontmatter { ... }` literals in one slice before running broader checks.
  - Applies To: `src/domain/model/story.rs`, `src/test_helpers.rs`, read-model fixture tests
  - Category: code


#### Verified Evidence
- [ac-1.log](../../../../stories/1vxyMtAbK/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vxyMtAbK/EVIDENCE/ac-2.log)

### Command And Projection Tests For Parallel Safety
- **ID:** 1vxyMtVpK
- **Status:** done

#### Summary
Add command-level and projection-level contract tests to keep human/JSON parallel outputs synchronized and deterministic.

#### Acceptance Criteria
- [x] [SRS-06/AC-02] Parallel recommendation order and candidate selection are deterministic across repeated runs for the same state. <!-- verify: cargo test --lib next_parallel_output_is_deterministic, SRS-06:start:end, proof: ac-1.log-->
- [x] [SRS-06/AC-03] Human and JSON projections expose consistent pairwise blocker semantics for selected and blocked candidates. <!-- verify: cargo test --lib next_parallel_pairwise_blockers_render_consistently, SRS-06:start:end, proof: ac-2.log-->

#### Implementation Insights
- **L001: Deterministic Projection Requires Ordered Containers End-To-End**
  - Insight: Stable candidate sorting is not enough; projection containers must also preserve ordering or serialized output can still drift across runs.
  - Suggested Action: Use ordered maps (`BTreeMap`) for projection payloads and shared projection helpers for all render paths.
  - Applies To: `src/cli/commands/management/next.rs` and other CLI JSON projection builders
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vxyMtVpK/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vxyMtVpK/EVIDENCE/ac-2.log)

### Doctor Check For Parallel Conflict Coherence
- **ID:** 1vxyMtaKP
- **Status:** done

#### Summary
Add doctor checks that validate explicit and inferred parallel conflict signals for coherence and actionable remediation.

#### Acceptance Criteria
- [x] [SRS-07/AC-01] Doctor reports invalid `blocked_by` references and contradictory pair constraints as errors. <!-- verify: cargo test --lib doctor_parallel_conflict_coherence_checks, SRS-07:start:end, proof: ac-1.log-->
- [x] [SRS-07/AC-02] Doctor output includes specific story pairs and remediation guidance. <!-- verify: cargo test --lib doctor_parallel_conflict_reports_actionable_pairs, SRS-07:start:end, proof: ac-2.log-->

#### Implementation Insights
- **L001: Coherence Checks Need Canonical Pair Normalization**
  - Insight: Pair-level diagnostics become deterministic and deduplicated only when pair IDs are normalized (`min/max`) before reporting.
  - Suggested Action: Always canonicalize relationship IDs before emitting pair-based doctor findings.
  - Applies To: `src/cli/commands/diagnostics/doctor/checks/stories.rs` and similar relationship validators
  - Category: code


#### Verified Evidence
- [ac-1.log](../../../../stories/1vxyMtaKP/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vxyMtaKP/EVIDENCE/ac-2.log)


