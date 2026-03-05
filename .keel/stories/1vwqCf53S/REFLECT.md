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

### 1vyDuwf3P: Build Typed Command Actions Before Dispatching

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
