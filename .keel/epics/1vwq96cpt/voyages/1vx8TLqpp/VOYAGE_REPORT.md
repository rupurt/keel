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
# Reflection - Remove Legacy Roots And Enforce Normalized Contracts

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

### L001: Enforce Root Layout With Contracts

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Large module migrations where old root files can remain importable after moves |
| **Insight** | Physical moves alone are not stable; contract tests must also assert forbidden `main.rs` module declarations and removed root file paths |
| **Suggested Action** | Pair every structural move with architecture contracts that check both declaration edges and on-disk paths |
| **Applies To** | src/main.rs, src/architecture_contract_tests.rs, src/**/mod.rs |
| **Observed At** | 2026-03-02T12:15:00Z |
| **Score** | 0.89 |
| **Confidence** | 0.92 |
| **Applied** | Added normalized-root and legacy-path assertions for all migrated root modules |

## Observations

Module relocation was straightforward once namespace rewires were done in bulk.
The main risk was silent drift from leftover root declarations and stale import paths,
which was mitigated by extending architecture contract tests and re-running full
verification (`test`, `quality`, `doctor`) on final formatted code.

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
# Reflection - Relocate Infrastructure Services Into Src Infrastructure

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

### L001: Relocated Source Files May Break Compile-Time Template Paths

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Moving `templates.rs` under `src/infrastructure` changed relative path depth for `include_str!` macro calls. |
| **Insight** | Compile-time embedded asset paths are location-sensitive; module moves must include immediate path rebasing for `include_str!` constants. |
| **Suggested Action** | After moving infrastructure modules, run a targeted compile/test immediately and patch relative asset paths before broader refactors. |
| **Applies To** | src/infrastructure/templates.rs |
| **Observed At** | 2026-03-02T19:38:18Z |
| **Score** | 0.86 |
| **Confidence** | 0.96 |
| **Applied** | |

## Observations

The relocation itself was mechanical with mass import rewrites, but moved
`include_str!` paths caused hard compile failures that were not obvious until test
build. Once template paths were corrected, the remaining cleanup was formatting
and import ordering.

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
# Reflection - Relocate Cli Command Surface Into Src Cli

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

### L001: Path-Wide Module Moves Need Import Rewrite First

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Relocating top-level module families (`commands`, `flow`, `next`) to a new root (`cli`) while preserving behavior. |
| **Insight** | Bulk file moves are low-risk only when import rewrites and architecture path fixtures are updated in the same slice; otherwise compile passes but contract tests drift. |
| **Suggested Action** | For physical normalization stories, perform move + import rewrite + fixture path updates atomically before running full test and doctor checks. |
| **Applies To** | src/main.rs, src/cli/**, src/architecture_contract_tests.rs |
| **Observed At** | 2026-03-02T19:15:47Z |
| **Score** | 0.84 |
| **Confidence** | 0.92 |
| **Applied** | |

## Observations

The migration stayed stable because the work was sliced by layer and verified with
an explicit failing architecture test before moving files. The key friction point
was doctor metadata coherence: acceptance criteria needed verification annotations
and the reopened epic status needed normalization (`strategic` -> `tactical`) to
clear board health checks.

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
# Reflection - Relocate Domain Core Modules Into Src Domain

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

### L001: Multi-Requirement Stories Can Create Queue Cycles

| Field | Value |
|-------|-------|
| **Category** | process |
| **Context** | Stories in the same voyage referenced overlapping SRS IDs, and queue dependency derivation blocked all stories from becoming ready. |
| **Insight** | Implementation dependency derivation is SRS-order based; a story that references both early and later requirements can create circular dependencies across siblings. |
| **Suggested Action** | Keep each implementation story mapped to a primary SRS requirement in sequence, and reserve aggregate contract cleanup requirements for the final story. |
| **Applies To** | .keel/stories/*/README.md, src/traceability.rs |
| **Observed At** | 2026-03-02T19:30:00Z |
| **Score** | 0.88 |
| **Confidence** | 0.95 |
| **Applied** | |

## Observations

Domain relocation was straightforward after the prior CLI move pattern: move files,
rewrite imports, then update fixture paths in architecture tests. The non-obvious
issue was queue readiness, which depended on SRS marker topology rather than index
order alone; fixing SRS mappings immediately unblocked `next --agent`.

#### Verified Evidence
- [ac-4.log](../../../../stories/1vx8V69uz/EVIDENCE/ac-4.log)
- [ac-1.log](../../../../stories/1vx8V69uz/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vx8V69uz/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vx8V69uz/EVIDENCE/ac-2.log)


