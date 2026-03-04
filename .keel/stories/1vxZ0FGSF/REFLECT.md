# Reflection - Add Drift Tests For Canonical Guidance Contracts

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

Coverage quality improved once drift tests were framed as explicit actionable vs informational guidance matrices instead of single-command unit checks. Exposing ADR and bearing guidance modules at `pub(crate)` made cross-command contract checks possible without changing CLI surface behavior.
