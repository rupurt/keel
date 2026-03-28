# AGENTS.md

Shared guidance for AI agents working with {{project_name}}.

## Downstream Contract

This repository uses Keel as its project-management engine. This file is downstream from Keel and should remain recognizable when upstream engine guidance changes.

`AGENTS.md` and `INSTRUCTIONS.md` are the sync-sensitive files in this scaffold. When you absorb a newer Keel version, preserve the `PROJECT-SPECIFIC` blocks instead of rewriting the whole file from memory.

## Read This First

1. `INSTRUCTIONS.md` for the repo's procedural turn loop.
2. `POLICY.md` for local operational invariants.
3. `ARCHITECTURE.md` and `USER_GUIDE.md` for product and system context.
4. `keel turn`, `keel mission next --status`, and `keel doctor --status` for the live board state.

## Core Principles

- Use Keel as the canonical planning and lifecycle surface.
- Prefer explicit proof over chat-only claims.
- Close loop debt with sealing commits instead of leaving dirty work behind.
- Escalate only when the work requires human product, design, legal, or operational judgment.

## Project-Specific Conventions

<!-- BEGIN PROJECT-SPECIFIC -->
- Hydrate stack-specific commands, runtime surfaces, and review constraints here.
- Add repo-local wrappers such as `just ...`, `bin/...`, or deploy helpers here.
- Add the local proof contract if this project must validate a runtime, UX, or business claim.
<!-- END PROJECT-SPECIFIC -->

## Sync Notes

- Upstream source: Keel's `AGENTS.md`
- Preserve the project-specific block above during syncs.
- Push detailed workflow rules into `INSTRUCTIONS.md`, not this file.
