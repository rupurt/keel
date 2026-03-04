# Reflection - Define Canonical Guidance Output Contract

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

The guidance contract landed cleanly once it was separated into a shared type (`CanonicalGuidance` and `GuidanceStep`) instead of embedding ad-hoc structures inside `next` command formatting. This reduced coupling and made it straightforward to add serialization regression tests for `next_step` and `recovery_step` behavior.

The largest source of drift risk was not JSON serialization itself but decision-to-guidance mapping consistency across code paths (`run` and `run_parallel`). Centralizing mapping logic into helper functions prevented subtle divergence and made it easy to assert parity in focused tests.

One workflow gap discovered during implementation was that voyage planning could thaw scaffold stories into backlog when acceptance criteria sections existed but content remained placeholder-quality. This led to follow-up gating and doctor hard checks to reject unresolved scaffold/default text before stories become actionable.
