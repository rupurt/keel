# Reflection - Add Canonical Next Guidance To Story Lifecycle Commands

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

Guidance mapping stayed stable once the lifecycle outcome was reduced to one canonical state-to-command table and shared across start/reflect/record/submit/accept adapters.
The main implementation risk was suggesting commands that are invalid for the resulting state, so explicit state-based tests were more effective than per-command snapshot assertions.
Keeping CLI surface unchanged while still proving JSON contract behavior through serialization tests avoided introducing extra interface churn mid-epic.
