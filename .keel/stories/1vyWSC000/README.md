---
id: 1vyWSC000
title: Author Epic Workflow Dogfood Tapes
type: feat
status: done
created_at: 2026-03-06T06:47:00
updated_at: 2026-03-06T08:03:22
scope: 1vyWLl000/1vyWNL000
index: 3
started_at: 2026-03-06T07:50:19
completed_at: 2026-03-06T08:03:22
---

# Author Epic Workflow Dogfood Tapes

## Summary

Author the epic-phase VHS scenarios so keel can dogfood epic creation, voyage/story decomposition, and the steering surfaces agents use to decide what to do next.

## Acceptance Criteria

- [x] [SRS-03/AC-01] A tape-driven epic workflow covers epic creation, voyage/story decomposition, and the core planning flow on the secondary workspace. <!-- verify: bash -lc 'vhs validate testdata/dogfood/scenarios/epic-flow.tape && cargo test -p keel epic_flow_tape_covers_creation_and_decomposition', SRS-03:start, proof: ac-1.log-->
- [x] [SRS-03/AC-02] The epic workflow surfaces `keel next` and `keel flow` at the steering points needed to guide implementation. <!-- verify: cargo test -p keel epic_flow_tape_surfaces_next_and_flow, SRS-03:end, proof: ac-2.log-->
- [x] [SRS-NFR-01/AC-01] The epic workflow remains repeatable on the same fixture state. <!-- verify: cargo test -p keel epic_flow_tape_avoids_fixed_entity_ids, SRS-NFR-01:start, proof: ac-3.log-->
