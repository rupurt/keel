# Tape-Driven Dogfood Workflow Suite - Software Design Description

> Add a local opt-in VHS dogfood suite on a dedicated secondary board that proves representative epic and bearing workflows and records manifest-linked artifacts.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage introduces a secondary, checked-in dogfood workspace that keel can drive like a real project board. A single local runner resets that workspace, executes authored VHS tapes for representative epic and bearing flows, and persists rendered evidence under dogfood stories so the proof chain stays inside keel's normal verification model.

Phase 1 stays deterministic by pairing rendered tape output with companion text artifacts. The rendered tape proves the real CLI experience exists; the companion transcript/log keeps machine validation stable enough to trust locally.

## Context & Boundaries

In scope:
- Secondary workspace and board layout for dogfood scenarios.
- Tape runner/orchestration for epic and bearing flows.
- Evidence capture into story `EVIDENCE/` plus manifest generation.

Out of scope:
- Semantic judging of artifacts.
- Default CI enforcement.
- Broader scenario expansion beyond the representative flows.

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Existing board discovery and workflow commands | Internal | Run real keel planning/execution flows inside the secondary workspace | current crate API |
| Verification executor + manifest generation | Internal | Persist dogfood evidence through the canonical proof path | current crate API |
| `vhs` / `ffmpeg` | External tooling | Render tape-driven CLI evidence | current dev shell |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Workspace location | Keep a checked-in secondary workspace inside the repo, separate from the primary `.keel` board. | Avoids mutating the main board while keeping scenario assets reviewable in git. |
| Reset model | Reset the secondary workspace in place through a targeted harness path rather than copying the entire repository for each run. | Matches the IO constraint and keeps the local suite fast enough to use. |
| Evidence model | Capture both rendered VHS output and companion transcript/log artifacts. | Gives us real rendering proof plus deterministic local assertions. |
| Coverage model | Represent epic and bearing flows as authored dogfood stories on the secondary board. | Keeps the e2e suite inside keel's own planning and verification language. |

## Architecture

1. Secondary workspace fixture rooted under repository testdata/examples with its own `.keel` board.
2. Dogfood runner that resets the workspace and executes named VHS scenarios.
3. Tape assets grouped by workflow (`epic-flow`, `bearing-flow`) and bound to dogfood stories.
4. Evidence sink that writes rendered artifacts and transcripts into story `EVIDENCE/`.
5. Manifest path that hashes those artifacts so keel can judge or audit them later.

## Components

- Secondary Workspace:
  - Contains a real `.keel` board plus any minimal project files required by the scenarios.
  - Owns dogfood stories whose acceptance criteria reference VHS and companion artifacts.

- Dogfood Runner:
  - Provides the single opt-in entrypoint for phase 1.
  - Resets workspace state, runs scenario tapes, and reports failure context.

- Tape Scenario Pack:
  - Holds authored `.tape` files for epic and bearing flows.
  - Encodes stable terminal dimensions, timings, and working directories.

- Evidence Bridge:
  - Stores rendered VHS output and companion text artifacts under `EVIDENCE/`.
  - Triggers or reuses manifest generation so artifacts enter the canonical proof chain.

## Interfaces

Expected interfaces:
- `just e2e-vhs` or equivalent local entrypoint for running the full phase 1 suite.
- Scenario-level runner API/command that can execute a named tape against the secondary workspace.
- Story verification annotations that reference the recorded tape and companion artifacts.

## Data Flow

1. Reset the secondary workspace to its canonical fixture state.
2. Run the selected VHS tape from inside that workspace.
3. Capture rendered output plus companion transcript/log artifacts.
4. Write artifacts into the target dogfood story `EVIDENCE/`.
5. Regenerate or validate the story manifest so artifact hashes are recorded.
6. Report scenario pass/fail with enough context to rerun or inspect manually.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Secondary workspace is dirty before a run | Reset guard or diff check fails | Abort with explicit dirty-path output | Reset the fixture workspace and rerun |
| Tape execution fails | `vhs` exits non-zero | Surface tape name, stderr, and partial artifact paths | Fix tape/runtime issue and rerun the scenario |
| Evidence artifacts missing after tape completion | Post-run artifact assertions fail | Mark scenario failed and name the missing artifact | Fix the capture path and rerun |
| Primary board mutation detected | Guard diff sees changes under the root `.keel` | Abort and report offending paths | Correct working directory/reset logic before retrying |
