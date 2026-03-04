# Planning Read Surfaces And Evidence Visibility - Software Design Description

> Make epic/voyage/story show outputs planning-ready, verification-aware, and acceptance-friendly.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage introduces a shared read-model projection for planning and evidence visibility, then uses it to re-render:
- `keel epic show` as a planning summary with progress, verification readiness, and ETA signals.
- `keel voyage show` as a requirement coverage/progress view.
- `keel story show` as a concrete evidence report with proof metadata and artifact visibility.

The design favors one canonical extraction/rendering path and deterministic terminal output suitable for human review and harness consumption.

## Context & Boundaries

In-bounds components:
- `src/cli/commands/management/epic/show.rs`
- `src/cli/commands/management/voyage/show.rs`
- `src/cli/commands/management/story/show.rs`
- New shared planning/evidence read-model and formatting helpers

External/adjacent data sources:
- Epic PRD markdown (`.keel/epics/<id>/PRD.md`)
- Voyage SRS markdown (`SRS.md`)
- Story AC verify annotations + `EVIDENCE/` + `manifest.yaml`
- Board throughput/completion timestamps for ETA derivation

Out of bounds:
- `verify` execution behavior
- Story lifecycle transitions
- Artifact storage format changes

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `load_board` aggregate | Internal | Canonical access to epics/voyages/stories | current crate API |
| `parse_verify_annotations` | Internal | Parse AC verification mode, requirement refs, proof links | current crate API |
| `parse_requirements` + traceability read-models | Internal | Requirement ID extraction and story mapping | current crate API |
| Throughput projection (`flow::throughput` / history model) | Internal | Estimate epic time-to-complete from observed velocity | current crate API |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Shared projection | Add a single planning/evidence projection module consumed by all three show commands. | Prevents drift and enables reuse for future chat summary surfaces. |
| Markdown extraction strategy | Parse authored sections/tables with explicit fallback placeholders for scaffold-only content. | Maintains usefulness on partially-authored planning docs. |
| Verification rollup | Classify AC evidence as automated/manual/missing, aggregate artifact inventory (including media extensions), and suggest additional automated verification techniques from project signals. | Surfaces acceptance readiness and raises verification quality directly in `show` commands. |
| ETA signal | Estimate epic completion using remaining stories divided by the most recent 4-week average stories/week; degrade when denominator is zero. | Gives directional planning signal without false precision and uses a consistent short-horizon window. |
| Evidence preview limits | Show up to 10 lines for text proofs; for media artifacts provide whole-asset playback guidance from the terminal when supported. | Keeps text output concise while still making visual artifacts directly reviewable. |
| Contract testing | Snapshot-style CLI output tests and deterministic ordering assertions. | Prevents regressions in user-visible command output. |

## Architecture

1. Add a read-model layer (for example `read_model::planning_show`) that produces:
   - `EpicShowProjection`
   - `VoyageShowProjection`
   - `StoryShowProjection`
2. Projection builders consume board aggregates plus artifact/document parsing helpers.
3. Show commands become thin renderers that print sections from projections.
4. Rendering helpers provide consistent section headers, status labels, and placeholders.

## Components

- Planning Document Extractor:
  - Reads PRD/SRS markdown and extracts authored problem/goal/scope/requirements/verification text.
  - Distinguishes scaffold placeholders from authored content.

- Requirement Progress Mapper:
  - Maps voyage SRS requirement IDs to linked stories via AC references.
  - Computes coverage states (unmapped, in-progress, done/verified).

- Evidence Surface Builder:
  - Joins AC verify annotations to proof files in `EVIDENCE/`.
  - Reads proof frontmatter metadata (`recorded_at`, `mode`, `command`) when present.
  - Classifies artifacts by type (text/log, media, supplemental), computes missing-proof flags, and emits playback commands/fallback hints for media.

- Verification Recommendation Builder:
  - Detects project stack/tooling signals from repository files (for example Rust crates, web E2E harnesses, snapshot/fuzz tooling).
  - Produces ranked automated verification recommendations suitable for epic-level planning output.

- Progress/ETA Calculator:
  - Computes epic progress from voyage/story completion counts.
  - Estimates completion duration from recent throughput with explicit confidence/fallback messaging.

## Interfaces

Proposed internal interfaces (names indicative):
- `project_epic_show(board, epic_id) -> EpicShowProjection`
- `project_voyage_show(board, voyage_id) -> VoyageShowProjection`
- `project_story_show(board, story_id) -> StoryShowProjection`
- `render_epic_show(projection)` / `render_voyage_show(projection)` / `render_story_show(projection)`

Each projection should expose:
- Planning summary sections
- Progress metrics
- Verification summary
- Deterministically ordered requirement/story/artifact collections
- Placeholder reasons when data is missing
- Recommended automated verification additions (when inferable)

## Data Flow

1. CLI command resolves entity ID and loads board.
2. Projection builder reads companion docs/artifacts from filesystem.
3. Projection normalizes and aggregates planning/evidence/progress data.
4. Renderer prints deterministic sections for terminal consumption.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| PRD/SRS file missing or unreadable | File read fails | Show section with explicit "not available" message; keep command successful when possible | Author/fix missing planning file |
| Requirement table malformed | Parse yields zero/partial IDs | Render warning placeholder and continue with available data | Repair SRS table markers/rows |
| Proof file referenced but missing | Annotation proof path absent in `EVIDENCE/` | Render missing-proof warning for affected AC | Re-run `story record` or repair proof reference |
| Throughput insufficient for ETA | Zero completed stories in 4-week window | Render "insufficient throughput data (4-week window)" ETA state | Complete stories or widen history window |
| Media playback tool unavailable | No supported terminal/media helper discovered | Render artifact path plus explicit command suggestion for local playback | Install/enable supported viewer tooling |
