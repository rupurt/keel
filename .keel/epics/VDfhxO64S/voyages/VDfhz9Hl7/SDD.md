# Canonicalize Voyage Artifact Ordering - Software Design Description

> Make repeated voyage artifact syncs byte-stable across equivalent board states and repeated runs.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage hardens the existing voyage artifact generators rather than replacing them. The design keeps the current report templates and document contracts, but normalizes every unstable traversal point before content is rendered or written.

## Context & Boundaries

### In Scope

- `src/infrastructure/generate/mod.rs`
- `src/infrastructure/generate/voyage_report.rs`
- `src/infrastructure/generate/compliance_report.rs`
- targeted generator regression tests

### Out of Scope

- `BoardGraph` frontier selection
- new generated artifact types
- report schema redesign

```
┌─────────────────────────────────────────┐
│              This Voyage                │
│                                         │
│  canonical sync → canonical render →   │
│  deterministic regression proof         │
└─────────────────────────────────────────┘
        ↑               ↑
   lifecycle sync   generated reports
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Rust stdlib ordering primitives | library | Replace unstable traversal and enumeration with canonical ordering | stable |
| Existing template rendering and artifact IO helpers | internal | Preserve current output contract while changing only ordering and normalization | current repo |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Canonical sync order | Sort epics and voyages before generator execution | Prevent HashMap traversal order from leaking into generation behavior |
| Proof ordering | Sort discovered evidence filenames before rendering links | `read_dir` order is nondeterministic and directly visible in reports |
| Output contract | Keep current filenames and markdown layout | This mission is about determinism, not a report redesign |

## Architecture

Generation remains layered the same way: lifecycle commands call board sync, board sync dispatches to voyage artifact generators, and each generator renders text through the existing template and artifact IO helpers. The change is to move all iteration over unordered sources behind explicit canonical sorting before text is assembled.

## Components

- `sync_board_artifacts`
  Purpose: orchestrates board-wide artifact refresh.
  Behavior: sorts epics and voyages before dispatching generator work.
- `voyage_report`
  Purpose: assemble narrative report content.
  Behavior: sorts stories and evidence filenames before rendering proof lists.
- `compliance_report`
  Purpose: assemble requirement-to-proof traceability matrix.
  Behavior: sorts stories, proof filenames, and rendered requirement coverage rows deterministically.

## Interfaces

No external interfaces change. The public generator functions keep their current signatures; this slice changes internal ordering only.

## Data Flow

1. A lifecycle command or test loads a `Board`.
2. `sync_board_artifacts` visits epics and voyages in canonical order.
3. Voyage generators gather stories and proof artifacts, normalizing ordering before rendering.
4. `write_if_changed` writes only when the byte output actually differs.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Missing or unreadable evidence directory | Filesystem checks fail during generation | Skip absent proof files just as today; do not invent placeholder output | Deterministic output still holds because ordering only applies to discovered files |
| Generator order regression | Targeted determinism tests fail | Block the story and surface the unstable path | Fix sorting and normalization before submission |
| Output contract drift | Existing generator tests or doctor fail | Treat as regression | Re-align implementation with current report schema |
