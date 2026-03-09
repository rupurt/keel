---
id: 1vzeUN000
title: Board Loader Mission Integration
type: feat
status: backlog
created_at: 2026-03-09T10:33:55
updated_at: 2026-03-09T13:22:51
scope: 1vzeJF000/1vzeMk000
index: 3
---

# Board Loader Mission Integration

## Summary

Integrate Mission into the Board struct and loader. Add missions HashMap to Board,
implement load_missions() that discovers and parses .keel/missions/*/README.md,
and wire it into load_board().

## Acceptance Criteria

- [ ] [SRS-06/AC-01] Board struct has `missions: HashMap<String, Mission>` field <!-- verify: test --> <!-- SRS-06:start:end -->
- [ ] [SRS-07/AC-01] load_missions() discovers all .keel/missions/*/README.md files and parses them <!-- verify: test --> <!-- SRS-07:start:end -->
- [ ] [SRS-07/AC-02] load_board() calls load_missions() and populates Board.missions <!-- verify: test --> <!-- SRS-07:start:end -->
- [ ] [SRS-07/AC-03] Malformed mission files are skipped with warning, not fatal <!-- verify: test --> <!-- SRS-07:start:end -->
- [ ] [SRS-08/AC-01] CHARTER.md scaffold has Goals table with MG-XX ID, Description, Verification columns <!-- verify: test --> <!-- SRS-08:start:end -->
- [ ] [SRS-08/AC-02] CHARTER.md scaffold has Constraints section <!-- verify: test --> <!-- SRS-08:start:end -->
- [ ] [SRS-08/AC-03] CHARTER.md scaffold has Halting Rules section <!-- verify: test --> <!-- SRS-08:start:end -->
