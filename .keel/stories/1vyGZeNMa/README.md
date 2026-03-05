---
id: 1vyGZeNMa
title: Parse Canonical Goal Lineage
type: feat
status: backlog
created_at: 2026-03-05T13:49:38
updated_at: 2026-03-05T14:10:01
scope: 1vyFgR2MA/1vyFmfjA9
---

# Parse Canonical Goal Lineage

## Summary

Introduce the canonical goal-lineage parser so PRD goals become machine-readable planning inputs with stable `GOAL-*` identifiers and deterministic fanout behavior.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] PRD `Goals & Objectives` entries use canonical `GOAL-*` identifiers in a parseable table form. <!-- verify: cargo test -p keel goal_lineage_parser_reads_canonical_goal_rows, SRS-01:start -->
- [ ] [SRS-01/AC-02] [SRS-NFR-01/AC-01] Equivalent PRD goal fixtures produce deterministic parsed output. <!-- verify: cargo test -p keel goal_lineage_parser_is_deterministic, SRS-NFR-01:start:end -->
