# PRESS RELEASE: Consolidate Read Models And Queue Policies

## Overview

## Narrative Summary
### Build Canonical Capacity Projection
Implement canonical capacity projection types and calculation service.

### Unify Queue Policy Consumption
Unify queue policy access across all consumers.

### Build Canonical Flow Status Projection
Implement canonical flow and status projection services for operational views.

### Remove Duplicate Projection Implementations
Remove duplicate metric and projection implementations from interface modules.

## Key Insights
### Insights from Build Canonical Capacity Projection
# Reflection - Build Canonical Capacity Projection

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

### L001: Canonical read models remove adapter drift

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | When both flow rendering and diagnostics need the same capacity semantics |
| **Insight** | Duplicated DTOs and charge enums across adapters force conversion shims and create drift risk in UI logic. |
| **Suggested Action** | Keep one projection type in `read_model` and make interface modules thin adapters over that projection. |
| **Applies To** | `src/read_model/capacity.rs`, `src/flow/capacity.rs`, `src/commands/diagnostics/capacity.rs`, `src/flow/display.rs` |
| **Observed At** | 2026-03-02T16:14:00Z |
| **Score** | 0.89 |
| **Confidence** | 0.96 |
| **Applied** | story `1vwqCfHz7` |

## Observations

Projection centralization worked cleanly once `flow/display` consumed the canonical type directly instead of translating between nearly identical structs.
The main friction was shell argument parsing for `story record --msg`; using a shell-safe token avoided that and preserved evidence capture workflow.

### Insights from Unify Queue Policy Consumption
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

### L001: Queue-policy facades prevent decision/rendering drift

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

### Insights from Build Canonical Flow Status Projection
# Reflection - Build Canonical Flow Status Projection

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

### L001: Keep Operational Metrics In A Single Read Model

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Consolidating repeated queue/flow/status metrics used across diagnostics and next-decision logic |
| **Insight** | A canonical projection DTO that embeds both flow metrics and status metrics removes drift and lets adapters format output without recalculating business metrics |
| **Suggested Action** | Add read-model projection services first, then migrate every consumer to the projection API before deleting local metric structs |
| **Applies To** | src/read_model/flow_status.rs; src/commands/diagnostics/{flow,status}.rs; src/next/algorithm.rs |
| **Observed At** | 2026-03-02T03:22:27Z |
| **Score** | 0.84 |
| **Confidence** | 0.89 |
| **Applied** | 1vwqCfS0F |

## Observations

The migration was low risk because existing flow metrics could be reused directly and wrapped with status-facing DTOs. The primary guardrail was ensuring all three consumers (`flow`, `status`, `next`) imported the same projection entrypoint instead of recreating counts locally.

### Insights from Remove Duplicate Projection Implementations
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

### L001: Interface Adapters Should Delegate Instead Of Recompute

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

## Verification Proof
### Proof for Build Canonical Capacity Projection
- [ac-1.log](../../../../stories/1vwqCfHz7/EVIDENCE/ac-1.log)

### Proof for Unify Queue Policy Consumption
- [ac-1.log](../../../../stories/1vwqCfgC4/EVIDENCE/ac-1.log)

### Proof for Build Canonical Flow Status Projection
- [ac-1.log](../../../../stories/1vwqCfS0F/EVIDENCE/ac-1.log)

### Proof for Remove Duplicate Projection Implementations
- [ac-1.log](../../../../stories/1vwqCfma0/EVIDENCE/ac-1.log)

