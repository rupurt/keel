# Reflection - Publish Migration Completion Checklist

## Knowledge

<!--
Institutional memory is mandatory. Capture what you learned during implementation.
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

### L001: Rollout Docs Need Explicit Gate Ownership

| Field | Value |
|-------|-------|
| **Category** | process |
| **Context** | Finalizing migration voyages where multiple maintainers coordinate acceptance and release sequencing |
| **Insight** | Checklist quality improves when each gate and rollout step is phrased as an explicit maintainer action with clear command references |
| **Suggested Action** | Keep voyage-local migration checklists with completion criteria, gate commands, rollout order, and deferred-item tracking |
| **Applies To** | `.keel/epics/*/voyages/*/MIGRATION_CHECKLIST.md`, voyage `README.md` document tables |
| **Observed At** | 2026-03-02T10:41:30-08:00 |
| **Score** | 0.78 |
| **Confidence** | 0.88 |
| **Applied** | Added `MIGRATION_CHECKLIST.md` for `1vwq9wpT7` and linked it from voyage documents |

## Observations

The checklist was straightforward to land because voyage docs already had a clear location and document table. Including the post-epic deferred item in the same checklist kept scope explicit without mixing it into current epic execution.
