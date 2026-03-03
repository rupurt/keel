# Template And CLI Contract Canonicalization - Software Design Description

> Adopt canonical schema-mirrored template tokens and align creation commands so only user-owned fields are CLI-settable.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage introduces a strict token and CLI ownership contract.
Template changes and command-surface changes are implemented together and validated through centralized tests that fail on unknown or out-of-bucket tokens and flags.
No compatibility aliases are retained.

## Context & Boundaries

In scope:
- `templates/**` token normalization and content updates where tokens are used.
- Creation command interfaces in `src/cli/command_tree.rs` and management command adapters.
- Rendering replacements in creation paths and template contract tests.

Out of scope:
- Doctor severity/escalation logic.
- Story submit/accept gate enforcement.
- Generated report artifact validation.

```
┌─────────────────────────────────────────┐
│ Token + CLI Contract Layer              │
│                                         │
│  Templates  Renderers  CLI Definitions │
│  │         │  │         │  │         │ │
│  └─────────┘  └─────────┘  └─────────┘ │
└─────────────────────────────────────────┘
        ↑               ↑
   [Domain models] [Command runtime]
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `clap` command tree | library | Defines required/optional creation flags | existing |
| `template_rendering::render` | internal service | Canonical token substitution | existing |
| Template constants module | internal service | Compile-time embedded markdown source | existing |
| Drift/template tests | internal tests | Contract enforcement for tokens and fields | existing + expanded |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Token naming policy | Canonical names mirror schema/frontmatter field names | Keeps template vocabulary aligned with data contracts |
| Ownership buckets | CLI-owned, system-owned, generated markers only | Prevents ambiguous token semantics |
| Voyage goal input | `--goal` required at parse time | Fails fast before runtime path |
| ADR creation inputs | Add `--context` and repeatable `--applies-to` | Aligns command input with ADR frontmatter semantics |
| Compatibility strategy | Hard cutover with no aliases | Reduces long-term complexity and drift |

## Architecture

1. Update template token usage in `templates/` and replacement callsites.
2. Update CLI definitions for creation commands in command tree + runtime mapping.
3. Extend ADR creation path to accept and persist context ownership fields.
4. Add contract-focused tests:
- token inventory and bucket validation
- CLI surface and required-flag assertions
- command behavior persistence checks

## Components

| Component | Purpose | Behavior |
|-----------|---------|----------|
| Template token contract | Enforce canonical token names | Rejects legacy token names in tests |
| Creation CLI contract | Enforce user-owned input surfaces | Exposes only approved flags and required arguments |
| ADR input adapter | Maps CLI context flags to frontmatter | Writes `context` and `applies-to` values deterministically |
| Contract regression tests | Prevents drift | Fails when token or flag policy is violated |

## Interfaces

CLI contract:
- `keel epic new <name> --goal <goal>`
- `keel voyage new <name> --epic <epic-id> --goal <goal>`
- `keel story new <title> --type <type>`
- `keel bearing new <name>`
- `keel adr new <title> [--context <context>] [--applies-to <scope> ...]`

Token contract:
- System-owned values are injected by runtime renderers and not CLI-entered.
- CLI-owned values map to explicit creation flags.
- Generated marker sections stay marker-based and not token-based.

## Data Flow

1. User invokes creation command with user-owned flags.
2. Command adapter validates and maps CLI input.
3. Renderer injects CLI-owned + system-owned values into templates.
4. File artifacts are written and regenerated.
5. Contract tests verify allowed token inventory and flag ownership.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Non-canonical token appears in template | token inventory test failure | block merge/build | replace token with canonical name |
| System-owned field exposed as CLI input | command tree/behavior test failure | block merge/build | remove disallowed flag |
| Required voyage goal omitted | clap parse failure | reject command invocation | provide `--goal` |
| ADR context flags not persisted | adr command test failure | block merge/build | fix frontmatter mapping |
