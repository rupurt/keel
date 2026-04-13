---
# system-managed
id: VGe8MfYJW
status: done
created_at: 2026-04-12T22:34:35
updated_at: 2026-04-12T22:42:31
# authored
title: Load Mission Stack Projection From Local Manifest
type: feat
operator-signal:
scope: VGe7mCcFW/VGe8Ad6Jy
index: 2
started_at: 2026-04-12T22:35:51
completed_at: 2026-04-12T22:42:31
---

# Load Mission Stack Projection From Local Manifest

## Summary

Add the first repo-local Mission Stack read model. Keel should be able to load
optional stack metadata from `.keel/stacks/<id>/manifest.yaml`, combine it with
current git/worktree state, and produce a deterministic local projection for
other adapters to consume without modifying the core board model.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Keel loads a Mission Stack projection from repo-local manifest metadata and derives local member role, stack mode, checkpoint, linked missions, and receipt state. <!-- verify: cargo test mission_stack_loads_projection_from_manifest_and_git_state --lib, SRS-01:start, proof: ac-1.log -->
- [x] [SRS-01/AC-02] The projection derives current branch and checkout/worktree metadata needed for later guardrails. <!-- verify: cargo test mission_stack_derives_branch_and_worktree_state --lib, SRS-01:end, proof: ac-2.log -->
- [x] [SRS-NFR-01/AC-01] Repos without stack metadata remain a no-op and preserve current single-repo behavior. <!-- verify: cargo test mission_stack_absent_repo_is_noop --lib, SRS-NFR-01:start:end, proof: ac-3.log -->
