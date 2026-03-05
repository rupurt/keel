---
id: 1vyGZEZNc
title: Render Epic Requirement Coverage
type: feat
status: backlog
created_at: 2026-03-05T13:49:12
updated_at: 2026-03-05T14:09:05
scope: 1vyFgR2MA/1vyFiQPoH
---

# Render Epic Requirement Coverage

## Summary

Surface epic-wide parent requirement coverage from the same lineage rules used by runtime enforcement so planners and reviewers can spot uncovered PRD requirements across voyages.

## Acceptance Criteria

- [ ] [SRS-04/AC-01] Doctor diagnostics report PRD-to-SRS lineage problems using the same coherence rules used by planning transitions. <!-- verify: cargo test -p keel doctor_and_gate_share_prd_lineage_rules, SRS-04:start:end -->
- [ ] [SRS-05/AC-01] Epic planning projections aggregate parent FR/NFR coverage across all voyages and identify uncovered parent requirements with linked-child counts. <!-- verify: cargo test -p keel epic_show_aggregates_prd_requirement_coverage_across_voyages, SRS-05:start:end -->
- [ ] [SRS-06/AC-01] Coverage aggregation preserves exactly-one-parent ownership for each SRS requirement while allowing one parent FR/NFR to fan out to multiple voyage requirements without double counting. <!-- verify: cargo test -p keel prd_requirement_coverage_preserves_one_to_many_parent_fanout, SRS-06:start:end -->
- [ ] [SRS-05/AC-02] [SRS-NFR-01/AC-02] Equivalent board state yields stable ordering for parent requirements, linked children, and uncovered-requirement output. <!-- verify: cargo test -p keel epic_prd_coverage_projection_is_deterministic, SRS-NFR-01:start:end -->
