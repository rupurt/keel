---
# system-managed
id: VF8hnX5s2
status: backlog
created_at: 2026-03-27T23:05:44
updated_at: 2026-03-27T23:10:55
# authored
title: Add Narrative Drift Tests For CLI Atlas And Turn Loop
type: feat
operator-signal:
scope: VF8hiVofm/VF8hkVhkI
index: 1
---

# Add Narrative Drift Tests For CLI Atlas And Turn Loop

## Summary

Turn the CLI atlas and turn-loop docs claims into executable drift tests so command families and turn guidance cannot silently drift away from code.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Drift tests fail when the documented CLI family lists diverge from the canonical command catalog. <!-- verify: manual, SRS-01:start:end -->
- [ ] [SRS-02/AC-01] Drift tests fail when the documented turn-loop command examples diverge from the canonical turn-loop projection. <!-- verify: manual, SRS-02:start:end -->
- [ ] [SRS-NFR-01/AC-01] The drift guards read focused docs fragments rather than brittle full-page snapshots. <!-- verify: manual, SRS-NFR-01:start:end -->
