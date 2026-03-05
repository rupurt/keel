---
id: 1vyGZgiTK
title: Render Scope Lineage In Planning Surfaces
type: feat
status: backlog
created_at: 2026-03-05T13:49:40
updated_at: 2026-03-05T14:10:02
scope: 1vyFgR2MA/1vyFn0OuN
---

# Render Scope Lineage In Planning Surfaces

## Summary

Expose scope lineage and drift findings in voyage and epic planning surfaces so reviewers can see scope alignment without opening every source document by hand.

## Acceptance Criteria

- [ ] [SRS-04/AC-01] Planning read surfaces summarize linked scope items and highlight scope drift findings for both the voyage and the parent epic. <!-- verify: cargo test -p keel planning_show_renders_scope_lineage_and_drift, SRS-04:start -->
- [ ] [SRS-04/AC-02] Scope-lineage output includes enough linked context for planners to distinguish valid in-scope coverage from out-of-scope contradictions. <!-- verify: cargo test -p keel planning_show_scope_drift_output_is_reviewable, SRS-04:end -->
- [ ] [SRS-05/AC-01] Planning views preserve authored descriptive scope text while still surfacing canonical scope IDs for machine-checkable lineage. <!-- verify: cargo test -p keel planning_show_preserves_scope_text_with_ids, SRS-05:start:end -->
