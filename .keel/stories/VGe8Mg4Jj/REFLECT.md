---
created_at: 2026-04-12T23:07:30
---

# Reflection - Enforce Mission Stack Diagnostics And Foreign Worktree Guards

## Knowledge

<!--
Link existing knowledge files when the insight already exists:
- [123abcDEF](../../knowledge/123abcDEF.md) Existing knowledge title

Capture only novel/actionable knowledge that is likely useful in future work as
an inline candidate block. Unique entries are promoted into `.keel/knowledge/<id>.md`
on submit/accept.

If there is no reusable insight for this story, leave the Knowledge section empty.
Format:
### VGeGeTEKr: Title
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

Putting Mission Stack enforcement into dedicated workflow doctor checks kept the
protocol violations legible and avoided coupling them to story- or pacemaker-
specific diagnostics. The most important test detail was exercising real git
worktree state instead of mocking it, because the closeout and fail-safe
behaviors depend on how `git worktree list` and checkout roots resolve in
practice.
