# Reflection - Add Contract Tests For Canonical Guidance Fields

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

Contract tests are most resilient when they assert full serialized object shape instead of checking only nested fields.
Using targeted `decision_to_json_` mapping tests makes deterministic command drift visible without relying on broad integration snapshots.
Keeping omitted-guidance behavior explicitly tested (`{}` object shape) prevents accidental key introduction that would break harness consumers.
