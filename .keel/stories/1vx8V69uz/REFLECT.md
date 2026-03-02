# Reflection - Relocate Domain Core Modules Into Src Domain

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

### L001: Multi-Requirement Stories Can Create Queue Cycles

| Field | Value |
|-------|-------|
| **Category** | process |
| **Context** | Stories in the same voyage referenced overlapping SRS IDs, and queue dependency derivation blocked all stories from becoming ready. |
| **Insight** | Implementation dependency derivation is SRS-order based; a story that references both early and later requirements can create circular dependencies across siblings. |
| **Suggested Action** | Keep each implementation story mapped to a primary SRS requirement in sequence, and reserve aggregate contract cleanup requirements for the final story. |
| **Applies To** | .keel/stories/*/README.md, src/traceability.rs |
| **Observed At** | 2026-03-02T19:30:00Z |
| **Score** | 0.88 |
| **Confidence** | 0.95 |
| **Applied** | |

## Observations

Domain relocation was straightforward after the prior CLI move pattern: move files,
rewrite imports, then update fixture paths in architecture tests. The non-obvious
issue was queue readiness, which depended on SRS marker topology rather than index
order alone; fixing SRS mappings immediately unblocked `next --agent`.
