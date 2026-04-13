---
created_at: 2026-04-12T22:56:37
---

# Reflection - Surface Mission Stack In Turn Next And Mission Status

## Knowledge

<!--
Link existing knowledge files when the insight already exists:
- [123abcDEF](../../knowledge/123abcDEF.md) Existing knowledge title

Capture only novel/actionable knowledge that is likely useful in future work as
an inline candidate block. Unique entries are promoted into `.keel/knowledge/<id>.md`
on submit/accept.

If there is no reusable insight for this story, leave the Knowledge section empty.
Format:
### VGeDuXiz0: Title
| Field | Value |
|-------|-------|
| **Category** | code/testing/process/architecture |
| **Context** | describe when this applies |
| **Insight** | the fundamental discovery |
| **Suggested Action** | what to do next time |
| **Applies To** | file patterns or components |
| **Linked Knowledge IDs** | optional canonical IDs this insight builds on |
| **Observed At** | RFC3339 timestamp (e.g. 2026-02-22T12:00:00Z) |
| **Score** | 0.0-1.0 (impact significance) |
| **Confidence** | 0.0-1.0 (insight quality) |
| **Applied** | |
-->

## Observations

Threading one Mission Stack execution gate through the read model kept the
surfaces aligned: `turn`, `next`, and `mission next` now all consume the same
stack projection instead of encoding separate heuristics. The main friction was
the existing command fanout around `NextDecision`; adding stack-aware yield and
block variants touched formatter, JSON, guidance, and mission-status helpers in
several places.
