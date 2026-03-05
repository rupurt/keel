---
id: 1vyGZdgBX
title: Hydrate Epic Problem Into Fresh Scaffolds
type: feat
status: backlog
created_at: 2026-03-05T13:49:37
updated_at: 2026-03-05T14:10:01
scope: 1vyFgR2MA/1vyFlAgHB
---

# Hydrate Epic Problem Into Fresh Scaffolds

## Summary

Hydrate authored problem text into fresh epic scaffolds so newly created epics start with real strategic context in the PRD and any remaining CLI-derived summary surface.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] `keel epic new --problem` writes authored narrative content into the PRD `## Problem Statement` section and any epic scaffold summary surface that depends on CLI strategic input. <!-- verify: cargo test -p keel epic_new_hydrates_problem_into_prd_and_summary_surface, SRS-02:start -->
- [ ] [SRS-03/AC-01] `keel epic new` leaves `Goals & Objectives` for direct PRD authoring instead of hydrating a single CLI-owned goal value. <!-- verify: cargo test -p keel epic_new_leaves_goal_table_for_direct_prd_authoring, SRS-03:start:end -->
- [ ] [SRS-04/AC-01] Template token inventory and rendering paths support the new problem-only strategic-input contract without placeholder drift or stale goal-token ownership. <!-- verify: cargo test -p keel epic_template_tokens_match_problem_only_contract, SRS-04:start:end -->
- [ ] [SRS-02/AC-02] [SRS-NFR-01/AC-01] Identical `epic new --problem` inputs yield deterministic scaffold output. <!-- verify: cargo test -p keel epic_problem_scaffold_is_deterministic, SRS-NFR-01:start:end -->
