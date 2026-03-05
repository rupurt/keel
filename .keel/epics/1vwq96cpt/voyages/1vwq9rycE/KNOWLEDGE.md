---
created_at: 2026-03-02T10:34:57
---

# Knowledge - 1vwq9rycE

> Automated synthesis of story reflections.

## Story Knowledge

## Story: Unify Queue Policy Consumption (1vwqCfgC4)

### 1vyDuwSPf: Queue-policy facades prevent decision/rendering drift

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Queue classifications were being consumed directly by multiple modules (`next`, `flow/bottleneck`, and `state_machine/flow`) with repeated policy calls. |
| **Insight** | A small read-model facade (`read_model::queue_policy`) creates one consumption surface for policy outputs while keeping source-of-truth thresholds in `policy::queue`. |
| **Suggested Action** | Add architecture contracts for policy-facade usage whenever policy semantics are consumed by multiple adapters or decision paths. |
| **Applies To** | `src/read_model/queue_policy.rs`, `src/next/algorithm.rs`, `src/flow/bottleneck.rs`, `src/state_machine/flow.rs`, `src/architecture_contract_tests.rs` |
| **Applied** | story `1vwqCfgC4` |



---

## Story: Build Canonical Capacity Projection (1vwqCfHz7)

### 1vyDuwl5B: Canonical read models remove adapter drift

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | When both flow rendering and diagnostics need the same capacity semantics |
| **Insight** | Duplicated DTOs and charge enums across adapters force conversion shims and create drift risk in UI logic. |
| **Suggested Action** | Keep one projection type in `read_model` and make interface modules thin adapters over that projection. |
| **Applies To** | `src/read_model/capacity.rs`, `src/flow/capacity.rs`, `src/commands/diagnostics/capacity.rs`, `src/flow/display.rs` |
| **Applied** | story `1vwqCfHz7` |



---

## Story: Build Canonical Flow Status Projection (1vwqCfS0F)

### 1vyDuwXBN: Keep Operational Metrics In A Single Read Model

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Consolidating repeated queue/flow/status metrics used across diagnostics and next-decision logic |
| **Insight** | A canonical projection DTO that embeds both flow metrics and status metrics removes drift and lets adapters format output without recalculating business metrics |
| **Suggested Action** | Add read-model projection services first, then migrate every consumer to the projection API before deleting local metric structs |
| **Applies To** | src/read_model/flow_status.rs; src/commands/diagnostics/{flow,status}.rs; src/next/algorithm.rs |
| **Applied** | 1vwqCfS0F |



---

## Story: Remove Duplicate Projection Implementations (1vwqCfma0)

### 1vyDuwCgL: Interface Adapters Should Delegate Instead Of Recompute

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | When both flow rendering and diagnostics commands need the same projection outputs |
| **Insight** | Duplicated adapter-level projection/load/render paths drift quickly and should be collapsed behind a single interface that consumes canonical read-model DTOs |
| **Suggested Action** | Keep one shared capacity interface and enforce delegation from command modules through architecture contracts |
| **Applies To** | `src/commands/diagnostics/capacity.rs`, `src/flow/capacity.rs`, `src/architecture_contract_tests.rs` |
| **Applied** | Delegated diagnostics capacity command to `flow::capacity` and added explicit contract test for shared interface usage |



---

## Synthesis

### qUF0j5GEl: Queue-policy facades prevent decision/rendering drift

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Queue classifications were being consumed directly by multiple modules (`next`, `flow/bottleneck`, and `state_machine/flow`) with repeated policy calls. |
| **Insight** | A small read-model facade (`read_model::queue_policy`) creates one consumption surface for policy outputs while keeping source-of-truth thresholds in `policy::queue`. |
| **Suggested Action** | Add architecture contracts for policy-facade usage whenever policy semantics are consumed by multiple adapters or decision paths. |
| **Applies To** | `src/read_model/queue_policy.rs`, `src/next/algorithm.rs`, `src/flow/bottleneck.rs`, `src/state_machine/flow.rs`, `src/architecture_contract_tests.rs` |
| **Linked Knowledge IDs** | 1vyDuwSPf |
| **Score** | 0.86 |
| **Confidence** | 0.95 |
| **Applied** | story `1vwqCfgC4` |

### ZXRUhJpNe: Canonical read models remove adapter drift

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | When both flow rendering and diagnostics need the same capacity semantics |
| **Insight** | Duplicated DTOs and charge enums across adapters force conversion shims and create drift risk in UI logic. |
| **Suggested Action** | Keep one projection type in `read_model` and make interface modules thin adapters over that projection. |
| **Applies To** | `src/read_model/capacity.rs`, `src/flow/capacity.rs`, `src/commands/diagnostics/capacity.rs`, `src/flow/display.rs` |
| **Linked Knowledge IDs** | 1vyDuwl5B |
| **Score** | 0.89 |
| **Confidence** | 0.96 |
| **Applied** | story `1vwqCfHz7` |

### IxTCT6ayY: Keep Operational Metrics In A Single Read Model

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Consolidating repeated queue/flow/status metrics used across diagnostics and next-decision logic |
| **Insight** | A canonical projection DTO that embeds both flow metrics and status metrics removes drift and lets adapters format output without recalculating business metrics |
| **Suggested Action** | Add read-model projection services first, then migrate every consumer to the projection API before deleting local metric structs |
| **Applies To** | src/read_model/flow_status.rs; src/commands/diagnostics/{flow,status}.rs; src/next/algorithm.rs |
| **Linked Knowledge IDs** | 1vyDuwXBN |
| **Score** | 0.84 |
| **Confidence** | 0.89 |
| **Applied** | 1vwqCfS0F |

### KwQM6oOZE: Interface Adapters Should Delegate Instead Of Recompute

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | When both flow rendering and diagnostics commands need the same projection outputs |
| **Insight** | Duplicated adapter-level projection/load/render paths drift quickly and should be collapsed behind a single interface that consumes canonical read-model DTOs |
| **Suggested Action** | Keep one shared capacity interface and enforce delegation from command modules through architecture contracts |
| **Applies To** | `src/commands/diagnostics/capacity.rs`, `src/flow/capacity.rs`, `src/architecture_contract_tests.rs` |
| **Linked Knowledge IDs** | 1vyDuwCgL |
| **Score** | 0.83 |
| **Confidence** | 0.90 |
| **Applied** | Delegated diagnostics capacity command to `flow::capacity` and added explicit contract test for shared interface usage |

