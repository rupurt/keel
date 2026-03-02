# PRESS RELEASE: Hard Schema Migration and Compatibility Cleanup

## Overview

## Narrative Summary
### Remove Stage and Status Compatibility Aliases
Remove compatibility aliases for story and voyage states so the codebase depends directly on canonical state-machine types.

### Purge Legacy Terminology from CLI and Documentation
Purge legacy state and workflow terminology from CLI help, docs, and tests so canonical language is the only supported vocabulary.

### Implement Hard Migration Command for Canonical Schema
Implement the hard migration command that rewrites legacy schema values to canonical forms and provides a deterministic migration report.

### Remove Legacy State and Status Deserializers
Remove legacy deserializer aliases after migration support is available so only canonical schema values are accepted at runtime.

### Fix Scaffold and Story Timestamp Doctor Findings at Source
Fix the real causes behind current doctor findings by generating datetime fields where state transitions/scaffolding write timestamps. This includes epic/voyage scaffold `created_at`, story lifecycle `submitted_at`/`completed_at`, and removal of default TODO placeholders from newly scaffolded planning artifacts.

### Normalize Epic Completion Field to Completed At
Normalize epic completion handling to `completed_at` across command handlers, loaders/models, and doctor diagnostics.

## Key Insights
### Insights from Remove Stage and Status Compatibility Aliases
# Reflection - Remove Stage and Status Compatibility Aliases

### L001: Alias removal is safest as a compile-driven codemod
Replacing compatibility aliases with canonical state-machine types across modules was most reliable when done mechanically and validated immediately with `cargo check` to catch stragglers.

### L002: Canonical-type migrations need both code and metadata cleanup
Removing alias modules required updating stale comments and board metadata checks so the migration is coherent in code, docs, and doctor gates.

### Insights from Purge Legacy Terminology from CLI and Documentation
# Reflection - Purge Legacy Terminology from CLI and Documentation

### L001: Canonical vocabulary is strongest when enforced at parse boundaries
Using explicit CLI value parsers for story/epic/voyage status filters prevents legacy terms from slipping through and keeps user-facing language aligned with canonical state machines.

### L002: Drift guards should target user-facing surfaces directly
A focused drift test over `README.md`, `ARCHITECTURE.md`, and CLI help source catches deprecated terms reappearing in documentation/help snapshots without over-constraining migration internals.

### Insights from Implement Hard Migration Command for Canonical Schema
# Reflection - Implement Hard Migration Command for Canonical Schema

### L001: Preflight validation prevents partial migrations
Collecting all unsupported status tokens before writing ensures hard migration fails safely and keeps board files unchanged on error.

### L002: Path-scoped entity classification keeps rewrites precise
Classifying story/voyage/epic README paths before applying mappings avoids accidental status normalization outside canonical schema surfaces.

### Insights from Remove Legacy State and Status Deserializers
# Reflection - Remove Legacy State and Status Deserializers

### L001: Hard-cut deserializers should fail with explicit replacement guidance
Legacy tokens are safest when rejected with deterministic, canonical replacement hints so migration gaps are obvious and actionable.

### L002: Canonical-state migrations require fixture cleanup beyond parser units
Once aliases are removed, integration fixtures that still emit legacy epic status values fail quickly, so test data must be normalized in the same change.

### Insights from Fix Scaffold and Story Timestamp Doctor Findings at Source
# Reflection - Fix Scaffold and Story Timestamp Doctor Findings at Source

### L001: Timestamp ownership belongs in transition/scaffold writes
Date consistency warnings were caused by generation points writing date-only values. Enforcing datetime output at those write boundaries prevents downstream drift checks from surfacing avoidable failures.

### L002: Template defaults must be doctor-clean at creation time
Scaffolded planning docs should ship with baseline non-placeholder content so newly created epics and voyages are valid by default and do not require immediate cleanup.

### Insights from Normalize Epic Completion Field to Completed At
# Reflection - Normalize Epic Completion Field to Completed At

### L001: Canonical timestamp fields need both writer and validator alignment
Switching epic completion to `completed_at` required updating command writers and doctor field validation together; changing only one side leaves schema drift.

### L002: Tightening frontmatter contracts should be backed by regression tests
Adding parser strictness (`deny_unknown_fields`) is safe when paired with targeted tests that lock rejection of legacy fields and acceptance of canonical fields.

## Verification Proof
