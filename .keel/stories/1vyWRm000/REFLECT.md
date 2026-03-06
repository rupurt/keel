---
created_at: 2026-03-06T09:02:41
---

# Reflection - Persist Judge Outputs In Verification Evidence

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
### 1vyYZV000: Title
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

Persisting judge outputs inside `execute_llm_judge` was the right boundary because both `verify run` and `story record --judge` already converge there. Writing a transcript placeholder plus a structured result file on every run, even when the wrapper fails before producing clean stdout, keeps the failure mode inspectable and gives the manifest a deterministic set of evidence files to hash.
