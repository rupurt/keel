---
created_at: 2026-03-04T19:02:37
---

# Reflection - Command And Projection Tests For Parallel Safety

## Knowledge

### 1vyDuwMlz: Deterministic Projection Requires Ordered Containers End-To-End
| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Rendering `next --parallel` output in both human and JSON projections |
| **Insight** | Stable candidate sorting is not enough; projection containers must also preserve ordering or serialized output can still drift across runs. |
| **Suggested Action** | Use ordered maps (`BTreeMap`) for projection payloads and shared projection helpers for all render paths. |
| **Applies To** | `src/cli/commands/management/next.rs` and other CLI JSON projection builders |
| **Observed At** | 2026-03-05T03:01:50Z |
| **Score** | 0.88 |
| **Confidence** | 0.94 |
| **Applied** | yes |

## Observations

- Extracting a shared `project_parallel_work` function reduced drift between command behavior and test assertions.
- The first blocker-consistency test failed because scoped stories without matching voyage metadata are not considered workable; fixture realism matters for command-level tests.
- AC-named tests now directly verify deterministic output bytes and human/JSON blocker parity from the same projection source.
