# PRESS RELEASE: Semantic Conflict Detection For Parallel Next

## Overview

## Narrative Summary
### Pairwise Blocker Rendering For Parallel Next
Render pairwise blocker explanations so operators can see exactly which story pairs are blocked and why.

### Parallel Queue Selection With Confidence Threshold
Integrate confidence thresholding into parallel selection so only high-confidence low-conflict candidates are surfaced as actionable work.

### Story Blocked By Metadata Override
Add optional story-level `blocked_by` metadata so planners can explicitly encode parallel constraints regardless of inferred semantic safety.

### Doctor Check For Parallel Conflict Coherence
Add doctor checks that validate explicit and inferred parallel conflict signals for coherence and actionable remediation.

### Command And Projection Tests For Parallel Safety
Add command-level and projection-level contract tests to keep human/JSON parallel outputs synchronized and deterministic.

### Semantic Conflict Feature Extraction
Implement deterministic semantic feature extraction for candidate story pairs so `keel next --parallel` can reason about difficult-to-resolve merge conflicts.

### Conservative Pairwise Conflict Scoring
Implement conservative scoring that transforms semantic feature vectors into pairwise conflict risk and confidence, biasing toward blocked outcomes when confidence is low.

## Key Insights
### Insights from Pairwise Blocker Rendering For Parallel Next
- **L001: Keep Blocker Schema Shared Across Human and JSON Paths**
  - Insight: A single blocker model (`story_id`, `blocked_by_story_id`, `reasons`, `confidence`) makes it easy to keep human and JSON outputs in sync
  - Suggested Action: Build future blocker explanations from the same canonical blocker payload and only vary presentation


### Insights from Parallel Queue Selection With Confidence Threshold
- **L001: Greedy Threshold Gate Gives Deterministic Safe Subset**
  - Insight: Sorting candidates by canonical work-item comparator before threshold filtering yields deterministic, reproducible safe subsets
  - Suggested Action: Keep canonical ID ordering and missing-pair fallback confidence (`0.0`) as hard invariants in gate logic


### Insights from Story Blocked By Metadata Override
- **L001: Frontmatter Field Additions Need Builder + Literal Sweep**
  - Insight: `#[serde(default)]` handles runtime parsing, but compile-time struct literals and test builders still require explicit wiring or defaults to avoid breakage and hidden drift in fixture generation.
  - Suggested Action: When adding frontmatter fields, immediately update `TestStory`, `StoryFactory`, and all explicit `StoryFrontmatter { ... }` literals in one slice before running broader checks.


### Insights from Doctor Check For Parallel Conflict Coherence
- **L001: Coherence Checks Need Canonical Pair Normalization**
  - Insight: Pair-level diagnostics become deterministic and deduplicated only when pair IDs are normalized (`min/max`) before reporting.
  - Suggested Action: Always canonicalize relationship IDs before emitting pair-based doctor findings.


### Insights from Command And Projection Tests For Parallel Safety
- **L001: Deterministic Projection Requires Ordered Containers End-To-End**
  - Insight: Stable candidate sorting is not enough; projection containers must also preserve ordering or serialized output can still drift across runs.
  - Suggested Action: Use ordered maps (`BTreeMap`) for projection payloads and shared projection helpers for all render paths.


### Insights from Semantic Conflict Feature Extraction
- **L001: Work Item Comparator Is Not Lexical**
  - Insight: `compare_work_item_ids` can order IDs differently from naive lexical sorting (for example `S10` before `S2`)
  - Suggested Action: Use `compare_work_item_ids` for all deterministic work-item ordering and avoid hard-coded lexical expectations in tests


### Insights from Conservative Pairwise Conflict Scoring
- **L001: Unknown Context Should Force Risk Floor**
  - Insight: Unresolved semantic context is easiest to keep safe when scoring applies an explicit risk floor and confidence ceiling instead of only additive penalties
  - Suggested Action: Keep conservative fallback thresholds as first-class scoring invariants and assert them directly in tests


## Verification Proof
### Proof for Pairwise Blocker Rendering For Parallel Next
- [ac-1.log](../../../../stories/1vxyMsbOj/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vxyMsbOj/EVIDENCE/ac-2.log)

### Proof for Parallel Queue Selection With Confidence Threshold
- [ac-1.log](../../../../stories/1vxyMsvug/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vxyMsvug/EVIDENCE/ac-2.log)

### Proof for Story Blocked By Metadata Override
- [ac-1.log](../../../../stories/1vxyMtAbK/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vxyMtAbK/EVIDENCE/ac-2.log)

### Proof for Doctor Check For Parallel Conflict Coherence
- [ac-1.log](../../../../stories/1vxyMtaKP/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vxyMtaKP/EVIDENCE/ac-2.log)

### Proof for Command And Projection Tests For Parallel Safety
- [ac-1.log](../../../../stories/1vxyMtVpK/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vxyMtVpK/EVIDENCE/ac-2.log)

### Proof for Semantic Conflict Feature Extraction
- [ac-1.log](../../../../stories/1vxyMr3U2/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vxyMr3U2/EVIDENCE/ac-2.log)

### Proof for Conservative Pairwise Conflict Scoring
- [ac-1.log](../../../../stories/1vxyMsepz/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vxyMsepz/EVIDENCE/ac-2.log)

