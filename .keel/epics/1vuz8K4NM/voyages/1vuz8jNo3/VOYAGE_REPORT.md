# VOYAGE REPORT: Hard Schema Migration and Compatibility Cleanup

## Voyage Metadata
- **ID:** 1vuz8jNo3
- **Epic:** 1vuz8K4NM
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 6/6 stories complete

## Implementation Narrative
### Remove Legacy State and Status Deserializers
- **ID:** 1vuz9hmGQ
- **Status:** done

#### Summary
Remove legacy deserializer aliases after migration support is available so only canonical schema values are accepted at runtime.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] Remove legacy alias parsing for story, voyage, and epic status values from canonical deserializers. <!-- verify: manual, SRS-02:start -->
- [x] [SRS-02/AC-02] Ensure parser errors for legacy tokens clearly identify canonical replacements. <!-- verify: manual, SRS-02:continues -->
- [x] [SRS-02/AC-03] Add tests that assert legacy inputs are rejected after migration cutover. <!-- verify: manual, SRS-02:end -->

#### Implementation Insights
# Reflection - Remove Legacy State and Status Deserializers

### L001: Hard-cut deserializers should fail with explicit replacement guidance
Legacy tokens are safest when rejected with deterministic, canonical replacement hints so migration gaps are obvious and actionable.

### L002: Canonical-state migrations require fixture cleanup beyond parser units
Once aliases are removed, integration fixtures that still emit legacy epic status values fail quickly, so test data must be normalized in the same change.

### Implement Hard Migration Command for Canonical Schema
- **ID:** 1vuz9hzxw
- **Status:** done

#### Summary
Implement the hard migration command that rewrites legacy schema values to canonical forms and provides a deterministic migration report.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Implement a migration command that rewrites legacy states and legacy date field keys to canonical schema values. <!-- verify: manual, SRS-01:start -->
- [x] [SRS-01/AC-02] Ensure migration is idempotent and reports changed files with actionable output. <!-- verify: manual, SRS-01:continues -->
- [x] [SRS-01/AC-03] Add fixture-based integration tests covering successful migration and unknown-token failure paths. <!-- verify: manual, SRS-01:end -->

#### Implementation Insights
# Reflection - Implement Hard Migration Command for Canonical Schema

### L001: Preflight validation prevents partial migrations
Collecting all unsupported status tokens before writing ensures hard migration fails safely and keeps board files unchanged on error.

### L002: Path-scoped entity classification keeps rewrites precise
Classifying story/voyage/epic README paths before applying mappings avoids accidental status normalization outside canonical schema surfaces.

### Normalize Epic Completion Field to Completed At
- **ID:** 1vuz9i4Ja
- **Status:** done

#### Summary
Normalize epic completion handling to `completed_at` across command handlers, loaders/models, and doctor diagnostics.

#### Acceptance Criteria
- [x] [SRS-04/AC-01] Update epic completion and reopen flows to read/write `completed_at` as the canonical completion timestamp field. <!-- verify: manual, SRS-04:start -->
- [x] [SRS-04/AC-02] Remove or migrate remaining references to legacy epic completion field names in loaders, serializers, and doctor validations. <!-- verify: manual, SRS-04:continues -->
- [x] [SRS-04/AC-03] Add tests covering epic completion and reopen behavior plus doctor checks to confirm only `completed_at` is accepted. <!-- verify: manual, SRS-04:end -->

#### Implementation Insights
# Reflection - Normalize Epic Completion Field to Completed At

### L001: Canonical timestamp fields need both writer and validator alignment
Switching epic completion to `completed_at` required updating command writers and doctor field validation together; changing only one side leaves schema drift.

### L002: Tightening frontmatter contracts should be backed by regression tests
Adding parser strictness (`deny_unknown_fields`) is safe when paired with targeted tests that lock rejection of legacy fields and acceptance of canonical fields.

### Purge Legacy Terminology from CLI and Documentation
- **ID:** 1vuz9iSmb
- **Status:** done

#### Summary
Purge legacy state and workflow terminology from CLI help, docs, and tests so canonical language is the only supported vocabulary.

