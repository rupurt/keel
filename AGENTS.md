# AGENTS.md

Shared guidance for AI agents working with this repository. This file can be imported
by harness-specific files (CLAUDE.md, GEMINI.md, etc.).

## Execution Workflow (Implementer)

1. **Pull Context**: Read current board health and identify bottlenecks with `just keel flow`.
2. **Claim Work**: Pull the highest-priority implementation item with `just keel next --agent`. Use `--parallel` to identify safe concurrent tasks.
3. **Open the Show Surfaces First**: Use the CLI read views as the default entry points for implementation context and clarification:
   - `just keel story show <story-id>` for the active work item, acceptance criteria, status, evidence, and story path.
   - `just keel voyage show <voyage-id>` for parent requirements, scope, drift, progress, and the rendered `SRS.md` / `SDD.md` paths.
   - `just keel epic show <epic-id>` for the problem statement, goals, requirement coverage, scope drift, and the rendered `PRD.md` path.
   - When you need full authored detail, follow the document paths shown in these views (`README.md`, `PRD.md`, `SRS.md`, `SDD.md`, and related artifacts) instead of guessing from summaries.
4. **Check Story Coherence Before Coding**: Confirm acceptance criteria are traceable and verifiable:
   - Acceptance criteria are linked to source requirements (for example `[SRS-XX/AC-YY]`).
   - Evidence strategy is clear for each criterion (test, CLI proof, or manual proof).
   - If requirements are ambiguous, loop back to the relevant `show` command and then the linked planning artifacts before implementation.
5. **Execute (TDD)**: Follow test-driven development:
   - Write a failing test first.
   - Implement only enough to pass.
   - Refactor within the same change slice.
6. **Record Evidence**: Capture proof of requirement satisfaction for each acceptance criterion:
   - `just keel story record <ID> --ac <NUM> --msg "Description of the proof"`
   - For manual proofs, use the `--msg` flag or editor integration.
7. **Reflect Selectively (Optional)**: Use `just keel story reflect <ID>` only when the work uncovered a novel, reusable insight that is likely to help future stories.
   - Start from the similar knowledge surfaced by the command. Prefer linking an existing knowledge file over creating a new one when the insight is already covered.
   - Capture only durable guidance another agent can reuse: a decision rule, failure mode, parser/rendering trap, verification lesson, or workflow guardrail.
   - Include the trigger/context, the reusable takeaway, and where it applies. The bar should be: would this help a future agent avoid drift on a different story?
   - Do not record story recap, commit summary, obvious implementation steps, or one-off cleanup details already visible in the diff, proofs, or authored artifacts.
   - If there is no reusable insight, skip reflection or leave the `## Knowledge` section empty.
