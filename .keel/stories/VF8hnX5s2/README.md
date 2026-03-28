---
# system-managed
id: VF8hnX5s2
status: done
created_at: 2026-03-27T23:05:44
updated_at: 2026-03-27T23:41:32
# authored
title: Add Narrative Drift Tests For CLI Atlas And Turn Loop
type: feat
operator-signal:
scope: VF8hiVofm/VF8hkVhkI
index: 1
started_at: 2026-03-27T23:39:51
submitted_at: 2026-03-27T23:41:28
completed_at: 2026-03-27T23:41:32
---

# Add Narrative Drift Tests For CLI Atlas And Turn Loop

## Summary

Turn the CLI atlas and turn-loop docs claims into executable drift tests so command families and turn guidance cannot silently drift away from code.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Drift tests fail when the documented CLI family lists diverge from the canonical command catalog. <!-- verify: manual, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] Drift tests fail when the documented turn-loop command examples diverge from the canonical turn-loop projection. <!-- verify: manual, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-01/AC-01] The drift guards read focused docs fragments rather than brittle full-page snapshots. <!-- verify: manual, SRS-NFR-01:start:end, proof: ac-3.log-->
