---
id: 1vyWNd000
title: Add Topology Command And Filters
type: feat
status: done
created_at: 2026-03-06T06:42:17
updated_at: 2026-03-06T07:45:08
scope: 1vyWIF000/1vyWIM000
started_at: 2026-03-06T07:37:51
completed_at: 2026-03-06T07:45:08
---

# Add Topology Command And Filters

## Summary

Introduce the `keel topology` command path, epic targeting, and done-visibility controls so the new projection is reachable from the terminal in the intended human workflow.

## Acceptance Criteria

- [x] [SRS-01/AC-02] `keel topology --epic <id>` resolves the target epic and renders the topology projection through a dedicated informational command path. <!-- verify: cargo test --lib topology_command_invokes_epic_projection, SRS-01:end, proof: ac-1.log -->
- [x] [SRS-02/AC-01] The command defaults to a focused view that hides done voyages and stories while preserving planned and in-progress flow. <!-- verify: cargo test --lib topology_command_hides_done_by_default, SRS-02:start, proof: ac-2.log -->
- [x] [SRS-02/AC-02] An explicit done-visibility option reveals done voyages and stories without changing the default operational view. <!-- verify: cargo test --lib topology_command_includes_done_when_requested, SRS-02:end, proof: ac-3.log -->
