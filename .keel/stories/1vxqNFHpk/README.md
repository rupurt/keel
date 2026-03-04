---
id: 1vxqNFHpk
title: Surface Technique Recommendations In Planning Shows
type: feat
status: done
created_at: 2026-03-04T09:51:05
updated_at: 2026-03-04T13:05:08
scope: 1vxqMtskC/1vxqN5jnA
started_at: 2026-03-04T12:51:01
completed_at: 2026-03-04T13:05:08
---

# Surface Technique Recommendations In Planning Shows

## Summary

Expose technique recommendations in planning read commands so teams can see which automated verification approaches are available, configured, and currently underused.

## Acceptance Criteria

- [x] [SRS-04/AC-01] `keel epic show`, `keel voyage show`, and `keel story show` render a recommendation section with ranked techniques and rationale. <!-- verify: cargo test --lib show_recommendation_sections, SRS-04:start, proof: ac-1.log-->
- [x] [SRS-04/AC-02] Recommendation output identifies whether techniques like `vhs` and `llm-judge` are configured/unused and provides adoption guidance snippets. <!-- verify: cargo test --lib show_recommendation_usage_status, SRS-04:end, proof: ac-2.log-->
- [x] [SRS-04/AC-03] Show rendering remains advisory-only and does not trigger execution of recommended techniques. <!-- verify: cargo test --lib show_recommendations_do_not_execute, SRS-NFR-02:start:end, proof: ac-3.log-->
