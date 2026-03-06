---
created_at: 2026-03-02T12:03:53
---

# Knowledge - 1vx8TLqpp

> Automated synthesis of story reflections.

## Story Knowledge

## Story: Remove Legacy Roots And Enforce Normalized Contracts (1vx8UtmC9)

### 1vyDuw8wW: Enforce Root Layout With Contracts

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Large module migrations where old root files can remain importable after moves |
| **Insight** | Physical moves alone are not stable; contract tests must also assert forbidden `main.rs` module declarations and removed root file paths |
| **Suggested Action** | Pair every structural move with architecture contracts that check both declaration edges and on-disk paths |
| **Applies To** | src/main.rs, src/architecture_contract_tests.rs, src/**/mod.rs |
| **Applied** | Added normalized-root and legacy-path assertions for all migrated root modules |



---

## Story: Relocate Cli Command Surface Into Src Cli (1vx8V5uUT)

### 1vyDuwLDX: Path-Wide Module Moves Need Import Rewrite First

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Relocating top-level module families (`commands`, `flow`, `next`) to a new root (`cli`) while preserving behavior. |
| **Insight** | Bulk file moves are low-risk only when import rewrites and architecture path fixtures are updated in the same slice; otherwise compile passes but contract tests drift. |
| **Suggested Action** | For physical normalization stories, perform move + import rewrite + fixture path updates atomically before running full test and doctor checks. |
| **Applies To** | src/main.rs, src/cli/**, src/architecture_contract_tests.rs |
| **Applied** |  |



---

## Story: Relocate Infrastructure Services Into Src Infrastructure (1vx8V5VeE)

### 1vyDuwGDS: Relocated Source Files May Break Compile-Time Template Paths

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Moving `templates.rs` under `src/infrastructure` changed relative path depth for `include_str!` macro calls. |
| **Insight** | Compile-time embedded asset paths are location-sensitive; module moves must include immediate path rebasing for `include_str!` constants. |
| **Suggested Action** | After moving infrastructure modules, run a targeted compile/test immediately and patch relative asset paths before broader refactors. |
| **Applies To** | src/infrastructure/templates.rs |
| **Applied** |  |



---

## Story: Relocate Domain Core Modules Into Src Domain (1vx8V69uz)

### 1vyDuwdhQ: Multi-Requirement Stories Can Create Queue Cycles

| Field | Value |
|-------|-------|
| **Category** | process |
| **Context** | Stories in the same voyage referenced overlapping SRS IDs, and queue dependency derivation blocked all stories from becoming ready. |
| **Insight** | Implementation dependency derivation is SRS-order based; a story that references both early and later requirements can create circular dependencies across siblings. |
| **Suggested Action** | Keep each implementation story mapped to a primary SRS requirement in sequence, and reserve aggregate contract cleanup requirements for the final story. |
| **Applies To** | .keel/stories/*/README.md, src/traceability.rs |
| **Applied** |  |



---

## Synthesis

### 3kBfhLmlY: Enforce Root Layout With Contracts

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Large module migrations where old root files can remain importable after moves |
| **Insight** | Physical moves alone are not stable; contract tests must also assert forbidden `main.rs` module declarations and removed root file paths |
| **Suggested Action** | Pair every structural move with architecture contracts that check both declaration edges and on-disk paths |
| **Applies To** | src/main.rs, src/architecture_contract_tests.rs, src/**/mod.rs |
| **Linked Knowledge IDs** | 1vyDuw8wW |
| **Score** | 0.89 |
| **Confidence** | 0.92 |
| **Applied** | Added normalized-root and legacy-path assertions for all migrated root modules |

### Q8pKzHiNH: Path-Wide Module Moves Need Import Rewrite First

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Relocating top-level module families (`commands`, `flow`, `next`) to a new root (`cli`) while preserving behavior. |
| **Insight** | Bulk file moves are low-risk only when import rewrites and architecture path fixtures are updated in the same slice; otherwise compile passes but contract tests drift. |
| **Suggested Action** | For physical normalization stories, perform move + import rewrite + fixture path updates atomically before running full test and doctor checks. |
| **Applies To** | src/main.rs, src/cli/**, src/architecture_contract_tests.rs |
| **Linked Knowledge IDs** | 1vyDuwLDX |
| **Score** | 0.84 |
| **Confidence** | 0.92 |
| **Applied** |  |

### Pa8P1V8dA: Relocated Source Files May Break Compile-Time Template Paths

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Moving `templates.rs` under `src/infrastructure` changed relative path depth for `include_str!` macro calls. |
| **Insight** | Compile-time embedded asset paths are location-sensitive; module moves must include immediate path rebasing for `include_str!` constants. |
| **Suggested Action** | After moving infrastructure modules, run a targeted compile/test immediately and patch relative asset paths before broader refactors. |
| **Applies To** | src/infrastructure/templates.rs |
| **Linked Knowledge IDs** | 1vyDuwGDS |
| **Score** | 0.86 |
| **Confidence** | 0.96 |
| **Applied** |  |

### Caz63yNKt: Multi-Requirement Stories Can Create Queue Cycles

| Field | Value |
|-------|-------|
| **Category** | process |
| **Context** | Stories in the same voyage referenced overlapping SRS IDs, and queue dependency derivation blocked all stories from becoming ready. |
| **Insight** | Implementation dependency derivation is SRS-order based; a story that references both early and later requirements can create circular dependencies across siblings. |
| **Suggested Action** | Keep each implementation story mapped to a primary SRS requirement in sequence, and reserve aggregate contract cleanup requirements for the final story. |
| **Applies To** | .keel/stories/*/README.md, src/traceability.rs |
| **Linked Knowledge IDs** | 1vyDuwdhQ |
| **Score** | 0.88 |
| **Confidence** | 0.95 |
| **Applied** |  |

