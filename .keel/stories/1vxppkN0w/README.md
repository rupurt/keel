---
id: 1vxppkN0w
title: Implement Epic Show Planning Summary
type: feat
status: done
created_at: 2026-03-04T09:16:28
updated_at: 2026-03-04T10:15:05
scope: 1vxYzSury/1vxpomgnN
started_at: 2026-03-04T09:56:05
completed_at: 2026-03-04T10:15:05
---

# Implement Epic Show Planning Summary

## Summary

Upgrade `keel epic show` to render an actionable planning report: authored summary, requirement/verification readiness, artifact visibility, and completion progress with ETA.

## Acceptance Criteria

- [x] [SRS-01/AC-02] `keel epic show <id>` renders authored problem statement, goals/objectives, and key requirements in a compact planning summary section. <!-- verify: cargo test --lib epic_show_planning_summary, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] `keel epic show <id>` renders progress metrics (voyages/stories complete) plus a time-to-complete estimate derived from a 4-week throughput window with fallback messaging when data is insufficient. <!-- verify: cargo test --lib epic_show_eta_projection_4w, SRS-02:start, proof: ac-2.log-->
- [x] [SRS-02/AC-02] `keel epic show <id>` renders verification readiness including automated/manual requirement coverage and linked artifact inventory (text + media). <!-- verify: cargo test --lib epic_show_verification_surface, SRS-02:continues, proof: ac-3.log-->
- [x] [SRS-02/AC-03] `keel epic show <id>` renders project-aware automated verification recommendations (for example stack-specific tooling suggestions) with rationale tied to detected project signals. <!-- verify: cargo test --lib epic_show_verification_recommendations, SRS-02:end, proof: ac-4.log-->
- [x] [SRS-02/AC-04] [SRS-NFR-02/AC-01] When authored planning sections or evidence are missing, `epic show` prints explicit placeholders/warnings instead of omitting sections. <!-- verify: cargo test --lib epic_show_missing_data_placeholders, SRS-NFR-02:start:end, proof: ac-5.log-->
