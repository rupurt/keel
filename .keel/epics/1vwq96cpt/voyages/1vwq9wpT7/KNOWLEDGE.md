---
created_at: 2026-03-02T10:42:29
---

# Knowledge - 1vwq9wpT7

> Automated synthesis of story reflections.

## Story Knowledge

## Story: Refactor Main Dispatch To Interface Adapters (1vwqCf53S)

### 1vyDuwf3P: Build Typed Command Actions Before Dispatching

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Refactoring large CLI dispatch trees while preserving behavior and clap argument contracts |
| **Insight** | Converting `ArgMatches` into typed action enums at the boundary and routing through module `run(action)` functions keeps `main` focused on parsing while pushing interface adaptation into command-group modules |
| **Suggested Action** | Keep adding action enums and single entrypoint adapters per command group so architecture tests can enforce delegation contracts cleanly |
| **Applies To** | src/main.rs; src/commands/*/mod.rs |
| **Applied** | 1vwqCf53S |



---

## Story: Publish Migration Completion Checklist (1vwqCfeFP)

### 1vyDuwiKv: Rollout Docs Need Explicit Gate Ownership

| Field | Value |
|-------|-------|
| **Category** | process |
| **Context** | Finalizing migration voyages where multiple maintainers coordinate acceptance and release sequencing |
| **Insight** | Checklist quality improves when each gate and rollout step is phrased as an explicit maintainer action with clear command references |
| **Suggested Action** | Keep voyage-local migration checklists with completion criteria, gate commands, rollout order, and deferred-item tracking |
| **Applies To** | `.keel/epics/*/voyages/*/MIGRATION_CHECKLIST.md`, voyage `README.md` document tables |
| **Applied** | Added `MIGRATION_CHECKLIST.md` for `1vwq9wpT7` and linked it from voyage documents |



---

## Story: Add Architecture Contract Verification Suite (1vwqCfdUl)

### 1vyDuwvt7: Production-only import checks reduce false positives

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | Architecture contract tests scanning modules that also contain `#[cfg(test)]` helper imports |
| **Insight** | Import-boundary checks should target production sections to avoid test-only imports triggering invalid architectural failures. |
| **Suggested Action** | Split source at `#[cfg(test)]` and enforce forbidden-edge patterns only on production content for adapter boundary tests. |
| **Applies To** | `src/architecture_contract_tests.rs`, `src/commands/diagnostics/*.rs`, `src/main.rs`, `src/next/algorithm.rs` |
| **Applied** | story `1vwqCfdUl` |



---

## Story: Add Command Behavior Regression Suite (1vwqCffzr)

### 1vyDuw5Ob: Regression Parity Needs Cross-Command Coverage

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | During migration of command handlers to shared application/read-model layers |
| **Insight** | Policy thresholds can drift silently unless `next` and `flow` are asserted together at the same boundary conditions |
| **Suggested Action** | Add paired regression tests that validate both command-level decisions and dashboard summaries for each queue policy boundary |
| **Applies To** | `src/next/*`, `src/flow/*`, `src/commands/story/*`, `src/command_regression_tests.rs` |
| **Applied** | Added `command_regression_tests` cases for human-block and flow-block boundaries plus lifecycle start/submit/accept chain |



---

## Synthesis

### xb3Zn8PjR: Build Typed Command Actions Before Dispatching

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Refactoring large CLI dispatch trees while preserving behavior and clap argument contracts |
| **Insight** | Converting `ArgMatches` into typed action enums at the boundary and routing through module `run(action)` functions keeps `main` focused on parsing while pushing interface adaptation into command-group modules |
| **Suggested Action** | Keep adding action enums and single entrypoint adapters per command group so architecture tests can enforce delegation contracts cleanly |
| **Applies To** | src/main.rs; src/commands/*/mod.rs |
| **Linked Knowledge IDs** | 1vyDuwf3P |
| **Score** | 0.80 |
| **Confidence** | 0.87 |
| **Applied** | 1vwqCf53S |

### xGmZTe7kR: Rollout Docs Need Explicit Gate Ownership

| Field | Value |
|-------|-------|
| **Category** | process |
| **Context** | Finalizing migration voyages where multiple maintainers coordinate acceptance and release sequencing |
| **Insight** | Checklist quality improves when each gate and rollout step is phrased as an explicit maintainer action with clear command references |
| **Suggested Action** | Keep voyage-local migration checklists with completion criteria, gate commands, rollout order, and deferred-item tracking |
| **Applies To** | `.keel/epics/*/voyages/*/MIGRATION_CHECKLIST.md`, voyage `README.md` document tables |
| **Linked Knowledge IDs** | 1vyDuwiKv |
| **Score** | 0.78 |
| **Confidence** | 0.88 |
| **Applied** | Added `MIGRATION_CHECKLIST.md` for `1vwq9wpT7` and linked it from voyage documents |

### erV4YlxVN: Production-only import checks reduce false positives

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | Architecture contract tests scanning modules that also contain `#[cfg(test)]` helper imports |
| **Insight** | Import-boundary checks should target production sections to avoid test-only imports triggering invalid architectural failures. |
| **Suggested Action** | Split source at `#[cfg(test)]` and enforce forbidden-edge patterns only on production content for adapter boundary tests. |
| **Applies To** | `src/architecture_contract_tests.rs`, `src/commands/diagnostics/*.rs`, `src/main.rs`, `src/next/algorithm.rs` |
| **Linked Knowledge IDs** | 1vyDuwvt7 |
| **Score** | 0.87 |
| **Confidence** | 0.95 |
| **Applied** | story `1vwqCfdUl` |

### gYNMP5JXn: Regression Parity Needs Cross-Command Coverage

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | During migration of command handlers to shared application/read-model layers |
| **Insight** | Policy thresholds can drift silently unless `next` and `flow` are asserted together at the same boundary conditions |
| **Suggested Action** | Add paired regression tests that validate both command-level decisions and dashboard summaries for each queue policy boundary |
| **Applies To** | `src/next/*`, `src/flow/*`, `src/commands/story/*`, `src/command_regression_tests.rs` |
| **Linked Knowledge IDs** | 1vyDuw5Ob |
| **Score** | 0.80 |
| **Confidence** | 0.89 |
| **Applied** | Added `command_regression_tests` cases for human-block and flow-block boundaries plus lifecycle start/submit/accept chain |

