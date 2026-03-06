---
id: DGFBEAR01
title: Capture Bearing Flow Tape Evidence
type: chore
status: backlog
created_at: 2026-03-06T00:00:00
updated_at: 2026-03-06T00:00:00
---

# Capture Bearing Flow Tape Evidence

## Summary

Own the rendered evidence for the dogfood bearing workflow scenario.

Tape source: `testdata/dogfood/scenarios/bearing-flow.tape`

Evidence chain:
- `EVIDENCE/bearing-flow.gif`
- `EVIDENCE/bearing-flow.transcript.txt`
- `EVIDENCE/bearing-flow.log`
- `manifest.yaml`

## Acceptance Criteria

- [ ] The tape at `testdata/dogfood/scenarios/bearing-flow.tape` records the representative bearing workflow on the secondary workspace. <!-- verify: vhs ../../../../scenarios/bearing-flow.tape -->
- [ ] The generated transcript, log, and manifest keep the proof chain auditable from this story directory.
