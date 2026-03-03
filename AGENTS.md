# AGENTS.md

Shared guidance for AI agents working with this repository. This file can be imported
by harness-specific files (CLAUDE.md, GEMINI.md, etc.).

## Execution Workflow (Implementer)

1. **Pull Context**: Read current board health and identify bottlenecks with `just keel flow`.
2. **Claim Work**: Pull the highest-priority implementation item with `just keel next --agent`. Use `--parallel` to identify safe concurrent tasks.
3. **Execute (TDD)**: Follow test-driven development:
   - Write a failing test first.
   - Implement only enough to pass.
   - Refactor within the same change slice.
4. **Record Evidence**: Capture proof of requirement satisfaction for each acceptance criterion:
   - `just keel story record <ID> --ac <NUM> --msg "Description of the proof"`
   - For manual proofs, use the `--msg` flag or editor integration.
5. **Reflect**: Mandatory observational capture. Run `just keel story reflect <ID>` and document what was learned or discovered during implementation.
6. **Submit**: Move to the human queue for review with `just keel story submit <ID>`. This triggers automated verification and generates the verification manifest.

## Planning Workflow (Architect)

1. **Identify Gaps**: Use `just keel flow` or `just keel status` to find Epics needing tactical decomposition.
2. **Scaffold Voyage**: Create a new tactical unit:
   - `just keel voyage new "<Title>" --epic <epic-id> --goal "<The specific outcome>"`
3. **Define Requirements (SRS)**: Fill out the `SRS.md` in the new voyage bundle. Ensure requirements are atomic and uniquely identified (e.g., `SRS-01`).
4. **Detail Design (SDD)**: Fill out the `SDD.md` describing the architectural approach and component changes.
5. **Decompose Stories**: Break the design into implementable units:
   - `just keel story new "<Title>" --epic <epic-id> --voyage <voyage-id>`
   - Link stories to SRS requirements using `[SRS-XX/AC-YY]` markers in the acceptance criteria.
6. **Seal Planning**: Promote the voyage from `draft` to `planned` with `just keel voyage plan <id>`. This validates requirement coverage and thaws stories into the agent backlog.

## Research Workflow (Explorer)

1. **Identify Fog**: Create a new Bearing when the path forward is ambiguous or requires exploration:
   - `just keel bearing new "<Name>"`
2. **Discovery (Play)**: Use `just keel play <id>` to trigger discovery sessions and explore the problem space through different "masks" or perspectives.
3. **Survey Findings**: Document research, competitive landscape, and technical constraints in `SURVEY.md`.
4. **Seal Survey**: Transition to the surveying phase with `just keel bearing survey <id>`.
5. **Assess Impact**: Perform impact analysis and document recommendations (Proceed, Park, or Decline) in `ASSESSMENT.md`.
6. **Seal Assessment**: Transition to the assessing phase with `just keel bearing assess <id>`.
7. **Graduate**: If research is conclusive, graduate the bearing to a strategic Epic with `just keel bearing lay <id>`.

## Global Hygiene Checklist

Apply these checks to **every change** before finalizing work:

