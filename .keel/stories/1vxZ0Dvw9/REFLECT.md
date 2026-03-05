---
created_at: 2026-03-03T19:38:17
---

# Reflection - Keep Informational Governance Commands Non Prescriptive

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

Explicitly modeling informational governance commands as `None` guidance contracts made non-prescriptive behavior testable instead of implicit.
Wiring read-only commands through the same guidance render path (with empty guidance) keeps behavior stable while preserving future extensibility for explicit contracts.
Adding JSON omission envelope tests alongside human rendering checks catches both accidental guidance field additions and accidental imperative terminal text.
