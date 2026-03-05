---
created_at: 2026-03-03T20:49:39
---

# Reflection - Document Command Guidance Contract For Harness Consumers

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

The guidance contract was easiest to verify when proof commands were written relative to the board root (`.keel`), not the repository root.
`story record` requires inline `<!-- verify: ... -->` annotations before evidence capture, so documentation stories still need explicit verification commands on each AC.
