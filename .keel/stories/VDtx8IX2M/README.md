---
id: VDtx8IX2M
title: Eliminate Report Tail Friction
type: feat
status: in-progress
scope: VDseuzIFg
milestone: null
created_at: 2026-03-15T03:03:41
updated_at: 2026-03-14T23:24:19
started_at: 2026-03-14T23:24:19
completed_at: null
submitted_at: null
index: 3
governed-by: []
blocked_by: []
role: null
operator-signal: pulse
---

<!-- keel:pulse-materialization: eliminate-report-tail-friction@2026-03-16T00:00:00Z -->

# Eliminate Report Tail Friction

## Summary

Materialized from routine `eliminate-report-tail-friction` for eligible window ending `2026-03-16T00:00:00Z`.

## Acceptance Criteria

- [x] [SRS-ROUTINE/AC-01] Complete the authored routine blueprint for this eligible window. <!-- verify: manual, SRS-ROUTINE:start, SRS-ROUTINE:end -->

## Routine Provenance

- Routine: `eliminate-report-tail-friction`
- Target scope: `VDseuzIFg`
- Eligible window ends: `2026-03-16T00:00:00Z`

## Blueprint

Investigate and implement a graph-based artifact computation system that generates Voyage and Compliance reports automatically during state transitions.

- **Current Problem:** Reports are often generated as a "tail" after implementation, leading to dirty Git trees.
- **Insight:** The agent harness often has to manually patch frontmatter because CLI transitions are sometimes incomplete or the metadata is mixed with authored content.
- **Goal:** 
  1. Move report generation into core state transition logic.
  2. Group frontmatter into "auto-generated" and "authored" sections to make agentic patching safer.
- **Exit Criteria:** Keel update that eliminates the need for manual `keel generate` and simplifies frontmatter management.
