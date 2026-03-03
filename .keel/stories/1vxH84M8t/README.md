---
id: 1vxH84M8t
title: Gate Story Submit And Accept On Coherent Artifacts
type: feat
status: done
created_at: 2026-03-02T20:13:04
updated_at: 2026-03-03T09:56:48
scope: 1vxGy5tco/1vxGzVpw5
index: 3
started_at: 2026-03-03T09:46:52
completed_at: 2026-03-03T09:56:48
---

# Gate Story Submit And Accept On Coherent Artifacts

## Summary

Enforce submit/accept lifecycle gating so unresolved scaffold/default story and reflection artifacts cannot advance to terminal states.

## Acceptance Criteria

- [x] [SRS-04/AC-01] Story submit is blocked when story README or REFLECT contains unresolved scaffold/default patterns. <!-- verify: cargo test -p keel domain::state_machine::gating::tests::evaluate_story_submit_blocks_unresolved_readme_scaffold, SRS-04:start, proof: ac-1.log -->
- [x] [SRS-04/AC-02] Story accept is blocked on the same coherency violations. <!-- verify: cargo test -p keel domain::state_machine::gating::tests::evaluate_story_accept_blocks_unresolved_reflect_scaffold, SRS-04:end, proof: ac-2.log -->
- [x] [SRS-05/AC-01] Generated report artifacts remain excluded from unresolved-scaffold enforcement scope. <!-- verify: cargo test -p keel domain::state_machine::gating::tests::evaluate_story_accept_ignores_generated_manifest_for_scaffold_gate, SRS-05:start:end, proof: ac-3.log -->
