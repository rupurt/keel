# Reflection - Doctor Check For Parallel Conflict Coherence

## Knowledge

### L001: Coherence Checks Need Canonical Pair Normalization
| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Detecting reciprocal `blocked_by` contradictions in doctor checks |
| **Insight** | Pair-level diagnostics become deterministic and deduplicated only when pair IDs are normalized (`min/max`) before reporting. |
| **Suggested Action** | Always canonicalize relationship IDs before emitting pair-based doctor findings. |
| **Applies To** | `src/cli/commands/diagnostics/doctor/checks/stories.rs` and similar relationship validators |
| **Observed At** | 2026-03-05T03:05:21Z |
| **Score** | 0.84 |
| **Confidence** | 0.95 |
| **Applied** | yes |

## Observations

- Integrating the new check directly into `doctor::validate` kept reporting behavior consistent with existing story-check sections.
- Targeted tests are stronger when one test validates detection semantics and another validates report-level phrasing in final doctor output.
- Using explicit remediation language in each message made AC-02 straightforward to verify and keeps the output actionable for planners.
