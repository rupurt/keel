---
created_at: 2026-03-03T20:07:26
---

# Reflection - Add Canonical Guidance To Verify And Audit Commands

## Knowledge

<!--
Capture only novel/actionable knowledge that is likely useful in future work.
If there is no reusable insight for this story, leave the Knowledge section empty.
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

## Observations

Shared guidance helpers reduced drift risk between `verify` and `story audit` by deriving both human text and JSON payload assertions from one canonical mapping. The main difficulty was preserving existing CLI semantics while still emitting deterministic recovery steps for non-fatal verification failures.
