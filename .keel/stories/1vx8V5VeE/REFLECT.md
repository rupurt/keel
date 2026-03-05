---
created_at: 2026-03-02T11:39:22
---

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

### 1vyDuwGDS: Relocated Source Files May Break Compile-Time Template Paths

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
