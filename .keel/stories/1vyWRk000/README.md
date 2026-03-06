---
id: 1vyWRk000
title: Author Bearing Workflow Dogfood Tapes
type: feat
status: done
created_at: 2026-03-06T06:46:32
updated_at: 2026-03-06T08:08:03
scope: 1vyWLl000/1vyWNL000
index: 4
started_at: 2026-03-06T08:04:26
completed_at: 2026-03-06T08:08:03
---

# Author Bearing Workflow Dogfood Tapes

## Summary

Author the bearing-phase VHS scenarios so keel can dogfood the research workflow from creation through graduation with the same proof model used for implementation work.

## Acceptance Criteria

- [x] [SRS-04/AC-01] A tape-driven bearing workflow covers `bearing new`, `bearing survey`, `bearing assess`, and `bearing lay` on the secondary workspace. <!-- verify: bash -lc 'vhs validate testdata/dogfood/scenarios/bearing-flow.tape && cargo test -p keel bearing_flow_tape_covers_research_lifecycle', SRS-04:start:end, proof: ac-1.log-->
- [x] [SRS-NFR-01/AC-02] The bearing workflow remains repeatable on the same fixture state. <!-- verify: cargo test -p keel bearing_flow_tape_avoids_fixed_entity_ids, SRS-NFR-01:end, proof: ac-2.log-->