1. **Doctor Check**: `just keel doctor` must pass with zero warnings or errors.
2. **Quality Check**: `just quality` must be clean (formatting and linting).
3. **Verification**: `just test` must pass 100%.
4. **Atomic Commits**: Commit once per logical unit of work. Use [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat:` (new feature)
   - `fix:` (bug fix)
   - `docs:` (documentation)
   - `refactor:` (code change, no behavior change)
   - `test:` (adding/updating tests)
   - `chore:` (build/tooling)

## Foundational Documents

These define constraints and workflow:

- `MANIFESTO.md` — collaboration philosophy and decision hierarchy.
- `ARCHITECTURE.md` — architecture, state machines, and flow model.

## Project Overview

This repository is the `keel` Rust crate — a CLI for agentic SDLC management.

| Path | Purpose |
|------|---------|
| `Cargo.toml` | Crate manifest (single `[[bin]]` target) |
| `src/` | All Rust source organized by layer roots: `cli`, `application`, `domain`, `infrastructure`, `read_model` |
| `templates/` | Markdown templates for ADRs, epics, voyages, stories, and bearings |
| `justfile` | Task runner recipes (build, test, quality, coverage) |
| `flake.nix` | Nix dev environment |

### Templates

The `templates/` directory contains markdown scaffolds that are **embedded into the
binary at compile time** via `include_str!()` in `src/infrastructure/templates.rs`. When the CLI
creates a new entity (`story new`, `epic new`, `voyage new`, `bearing new`, `adr new`),
it renders the matching template by replacing `{{placeholder}}` tokens with actual
values. If you edit a template file, the binary must be recompiled for changes to
take effect.

| Template | Created by |
|----------|------------|
| `templates/stories/STORY.md` | `keel story new` |
| `templates/epic/README.md`, `templates/epic/PRD.md` | `keel epic new` |
| `templates/epic/voyages/README.md`, `SRS.md`, `SDD.md` | `keel voyage new` |
| `templates/bearings/BRIEF.md` | `keel bearing new` |
| `templates/bearings/SURVEY.md`, `ASSESSMENT.md` | bearing state transitions |
| `templates/adrs/ADR.md` | `keel adr new` |

### Board directory (`.keel/`)

A `.keel/` directory is the runtime data directory that `keel` operates on. It lives
in the project being managed (not in this repository).

| Path | Contents |
|------|----------|
| `.keel/adrs/` | Architecture decision records |
| `.keel/epics/` | Epic-level planning artifacts (PRDs) |
| `.keel/voyages/` | Voyage planning artifacts (SRS + SDD) |
| `.keel/stories/` | Implementable work items |
| `.keel/README.md` | Board state overview (auto-generated) |

## Commands

### Command execution model (DRY)

Use one path for each concern:

- `just ...` for repo/build/test workflows.
- `just keel ...` for all board/workflow operations.

### `just` workflow commands

| Command | Purpose |
|---------|---------|
| `just` | List available recipes |
| `just setup` | Install helper tooling (`cargo-nextest`, `cargo-llvm-cov`) |
| `just build` | Alias to `just build-debug` |
| `just build-debug` | Build debug artifact and copy to `target/debug/keel` |
| `just build-release` | Build release artifact and copy to `target/release/keel` |
| `just run` | Run the CLI |
| `just test` | Run test suite (uses nextest if available) |
| `just quality` | Run formatting and clippy checks |
| `just coverage` | Produce `coverage/lcov.info` |
| `just pre-commit` | Run quality + tests |

### `just keel` board workflow commands

Run `just keel --help` for the full command tree. The core commands you should rely on:

| Category | Commands |
|----------|----------|
| Discovery | `just keel bearing new <name>` `just keel bearing survey <id>` `just keel bearing assess <id>` `just keel bearing list` |
| Planning | `just keel epic new <name> --goal <goal>` `just keel voyage new <name> --epic <epic-id> --goal <goal>` |
| Execution | `just keel story new <title> --epic <epic-id> --voyage <voyage-id>` |
| Board Ops | `just keel next --agent` `just keel next` `just keel status` `just keel flow` `just keel doctor` `just keel generate` |
| Lifecycle | Story/voyage/epic transitions in the table below |

## Story and Milestone State Changes

Use CLI commands only; do not move `.keel` files manually.

| Action | Command |
|--------|---------|
| Start | `just keel story start <id>` |
| Reflect | `just keel story reflect <id>` |
| Submit | `just keel story submit <id>` |
| Reject | `just keel story reject <id> "reason"` |
| Accept | `just keel story accept <id>` |
| Ice | `just keel story ice <id>` |
| Thaw | `just keel story thaw <id>` |
| Voyage done | `just keel voyage done <id>` |
