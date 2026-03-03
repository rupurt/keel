---
id: 1vxH84jzB
title: Add Hard Cutover Regression Coverage
type: chore
status: done
created_at: 2026-03-02T20:13:04
updated_at: 2026-03-03T11:52:36
scope: 1vxGy5tco/1vxGzVpw5
index: 4
started_at: 2026-03-03T10:55:35
submitted_at: 2026-03-03T11:50:46
completed_at: 2026-03-03T11:52:36
---

# Add Hard Cutover Regression Coverage

## Summary

Add regression coverage that enforces hard-cutover behavior across doctor and transition gates with no warning-oriented legacy expectations.

## Acceptance Criteria

- [x] [SRS-06/AC-01] Add regression tests proving doctor and transition paths enforce hard errors for unresolved scaffold/default text. <!-- verify: cargo test -p keel validate_detects_terminal_story_scaffold_text, SRS-06:start, proof: ac-1.log-->
- [x] [SRS-06/AC-02] Replace legacy warning-oriented expectations with hard-failure assertions. <!-- verify: cargo test -p keel evaluate_story_submit_blocks_unresolved_readme_scaffold, SRS-06:continues, proof: ac-2.log-->
- [x] [SRS-06/AC-03] Ensure updated suites remain green under `just quality` and `just test`. <!-- verify: manual, SRS-06:end, proof: ac-3.log -->
