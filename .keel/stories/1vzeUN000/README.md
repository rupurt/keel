---
id: 1vzeUN000
title: Board Loader Mission Integration
type: feat
status: done
created_at: 2026-03-09T10:33:55
updated_at: 2026-03-09T13:40:40
scope: 1vzeJF000/1vzeMk000
index: 3
started_at: 2026-03-09T13:34:48
completed_at: 2026-03-09T13:40:40
---

# Board Loader Mission Integration

## Summary

Integrate Mission into the Board struct and loader. Add missions HashMap to Board,
implement load_missions() that discovers and parses .keel/missions/*/README.md,
and wire it into load_board().

## Acceptance Criteria

- [x] [SRS-06/AC-01] Board struct has `missions: HashMap<String, Mission>` field <!-- verify: cargo test --lib infrastructure::loader::tests::load_board_has_missions_field, SRS-06:start:end, proof: ac-1.log-->
- [x] [SRS-07/AC-01] load_missions() discovers all .keel/missions/*/README.md files and parses them <!-- verify: cargo test --lib infrastructure::loader::tests::load_board_finds_missions, SRS-07:start, proof: ac-2.log-->
- [x] [SRS-07/AC-02] load_board() calls load_missions() and populates Board.missions <!-- verify: cargo test --lib infrastructure::loader::tests::load_board_populates_missions, proof: ac-3.log-->
- [x] [SRS-07/AC-03] Malformed mission files are skipped with warning, not fatal <!-- verify: cargo test --lib infrastructure::loader::tests::load_board_skips_malformed_missions, SRS-07:end, proof: ac-4.log-->
- [x] [SRS-08/AC-01] CHARTER.md scaffold has Goals table with MG-XX ID, Description, Verification columns <!-- verify: cargo test --lib infrastructure::templates::tests::mission_charter_has_goals_table, SRS-08:start, proof: ac-5.log-->
- [x] [SRS-08/AC-02] CHARTER.md scaffold has Constraints section <!-- verify: cargo test --lib infrastructure::templates::tests::mission_charter_has_constraints_section, proof: ac-6.log-->
- [x] [SRS-08/AC-03] CHARTER.md scaffold has Halting Rules section <!-- verify: cargo test --lib infrastructure::templates::tests::mission_charter_has_halting_rules_section, SRS-08:end, proof: ac-7.log-->
