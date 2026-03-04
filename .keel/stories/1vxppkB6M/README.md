---
id: 1vxppkB6M
title: Implement Voyage Show Requirement Progress
type: feat
status: done
created_at: 2026-03-04T09:16:28
updated_at: 2026-03-04T11:16:01
scope: 1vxYzSury/1vxpomgnN
started_at: 2026-03-04T10:46:03
completed_at: 2026-03-04T11:16:01
---

# Implement Voyage Show Requirement Progress

## Summary

Upgrade `keel voyage show` so it reports voyage intent, scope boundaries, and requirement-level completion/verification progress instead of dumping raw markdown.

## Acceptance Criteria

- [x] [SRS-03/AC-01] `keel voyage show <id>` renders high-level goal plus explicit in-scope/out-of-scope summary extracted from voyage docs. <!-- verify: cargo test --lib voyage_show_goal_scope, SRS-03:start, proof: ac-1.log-->
- [x] [SRS-03/AC-02] `keel voyage show <id>` renders a requirements table listing each SRS requirement with linked stories and completion/verification status. <!-- verify: cargo test --lib voyage_show_requirement_matrix, SRS-03:continues, proof: ac-2.log-->
- [x] [SRS-03/AC-03] `keel voyage show <id>` renders progress indicators for both stories and requirements so completion state is immediately visible. <!-- verify: cargo test --lib voyage_show_progress, SRS-03:end, proof: ac-3.log-->
- [x] [SRS-03/AC-04] [SRS-NFR-01/AC-02] Voyage requirement and story rows are deterministically sorted so equivalent board state yields stable output. <!-- verify: cargo test --lib voyage_show_deterministic_ordering, SRS-NFR-01:start:end, proof: ac-4.log-->
