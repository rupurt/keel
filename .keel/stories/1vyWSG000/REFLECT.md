---
created_at: 2026-03-06T08:58:47
---

# Reflection - Wire Provider Agnostic Llm Judge Execution

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
### 1vyYVj000: Title
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

Switching from an in-crate stub to an external `llm-judge <bundle-path>` contract kept the change narrow: bundle construction stayed in keel while provider behavior moved completely behind a mockable process boundary. The useful test pattern was to prepend a temporary `llm-judge` wrapper onto `PATH`, which verifies the real contract without introducing a vendor client or transport dependency.
