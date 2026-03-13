---
id: VDlzEqbZk
title: Add Comedy and Shakespeare Modes
type: feat
status: icebox
created_at: 2026-03-13T11:21:52
updated_at: 2026-03-13T11:21:52
operator-signal: 
scope: VDlzCqxr9/VDlzEF2OP
index: 3
---

# Add Comedy and Shakespeare Modes

## Summary

Add persona and session-type adapters for stand-up comedy and Shakespeare/Broadway style so theater sessions can intentionally change narration tone.

## Acceptance Criteria

- [ ] [SRS-03/AC-01] Add at least four personas (`standup`, `shakespeare`, `broadway`, `neutral`) with distinct narration templates. <!-- verify: CLI proof -->
- [ ] [SRS-03/AC-02] `keel play --theater --persona shakespeare` emits a style-marked line distinct from `--persona standup`. <!-- verify: CLI proof -->
