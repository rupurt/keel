# Hard Schema Migration and Compatibility Cleanup - Software Design Description

> Ship a hard migration to canonical states and fields, then remove compatibility paths and stale terminology.

**SRS:** [SRS.md](SRS.md)

## Overview

Introduce a migration-first cutover. A dedicated migration command upgrades board files to canonical schema values. After migration exists, runtime parsing and type aliases that preserve legacy compatibility are removed. CLI/docs/tests are then normalized to canonical terminology.

## Context & Boundaries

```
┌───────────────────────────────────────────────┐
│            migrate_schema command             │
│    (legacy values -> canonical values)        │
└───────────────────┬───────────────────────────┘
                    │
      canonical model/state-machine parsing
                    │
            commands + doctor + docs/tests
```

In scope:
- Migration command and mappings.
- Removal of compatibility aliases/deserializers.
- Canonical field and terminology normalization.
- Scaffold hygiene for freshly generated epic/voyage planning artifacts.

Out of scope:
- Queue policy semantics.
- Transition-enforcement service architecture.

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `src/model/*` and `src/state_machine/*` | Internal modules | Canonical state representations and parsing behavior. | current |
| `.keel/` file schema | Data contract | Migration input/output target. | current |
| `src/commands/diagnostics/doctor` | Internal module | Detect drift and validate post-migration consistency. | current |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Migration strategy | In-place rewrite with explicit mappings | Fastest path for hard cutover. |
| Compatibility policy | Remove legacy parsing after migration lands | Enforces user-requested hard migration. |
| Field normalization | `completed_at` is canonical for epic completion | Aligns command behavior with model expectations. |
| Terminology policy | Canonical names only in user-facing text | Eliminates conceptual drift. |
| Scaffold quality policy | Freshly generated planning artifacts should not fail doctor for timestamp/placeholder defaults | Keeps new boards coherent without immediate manual cleanup. |

## Architecture

- Migration module:
  - Scans relevant `.keel` markdown files.
  - Applies deterministic mapping transforms.
  - Emits a summary report of changed files.
- Runtime schema:
  - Canonical deserializers only (no legacy alias handling).
  - Canonical type names only in code paths.
- Validation:
  - Doctor verifies no legacy tokens remain.
  - Tests cover both migration and strict post-cutover parsing.

## Components

- `migrate_schema` command:
  - Purpose: upgrade board state once per repository.
  - Behavior: idempotent rewrites and actionable error output.
- Canonical parser updates:
  - Purpose: reject legacy values after cutover.
  - Behavior: strict deserialization with explicit unknown-value errors.
- Terminology normalization sweep:
  - Purpose: align CLI/docs/tests.
  - Behavior: remove/replace legacy strings and update snapshots.
- Scaffolding normalization updates:
  - Purpose: ensure new epic/voyage artifacts are emitted in doctor-compliant baseline form.
  - Behavior: datetime `created_at` generation and non-placeholder default content in generated docs.

## Interfaces

- `keel migrate schema --hard` (or equivalent command surface)
- Migration result model:
  - files scanned
  - files updated
  - warnings/errors per file

## Data Flow

1. User runs migration command.
2. Command loads markdown files under `.keel`.
3. Transform engine rewrites legacy values to canonical forms.
4. Command persists updates and prints summary.
5. Runtime commands load canonical schema only.
6. Doctor/tests validate no legacy terms remain.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Unknown legacy token not covered by mapping | Migration parser emits unknown token error | Stop migration and report file + token | Extend mapping and rerun migration. |
| Partial write failure | File I/O error during rewrite | Abort with explicit path and reason | Fix filesystem issue and rerun migration (idempotent). |
| Post-cutover runtime sees legacy value | Strict deserializer error | Fail fast with canonical value guidance | Run migration command or manually fix file. |
