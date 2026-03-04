---
id: 1vxppkN0w
title: Implement Epic Show Planning Summary
type: feat
status: backlog
created_at: 2026-03-04T09:16:28
updated_at: 2026-03-04T09:17:01
scope: 1vxYzSury/1vxpomgnN
---

# Implement Epic Show Planning Summary

## Summary

Upgrade `keel epic show` to render an actionable planning report: authored summary, requirement/verification readiness, artifact visibility, and completion progress with ETA.

## Acceptance Criteria

- [ ] [SRS-01/AC-02] `keel epic show <id>` renders authored problem statement, goals/objectives, and key requirements in a compact planning summary section. <!-- verify: cargo test --lib epic_show_planning_summary, SRS-01:end -->
- [ ] [SRS-02/AC-01] `keel epic show <id>` renders progress metrics (voyages/stories complete) plus a time-to-complete estimate derived from recent throughput with fallback messaging when data is insufficient. <!-- verify: cargo test --lib epic_show_eta_projection, SRS-02:start -->
- [ ] [SRS-02/AC-02] `keel epic show <id>` renders verification readiness including automated/manual requirement coverage and linked artifact inventory (text + media). <!-- verify: cargo test --lib epic_show_verification_surface, SRS-02:continues -->
- [ ] [SRS-NFR-02/AC-01] When authored planning sections or evidence are missing, `epic show` prints explicit placeholders/warnings instead of omitting sections. <!-- verify: cargo test --lib epic_show_missing_data_placeholders, SRS-NFR-02:start -->
