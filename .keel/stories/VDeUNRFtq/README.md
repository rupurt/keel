---
id: VDeUNRFtq
title: Wire Voyage Completion as a Real Event Path
type: feat
status: done
created_at: 2026-03-12T04:35:23
updated_at: 2026-03-12T04:56:25
operator-signal: 
scope: VDeRV9CAo/VDeUIiB3Q
index: 3
started_at: 2026-03-12T04:51:49
completed_at: 2026-03-12T04:56:25
---

# Wire Voyage Completion as a Real Event Path

## Summary

Make voyage completion a real end-to-end event path consumed by an explicit
reactor, and document the resulting reactor ownership rules in the architecture
surfaces.

## Acceptance Criteria

- [x] [SRS-03/AC-01] Voyage completion is emitted end-to-end and consumed by an explicit reactor that preserves current epic-finalization behavior. <!-- verify: cargo test voyage_completed_event --lib, SRS-03:start:end, proof: ac-1.log-->
- [x] [SRS-04/AC-01] Architecture documentation states that reactors live in the application layer and preserve current CLI semantics. <!-- verify: llm-judge, SRS-04:start:end, proof: ac-2.log-->
