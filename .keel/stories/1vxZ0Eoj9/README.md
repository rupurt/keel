---
id: 1vxZ0Eoj9
title: Add Canonical Guidance To Play Command Outcomes
type: feat
status: done
created_at: 2026-03-03T15:18:10
updated_at: 2026-03-03T20:14:53
scope: 1vxYzSury/1vxYzrwma
started_at: 2026-03-03T20:09:06
completed_at: 2026-03-03T20:14:53
---

# Add Canonical Guidance To Play Command Outcomes

## Summary

Add canonical guidance handling to play command outcomes so deterministic results emit one next action while exploratory outputs remain non-prescriptive.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Emit canonical `next_step` guidance for play outcomes that have a deterministic follow-up command. <!-- verify: cargo test --lib play_guidance::tests::suggest_outcome_maps_to_canonical_next_step, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Keep exploratory play outputs non-prescriptive when no deterministic command can be recommended. <!-- verify: cargo test --lib play_guidance::tests::exploratory_outcomes_emit_no_guidance, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Add tests that cover both prescriptive and non-prescriptive play outcomes across human and JSON output modes. <!-- verify: cargo test --lib play_guidance::tests::play_outcomes_keep_human_and_json_guidance_in_sync, SRS-01:end, proof: ac-3.log-->
