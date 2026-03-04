# Reflection - Add Canonical Guidance To Voyage Lifecycle Commands

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

Keeping voyage guidance mapping strictly tied to resulting lifecycle state (`planned -> start`, `in-progress -> done`) made command suggestions defensible against transition guards.
Applying the existing shared canonical renderer (`CommandGuidance -> CanonicalGuidance`) prevented duplicate payload-shape logic and reduced drift risk.
Small, state-focused tests gave clear protection for both human next-step text and machine-consumable JSON contract shape.
