# Verification Technique Command Surface Cutover - Software Design Description

> Move verification-technique surfacing to config/verify commands with hard-cutover verify subcommands and machine-readable output

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage re-centers verification-technique visibility around explicit verification commands and config introspection:
1. `keel config show` becomes the canonical technique inventory view.
2. Verification execution moves to `keel verify run` (hard cutover from `keel verify`).
3. Technique selection guidance moves to `keel verify recommend`.
4. Planning read commands (`epic/voyage/story show`) stop rendering recommendations directly.

The design keeps one technique-resolution pipeline from the existing catalog/detection stack while splitting render/execution responsibilities by command intent.

## Context & Boundaries

In scope:
- `config show` rendering and command options for technique matrix + machine-readable output.
- `verify` command tree hard cutover (`run`, `recommend`) and legacy root error path.
- Recommendation filtering contract (`detected && active`) and advisory-only behavior.
- Removal of recommendation sections from planning read commands.
- Planning guidance updates in `AGENTS.md`.

Out of scope:
- `config mode` behavior and scoring policy surfaces.
- Changes to underlying technique detection algorithms beyond what is needed to expose current state.
- Automatic invocation of recommended verification techniques.

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `read_model::verification_techniques` modules | Internal | Canonical source for built-ins, custom entries, detection, and disabled state | current crate API |
| CLI command tree (`src/cli/commands`) | Internal | Route `config show`, `verify run`, `verify recommend`, and legacy error handling | current crate API |
| Existing verification executor path | Internal | Execution behavior reused by `verify run` | current crate API |
| `serde_json` output helpers | Internal | Stable machine-readable output contracts | current crate API |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Technique row identity | `label` equals hyphenated technique id. | Matches user contract and is stable for both human/JSON consumers. |
| Active flag derivation | `active = detected && !disabled` (derived, not independently configured). | Keeps one source of truth and avoids state drift. |
| Command responsibility split | `config show` = full inventory, `verify recommend` = filtered suggestions, `verify run` = execution only. | Reduces cognitive overload and clarifies intent per command. |
| Hard cutover behavior | Legacy `keel verify` fails fast with direct migration message to `keel verify run`. | Enforces single canonical path under repository hard-cutover policy. |
| JSON contract strategy | Add `--json` for all three commands with deterministic ordering keyed by hyphenated technique id. | Supports automation and stable diff/snapshot behavior. |

## Architecture

1. Reuse resolved technique inventory from existing verification-technique read-model.
2. Add a shared projection (for example `TechniqueStatusRow`) consumed by both `config show` and `verify recommend`.
3. Extend CLI routing:
   - `verify run` dispatches current execution logic.
   - `verify recommend` renders filtered advisory output.
   - bare `verify` prints hard-cutover error/help and exits non-zero.
4. Remove recommendation rendering path from planning show projections/renderers.
5. Add parallel human + JSON renderers with deterministic sorting and explicit field names.

## Components

- Technique Status Projection:
  - Inputs: merged catalog (built-in + custom), detection results, disabled set.
  - Outputs: sorted rows `{ label, detected, disabled, active, metadata }`.

- Config Show Renderer:
  - Displays full inventory (all techniques regardless of active state).
  - Removes scoring section from `config show`.
  - Adds `--json` payload for automation.

- Verify Command Router:
  - `run`: preserves current executor semantics for `<id>` and `--all`.
  - `recommend`: advisory recommendation report from active+detected rows only.
  - root `verify`: migration-only error path.

- Recommend Renderer:
  - Applies strict filter: include rows only when `detected=true` and `active=true`.
  - Provides rationale/adoption hints without execution.
  - Supports `--json` with deterministic ordering.

- Planning Show Cleanup:
  - Removes recommendation sections from epic/voyage/story show output surfaces.
  - Keeps progress/evidence summaries intact.

## Interfaces

CLI contracts:
- `keel config show [--json]`
- `keel verify run [<id> | --all] [--json]`
- `keel verify recommend [--json]`
- `keel verify` (no subcommand) -> non-zero exit + guidance: use `keel verify run`

Indicative JSON shape (field names are contractual):
- `config show --json`
  - top-level: config source/context
  - `techniques`: ordered array of `{ label, detected, disabled, active, ... }`
- `verify recommend --json`
  - `recommendations`: ordered array filtered to `detected && active`
- `verify run --json`
  - execution summary equivalent to text output (targets, statuses, artifacts)

## Data Flow

1. Command parses flags/subcommands and loads board/config context.
2. Technique bank resolution returns built-ins + custom entries + detection state.
3. Command-specific behavior:
   - `config show`: render all rows.
   - `verify recommend`: filter active+detected rows and render advisory output.
   - `verify run`: execute current verification flow and render results.
4. Optional JSON serializer emits deterministic ordered payloads.
5. Planning show commands render without recommendation blocks.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Bare `keel verify` invoked after cutover | Subcommand missing at verify root | Exit non-zero with explicit migration message (`use keel verify run`) | Update scripts/aliases to `keel verify run` |
| Technique metadata missing fields for row render | Projection validation failure | Fail command with actionable diagnostics | Correct technique definition in built-in or `keel.toml` custom entry |
| JSON requested but serialization contract violated | Serializer error in tests/runtime | Return command error; include field/path context | Fix serializer mapping and add regression test |
| `config mode` inadvertently altered by refactor | Regression tests on mode command fail | Block merge via tests | Restore mode behavior and keep cutover scoped |
| Recommendation path attempts execution | Side-effect test detects spawned command/process | Treat as regression and fail tests | Keep recommend path read-only |
