---
id: 1vyGZd1to
title: Replace Epic Goal CLI With Problem Input
type: feat
status: done
created_at: 2026-03-05T13:49:37
updated_at: 2026-03-05T16:44:26
scope: 1vyFgR2MA/1vyFlAgHB
started_at: 2026-03-05T16:07:24
completed_at: 2026-03-05T16:44:26
---

# Replace Epic Goal CLI With Problem Input

## Summary

Replace the current epic-creation CLI contract so authored problem text is the only strategic input collected at scaffold time and CLI-owned goal hydration is removed.

## Acceptance Criteria

- [x] [SRS-01/AC-01] `keel epic new` accepts a required `--problem` argument and rejects missing or empty problem values during CLI/runtime validation. <!-- verify: cargo test -p keel cli_parses_epic_new_with_required_problem, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] [SRS-NFR-02/AC-01] The new CLI path fails fast instead of injecting defaults or placeholder strategic text when required input is absent. <!-- verify: cargo test -p keel epic_new_problem_input_fails_fast_without_defaults, SRS-NFR-02:start:end, SRS-01:end, proof: ac-2.log-->
