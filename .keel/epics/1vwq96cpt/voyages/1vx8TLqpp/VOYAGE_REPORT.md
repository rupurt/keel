# VOYAGE REPORT: Normalize Physical DDD Module Layout

## Voyage Metadata
- **ID:** 1vx8TLqpp
- **Epic:** 1vwq96cpt
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 4/4 stories complete

## Implementation Narrative
### Remove Legacy Roots And Enforce Normalized Contracts
- **ID:** 1vx8UtmC9
- **Status:** done

#### Summary
Remove transitional legacy module roots and add explicit architecture contracts
that fail if old top-level families are reintroduced or normalized layer
boundaries are violated.

#### Acceptance Criteria
- [x] [SRS-05/AC-01] Architecture contract tests explicitly enforce normalized roots (`cli`, `application`, `domain`, `infrastructure`, `read_model`) and fail on forbidden legacy roots. <!-- verify: manual, SRS-05:start:end, proof: ac-1.log -->
- [x] [SRS-05/AC-02] Repository tree no longer contains active legacy top-level module families used before normalization. <!-- verify: manual, SRS-05:continues, proof: ac-2.log -->
- [x] [SRS-05/AC-03] Story-level evidence includes test output proving normalized contracts and behavior parity. <!-- verify: manual, SRS-05:end, proof: ac-3.log -->

#### Implementation Insights
- **1vyDuw8wW: Enforce Root Layout With Contracts**
  - Insight: Physical moves alone are not stable; contract tests must also assert forbidden `main.rs` module declarations and removed root file paths
  - Suggested Action: Pair every structural move with architecture contracts that check both declaration edges and on-disk paths
  - Applies To: src/main.rs, src/architecture_contract_tests.rs, src/**/mod.rs
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vx8UtmC9/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vx8UtmC9/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vx8UtmC9/EVIDENCE/ac-2.log)

### Relocate Infrastructure Services Into Src Infrastructure
- **ID:** 1vx8V5VeE
- **Status:** done

#### Summary
Relocate filesystem adapters, parsing/loading, template rendering, generation,
and verification modules into `src/infrastructure/**` so external IO concerns
are physically separated from domain and application logic.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] Parser/loader/template/generation/verification modules are moved under `src/infrastructure/**` and referenced from normalized paths. <!-- verify: manual, SRS-03:start:end, proof: ac-1.log -->
- [x] [SRS-03/AC-02] Legacy top-level service roots are removed from active module declarations with no duplicate implementations retained. <!-- verify: manual, SRS-03:continues, proof: ac-2.log -->
- [x] [SRS-03/AC-03] `main.rs` and dependent modules compile with infrastructure modules imported from `src/infrastructure/**`. <!-- verify: manual, SRS-03:continues, proof: ac-3.log -->
- [x] [SRS-03/AC-04] Regression and lifecycle tests stay green after infrastructure relocation. <!-- verify: manual, SRS-03:end, proof: ac-4.log -->

#### Implementation Insights
- **1vyDuwGDS: Relocated Source Files May Break Compile-Time Template Paths**
  - Insight: Compile-time embedded asset paths are location-sensitive; module moves must include immediate path rebasing for `include_str!` constants.
  - Suggested Action: After moving infrastructure modules, run a targeted compile/test immediately and patch relative asset paths before broader refactors.
  - Applies To: src/infrastructure/templates.rs
  - Category: code


#### Verified Evidence
- [ac-4.log](../../../../stories/1vx8V5VeE/EVIDENCE/ac-4.log)
- [ac-1.log](../../../../stories/1vx8V5VeE/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vx8V5VeE/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vx8V5VeE/EVIDENCE/ac-2.log)

### Relocate Cli Command Surface Into Src Cli
- **ID:** 1vx8V5uUT
- **Status:** done

#### Summary
Move command parsing, command adapter modules, and flow/next terminal presentation
code into `src/cli/**` so the physical layout matches the DDD interface boundary.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] `main.rs` dispatches through `src/cli/**` and active command adapter modules live under `src/cli/**`. <!-- verify: manual, SRS-01:start:end, proof: ac-1.log -->
- [x] [SRS-01/AC-02] Legacy top-level CLI families (`src/commands/**`, `src/flow/**`, `src/next/**`) are removed from active module declarations. <!-- verify: manual, SRS-01:continues, proof: ac-2.log -->
- [x] [SRS-04/AC-01] Root module declarations expose `cli` as the interface entrypoint layer without regressing command behavior. <!-- verify: manual, SRS-04:start:end, proof: ac-3.log -->
- [x] [SRS-04/AC-02] `just test` remains green after CLI relocation. <!-- verify: manual, SRS-04:end, proof: ac-4.log -->

#### Implementation Insights
- **1vyDuwLDX: Path-Wide Module Moves Need Import Rewrite First**
  - Insight: Bulk file moves are low-risk only when import rewrites and architecture path fixtures are updated in the same slice; otherwise compile passes but contract tests drift.
  - Suggested Action: For physical normalization stories, perform move + import rewrite + fixture path updates atomically before running full test and doctor checks.
  - Applies To: src/main.rs, src/cli/**, src/architecture_contract_tests.rs
  - Category: architecture


#### Verified Evidence
- [ac-4.log](../../../../stories/1vx8V5uUT/EVIDENCE/ac-4.log)
- [ac-1.log](../../../../stories/1vx8V5uUT/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vx8V5uUT/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vx8V5uUT/EVIDENCE/ac-2.log)

### Relocate Domain Core Modules Into Src Domain
- **ID:** 1vx8V69uz
- **Status:** done

#### Summary
Relocate core entities, policies, transition logic, and domain validation from
legacy top-level modules into `src/domain/**` so domain logic is physically
grouped and independent from adapters.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] Core domain families (`model`, `policy`, `state_machine`, `transitions`) are moved into `src/domain/**` and compile from their new paths. <!-- verify: manual, SRS-02:start:end, proof: ac-1.log -->
- [x] [SRS-02/AC-02] No active top-level module declarations remain for `src/model/**`, `src/policy/**`, `src/state_machine/**`, or `src/transitions/**`. <!-- verify: manual, SRS-02:continues, proof: ac-2.log -->
- [x] [SRS-02/AC-03] Layer dependencies continue to flow from `cli/application` into `domain` without cyclical imports. <!-- verify: manual, SRS-02:continues, proof: ac-3.log -->
- [x] [SRS-02/AC-04] Domain move preserves behavior validated by existing test suites. <!-- verify: manual, SRS-02:end, proof: ac-4.log -->

#### Implementation Insights
- **1vyDuwdhQ: Multi-Requirement Stories Can Create Queue Cycles**
  - Insight: Implementation dependency derivation is SRS-order based; a story that references both early and later requirements can create circular dependencies across siblings.
  - Suggested Action: Keep each implementation story mapped to a primary SRS requirement in sequence, and reserve aggregate contract cleanup requirements for the final story.
  - Applies To: .keel/stories/*/README.md, src/traceability.rs
  - Category: process


#### Verified Evidence
- [ac-4.log](../../../../stories/1vx8V69uz/EVIDENCE/ac-4.log)
- [ac-1.log](../../../../stories/1vx8V69uz/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vx8V69uz/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vx8V69uz/EVIDENCE/ac-2.log)


