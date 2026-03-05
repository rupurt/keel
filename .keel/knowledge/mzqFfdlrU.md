---
source_type: Voyage
source: epics/1vwq96cpt/voyages/1vwq9Pj97/KNOWLEDGE.md
created_at: 2026-03-02T07:36:50
---

### mzqFfdlrU: Source-Scanning Contracts Catch Layer Drift Early

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Enforcing adapter and read-model boundaries in a mixed legacy/refactor codebase where imports can regress silently. |
| **Insight** | Lightweight file-content contract tests are effective guardrails for forbidden dependency edges and can be validated in normal unit-test flow. |
| **Suggested Action** | Add/adjust architecture contract tests whenever a boundary or layering rule is introduced so regressions fail fast in CI. |
| **Applies To** | src/architecture_contract_tests.rs, src/main.rs, src/next/algorithm.rs, src/commands/diagnostics/*, src/read_model/* |
| **Linked Knowledge IDs** | 1vyDuw41L |
| **Observed At** |  |
| **Score** | 0.80 |
| **Confidence** | 0.90 |
| **Applied** | Added enforceable boundary tests and wired them into test compilation via main test module declarations. |
