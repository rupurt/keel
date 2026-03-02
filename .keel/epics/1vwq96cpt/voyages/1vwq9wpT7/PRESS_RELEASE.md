# PRESS RELEASE: Migrate CLI Interfaces And Verification Coverage

## Overview

## Narrative Summary
### Publish Migration Completion Checklist
Publish migration completion criteria and rollout checklist for maintainers.

### Add Command Behavior Regression Suite
Add regression suites for key command behavior and output parity.

### Refactor Main Dispatch To Interface Adapters
Refactor top-level CLI dispatch into thin interface adapters.

### Add Architecture Contract Verification Suite
Add architecture verification suites that enforce layering and context contracts.

## Key Insights
### Insights from Publish Migration Completion Checklist
# Reflection - Publish Migration Completion Checklist

## Knowledge

<!--
Institutional memory is mandatory. Capture what you learned during implementation.
Format:
### L001: Title
| Field | Value |
|-------|-------|
| **Category** | code/testing/process/architecture |
| **Context** | describe when this applies |
| **Insight** | the fundamental discovery |
| **Suggested Action** | what to do next time |
| **Applies To** | file patterns or components |
| **Observed At** | RFC3339 timestamp (e.g. 2026-02-22T12:00:00Z) |
| **Score** | 0.0-1.0 (impact significance) |
| **Confidence** | 0.0-1.0 (insight quality) |
| **Applied** | |
-->

### L001: Rollout Docs Need Explicit Gate Ownership

| Field | Value |
|-------|-------|
| **Category** | process |
| **Context** | Finalizing migration voyages where multiple maintainers coordinate acceptance and release sequencing |
| **Insight** | Checklist quality improves when each gate and rollout step is phrased as an explicit maintainer action with clear command references |
| **Suggested Action** | Keep voyage-local migration checklists with completion criteria, gate commands, rollout order, and deferred-item tracking |
| **Applies To** | `.keel/epics/*/voyages/*/MIGRATION_CHECKLIST.md`, voyage `README.md` document tables |
| **Observed At** | 2026-03-02T10:41:30-08:00 |
| **Score** | 0.78 |
| **Confidence** | 0.88 |
| **Applied** | Added `MIGRATION_CHECKLIST.md` for `1vwq9wpT7` and linked it from voyage documents |

## Observations

The checklist was straightforward to land because voyage docs already had a clear location and document table. Including the post-epic deferred item in the same checklist kept scope explicit without mixing it into current epic execution.

### Insights from Add Command Behavior Regression Suite
# Reflection - Add Command Behavior Regression Suite

## Knowledge

<!--
Institutional memory is mandatory. Capture what you learned during implementation.
Format:
### L001: Title
| Field | Value |
|-------|-------|
| **Category** | code/testing/process/architecture |
| **Context** | describe when this applies |
| **Insight** | the fundamental discovery |
| **Suggested Action** | what to do next time |
| **Applies To** | file patterns or components |
| **Observed At** | RFC3339 timestamp (e.g. 2026-02-22T12:00:00Z) |
| **Score** | 0.0-1.0 (impact significance) |
| **Confidence** | 0.0-1.0 (insight quality) |
| **Applied** | |
-->

### L001: Regression Parity Needs Cross-Command Coverage

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | During migration of command handlers to shared application/read-model layers |
| **Insight** | Policy thresholds can drift silently unless `next` and `flow` are asserted together at the same boundary conditions |
| **Suggested Action** | Add paired regression tests that validate both command-level decisions and dashboard summaries for each queue policy boundary |
| **Applies To** | `src/next/*`, `src/flow/*`, `src/commands/story/*`, `src/command_regression_tests.rs` |
| **Observed At** | 2026-03-02T10:21:00-08:00 |
| **Score** | 0.80 |
| **Confidence** | 0.89 |
| **Applied** | Added `command_regression_tests` cases for human-block and flow-block boundaries plus lifecycle start/submit/accept chain |

## Observations

The dedicated regression module gave a stable migration guardrail without coupling to terminal formatting details. The primary friction was sandbox command execution requiring escalated runs for board commands and validation gates.

