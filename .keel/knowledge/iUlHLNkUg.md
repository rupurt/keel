---
source_type: Voyage
source: epics/1vxyM0hvn/voyages/1vxyMT6nz/KNOWLEDGE.md
created_at: 2026-03-04T19:11:59
---

### iUlHLNkUg: Greedy Threshold Gate Gives Deterministic Safe Subset

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Selecting parallel-ready stories from pairwise confidence scores |
| **Insight** | Sorting candidates by canonical work-item comparator before threshold filtering yields deterministic, reproducible safe subsets |
| **Suggested Action** | Keep canonical ID ordering and missing-pair fallback confidence (`0.0`) as hard invariants in gate logic |
| **Applies To** | `src/cli/commands/management/next_support/parallel_threshold.rs` |
| **Linked Knowledge IDs** | 1vyDuwBZS |
| **Observed At** |  |
| **Score** | 0.82 |
| **Confidence** | 0.91 |
| **Applied** | yes |
