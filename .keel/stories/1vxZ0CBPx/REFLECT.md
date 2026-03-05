---
created_at: 2026-03-03T19:17:59
---

# Reflection - Add Canonical Recovery Guidance To Story Lifecycle Errors

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

Mapping recovery commands by stable enforcement message fragments kept the adapter changes small while still covering the most common blocked transitions and precondition failures.
Routing all lifecycle adapters through one `error_with_recovery` helper reduced duplicate formatting logic and made recovery behavior testable in one place.
Explicit command-level regression tests (for `start`, `submit`, `accept`, `reject`, `ice`, `thaw`, and `reflect`) caught integration gaps that pure guidance-unit tests would not.
