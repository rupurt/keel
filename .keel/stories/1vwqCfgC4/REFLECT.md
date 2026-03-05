---
created_at: 2026-03-02T09:48:09
---

# Reflection - Unify Queue Policy Consumption

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

### 1vyDuwSPf: Queue-policy facades prevent decision/rendering drift

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Queue classifications were being consumed directly by multiple modules (`next`, `flow/bottleneck`, and `state_machine/flow`) with repeated policy calls. |
| **Insight** | A small read-model facade (`read_model::queue_policy`) creates one consumption surface for policy outputs while keeping source-of-truth thresholds in `policy::queue`. |
| **Suggested Action** | Add architecture contracts for policy-facade usage whenever policy semantics are consumed by multiple adapters or decision paths. |
| **Applies To** | `src/read_model/queue_policy.rs`, `src/next/algorithm.rs`, `src/flow/bottleneck.rs`, `src/state_machine/flow.rs`, `src/architecture_contract_tests.rs` |
| **Observed At** | 2026-03-02T17:46:09Z |
| **Score** | 0.86 |
| **Confidence** | 0.95 |
| **Applied** | story `1vwqCfgC4` |

## Observations

The refactor stayed low risk because behavior-preserving policy functions remained in one domain module and only consumption points changed.
The most effective safety check was making the architecture test fail first, then rewiring call sites until the contract passed.
