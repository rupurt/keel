---
id: VDhu6M89X
title: Wire HEAD Syntax Into Show Commands
type: feat
status: backlog
created_at: 2026-03-12T18:36:23
updated_at: 2026-03-12T18:39:50
operator-signal: 
scope: VDhtrxgW6/VDhtzKSNF
index: 2
---

# Wire HEAD Syntax Into Show Commands

## Summary

Adopt the shared HEAD-selector path in the supported show commands so users can navigate by relative position instead of only by exact IDs.

## Acceptance Criteria

- [ ] [SRS-03/AC-01] Mission, epic, voyage, and story show commands resolve HEAD-relative selectors through the shared selector path while preserving exact-ID behavior. <!-- verify: cargo test --bin keel head_show_commands_resolve_management_entities, SRS-03:start:end -->
- [ ] [SRS-03/AC-02] Bearing, ADR, and routine show commands resolve HEAD-relative selectors through the same shared selector path while preserving exact-ID behavior. <!-- verify: cargo test --bin keel head_show_commands_resolve_governance_entities, SRS-03:start:end -->
- [ ] [SRS-04/AC-01] Empty-set and out-of-range failures surface actionable, deterministic errors for the affected show commands. <!-- verify: cargo test --bin keel head_show_commands_report_selector_errors, SRS-04:start:end -->
