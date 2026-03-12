---
id: VDcFgn0KR
title: Routine Board Integration
type: feat
status: done
created_at: 2026-03-11T19:24:30
updated_at: 2026-03-11T20:38:26
operator-signal: 
scope: VDakm8eVW/VDcFd11nc
index: 2
started_at: 2026-03-11T20:31:52
completed_at: 2026-03-11T20:38:26
---

# Routine Board Integration

## Summary

Teach the board model and filesystem adapter to discover and persist routine
bundles alongside the existing entity graph.

## Acceptance Criteria

- [x] [SRS-02/AC-01] Board loading discovers routine bundles and exposes them through canonical board structures. <!-- verify: cargo test load_board_finds_routines --lib, SRS-02:start, proof: ac-1.log-->
- [x] [SRS-02/AC-02] Filesystem persistence writes and reloads routine bundles alongside existing entities. <!-- verify: cargo test filesystem_board_store_save_persists_routines_alongside_existing_entities --lib, SRS-02:end, proof: ac-2.log-->
- [x] [SRS-NFR-01/AC-01] Routine loading and listing remain deterministic and succeed when the board contains zero routines. <!-- verify: cargo test filesystem_routine_entity_store_lists_empty_when_no_routines_exist --lib, SRS-NFR-01:start:end, proof: ac-3.log-->
