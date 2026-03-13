---
id: VDiJkcePZ
title: Add Roadmap Mode
type: feat
status: done
created_at: 2026-03-12T20:18:16
updated_at: 2026-03-12T20:41:05
operator-signal: 
scope: VDiHw85WK/VDiJfjvVG
index: 1
started_at: 2026-03-12T20:18:41
submitted_at: 2026-03-12T20:40:17
completed_at: 2026-03-12T20:41:05
---

# Add Roadmap Mode

## Summary

Define and render a canonical roadmap view for management planning that surfaces mission/epic/voyage/story priorities, dependencies, and proceed/park posture without relying on ad-hoc file inspection.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Add a roadmap output surface that lists roadmap-relevant entities (missions/epics/voyages/stories) with explicit posture for each row (`proceed`, `park`, or `blocked`). <!-- verify: cargo test -- --nocapture roadmap_render_includes_posture, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] Ensure each roadmap row includes dependency blocking context (`blocking_ids`, `blocking_count`) and uses a deterministic ordering strategy. <!-- verify: cargo test -- --nocapture roadmap_rows_include_blockers_and_deterministic_sort, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-03/AC-01] Produce roadmap output directly in CLI text mode so operators can read it without reading raw mission/epic/story files. <!-- verify: manual, SRS-03:start:end, proof: ac-3.log-->
- [x] [SRS-NFR-01/AC-01] Validate deterministic ordering stability by running roadmap output repeatedly against a fixed board fixture. <!-- verify: cargo test -- --nocapture roadmap_output_is_deterministic, SRS-NFR-01:start:end, proof: ac-4.log-->
- [x] [SRS-NFR-02/AC-01] Verify management command runtime remains within expected command profile bounds. <!-- verify: cargo test -- --nocapture roadmap_render_performance, SRS-NFR-02:start:end, proof: ac-5.log-->
