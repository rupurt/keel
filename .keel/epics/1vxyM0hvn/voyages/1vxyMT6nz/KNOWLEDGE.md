---
created_at: 2026-03-04T19:11:59
---

# Knowledge - 1vxyMT6nz

> Automated synthesis of story reflections.

## Story Knowledge

## Story: Pairwise Blocker Rendering For Parallel Next (1vxyMsbOj)

### 1vyDuwzyf: Keep Blocker Schema Shared Across Human and JSON Paths

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Rendering pairwise confidence blockers in CLI and machine-readable output |
| **Insight** | A single blocker model (`story_id`, `blocked_by_story_id`, `reasons`, `confidence`) makes it easy to keep human and JSON outputs in sync |
| **Suggested Action** | Build future blocker explanations from the same canonical blocker payload and only vary presentation |
| **Applies To** | `src/cli/commands/management/next.rs`, `src/cli/commands/management/next_support/parallel_threshold.rs` |
| **Applied** | yes |



---

## Story: Conservative Pairwise Conflict Scoring (1vxyMsepz)

### 1vyDuwXCw: Unknown Context Should Force Risk Floor

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Pairwise scoring for partial architectural metadata in `next --parallel` |
| **Insight** | Unresolved semantic context is easiest to keep safe when scoring applies an explicit risk floor and confidence ceiling instead of only additive penalties |
| **Suggested Action** | Keep conservative fallback thresholds as first-class scoring invariants and assert them directly in tests |
| **Applies To** | `src/cli/commands/management/next_support/parallel_*.rs` |
| **Applied** | yes |



---

## Story: Semantic Conflict Feature Extraction (1vxyMr3U2)

### 1vyDuw9iN: Work Item Comparator Is Not Lexical

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Building deterministic pairwise vectors for story IDs with numeric suffixes |
| **Insight** | `compare_work_item_ids` can order IDs differently from naive lexical sorting (for example `S10` before `S2`) |
| **Suggested Action** | Use `compare_work_item_ids` for all deterministic work-item ordering and avoid hard-coded lexical expectations in tests |
| **Applies To** | `src/cli/commands/management/next_support/*` |
| **Applied** | yes |



---

## Story: Story Blocked By Metadata Override (1vxyMtAbK)

### 1vyDuwlIj: Frontmatter Field Additions Need Builder + Literal Sweep

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Adding a new key to `StoryFrontmatter` that is constructed in many tests and read models |
| **Insight** | `#[serde(default)]` handles runtime parsing, but compile-time struct literals and test builders still require explicit wiring or defaults to avoid breakage and hidden drift in fixture generation. |
| **Suggested Action** | When adding frontmatter fields, immediately update `TestStory`, `StoryFactory`, and all explicit `StoryFrontmatter { ... }` literals in one slice before running broader checks. |
| **Applies To** | `src/domain/model/story.rs`, `src/test_helpers.rs`, read-model fixture tests |
| **Applied** | yes |



---

## Story: Doctor Check For Parallel Conflict Coherence (1vxyMtaKP)

### 1vyDuw2wf: Coherence Checks Need Canonical Pair Normalization

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Detecting reciprocal `blocked_by` contradictions in doctor checks |
| **Insight** | Pair-level diagnostics become deterministic and deduplicated only when pair IDs are normalized (`min/max`) before reporting. |
| **Suggested Action** | Always canonicalize relationship IDs before emitting pair-based doctor findings. |
| **Applies To** | `src/cli/commands/diagnostics/doctor/checks/stories.rs` and similar relationship validators |
| **Applied** | yes |



---

## Story: Parallel Queue Selection With Confidence Threshold (1vxyMsvug)

### 1vyDuwBZS: Greedy Threshold Gate Gives Deterministic Safe Subset

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Selecting parallel-ready stories from pairwise confidence scores |
| **Insight** | Sorting candidates by canonical work-item comparator before threshold filtering yields deterministic, reproducible safe subsets |
| **Suggested Action** | Keep canonical ID ordering and missing-pair fallback confidence (`0.0`) as hard invariants in gate logic |
| **Applies To** | `src/cli/commands/management/next_support/parallel_threshold.rs` |
| **Applied** | yes |



---

## Story: Command And Projection Tests For Parallel Safety (1vxyMtVpK)

### 1vyDuwMlz: Deterministic Projection Requires Ordered Containers End-To-End

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Rendering `next --parallel` output in both human and JSON projections |
| **Insight** | Stable candidate sorting is not enough; projection containers must also preserve ordering or serialized output can still drift across runs. |
| **Suggested Action** | Use ordered maps (`BTreeMap`) for projection payloads and shared projection helpers for all render paths. |
| **Applies To** | `src/cli/commands/management/next.rs` and other CLI JSON projection builders |
| **Applied** | yes |



---

## Synthesis

