---
created_at: 2026-03-03T19:50:46
---

# Reflection - Align Next Command Guidance In Human And Json Output

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

Rendering human guidance from the same canonical `guidance_for_decision` payload eliminated command drift without changing decision selection logic.
Keeping formatter modules descriptive and moving imperative command emission into one shared guidance renderer made parity tests straightforward and reliable.
Parallel mode needed the same treatment as single-decision mode to keep guidance behavior consistent across all `keel next` output paths.