8. **Submit**: Move to the human queue for review with `just keel story submit <ID>`. This triggers automated verification and generates the verification manifest. Resolve any failures and rerun `submit` until the story reaches its post-submit state.
9. **Commit (Required)**: Create exactly one atomic [Conventional Commit](https://www.conventionalcommits.org/) for this story after `submit`, not before. Include the resulting `.keel` changes from submission in the same commit (for example story status updates, manifests, synthesized knowledge, and board projections). Do not batch multiple stories into one commit.

## Planning Workflow (Architect)

1. **Identify Gaps**: Use `just keel flow` or `just keel status` to find Epics needing tactical decomposition.
2. **Scaffold Planning Unit**:
   - For new strategic work, create an Epic: `just keel epic new "<Title>" --problem "<Problem>"`
   - For tactical decomposition, create a Voyage: `just keel voyage new "<Title>" --epic <epic-id> --goal "<The specific outcome>"`
3. **Author Epic PRD Immediately After Creation**: Before decomposing into voyages/stories, fill out `epics/<epic-id>/PRD.md` with authored content for every required section:
   - `## Problem Statement`
   - `## Goals & Objectives`
   - `## Users`
   - `## Scope` (`### In Scope` and `### Out of Scope`)
   - `## Requirements` (`FUNCTIONAL_REQUIREMENTS` and `NON_FUNCTIONAL_REQUIREMENTS` marker blocks)
   - `## Verification Strategy`
   - `## Assumptions`
   - `## Open Questions & Risks`
   - `## Success Criteria` (`SUCCESS_CRITERIA` marker block)
   - Author `Goals & Objectives` with canonical `GOAL-*` rows.
   - Author both scope lists with canonical `[SCOPE-*]` bullets.
   - Keep every PRD requirement row linked to one or more valid `GOAL-*` IDs in the `Goals` column so goal coverage is explicit.
   - Optional: create `epics/<epic-id>/PRESS_RELEASE.md` only for large user-facing value shifts. Skip it for incremental improvements, refactors, and architecture-only changes.
4. **Define Scope + Requirements (SRS)**: Fill out the `SRS.md` in the new voyage bundle.
   - In `## Scope`, map the parent epic scope with canonical `[SCOPE-*]` bullets so the voyage explicitly states what it takes on and what it defers.
   - Keep requirements atomic and uniquely identified (for example `SRS-01`).
   - The `Scope` column must use one or more canonical `SCOPE-*` IDs already declared by the voyage scope mapping.
   - The `Source` column must use exactly one canonical parent PRD requirement ID (`FR-*` or `NFR-*`).
   - Write requirements so they map directly to story acceptance criteria and verification evidence.
5. **Detail Design (SDD)**: Fill out the `SDD.md` describing the architectural approach and component changes, with enough specificity that implementers can produce objective proofs.
6. **Decompose Stories**: Break the design into implementable units:
   - `just keel story new "<Title>" --epic <epic-id> --voyage <voyage-id>`
   - Link stories to SRS requirements using `[SRS-XX/AC-YY]` markers in the acceptance criteria.
7. **Align Verification Techniques From Config**: Run `just keel config show`, `just keel verify detect`, and `just keel verify recommend` before finalizing verification planning:
   - Use `just keel config show` as the full technique inventory (built-in + custom) and review each option's `detected`, `disabled`, and `active` flags.
   - Use `just keel verify detect` to review project signal detection inputs (files, hints, stack confidence) and per-technique detected/active status.
   - Use `just keel verify recommend` to plan against detected+active options for the current project.
   - If needed techniques are missing or disabled, update `keel.toml` first, then continue decomposition.
8. **Run Coherence Review (Downstream Check)**: Before planning is sealed, review the full chain:
   - Every PRD requirement row has valid `Goals` links, and every authored `GOAL-*` is covered by at least one PRD requirement.
   - Voyage scope bullets and SRS `Scope` cells use canonical parent `SCOPE-*` IDs consistently.
   - Every SRS requirement has exactly one valid parent PRD `Source` (`FR-*` or `NFR-*`) and at least one linked story acceptance criterion.
   - Every acceptance criterion has a clear verification approach (automated test, CLI proof, or documented manual evidence).
   - Verification commands align with `just keel verify recommend` output, informed by `just keel verify detect`, unless explicitly justified.
   - CLI options and authored entity content are explicit enough for downstream automation and transitions.
9. **Loop Back Upstream if Needed**: If decomposition or verification design exposes ambiguity, update PRD/SRS/SDD first, then re-check story acceptance criteria.
10. **Generate Planning Summary Report In Chat (Required)**: For every newly planned Epic or Voyage, publish a terse, actionable planning summary directly in the chat/harness response (do not create a dedicated summary file).
   - Include:
     - Objective and scope boundaries
     - Requirement-to-story coverage status
     - Verification strategy summary (how requirements will be proven)
     - Key risks/assumptions
     - Canonical next step command
11. **Seal Planning**: Promote the voyage from `draft` to `planned` with `just keel voyage plan <id>`. This validates requirement coverage and thaws stories into the agent backlog.
12. **Commit (Required)**: Create exactly one atomic [Conventional Commit](https://www.conventionalcommits.org/) for this planning unit after sealing so the resulting `.keel` state is captured in the same commit. Do not batch unrelated planning units into one commit.

## Research Workflow (Explorer)

1. **Identify Fog**: Create a new Bearing when the path forward is ambiguous or requires exploration:
   - `just keel bearing new "<Name>"`
   - **Mandatory Structure**: Always use the CLI to scaffold. A bearing must contain:
     - `README.md`: Entry point with frontmatter.
     - `BRIEF.md`: Core research details (see required sections below).
     - `SURVEY.md`: Market and technical research.
     - `ASSESSMENT.md`: Impact scoring and recommendation.
2. **Discovery (Play)**: Use `just keel play <id>` to trigger discovery sessions and explore the problem space through different "masks" or perspectives.
3. **Draft Brief**: Fill out `BRIEF.md`. The following sections are **mandatory** for `keel doctor` to pass:
   - `## Hypothesis`
   - `## Problem Space`
   - `## Success Criteria`
   - `## Open Questions`
4. **Survey Findings**: Document research, competitive landscape, and technical constraints in `SURVEY.md`.
5. **Seal Survey**: Transition to the surveying phase with `just keel bearing survey <id>`.
6. **Assess Impact**: Perform impact analysis and document recommendations (Proceed, Park, or Decline) in `ASSESSMENT.md`.
7. **Seal Assessment**: Transition to the assessing phase with `just keel bearing assess <id>`.
8. **Graduate**: If research is conclusive, graduate the bearing to a strategic Epic with `just keel bearing lay <id>`.
9. **Commit (Required)**: Create exactly one atomic [Conventional Commit](https://www.conventionalcommits.org/) for this bearing research package after the final lifecycle transition you take for it (for example `survey`, `assess`, or `lay`) so generated `.keel` artifacts are included.

## Global Hygiene Checklist

Apply these checks to **every change** before finalizing work:

1. **Doctor Check**: `just keel doctor` must pass with zero warnings or errors.
2. **Quality Check**: `just quality` must be clean (formatting and linting).
3. **Verification**: `just test` must pass 100%.
4. **Lifecycle Before Commit**: Run board-mutating lifecycle commands before the atomic commit when they generate or rewrite `.keel` artifacts (for example `story submit`, `voyage plan`, `voyage done`, `bearing assess`, `bearing lay`). After the transition, inspect `git status` and include the resulting `.keel` churn in the same commit.
5. **Atomic Commits**: Commit once per logical unit of work. Use [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat:` (new feature)
   - `fix:` (bug fix)
   - `docs:` (documentation)
   - `refactor:` (code change, no behavior change)
   - `test:` (adding/updating tests)
   - `chore:` (build/tooling)
6. **Knowledge Quality Bar**: Prefer no new knowledge over low-signal knowledge. A new knowledge entry should be novel, reusable across stories, and materially reduce future drift; otherwise link existing knowledge or omit capture entirely.

## Compatibility Policy (Hard Cutover)

At this stage of development, this repository uses a **hard cutover** policy by default.

1. **No Backward Compatibility by Default**: Do not add compatibility aliases, dual-write logic, soft-deprecated schema fields, or fallback parsing for legacy formats unless a story explicitly requires it.
2. **Replace, Don’t Bridge**: When introducing a new canonical token, field, command behavior, or document contract, remove the old path in the same change slice.
3. **Fail Fast in Validation**: `keel doctor` and transition gates should treat legacy or unfilled scaffold patterns as hard failures when they violate the new contract.
4. **Single Canonical Path**: Keep one source of truth for rendering, parsing, and validation; avoid parallel implementations meant only to preserve old behavior.
5. **Migration Is Explicit Work**: If existing board artifacts need updates, handle that in a dedicated migration pass/story instead of embedding runtime compatibility logic.

## Foundational Documents

These define constraints and workflow:

- `README.md` — entrypoint and canonical document navigation.
- `.keel/adrs/` — binding architecture decisions.
- `CONSTITUTION.md` — collaboration philosophy and decision hierarchy.
- `ARCHITECTURE.md` — implementation architecture, state machines, and flow model.

Use this order when interpreting constraints: ADRs → Constitution → Architecture → Planning artifacts.

## Project Overview

This repository is the `keel` Rust crate — a CLI for agentic SDLC management.

| Path | Purpose |
|------|---------|
| `Cargo.toml` | Crate manifest (single `[[bin]]` target) |
| `src/` | All Rust source organized by layer roots: `cli`, `application`, `domain`, `infrastructure`, `read_model` |
| `justfile` | Task runner recipes (build, test, quality, coverage) |
| `flake.nix` | Nix dev environment |

### Board directory (`.keel/`)

A `.keel/` directory is the runtime data directory that `keel` operates on. It lives
in the project being managed (not in this repository).

| Path | Contents |
|------|----------|
| `.keel/adrs/` | Architecture decision records |
| `.keel/epics/` | Epic-level planning artifacts (PRDs) |
| `.keel/epics/<epic-id>/voyages/` | Voyage planning artifacts (SRS + SDD) |
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
| Planning | `just keel epic new <name> --problem <problem>` `just keel voyage new <name> --epic <epic-id> --goal <goal>` |
| Execution | `just keel story new <title> --epic <epic-id> --voyage <voyage-id>` |
| Board Ops | `just keel next --agent` `just keel next` `just keel status` `just keel flow` `just keel doctor` `just keel generate` `just keel config show` |
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
