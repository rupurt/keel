# Knowledge - 1vx8TLqpp

> Automated synthesis of story reflections.

## Story: Remove Legacy Roots And Enforce Normalized Contracts (1vx8UtmC9)

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


---

## Story: Relocate Infrastructure Services Into Src Infrastructure (1vx8V5VeE)

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


---

## Story: Relocate Cli Command Surface Into Src Cli (1vx8V5uUT)

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


---

## Story: Relocate Domain Core Modules Into Src Domain (1vx8V69uz)

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


---

