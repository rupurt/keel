---
id: DGFEPIC01
title: Capture Epic Flow Tape Evidence
type: chore
status: backlog
created_at: 2026-03-06T00:00:00
updated_at: 2026-03-06T00:00:00
---

# Capture Epic Flow Tape Evidence

## Summary

Own the rendered evidence for the dogfood epic workflow scenario.

Tape source: `testdata/dogfood/scenarios/epic-flow.tape`

Evidence chain:
- `EVIDENCE/epic-flow.gif`
- `EVIDENCE/epic-flow.transcript.txt`
- `EVIDENCE/epic-flow.log`
- `manifest.yaml`

## Acceptance Criteria

- [ ] The tape at `testdata/dogfood/scenarios/epic-flow.tape` records the representative epic workflow on the secondary workspace. <!-- verify: vhs ../../../../scenarios/epic-flow.tape -->
- [ ] The generated transcript, log, and manifest keep the proof chain auditable from this story directory.
