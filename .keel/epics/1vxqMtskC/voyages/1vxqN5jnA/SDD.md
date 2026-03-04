# Technique Catalog Configuration And Autodetection - Software Design Description

> Design and implement a verification-technique bank with keel.toml configuration, project autodetection, and recommendation output surfaces.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage introduces a recommendation pipeline for automated verification:
1. Built-in technique catalog (including `vhs` and `llm-judge`).
2. `keel.toml` override layer.
3. Project autodetection/ranking engine.
4. Show-command projection and rendering surfaces.

The result is a configurable, deterministic bank of verification techniques that increases practical usage of advanced automation modes.

## Context & Boundaries

In scope:
- Catalog/read-model modules for verification techniques.
- Configuration load/merge path from `keel.toml`.
- Project signal detection and recommendation ranking.
- Planning-read command presentation (`epic show`, `voyage show`, `story show`).

Out of scope:
- Automatic execution scheduling of recommendations.
- Rewriting current verification executor contracts.

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Existing verification parser/executor | Internal | Ground truth for runnable technique kinds (`manual`, `vhs`, `llm-judge`, command) | current crate API |
| Board/config discovery | Internal | Locate and parse `keel.toml` | current crate API |
| Show command adapters | Internal | Surface recommendation projections to users | current crate API |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Catalog source | Seed built-in catalog in code; allow project overrides in `keel.toml`. | Gives immediate value while supporting project customization. |
| Merge precedence | `built-in` < `autodetected adjustments` < `keel.toml overrides`. | Keeps defaults usable and local config authoritative. |
| Ranking model | Score by stack signal confidence + current evidence gaps + technique applicability. | Produces actionable, context-aware recommendations. |
| Safety model | Recommendations are advisory; execution remains explicit via existing commands. | Avoids unexpected command execution risks. |

## Architecture

1. `verification_technique_catalog` module defines `TechniqueDefinition` and built-in bank.
2. `verification_technique_config` module parses and validates `keel.toml` overrides.
3. `verification_technique_detector` module infers stack signals and applicability.
4. `verification_technique_recommender` module ranks recommendations and produces rationale.
5. `planning_show` projections include recommendation payloads for epic/voyage/story views.

## Components

- Technique Catalog Model:
  - Fields: id, label, modality, default command template, prerequisites, applicable stacks, evidence artifact expectations.

- Config Override Resolver:
  - Supports enabling/disabling built-ins and adding project-specific entries.
  - Emits deterministic merged catalog and validation diagnostics.

- Project Signal Detector:
  - Detects stack markers (Rust CLI, browser/E2E, etc.).
  - Computes signal confidence used by ranking.

- Recommendation Engine:
  - Produces top-N technique suggestions with rationale and adoption snippets.

- Show Surface Adapter:
  - Renders recommendation sections and “currently unused” techniques in planning read commands.

## Interfaces

Proposed internal interfaces (indicative):
- `load_builtin_techniques() -> Vec<TechniqueDefinition>`
- `load_technique_overrides(board_dir) -> TechniqueOverrides`
- `merge_techniques(builtin, overrides) -> TechniqueCatalog`
- `detect_project_signals(repo_root) -> ProjectSignals`
- `recommend_techniques(catalog, signals, board_context) -> Vec<TechniqueRecommendation>`

## Data Flow

1. Load built-in technique catalog.
2. Parse/validate `keel.toml` overrides.
3. Merge into resolved catalog.
4. Detect project signals and board verification gaps.
5. Rank recommendations and project into show-command output.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Invalid `keel.toml` technique schema | Parse/validation failure | Emit explicit config diagnostics; fall back to built-in catalog | Fix config fields and rerun |
| Unknown override technique ID | Merge-time lookup failure | Warn and ignore invalid override entry | Correct technique ID or add custom entry definition |
| No detectable project signals | Detector returns empty/low confidence | Render conservative baseline recommendations with low-confidence label | Add explicit config overrides in `keel.toml` |
| Rendering surface missing context | Projection build failure | Render placeholder recommendation section with reason | Ensure board/config discovery succeeds |
