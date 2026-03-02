# VOYAGE REPORT: Consolidate Read Models And Queue Policies

## Voyage Metadata
- **ID:** 1vwq9rycE
- **Epic:** 1vwq96cpt
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 4/4 stories complete

## Implementation Narrative
### Build Canonical Capacity Projection
- **ID:** 1vwqCfHz7
- **Status:** done

#### Summary
Implement canonical capacity projection types and calculation service.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] Capacity projection is exposed through a single canonical type used by diagnostics and flow renderers. <!-- verify: manual, SRS-02:start:end, proof: ac-1.log-->

#### Implementation Insights
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

#### Verified Evidence
- [ac-1.log](../../../../stories/1vwqCfHz7/EVIDENCE/ac-1.log)

### Build Canonical Flow Status Projection
- **ID:** 1vwqCfS0F
- **Status:** done

#### Summary
Implement canonical flow and status projection services for operational views.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Canonical projection service provides flow/status data consumed by flow, status, and next command adapters. <!-- verify: manual, SRS-01:start:end, proof: ac-1.log-->

#### Implementation Insights
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

#### Verified Evidence
- [ac-1.log](../../../../stories/1vwqCfS0F/EVIDENCE/ac-1.log)

### Unify Queue Policy Consumption
- **ID:** 1vwqCfgC4
- **Status:** done

#### Summary
Unify queue policy access across all consumers.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] Queue policy classification is consumed through a shared API by all decision and rendering paths. <!-- verify: manual, SRS-03:start:end, proof: ac-1.log-->

#### Implementation Insights
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

#### Verified Evidence
- [ac-1.log](../../../../stories/1vwqCfgC4/EVIDENCE/ac-1.log)

### Remove Duplicate Projection Implementations
- **ID:** 1vwqCfma0
- **Status:** done

#### Summary
Remove duplicate metric and projection implementations from interface modules.

#### Acceptance Criteria
- [x] [SRS-04/AC-01] Duplicate projection structs/calculations are removed from interface modules in favor of canonical read-model DTOs. <!-- verify: manual, SRS-04:start:end, proof: ac-1.log-->

#### Implementation Insights
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

#### Verified Evidence
- [ac-1.log](../../../../stories/1vwqCfma0/EVIDENCE/ac-1.log)


