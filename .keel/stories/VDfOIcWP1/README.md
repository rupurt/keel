---
id: VDfOIcWP1
title: Add Graph Integrity Doctor Check
type: feat
status: backlog
created_at: 2026-03-12T08:17:31
updated_at: 2026-03-12T08:18:05
operator-signal: 
scope: VDfNdssJL/VDfO1dN84
index: 2
---

# Add Graph Integrity Doctor Check

## Summary

Add the first graph-level doctor check so Keel can validate structural tree integrity from `BoardGraph` instead of repeated local relationship scans.

## Acceptance Criteria

- [ ] [SRS-04/AC-01] Introduce a doctor check that reports orphaned nodes and containment cycles from the canonical `BoardGraph`. <!-- verify: cargo test --lib doctor_graph_integrity_reports_orphans_and_cycles, SRS-04:start -->
- [ ] [SRS-04/AC-02] The graph-integrity path reports terminal-parent violations when descendants remain non-terminal beneath a terminal strategic node. <!-- verify: cargo test --lib doctor_graph_integrity_reports_terminal_parent_violations, SRS-04:end -->
- [ ] [SRS-NFR-02/AC-01] The integrity check builds and reuses one graph per validation path instead of rebuilding whole-board relationship scans inside the check. <!-- verify: cargo test --lib doctor_graph_integrity_uses_single_graph_build, SRS-NFR-02:start:end -->
