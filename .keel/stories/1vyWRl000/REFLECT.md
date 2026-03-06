---
created_at: 2026-03-06T08:55:29
---

# Reflection - Build Artifact Bundle Materialization

## Knowledge

- [1vyYK1g00](../../knowledge/1vyYK1g00.md) Judge Bundles Should Carry References And Hashes

<!--
Link existing knowledge files when the insight already exists:
- [123abcDEF](../../knowledge/123abcDEF.md) Existing knowledge title

Capture only novel/actionable knowledge that is likely useful in future work as
an inline candidate block. Unique entries are promoted into `.keel/knowledge/<id>.md`
on submit/accept.

If there is no reusable insight for this story, leave the Knowledge section empty.
Format:
### 1vyYSX000: Title
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

Materializing the bundle at executor time kept the contract narrow: the judge path now always receives a stable JSON artifact without forcing callers to know how to build it. The main integration wrinkle was that the executor tests needed real story frontmatter because bundle construction loads the canonical board model rather than operating on raw strings.
