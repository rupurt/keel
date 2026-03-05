# Reflection - Conservative Pairwise Conflict Scoring

## Knowledge

### L001: Unknown Context Should Force Risk Floor
| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Pairwise scoring for partial architectural metadata in `next --parallel` |
| **Insight** | Unresolved semantic context is easiest to keep safe when scoring applies an explicit risk floor and confidence ceiling instead of only additive penalties |
| **Suggested Action** | Keep conservative fallback thresholds as first-class scoring invariants and assert them directly in tests |
| **Applies To** | `src/cli/commands/management/next_support/parallel_*.rs` |
| **Observed At** | 2026-03-05T02:45:20Z |
| **Score** | 0.84 |
| **Confidence** | 0.92 |
| **Applied** | yes |

## Observations

- The scoring module fit cleanly as a separate layer after feature extraction, which made the two AC tests direct and deterministic.
- Running `just quality` before full test runs remains important; rustfmt surfaced a compact-line formatting mismatch immediately.
- Pairwise ordering stayed stable by reusing `compare_work_item_ids` for score sorting, preventing ID-order regressions in tests.
