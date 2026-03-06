---
id: 1vyWSB000
title: Create Secondary Dogfood Workspace
type: feat
status: done
created_at: 2026-03-06T06:46:59
updated_at: 2026-03-06T07:31:51
scope: 1vyWLl000/1vyWNL000
index: 1
started_at: 2026-03-06T07:14:46
completed_at: 2026-03-06T07:31:51
---

# Create Secondary Dogfood Workspace

## Summary

Establish a checked-in secondary workspace with its own `.keel` board and deterministic reset path so dogfood runs exercise real workflow state without touching the repository's primary board.

## Acceptance Criteria

- [x] [SRS-01/AC-01] A checked-in secondary workspace exists with its own `.keel` board and enough authored fixture state to support epic and bearing workflow tapes. <!-- verify: cargo test -p keel dogfood_workspace_scaffold_has_secondary_board, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] The secondary workspace exposes a deterministic reset path. <!-- verify: cargo test -p keel dogfood_workspace_reset_preserves_primary_board, SRS-01:end, proof: ac-2.log-->
- [x] [SRS-NFR-02/AC-01] The deterministic reset path leaves the repository's primary `.keel` board unchanged. <!-- verify: cargo test -p keel dogfood_workspace_reset_preserves_primary_board, SRS-NFR-02:start:end, proof: ac-3.log-->
