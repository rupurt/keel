---
# system-managed
id: VF7GiXi58
status: icebox
created_at: 2026-03-27T17:11:54
updated_at: 2026-03-27T17:11:54
# authored
title: Cut Hooks And Poke Over To The Derived Pacemaker
type: feat
operator-signal:
scope: VF7Geb3Wa/VF7Gfkizo
index: 2
---

# Cut Hooks And Poke Over To The Derived Pacemaker

## Summary

Remove heartbeat-file mutation from the operator loop by cutting hooks and `keel poke` over to the derived pacemaker model while preserving their remaining responsibilities.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] The pre-commit hook stops auto-poking and staging `.keel/heartbeat`. <!-- verify: manual, SRS-02:start:end -->
- [ ] [SRS-02/AC-02] `keel poke` preserves comms and self-heal behavior without mutating heartbeat state on disk. <!-- verify: manual, SRS-02:start:end -->
- [ ] [SRS-03/AC-01] Operator messaging around pacemaker stability explains the new derived model and the real governor role of hook plus commit lifecycle. <!-- verify: manual, SRS-03:start:end -->
