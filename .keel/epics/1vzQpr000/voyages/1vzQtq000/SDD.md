# Bearing Contract Cutover and Migration - Software Design Description

> Replace the survey-era bearing artifact and lifecycle contract with a framing/evidence/assessment workflow and hard-cutover migration rules.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage performs the structural hard cutover from the current survey-era bearing workflow to a framing/evidence/assessment workflow. The implementation updates templates, lifecycle transitions, CLI command names and help text, read surfaces, doctor validation, fixture boards, and migration-facing error messages together so the repository presents one canonical bearing contract everywhere.

The voyage intentionally does not implement the full evidence model. Instead, it establishes the renamed artifact boundaries and the contract enforcement that later voyages can build on.

## Context & Boundaries

```text
┌──────────────────────────────────────────────────────────────┐
│                   bearing contract cutover                   │
│                                                              │
│  templates ─┬─> lifecycle commands ─┬─> read surfaces        │
│             │                       │                        │
│             ├─> doctor validation   ├─> docs / guidance      │
│             │                       │                        │
│             └─> fixture boards      └─> migration errors     │
└──────────────────────────────────────────────────────────────┘
```

In scope:
- Rename and re-describe the bearing artifact contract.
- Replace survey-era command and guidance language.
- Enforce the new contract in doctor and read surfaces.
- Update docs and fixtures so tests and user guidance stay aligned.

Out of scope:
- Provider orchestration or evidence ingestion logic.
- Evidence ranking or EV-scoring changes.
- Backward-compatible support for legacy survey-era behavior.

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| bearing lifecycle adapters and transition specs | internal service | Rename the command path and artifact creation contract | existing crate API |
| template rendering and template fixtures | internal service | Update scaffolded document names and descriptions | existing crate API |
| structural validation and doctor checks | internal service | Enforce the new artifact contract and migration errors | existing crate API |
| read surfaces (`bearing show`, `bearing file`, flow guidance) | internal service | Keep human-facing terminology and document references coherent | existing crate API |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Cutover policy | Remove survey-era path in one slice | Prevents the old and new contracts from coexisting and creating more ambiguity |
| Artifact rename | `SURVEY.md` becomes `EVIDENCE.md` and lifecycle language shifts to research | Aligns the filename with the job the document is meant to do |
| Migration strategy | Fail fast with actionable errors instead of silent fallback parsing | Makes stale boards explicit and keeps the new contract authoritative |
| Scope of this voyage | Establish contract and migration only, not provider internals | Keeps the slice implementable while unblocking later evidence work |

## Architecture

The cutover touches five layers:
- Templates and generators define the new artifact names and README document references.
- CLI command and guidance adapters rename the lifecycle terminology from survey to research.
- Domain transition specs reflect the new command naming and file creation target.
- Read surfaces and projections expose the new document paths and labels.
- Validation rejects the legacy survey-era structure and drives migration messaging.

The implementation should prefer renaming shared helpers and canonical enums over adding translation layers. Any existing test fixture or generated board artifact that still references survey-era semantics must be updated in the same slice.

## Components

| Component | Purpose | Interface | Notes |
|-----------|---------|-----------|-------|
| Bearing template set | Define canonical artifact names and descriptions | scaffold rendering | README documents table and evidence template entrypoint change here |
| Bearing lifecycle command surface | Replace survey terminology with research terminology | clap actions + guidance helpers | One canonical command shape only |
| Bearing doctor rules | Reject legacy artifact layout and stale references | diagnostics checks + structural validation | Must provide actionable replacement text |
| Read surfaces | Show the new artifact names and paths | `bearing show`, `bearing file`, flow/next guidance | No mixed terminology after cutover |
| Fixture and docs updater | Keep tests and human docs aligned with the cutover | fixture builders + docs | Avoid drift between CLI contract and documentation |

## Interfaces

User-facing command contract after cutover:
- `keel bearing new <name>`
- `keel bearing research <id>`
- `keel bearing assess <id>`
- `keel bearing file <id> EVIDENCE`

Canonical artifact contract after cutover:
- `BRIEF.md` for framing
- `EVIDENCE.md` for cited research capture
- `ASSESSMENT.md` for synthesis and recommendation

Legacy command and file names should return actionable migration errors rather than being parsed as aliases.

## Data Flow

1. A bearing is created with `README.md` and `BRIEF.md`.
2. The research lifecycle command materializes `EVIDENCE.md` instead of `SURVEY.md`.
3. Read surfaces resolve and render the new document names.
4. Doctor and structural validation inspect bearing artifacts and flag any survey-era files, scaffold residue, or missing evidence doc requirements.
5. Updated docs and fixtures keep tests and operator guidance aligned with the new contract.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| User invokes removed survey-era command | CLI parse or action dispatch | Return a hard-cutover error naming the replacement research command | Re-run with the canonical research command |
| Board still contains `SURVEY.md` or survey-era README references | Doctor/structural validation | Emit actionable error explaining the expected `EVIDENCE.md` contract | Migrate the bearing artifacts to the new names |
| Generated/read surfaces still reference survey terminology | Snapshot or docs regression tests | Fail CI and keep the cutover incomplete | Update the remaining surface to the canonical contract |
| Fixture or generated board drift leaves mixed terminology | Fixture tests or doctor | Fail fast instead of silently accepting both paths | Regenerate or rewrite fixture artifacts to the new contract |
