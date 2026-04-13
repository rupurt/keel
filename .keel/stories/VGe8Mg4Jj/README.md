---
# system-managed
id: VGe8Mg4Jj
status: backlog
created_at: 2026-04-12T22:34:35
updated_at: 2026-04-12T22:35:36
# authored
title: Enforce Mission Stack Diagnostics And Foreign Worktree Guards
type: feat
operator-signal:
scope: VGe7mCcFW/VGe8Ad6Jy
index: 3
---

# Enforce Mission Stack Diagnostics And Foreign Worktree Guards

## Summary

Add the first enforcement layer for Mission Stack protocol rules. `doctor`
should diagnose wrong-branch, checkpoint, foreign-worktree, and stack-close
leftover violations, and execution surfaces should refuse unsupported foreign
checkout paths instead of silently proceeding.

## Acceptance Criteria

- [ ] [SRS-05/AC-01] `keel doctor` reports Mission Stack violations for wrong branch, missing checkpoint acknowledgment, and unsupported foreign execution state. <!-- verify: cargo test -p keel doctor_reports_mission_stack_violations, SRS-05:start, proof: ac-1.log -->
- [ ] [SRS-05/AC-02] Closed stacks with leftover managed foreign worktrees are reported conservatively instead of being deleted automatically. <!-- verify: cargo test -p keel doctor_reports_closed_stack_worktree_leftovers, SRS-05:continues, proof: ac-2.log -->
- [ ] [SRS-06/AC-01] Stack-aware adapter output exposes deterministic machine-readable fields for Mission Stack context and gating decisions. <!-- verify: cargo test -p keel mission_stack_surfaces_expose_deterministic_json, SRS-06:continues, proof: ac-3.log -->
- [ ] [SRS-NFR-02/AC-01] Stack-aware surfaces preserve repo-local heartbeat semantics and do not redefine pacemaker state. <!-- verify: cargo test -p keel mission_stack_surfaces_preserve_heartbeat_semantics, SRS-NFR-02:continues, proof: ac-4.log -->
- [ ] [SRS-NFR-03/AC-01] Foreign-worktree guardrails fail safe by blocking unsupported execution without mutating uncertain checkouts. <!-- verify: cargo test -p keel mission_stack_guardrails_fail_safe, SRS-NFR-03:end, proof: ac-5.log -->
