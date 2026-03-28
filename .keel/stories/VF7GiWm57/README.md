---
# system-managed
id: VF7GiWm57
status: icebox
created_at: 2026-03-27T17:11:54
updated_at: 2026-03-27T17:11:54
# authored
title: Update Pacemaker Docs And Downstream Instructions
type: docs
operator-signal:
scope: VF7Geb3Wa/VF7Gfkizo
index: 3
---

# Update Pacemaker Docs And Downstream Instructions

## Summary

Update foundational docs, MDX docs, and downstream upgrade guidance so the public contract teaches a derived heartbeat and stops instructing users to commit `.keel/heartbeat`.

## Acceptance Criteria

- [ ] [SRS-04/AC-01] Foundational docs explain heartbeat as a derived Git/worktree signal and remove instructions to commit `.keel/heartbeat`. <!-- verify: manual, SRS-04:start:end -->
- [ ] [SRS-04/AC-02] Public MDX docs and downstream-upgrade guidance describe the new pacemaker model and sync steps for adopters. <!-- verify: manual, SRS-04:start:end -->
- [ ] [SRS-NFR-02/AC-01] Documentation surfaces stay internally consistent about the new heartbeat semantics after the cutover. <!-- verify: manual, SRS-NFR-02:start:end -->
