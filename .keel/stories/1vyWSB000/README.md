---
id: 1vyWSB000
title: Create Secondary Dogfood Workspace
type: feat
status: backlog
created_at: 2026-03-06T06:46:59
updated_at: 2026-03-06T06:50:33
scope: 1vyWLl000/1vyWNL000
index: 1
---

# Create Secondary Dogfood Workspace

## Summary

Establish a checked-in secondary workspace with its own `.keel` board and deterministic reset path so dogfood runs exercise real workflow state without touching the repository's primary board.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] [SRS-NFR-02/AC-01] A checked-in secondary workspace exists with its own `.keel` board, enough authored fixture state to support epic and bearing workflow tapes, and a reset path that leaves the repository's primary `.keel` board unchanged. <!-- verify: cargo test -p keel dogfood_workspace_scaffold_has_secondary_board, SRS-01:start:end, SRS-NFR-02:start:end -->
