# Reflection - Semantic Conflict Feature Extraction

## Knowledge

### 1vyDuw9iN: Work Item Comparator Is Not Lexical
| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Building deterministic pairwise vectors for story IDs with numeric suffixes |
| **Insight** | `compare_work_item_ids` can order IDs differently from naive lexical sorting (for example `S10` before `S2`) |
| **Suggested Action** | Use `compare_work_item_ids` for all deterministic work-item ordering and avoid hard-coded lexical expectations in tests |
| **Applies To** | `src/cli/commands/management/next_support/*` |
| **Observed At** | 2026-03-05T02:40:28Z |
| **Score** | 0.86 |
| **Confidence** | 0.93 |
| **Applied** | yes |

## Observations

- The feature extractor was straightforward to isolate as a dedicated module, which made deterministic behavior easy to validate with focused tests.
- The main friction point was CLI evidence recording: multi-word `--msg` values were split by `just keel` argument interpolation, so proof messages had to be recorded as single-token strings.
- Wiring extraction into `next --parallel` now gives downstream stories a stable pairwise signal contract to build conservative scoring and blocker rendering on top of.
