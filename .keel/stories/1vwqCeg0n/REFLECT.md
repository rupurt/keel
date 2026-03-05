---
created_at: 2026-03-02T07:36:50
---

# Reflection - Enforce Architecture Contract Tests

## Knowledge

### 1vyDuw41L: Source-Scanning Contracts Catch Layer Drift Early

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Enforcing adapter and read-model boundaries in a mixed legacy/refactor codebase where imports can regress silently. |
| **Insight** | Lightweight file-content contract tests are effective guardrails for forbidden dependency edges and can be validated in normal unit-test flow. |
| **Suggested Action** | Add/adjust architecture contract tests whenever a boundary or layering rule is introduced so regressions fail fast in CI. |
| **Applies To** | src/architecture_contract_tests.rs, src/main.rs, src/next/algorithm.rs, src/commands/diagnostics/*, src/read_model/* |
| **Observed At** | 2026-03-02T15:28:07Z |
| **Score** | 0.8 |
| **Confidence** | 0.9 |
| **Applied** | Added enforceable boundary tests and wired them into test compilation via main test module declarations. |

## Observations

The key implementation detail was balancing strictness with false positives by asserting explicit forbidden edges and expected projection call sites.
A fixture-based detector test helped prove the contract helper fails correctly when patterns are introduced.
Running the full suite after adding these checks verified that existing architecture already conforms to the new boundary expectations.
