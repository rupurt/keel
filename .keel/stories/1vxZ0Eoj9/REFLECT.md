---
created_at: 2026-03-03T20:14:53
---

# Reflection - Add Canonical Guidance To Play Command Outcomes

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

Routing guidance through a dedicated play guidance helper made it easy to keep deterministic `--suggest` behavior prescriptive while leaving exploratory paths intentionally silent. The key tradeoff was preserving existing play output text and command flow while adding canonical next-step emission only where follow-up is unambiguous.
