# PRESS RELEASE: Unified Transition Enforcement

## Overview

## Narrative Summary
### Route Doctor Checks Through Gate Evaluators
Make doctor transition and completion checks reuse the same gate-evaluation paths and reporting policy semantics as runtime enforcement.

### Route Story and Voyage Commands Through Unified Enforcer
Route runtime story and voyage command handlers through the shared transition enforcer so gate behavior is applied consistently.

### Remove Duplicate Command Side Checks and Error Formatters
Remove duplicate command-side checks and centralize transition error formatting so validation outcomes are consistent across callers.

### Add Regression Tests for Gate Runtime and Reporting Modes
Add regression coverage that proves strict runtime blocking and reporting-mode visibility remain coherent for shared gate rules.

### Introduce Unified Transition Enforcement Service
Introduce a shared transition-enforcement service that combines transition legality checks, gate evaluation, and policy-based blocking classification.

### Gate Voyage Report Artifacts to Done State
Make voyage reporting artifacts lifecycle-aware so `VOYAGE_REPORT.md` and `COMPLIANCE_REPORT.md` are produced only when a voyage reaches `done`, not during draft/planned/in-progress generation.

## Key Insights
### Insights from Route Doctor Checks Through Gate Evaluators
# Reflection - Route Doctor Checks Through Gate Evaluators

### L001: Doctor parity is strongest when diagnostics consume enforcement outputs directly
Using `enforce_transition(..., EnforcementPolicy::REPORTING)` in doctor checks keeps transition/completion findings aligned with runtime gate rules while preserving non-blocking reporting semantics.

### L002: Reporting semantics are testable via the blocking subset
Asserting `blocking_problems.is_empty()` in parity tests provides a clear contract that doctor visibility does not inherit runtime blocking behavior.

### Insights from Route Story and Voyage Commands Through Unified Enforcer
# Reflection - Route Story and Voyage Commands Through Unified Enforcer

### L001: Enforcer intent must match command semantics
`story start` needed `Restart` intent for rejected stories to preserve existing behavior while still routing through unified enforcement.

### L002: Policy flags cleanly preserve command-specific behavior
Passing `require_requirements_coverage: !force` for voyage start retained force bypass semantics without duplicating gate logic in command handlers.

### Insights from Remove Duplicate Command Side Checks and Error Formatters
# Reflection - Remove Duplicate Command Side Checks and Error Formatters

### L-01: Centralized transition formatting removes message drift

Consolidating transition/gate error rendering into one shared formatter kept command and reporting outputs structurally identical, which reduced test brittleness and command-specific string logic.

### Insights from Add Regression Tests for Gate Runtime and Reporting Modes
# Reflection - Add Regression Tests for Gate Runtime and Reporting Modes

### L-01: Keep parity assertions focused on normalized findings

Runtime and reporting flows can differ in blocking classification while still sharing identical gate findings. Comparing normalized severity/message fingerprints catches rule-source drift without coupling tests to board file paths.

### Insights from Introduce Unified Transition Enforcement Service
### L001: Legality and gates should be separate outputs before classification
Returning legality findings and gate findings as distinct fields makes enforcement decisions easier to reason about and keeps debugging focused when a transition fails.

### L002: Blocking policy should be explicit at evaluation time
Modeling strict/runtime/reporting blocking modes directly in enforcement policy removes ad-hoc severity filtering and creates deterministic behavior across runtime and diagnostic paths.

### Insights from Gate Voyage Report Artifacts to Done State
# Reflection - Gate Voyage Report Artifacts to Done State

### L001: Artifact lifecycle rules should live where artifacts are produced
Gating report generation in the board generation path and explicit voyage completion path keeps report visibility coherent with voyage state.

### L002: State-aware document sections prevent stale links and stale files from drifting apart
Deriving README document links from voyage status and deleting stale non-done artifacts keeps rendered docs and filesystem artifacts consistent.

## Verification Proof
