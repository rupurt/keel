---
created_at: 2026-03-02T08:14:31
---

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

### 1vyDuwl5B: Canonical read models remove adapter drift

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
