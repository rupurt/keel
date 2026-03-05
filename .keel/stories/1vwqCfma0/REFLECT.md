---
created_at: 2026-03-02T10:34:57
---

# Reflection - Remove Duplicate Projection Implementations

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

### 1vyDuwCgL: Interface Adapters Should Delegate Instead Of Recompute

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | When both flow rendering and diagnostics commands need the same projection outputs |
| **Insight** | Duplicated adapter-level projection/load/render paths drift quickly and should be collapsed behind a single interface that consumes canonical read-model DTOs |
| **Suggested Action** | Keep one shared capacity interface and enforce delegation from command modules through architecture contracts |
| **Applies To** | `src/commands/diagnostics/capacity.rs`, `src/flow/capacity.rs`, `src/architecture_contract_tests.rs` |
| **Observed At** | 2026-03-02T10:35:00-08:00 |
| **Score** | 0.83 |
| **Confidence** | 0.90 |
| **Applied** | Delegated diagnostics capacity command to `flow::capacity` and added explicit contract test for shared interface usage |

## Observations

The refactor stayed low-risk because a contract test was added first, making the target architecture explicit. The main correction afterward was updating older contract assumptions and removing dead-code wrappers to satisfy strict linting.
