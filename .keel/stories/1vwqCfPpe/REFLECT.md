# Reflection - Rewire Command Handlers To Use Cases

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

### 1vyDuwnad: Enforce policy invariants in application services, not command handlers

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Story and voyage lifecycle commands need consistent enforcement of manual-verification acceptance and requirements-coverage gating. |
| **Insight** | Putting enforcement in application services centralizes lifecycle invariants and avoids drift across multiple command entrypoints. |
| **Suggested Action** | Keep command handlers thin and add service-level tests for every lifecycle policy; use architecture contract tests to block direct transition orchestration from handlers. |
| **Applies To** | `src/application/story_lifecycle.rs`, `src/application/voyage_epic_lifecycle.rs`, `src/commands/{story,voyage,epic}/*.rs`, `src/architecture_contract_tests.rs` |
| **Observed At** | 2026-03-02T17:32:48Z |
| **Score** | 0.90 |
| **Confidence** | 0.96 |
| **Applied** | story `1vwqCfPpe` |

## Observations

The service-level tests were straightforward to add and made policy behavior explicit at the boundary where it matters.
The main friction was tooling inconsistency between `just keel` argument handling and direct binary invocation; using shorter evidence text and direct proof files avoided ambiguity.
