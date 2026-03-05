# Reflection - Parallel Queue Selection With Confidence Threshold

## Knowledge

### L001: Greedy Threshold Gate Gives Deterministic Safe Subset
| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Selecting parallel-ready stories from pairwise confidence scores |
| **Insight** | Sorting candidates by canonical work-item comparator before threshold filtering yields deterministic, reproducible safe subsets |
| **Suggested Action** | Keep canonical ID ordering and missing-pair fallback confidence (`0.0`) as hard invariants in gate logic |
| **Applies To** | `src/cli/commands/management/next_support/parallel_threshold.rs` |
| **Observed At** | 2026-03-05T02:47:59Z |
| **Score** | 0.82 |
| **Confidence** | 0.91 |
| **Applied** | yes |

## Observations

- The threshold gate integrated cleanly by reusing the pairwise scores produced in the previous story.
- A small type-inference issue in the selection closure required explicit `Vec<&Story>` typing for `selected`.
- The acceptance test captures both threshold semantics and conservative unknown-context behavior in one deterministic integration flow.
