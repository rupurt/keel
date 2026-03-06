---
id: 1vyGZeNMa
title: Parse Canonical Goal Lineage
type: feat
status: done
created_at: 2026-03-05T13:49:38
updated_at: 2026-03-05T16:44:47
scope: 1vyFgR2MA/1vyFmfjA9
started_at: 2026-03-05T16:25:41
completed_at: 2026-03-05T16:44:47
---

# Parse Canonical Goal Lineage

## Summary

Introduce the canonical goal-lineage parser so PRD goals become machine-readable planning inputs with stable `GOAL-*` identifiers and deterministic fanout behavior.

## Acceptance Criteria

- [x] [SRS-01/AC-01] PRD `Goals & Objectives` entries use canonical `GOAL-*` identifiers in a parseable table form. <!-- verify: cargo test -p keel goal_lineage_parser_reads_canonical_goal_rows, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] [SRS-NFR-01/AC-01] Equivalent PRD goal fixtures produce deterministic parsed output. <!-- verify: cargo test -p keel goal_lineage_parser_is_deterministic, SRS-NFR-01:start:end, SRS-01:end, proof: ac-2.log-->
