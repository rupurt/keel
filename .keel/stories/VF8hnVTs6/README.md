---
# system-managed
id: VF8hnVTs6
status: backlog
created_at: 2026-03-27T23:05:44
updated_at: 2026-03-27T23:10:55
# authored
title: Add Turn Loop Projection And CLI Surface
type: feat
operator-signal:
scope: VF8hiVofm/VF8hkUKk1
index: 2
---

# Add Turn Loop Projection And CLI Surface

## Summary

Expose the documented Orient/Inspect/Pull/Ship/Close rhythm as a first-class projection and command so the turn loop becomes inspectable instead of prose-only.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] A turn-loop projection models the documented phases and associates the correct command surfaces with each phase. <!-- verify: manual, SRS-01:start:end -->
- [ ] [SRS-02/AC-01] `keel turn` renders the projection in plain text and JSON for operator and harness use. <!-- verify: manual, SRS-02:start:end -->
- [ ] [SRS-NFR-02/AC-01] Turn output is deterministic enough for regression testing. <!-- verify: manual, SRS-NFR-02:start:end -->
