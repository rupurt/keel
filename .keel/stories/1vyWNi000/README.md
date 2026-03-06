---
id: 1vyWNi000
title: Render Topology Drift Hotspots
type: feat
status: done
created_at: 2026-03-06T06:42:22
updated_at: 2026-03-06T08:00:51
scope: 1vyWIF000/1vyWIM000
started_at: 2026-03-06T07:45:47
completed_at: 2026-03-06T08:00:51
---

# Render Topology Drift Hotspots

## Summary

Render the topology tree and inline hotspot annotations so operators can immediately see where planning or execution drift is accumulating inside the epic flow.

## Acceptance Criteria

- [x] [SRS-03/AC-01] The topology renderer highlights epic and voyage drift hotspots from scope lineage issues and uncovered PRD or SRS requirement coverage. <!-- verify: cargo test --lib topology_renderer_surfaces_scope_and_coverage_hotspots, SRS-03:start, proof: ac-1.log -->
- [x] [SRS-03/AC-02] The topology renderer highlights story-level blockage from unmet dependencies and missing proof or verification coverage. <!-- verify: cargo test --lib topology_renderer_surfaces_dependency_and_proof_gaps, SRS-03:end, proof: ac-2.log -->
- [x] [SRS-03/AC-03] [SRS-NFR-02/AC-01] The renderer remains readable at common terminal widths by collapsing or summarizing detail instead of producing ambiguous layout. <!-- verify: vhs tapes/topology-epic-drift.tape, SRS-NFR-02:start:end, SRS-03:continues, proof: ac-3.gif -->
