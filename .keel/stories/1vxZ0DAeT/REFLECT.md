# Reflection - Add Canonical Guidance To Bearing Transition Commands

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

Using a dedicated bearing guidance mapper kept transition adapters thin and avoided duplicating command-string and rendering logic in each command.
Treating terminal bearing actions (`park`, `decline`, `lay`) as `keel next --human` next-step guidance provided consistent handoff behavior similar to other governance transitions.
Recovery mapping by stable message classes (`not found`, invalid transition/state, already-exists conflicts) gave deterministic remediation commands without introducing any new output mode.