### 4DVX5dewJ: Keep Blocker Schema Shared Across Human and JSON Paths

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Rendering pairwise confidence blockers in CLI and machine-readable output |
| **Insight** | A single blocker model (`story_id`, `blocked_by_story_id`, `reasons`, `confidence`) makes it easy to keep human and JSON outputs in sync |
| **Suggested Action** | Build future blocker explanations from the same canonical blocker payload and only vary presentation |
| **Applies To** | `src/cli/commands/management/next.rs`, `src/cli/commands/management/next_support/parallel_threshold.rs` |
| **Linked Knowledge IDs** | 1vyDuwzyf |
| **Score** | 0.83 |
| **Confidence** | 0.92 |
| **Applied** | yes |

### vjKuUwTsz: Unknown Context Should Force Risk Floor

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Pairwise scoring for partial architectural metadata in `next --parallel` |
| **Insight** | Unresolved semantic context is easiest to keep safe when scoring applies an explicit risk floor and confidence ceiling instead of only additive penalties |
| **Suggested Action** | Keep conservative fallback thresholds as first-class scoring invariants and assert them directly in tests |
| **Applies To** | `src/cli/commands/management/next_support/parallel_*.rs` |
| **Linked Knowledge IDs** | 1vyDuwXCw |
| **Score** | 0.84 |
| **Confidence** | 0.92 |
| **Applied** | yes |

### 0LMiWqrFa: Work Item Comparator Is Not Lexical

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Building deterministic pairwise vectors for story IDs with numeric suffixes |
| **Insight** | `compare_work_item_ids` can order IDs differently from naive lexical sorting (for example `S10` before `S2`) |
| **Suggested Action** | Use `compare_work_item_ids` for all deterministic work-item ordering and avoid hard-coded lexical expectations in tests |
| **Applies To** | `src/cli/commands/management/next_support/*` |
| **Linked Knowledge IDs** | 1vyDuw9iN |
| **Score** | 0.86 |
| **Confidence** | 0.93 |
| **Applied** | yes |

### sTJiMO70u: Frontmatter Field Additions Need Builder + Literal Sweep

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Adding a new key to `StoryFrontmatter` that is constructed in many tests and read models |
| **Insight** | `#[serde(default)]` handles runtime parsing, but compile-time struct literals and test builders still require explicit wiring or defaults to avoid breakage and hidden drift in fixture generation. |
| **Suggested Action** | When adding frontmatter fields, immediately update `TestStory`, `StoryFactory`, and all explicit `StoryFrontmatter { ... }` literals in one slice before running broader checks. |
| **Applies To** | `src/domain/model/story.rs`, `src/test_helpers.rs`, read-model fixture tests |
| **Linked Knowledge IDs** | 1vyDuwlIj |
| **Score** | 0.81 |
| **Confidence** | 0.92 |
| **Applied** | yes |

### d57774eI9: Coherence Checks Need Canonical Pair Normalization

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Detecting reciprocal `blocked_by` contradictions in doctor checks |
| **Insight** | Pair-level diagnostics become deterministic and deduplicated only when pair IDs are normalized (`min/max`) before reporting. |
| **Suggested Action** | Always canonicalize relationship IDs before emitting pair-based doctor findings. |
| **Applies To** | `src/cli/commands/diagnostics/doctor/checks/stories.rs` and similar relationship validators |
| **Linked Knowledge IDs** | 1vyDuw2wf |
| **Score** | 0.84 |
| **Confidence** | 0.95 |
| **Applied** | yes |

### iUlHLNkUg: Greedy Threshold Gate Gives Deterministic Safe Subset

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Selecting parallel-ready stories from pairwise confidence scores |
| **Insight** | Sorting candidates by canonical work-item comparator before threshold filtering yields deterministic, reproducible safe subsets |
| **Suggested Action** | Keep canonical ID ordering and missing-pair fallback confidence (`0.0`) as hard invariants in gate logic |
| **Applies To** | `src/cli/commands/management/next_support/parallel_threshold.rs` |
| **Linked Knowledge IDs** | 1vyDuwBZS |
| **Score** | 0.82 |
| **Confidence** | 0.91 |
| **Applied** | yes |

### EqSN1h8Jj: Deterministic Projection Requires Ordered Containers End-To-End

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Rendering `next --parallel` output in both human and JSON projections |
| **Insight** | Stable candidate sorting is not enough; projection containers must also preserve ordering or serialized output can still drift across runs. |
| **Suggested Action** | Use ordered maps (`BTreeMap`) for projection payloads and shared projection helpers for all render paths. |
| **Applies To** | `src/cli/commands/management/next.rs` and other CLI JSON projection builders |
| **Linked Knowledge IDs** | 1vyDuwMlz |
| **Score** | 0.88 |
| **Confidence** | 0.94 |
| **Applied** | yes |