#### Acceptance Criteria
- [x] [SRS-05/AC-01] Replace legacy terminology in CLI help/output strings with canonical state names and workflow terms only. <!-- verify: manual, SRS-05:start -->
- [x] [SRS-05/AC-02] Update architecture and user-facing docs to remove legacy wording and align examples with canonical terminology. <!-- verify: manual, SRS-05:continues -->
- [x] [SRS-05/AC-03] Add or update tests/checks that fail when deprecated terminology reappears in CLI snapshots or documentation fixtures. <!-- verify: manual, SRS-05:end -->

#### Implementation Insights
# Reflection - Purge Legacy Terminology from CLI and Documentation

### L001: Canonical vocabulary is strongest when enforced at parse boundaries
Using explicit CLI value parsers for story/epic/voyage status filters prevents legacy terms from slipping through and keeps user-facing language aligned with canonical state machines.

### L002: Drift guards should target user-facing surfaces directly
A focused drift test over `README.md`, `ARCHITECTURE.md`, and CLI help source catches deprecated terms reappearing in documentation/help snapshots without over-constraining migration internals.

### Remove Stage and Status Compatibility Aliases
- **ID:** 1vuz9ij1L
- **Status:** done

#### Summary
Remove compatibility aliases for story and voyage states so the codebase depends directly on canonical state-machine types.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] Replace usage of compatibility aliases with direct canonical state types in model, commands, and flow modules. <!-- verify: manual, SRS-03:start -->
- [x] [SRS-03/AC-02] Remove alias definitions and related compatibility comments/tests that are no longer valid after migration. <!-- verify: manual, SRS-03:continues -->
- [x] [SRS-03/AC-03] Ensure the project compiles and test coverage reflects canonical type usage only. <!-- verify: manual, SRS-03:end -->

#### Implementation Insights
# Reflection - Remove Stage and Status Compatibility Aliases

### L001: Alias removal is safest as a compile-driven codemod
Replacing compatibility aliases with canonical state-machine types across modules was most reliable when done mechanically and validated immediately with `cargo check` to catch stragglers.

### L002: Canonical-type migrations need both code and metadata cleanup
Removing alias modules required updating stale comments and board metadata checks so the migration is coherent in code, docs, and doctor gates.

### Fix Scaffold and Story Timestamp Doctor Findings at Source
- **ID:** 1vuzUa2mf
- **Status:** done

#### Summary
Fix the real causes behind current doctor findings by generating datetime fields where state transitions/scaffolding write timestamps. This includes epic/voyage scaffold `created_at`, story lifecycle `submitted_at`/`completed_at`, and removal of default TODO placeholders from newly scaffolded planning artifacts.

#### Acceptance Criteria
- [x] [SRS-06/AC-01] Update epic/voyage scaffolding paths so generated frontmatter uses datetime `created_at` format (`YYYY-MM-DDTHH:MM:SS`) instead of date-only values. <!-- verify: manual, SRS-06:start -->
- [x] [SRS-06/AC-02] Replace generated TODO placeholders in default epic/voyage planning artifacts with non-placeholder baseline content that does not trigger doctor placeholder warnings on creation. <!-- verify: manual, SRS-06:continues -->
- [x] [SRS-06/AC-03] Ensure `keel story submit` writes `submitted_at` using datetime format (`YYYY-MM-DDTHH:MM:SS`) and never date-only values. <!-- verify: manual, SRS-06:continues -->
- [x] [SRS-06/AC-04] Ensure `keel story accept` writes `completed_at` using datetime format (`YYYY-MM-DDTHH:MM:SS`) and never date-only values. <!-- verify: manual, SRS-06:continues -->
- [x] [SRS-06/AC-05] Add regression coverage showing fresh scaffolded epic+voyage and story submit/accept flows pass relevant doctor datetime and placeholder checks. <!-- verify: manual, SRS-06:end -->

#### Implementation Insights
# Reflection - Fix Scaffold and Story Timestamp Doctor Findings at Source

### L001: Timestamp ownership belongs in transition/scaffold writes
Date consistency warnings were caused by generation points writing date-only values. Enforcing datetime output at those write boundaries prevents downstream drift checks from surfacing avoidable failures.

### L002: Template defaults must be doctor-clean at creation time
Scaffolded planning docs should ship with baseline non-placeholder content so newly created epics and voyages are valid by default and do not require immediate cleanup.


