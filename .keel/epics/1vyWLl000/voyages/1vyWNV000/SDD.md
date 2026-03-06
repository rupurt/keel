# Artifact-Aware Judge Contract - Software Design Description

> Upgrade llm-judge so it evaluates artifact bundles from dogfood runs through a provider-agnostic contract instead of judging only git diff text.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage replaces the current diff-based `llm-judge` stub with an artifact-aware execution path. Keel will package acceptance-criterion context plus story evidence into a deterministic bundle, invoke an external `llm-judge` contract with that bundle, and persist the resulting transcript as canonical evidence.

The design intentionally keeps provider choice outside keel. Keel owns bundle construction, command invocation, and evidence persistence; the external judge wrapper owns provider-specific prompting, transport, and credentials.

## Context & Boundaries

In scope:
- Artifact-bundle schema and materialization.
- External judge invocation contract.
- Verify/record evidence persistence for judge outputs.

Out of scope:
- New tape scenarios.
- Provider-specific client code inside the keel crate.
- CI rollout for the judge path.

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Existing verification executor/reporter | Internal | Host the new judge execution path and surface results | current crate API |
| Story evidence + manifest model | Internal | Source artifact references and persist transcripts | current crate API |
| External `llm-judge` executable/wrapper | External tooling | Perform provider-specific artifact evaluation | file/CLI contract |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Judge input contract | Materialize a bundle file/directory and pass its path to `llm-judge`. | Keeps the keel side deterministic and provider-neutral. |
| Bundle contents | Include story metadata, acceptance criterion text, artifact references, and normalized evidence descriptors. | Gives the judge enough context without requiring repo-global diff inspection. |
| Evidence persistence | Store judge transcript/results under story `EVIDENCE/` and include them in the manifest. | Preserves the same proof chain used by other verification techniques. |
| Failure preservation | Keep bundle/debug artifacts on failed judge runs. | Makes semantic verification failures inspectable instead of opaque. |

## Architecture

1. Artifact Bundle Builder:
   - Reads story evidence and acceptance-criterion context.
   - Emits a deterministic bundle payload and file layout.

2. Judge Adapter:
   - Invokes external `llm-judge <bundle-path>` (or equivalent canonical contract).
   - Collects exit status, stdout/stderr, and transcript artifact paths.

3. Verification Integration Layer:
   - Reuses the judge path from both `verify run` and `story record --judge`.
   - Maps judge output into the existing verification report/result model.

4. Evidence Persistence:
   - Writes transcripts and any companion judge outputs into story `EVIDENCE/`.
   - Regenerates manifest hashes so the new artifacts become auditable.

## Components

- Bundle Schema:
  - Stable machine-readable format for criterion text, story metadata, and artifact references.

- Bundle Materializer:
  - Collects canonical evidence paths from story context and writes the bundle payload.

- External Judge Contract:
  - Thin process boundary that keeps provider concerns out of the keel crate.

- Transcript/Result Parser:
  - Normalizes pass/fail outcomes and captures operator-readable diagnostics.

## Interfaces

Expected interfaces:
- Internal: `build_judge_bundle(board_dir, story_id, criterion) -> bundle_path`
- External: `llm-judge <bundle-path>`
- Existing CLI surfaces:
  - `keel verify run <story-id>`
  - `keel story record <story-id> --judge`

## Data Flow

1. Locate the target story and acceptance criterion.
2. Gather referenced evidence artifacts from story context.
3. Materialize a deterministic artifact bundle.
4. Invoke the external `llm-judge` command with the bundle path.
5. Capture stdout/stderr plus any emitted transcript artifact.
6. Persist judge outputs into story `EVIDENCE/` and update the manifest.
7. Report pass/fail in the normal verification result surface.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Bundle materialization fails | Missing/invalid evidence or serialization error | Abort with explicit bundle-path/context diagnostics | Fix evidence or schema mismatch and rerun |
| `llm-judge` executable missing | Process spawn failure | Fail fast with provider-agnostic install guidance | Install/configure a judge wrapper on PATH |
| Judge returns malformed output | Parser or contract validation failure | Mark verification failed and preserve raw outputs | Fix wrapper contract and rerun |
| Judge returns semantic failure | Exit code or transcript status indicates fail | Surface failing criterion with transcript path | Review artifacts, improve implementation, or retry |