### Insights from Refactor Main Dispatch To Interface Adapters
# Reflection - Refactor Main Dispatch To Interface Adapters

## Knowledge

<!--
Institutional memory is mandatory. Capture what you learned during implementation.
Format:
### L001: Title
| Field | Value |
|-------|-------|
| **Category** | code/testing/process/architecture |
| **Context** | describe when this applies |
| **Insight** | the fundamental discovery |
| **Suggested Action** | what to do next time |
| **Applies To** | file patterns or components |
| **Observed At** | RFC3339 timestamp (e.g. 2026-02-22T12:00:00Z) |
| **Score** | 0.0-1.0 (impact significance) |
| **Confidence** | 0.0-1.0 (insight quality) |
| **Applied** | |
-->

### L001: Build Typed Command Actions Before Dispatching

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Refactoring large CLI dispatch trees while preserving behavior and clap argument contracts |
| **Insight** | Converting `ArgMatches` into typed action enums at the boundary and routing through module `run(action)` functions keeps `main` focused on parsing while pushing interface adaptation into command-group modules |
| **Suggested Action** | Keep adding action enums and single entrypoint adapters per command group so architecture tests can enforce delegation contracts cleanly |
| **Applies To** | src/main.rs; src/commands/*/mod.rs |
| **Observed At** | 2026-03-02T02:18:25Z |
| **Score** | 0.80 |
| **Confidence** | 0.87 |
| **Applied** | 1vwqCf53S |

## Observations

Updating the existing action enums to match real CLI flags (`story list` reflections, `story record` judge/files) prevented drift between parser and adapter layers. Once those fields matched, moving dispatch to typed adapters was low risk and easy to validate with the existing CLI parse test suite.

### Insights from Add Architecture Contract Verification Suite
# Reflection - Add Architecture Contract Verification Suite

## Knowledge

<!--
Institutional memory is mandatory. Capture what you learned during implementation.
Format:
### L001: Title
| Field | Value |
|-------|-------|
| **Category** | code/testing/process/architecture |
| **Context** | describe when this applies |
| **Insight** | the fundamental discovery |
| **Suggested Action** | what to do next time |
| **Applies To** | file patterns or components |
| **Observed At** | RFC3339 timestamp (e.g. 2026-02-22T12:00:00Z) |
| **Score** | 0.0-1.0 (impact significance) |
| **Confidence** | 0.0-1.0 (insight quality) |
| **Applied** | |
-->

### L001: Production-only import checks reduce false positives

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | Architecture contract tests scanning modules that also contain `#[cfg(test)]` helper imports |
| **Insight** | Import-boundary checks should target production sections to avoid test-only imports triggering invalid architectural failures. |
| **Suggested Action** | Split source at `#[cfg(test)]` and enforce forbidden-edge patterns only on production content for adapter boundary tests. |
| **Applies To** | `src/architecture_contract_tests.rs`, `src/commands/diagnostics/*.rs`, `src/main.rs`, `src/next/algorithm.rs` |
| **Observed At** | 2026-03-02T16:48:53Z |
| **Score** | 0.87 |
| **Confidence** | 0.95 |
| **Applied** | story `1vwqCfdUl` |

## Observations

The suite extension was straightforward once we introduced a dedicated production-source reader and targeted diagnostics adapters explicitly.
The most important guard added here is cross-context forbiddance (`commands::story`, `commands::voyage`, `commands::epic`, transitions) with deterministic fixture assertions proving failure behavior.

## Verification Proof
### Proof for Publish Migration Completion Checklist
- [ac-1.log](../../../../stories/1vwqCfeFP/EVIDENCE/ac-1.log)

### Proof for Add Command Behavior Regression Suite
- [ac-1.log](../../../../stories/1vwqCffzr/EVIDENCE/ac-1.log)

### Proof for Refactor Main Dispatch To Interface Adapters
- [ac-1.log](../../../../stories/1vwqCf53S/EVIDENCE/ac-1.log)

### Proof for Add Architecture Contract Verification Suite
- [ac-1.log](../../../../stories/1vwqCfdUl/EVIDENCE/ac-1.log)

