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
- **L001: Canonical read models remove adapter drift**
  - Insight: Duplicated DTOs and charge enums across adapters force conversion shims and create drift risk in UI logic.
  - Suggested Action: Keep one projection type in `read_model` and make interface modules thin adapters over that projection.
  - Applies To: `src/read_model/capacity.rs`, `src/flow/capacity.rs`, `src/commands/diagnostics/capacity.rs`, `src/flow/display.rs`
  - Category: architecture


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
- **L001: Keep Operational Metrics In A Single Read Model**
  - Insight: A canonical projection DTO that embeds both flow metrics and status metrics removes drift and lets adapters format output without recalculating business metrics
  - Suggested Action: Add read-model projection services first, then migrate every consumer to the projection API before deleting local metric structs
  - Applies To: src/read_model/flow_status.rs; src/commands/diagnostics/{flow,status}.rs; src/next/algorithm.rs
  - Category: architecture


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
- **L001: Queue-policy facades prevent decision/rendering drift**
  - Insight: A small read-model facade (`read_model::queue_policy`) creates one consumption surface for policy outputs while keeping source-of-truth thresholds in `policy::queue`.
  - Suggested Action: Add architecture contracts for policy-facade usage whenever policy semantics are consumed by multiple adapters or decision paths.
  - Applies To: `src/read_model/queue_policy.rs`, `src/next/algorithm.rs`, `src/flow/bottleneck.rs`, `src/state_machine/flow.rs`, `src/architecture_contract_tests.rs`
  - Category: architecture


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
- **L001: Interface Adapters Should Delegate Instead Of Recompute**
  - Insight: Duplicated adapter-level projection/load/render paths drift quickly and should be collapsed behind a single interface that consumes canonical read-model DTOs
  - Suggested Action: Keep one shared capacity interface and enforce delegation from command modules through architecture contracts
  - Applies To: `src/commands/diagnostics/capacity.rs`, `src/flow/capacity.rs`, `src/architecture_contract_tests.rs`
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vwqCfma0/EVIDENCE/ac-1.log)


