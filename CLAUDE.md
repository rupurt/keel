# CLAUDE.md

Guidance for **Claude Code** when working with this repository.

## Shared Contract

Before doing work, read:

1. `AGENTS.md`
2. `INSTRUCTIONS.md`
3. `POLICY.md`
4. `ARCHITECTURE.md`

Those files are the repo-wide operating contract. This file should stay thin and only capture Claude-specific harness notes.

## Project-Specific Claude Notes

<!-- BEGIN PROJECT-SPECIFIC -->
- Rebase `include_str!` and other compile-time asset paths immediately after moving source files, then run a targeted compile before continuing the refactor.
- Treat path-wide module moves as one atomic slice: move files, rewrite imports, and update architecture path fixtures and contract tests together.
- Keep template token inventories and CLI `new` argument surfaces coupled by drift tests; update both in the same change when tokenized fields move.
- For new CLI flags, update clap parsing, command-tree wiring, runtime extraction, template inputs, and parse plus persisted-artifact tests in the same slice.
- Preserve empty interior markdown table cells in planning parsers; trim boundary pipes and whitespace without collapsing column positions.
- Route goal-lineage and other planning invariants through shared invariant helpers before wiring doctor checks or read-model surfaces.
- Reuse the shared structural placeholder detector in runtime lifecycle gates instead of introducing command-local marker regexes.
- Keep implementation stories aligned to one primary SRS requirement in sequence; reserve aggregate cleanup requirements for the final story to avoid queue cycles.
<!-- END PROJECT-SPECIFIC -->
