---
id: DGFSMOKE01
title: Capture Smoke Flow Tape Evidence
type: chore
status: backlog
created_at: 2026-03-06T00:00:00
updated_at: 2026-03-06T00:00:00
---

# Capture Smoke Flow Tape Evidence

## Summary

Own the rendered evidence for the harness smoke scenario.

Tape source: `testdata/dogfood/scenarios/smoke-flow.tape`

Evidence chain:
- `EVIDENCE/smoke-flow.gif`
- `EVIDENCE/smoke-flow.transcript.txt`
- `EVIDENCE/smoke-flow.log`
- `manifest.yaml`

## Acceptance Criteria

- [ ] The tape at `testdata/dogfood/scenarios/smoke-flow.tape` exercises the shared VHS harness on the secondary workspace. <!-- verify: vhs ../../../../scenarios/smoke-flow.tape -->
- [ ] The generated transcript, log, and manifest keep the proof chain auditable from this story directory.
