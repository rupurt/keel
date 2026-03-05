---
created_at: 2026-03-03T18:30:08
---

# Reflection - Add Canonical Guidance To ADR Transition Commands

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

Centralizing ADR transition guidance logic in one helper module made it straightforward to keep next/recovery behavior consistent across accept/reject/deprecate/supersede.
Attaching recovery guidance directly to transition validation failures produced deterministic remediation commands without changing CLI surface area.
Contract-style tests around rendered human blocks and canonical JSON payload shape were more stable than command-output snapshot tests tied to board state.
