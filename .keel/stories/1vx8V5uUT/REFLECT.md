# Reflection - Relocate Cli Command Surface Into Src Cli

## Knowledge

<!--
Institutional memory is mandatory. Capture what you learned during implementation.
Format:
### L001: Title
| Field | Value |
|-------|-------|
| **Category** | code/testing/process/architecture |
| **Context** | describe when this applies |
| **Insight** | the fundamental discovery |
| **Suggested Action** | what to do next time |
| **Applies To** | file patterns or components |
| **Observed At** | RFC3339 timestamp (e.g. 2026-02-22T12:00:00Z) |
| **Score** | 0.0-1.0 (impact significance) |
| **Confidence** | 0.0-1.0 (insight quality) |
| **Applied** | |
-->

### L001: Path-Wide Module Moves Need Import Rewrite First

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Relocating top-level module families (`commands`, `flow`, `next`) to a new root (`cli`) while preserving behavior. |
| **Insight** | Bulk file moves are low-risk only when import rewrites and architecture path fixtures are updated in the same slice; otherwise compile passes but contract tests drift. |
| **Suggested Action** | For physical normalization stories, perform move + import rewrite + fixture path updates atomically before running full test and doctor checks. |
| **Applies To** | src/main.rs, src/cli/**, src/architecture_contract_tests.rs |
| **Observed At** | 2026-03-02T19:15:47Z |
| **Score** | 0.84 |
| **Confidence** | 0.92 |
| **Applied** | |

## Observations

The migration stayed stable because the work was sliced by layer and verified with
an explicit failing architecture test before moving files. The key friction point
was doctor metadata coherence: acceptance criteria needed verification annotations
and the reopened epic status needed normalization (`strategic` -> `tactical`) to
clear board health checks.
