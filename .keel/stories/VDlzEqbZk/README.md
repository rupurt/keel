---
id: VDlzEqbZk
title: Add Comedy and Shakespeare Modes
type: feat
status: done
created_at: 2026-03-13T11:21:52
updated_at: 2026-03-13T11:29:45
operator-signal: 
scope: VDlzCqxr9/VDlzEF2OP
index: 3
started_at: 2026-03-13T11:28:12
completed_at: 2026-03-13T11:29:45
---

# Add Comedy and Shakespeare Modes

## Summary

Add persona and session-type adapters for stand-up comedy and Shakespeare/Broadway style so theater sessions can intentionally change narration tone.

## Acceptance Criteria

- [x] [SRS-03/AC-01] Add at least four personas (`standup`, `shakespeare`, `broadway`, `neutral`) with distinct narration templates. <!-- verify: bash -c "cargo run --quiet -- play --theater --persona neutral 2>/dev/null | rg '^Cue:' && cargo run --quiet -- play --theater --persona standup 2>/dev/null | rg '^Cue:' && cargo run --quiet -- play --theater --persona shakespeare 2>/dev/null | rg '^Cue:' && cargo run --quiet -- play --theater --persona broadway 2>/dev/null | rg '^Cue:'", SRS-03:start:end, proof: ac-1.log -->
- [x] [SRS-03/AC-02] `keel play --theater --persona shakespeare` emits a style-marked line distinct from `--persona standup`. <!-- verify: bash -c "! diff -q <(cargo run --quiet -- play --theater --persona shakespeare 2>/dev/null | rg '^Cue:') <(cargo run --quiet -- play --theater --persona standup 2>/dev/null | rg '^Cue:')", SRS-03:start:end, proof: ac-2.log -->
