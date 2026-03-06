---
id: 1vyGZgiTK
title: Render Scope Lineage In Planning Surfaces
type: feat
status: done
created_at: 2026-03-05T13:49:40
updated_at: 2026-03-05T18:05:00
scope: 1vyFgR2MA/1vyFn0OuN
started_at: 2026-03-05T17:57:21
completed_at: 2026-03-05T18:05:00
---

# Render Scope Lineage In Planning Surfaces

## Summary

Expose scope lineage and drift findings in voyage and epic planning surfaces so reviewers can see scope alignment without opening every source document by hand.

## Acceptance Criteria

- [x] [SRS-04/AC-01] Planning read surfaces summarize linked scope items and highlight scope drift findings for both the voyage and the parent epic. <!-- verify: cargo test -p keel planning_show_renders_scope_lineage_and_drift, SRS-04:start, proof: ac-1.log-->
- [x] [SRS-04/AC-02] Scope-lineage output includes enough linked context for planners to distinguish valid in-scope coverage from out-of-scope contradictions. <!-- verify: cargo test -p keel planning_show_scope_drift_output_is_reviewable, SRS-04:end, proof: ac-2.log-->
- [x] [SRS-05/AC-01] Planning views preserve authored descriptive scope text while still surfacing canonical scope IDs for machine-checkable lineage. <!-- verify: cargo test -p keel planning_show_preserves_scope_text_with_ids, SRS-05:start:end, proof: ac-3.log-->
