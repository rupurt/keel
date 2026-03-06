---
id: 1vyGZeEI7
title: Validate Goal Links In PRD Requirements
type: feat
status: backlog
created_at: 2026-03-05T13:49:38
updated_at: 2026-03-05T14:10:01
scope: 1vyFgR2MA/1vyFmfjA9
---

# Validate Goal Links In PRD Requirements

## Summary

Validate that each PRD requirement links back to strategic goals and make those failures actionable in doctor so orphaned goals and unlinked requirements cannot hide in planning.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] PRD FR/NFR requirement rows reference one or more valid `GOAL-*` identifiers. <!-- verify: cargo test -p keel prd_requirements_require_valid_goal_links, SRS-02:start -->
- [ ] [SRS-03/AC-01] Doctor diagnostics report invalid goal references, orphaned goals, and PRD requirements with no goal linkage. <!-- verify: cargo test -p keel doctor_reports_goal_lineage_gaps, SRS-03:start:end -->
- [ ] [SRS-03/AC-02] [SRS-NFR-02/AC-01] Goal-lineage failures identify the offending goal ID, requirement ID, and artifact path. <!-- verify: cargo test -p keel goal_lineage_errors_are_actionable, SRS-NFR-02:start:end -->
- [ ] [SRS-02/AC-02] [SRS-NFR-03/AC-01] Non-canonical or legacy goal tokens fail hard without compatibility aliases. <!-- verify: cargo test -p keel goal_lineage_rejects_legacy_tokens, SRS-NFR-03:start:end, SRS-02:end -->
