---
created_at: 2026-03-02T07:36:50
---

# Knowledge - 1vwq9Pj97

> Automated synthesis of story reflections.

## Story Knowledge

## Story: Enforce Architecture Contract Tests (1vwqCeg0n)

### 1vyDuw41L: Source-Scanning Contracts Catch Layer Drift Early

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Enforcing adapter and read-model boundaries in a mixed legacy/refactor codebase where imports can regress silently. |
| **Insight** | Lightweight file-content contract tests are effective guardrails for forbidden dependency edges and can be validated in normal unit-test flow. |
| **Suggested Action** | Add/adjust architecture contract tests whenever a boundary or layering rule is introduced so regressions fail fast in CI. |
| **Applies To** | src/architecture_contract_tests.rs, src/main.rs, src/next/algorithm.rs, src/commands/diagnostics/*, src/read_model/* |
| **Applied** | Added enforceable boundary tests and wired them into test compilation via main test module declarations. |



---

## Story: Define Layer Dependency Matrix (1vwqCd6wg)

### 1vyDuw12T: Matrix Contracts Need Both Narrative and Table Forms

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Defining a dependency contract that engineers can read quickly and tests can consume with minimal interpretation |
| **Insight** | A single layer table is not enough for review; adding a compact `From \\ To` matrix reduces ambiguity about allowed and forbidden dependencies. |
| **Suggested Action** | Keep layer contracts in two forms: descriptive per-layer rules plus a normalized matrix that can be translated directly into architecture tests. |
| **Applies To** | ARCHITECTURE.md, upcoming architecture contract tests |
| **Applied** | yes |



---

## Story: Map Bounded Context Ownership (1vwqCdXMs)

### 1vyDuwbHB: Context Maps Need Enforceable Ownership

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Defining DDD boundaries for migration planning before code extraction starts |
| **Insight** | A useful context map must include both owned module paths and forbidden coupling rules, not just conceptual labels. |
| **Suggested Action** | For each new context, document ownership, allowed seams, and forbidden dependencies in one table before implementation stories begin. |
| **Applies To** | ARCHITECTURE.md, src/commands/**, src/model/**, src/state_machine/**, src/flow/** |
| **Applied** | yes |



---

## Synthesis

### mzqFfdlrU: Source-Scanning Contracts Catch Layer Drift Early

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Enforcing adapter and read-model boundaries in a mixed legacy/refactor codebase where imports can regress silently. |
| **Insight** | Lightweight file-content contract tests are effective guardrails for forbidden dependency edges and can be validated in normal unit-test flow. |
| **Suggested Action** | Add/adjust architecture contract tests whenever a boundary or layering rule is introduced so regressions fail fast in CI. |
| **Applies To** | src/architecture_contract_tests.rs, src/main.rs, src/next/algorithm.rs, src/commands/diagnostics/*, src/read_model/* |
| **Linked Knowledge IDs** | 1vyDuw41L |
| **Score** | 0.80 |
| **Confidence** | 0.90 |
| **Applied** | Added enforceable boundary tests and wired them into test compilation via main test module declarations. |

### 1jCvlheVL: Matrix Contracts Need Both Narrative and Table Forms

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Defining a dependency contract that engineers can read quickly and tests can consume with minimal interpretation |
| **Insight** | A single layer table is not enough for review; adding a compact `From \\ To` matrix reduces ambiguity about allowed and forbidden dependencies. |
| **Suggested Action** | Keep layer contracts in two forms: descriptive per-layer rules plus a normalized matrix that can be translated directly into architecture tests. |
| **Applies To** | ARCHITECTURE.md, upcoming architecture contract tests |
| **Linked Knowledge IDs** | 1vyDuw12T |
| **Score** | 0.84 |
| **Confidence** | 0.89 |
| **Applied** | yes |

### Xl3pnhZXo: Context Maps Need Enforceable Ownership

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Defining DDD boundaries for migration planning before code extraction starts |
| **Insight** | A useful context map must include both owned module paths and forbidden coupling rules, not just conceptual labels. |
| **Suggested Action** | For each new context, document ownership, allowed seams, and forbidden dependencies in one table before implementation stories begin. |
| **Applies To** | ARCHITECTURE.md, src/commands/**, src/model/**, src/state_machine/**, src/flow/** |
| **Linked Knowledge IDs** | 1vyDuwbHB |
| **Score** | 0.86 |
| **Confidence** | 0.90 |
| **Applied** | yes |

