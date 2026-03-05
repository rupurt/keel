---
created_at: 2026-03-03T10:53:41
---

# Reflection - Codify Token Bucket Contract Tests

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

### 1vyDuwGh9: Keep token inventories and CLI `new` surfaces coupled by drift tests

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | Token ownership policy spans templates and command interfaces; either side can drift silently without explicit coupling tests. |
| **Insight** | A two-layer contract works best: template bucket tests catch unknown/out-of-bucket tokens while drift tests lock exact `new` command argument sets for ownership boundaries. |
| **Suggested Action** | When adding new tokenized fields, update bucket inventories and expected `new` arg sets in the same change to keep policy deterministic. |
| **Applies To** | src/infrastructure/templates.rs, src/drift_tests.rs, src/cli/command_tree.rs |
| **Observed At** | 2026-03-03T18:50:00Z |
| **Score** | 0.83 |
| **Confidence** | 0.92 |
| **Applied** | |

## Observations

Adding mirrored tests in both template and drift suites made the policy easier to reason about and prevented partial coverage. The main difficulty was defining bucket boundaries clearly enough that generated marker semantics remained explicit while planning templates stayed strict.
