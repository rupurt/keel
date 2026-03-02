# Knowledge - 1vwq9Zf67

> Automated synthesis of story reflections.

## Story: Rewire Command Handlers To Use Cases (1vwqCfPpe)

# Reflection - Rewire Command Handlers To Use Cases

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

### L001: Enforce policy invariants in application services, not command handlers

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Story and voyage lifecycle commands need consistent enforcement of manual-verification acceptance and requirements-coverage gating. |
| **Insight** | Putting enforcement in application services centralizes lifecycle invariants and avoids drift across multiple command entrypoints. |
| **Suggested Action** | Keep command handlers thin and add service-level tests for every lifecycle policy; use architecture contract tests to block direct transition orchestration from handlers. |
| **Applies To** | `src/application/story_lifecycle.rs`, `src/application/voyage_epic_lifecycle.rs`, `src/commands/{story,voyage,epic}/*.rs`, `src/architecture_contract_tests.rs` |
| **Observed At** | 2026-03-02T17:32:48Z |
| **Score** | 0.90 |
| **Confidence** | 0.96 |
| **Applied** | story `1vwqCfPpe` |

## Observations

The service-level tests were straightforward to add and made policy behavior explicit at the boundary where it matters.
The main friction was tooling inconsistency between `just keel` argument handling and direct binary invocation; using shorter evidence text and direct proof files avoided ambiguity.


---

## Story: Implement Story Lifecycle Use Cases (1vwqCe5T0)

# Reflection - Implement Story Lifecycle Use Cases

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

### L001: Thin Command Adapters Preserve Behavior During Refactors

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Extracting orchestration out of CLI command handlers while keeping existing workflow behavior stable |
| **Insight** | Moving orchestration to an application service is low-risk when command handlers become thin pass-through adapters and existing command tests remain the compatibility suite. |
| **Suggested Action** | For future migrations, extract service logic first, then convert command files to wrappers and keep legacy helper behavior behind `#[cfg(test)]` shims only where needed. |
| **Applies To** | src/application/story_lifecycle.rs, src/commands/story/{start,submit,accept,reject,ice,thaw}.rs |
| **Observed At** | 2026-03-02T00:26:18Z |
| **Score** | 0.90 |
| **Confidence** | 0.91 |
| **Applied** | yes |

## Observations

Delegation worked cleanly after centralizing transition enforcement and side effects into one service module. The main friction was preserving unit tests that referenced command-local helpers, which was handled with test-only wrapper functions.


---

## Story: Implement Voyage And Epic Lifecycle Use Cases (1vwqCejs5)

# Reflection - Implement Voyage And Epic Lifecycle Use Cases

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

### L001: Keep Lifecycle Command Handlers As Thin Adapters

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Refactoring voyage and epic lifecycle commands to align with application-service orchestration boundaries |
| **Insight** | Moving orchestration into a dedicated application service lets command modules stay stable adapters while preserving behavior through existing command tests |
| **Suggested Action** | Add use-case methods first, then delegate command `run` entrypoints to those methods and update cross-command callsites to service APIs |
| **Applies To** | src/application/*.rs; src/commands/voyage/*.rs; src/commands/epic/*.rs |
| **Observed At** | 2026-03-02T02:02:28Z |
| **Score** | 0.82 |
| **Confidence** | 0.88 |
| **Applied** | 1vwqCejs5 |

## Observations

The extraction was straightforward once `story_lifecycle` became the reference pattern: most work was relocating orchestration and preserving output semantics. The main friction was CLI quoting for evidence messages through `just keel`, so concise tokenized evidence messages are safer in this shell path.


---

## Story: Introduce Domain Events And Process Managers (1vwqCeVSm)

# Reflection - Introduce Domain Events And Process Managers

## Knowledge

### L001: Event-First Cross-Aggregate Orchestration Preserves Boundaries

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Story start/accept and voyage completion previously embedded cross-aggregate calls inside lifecycle services. |
| **Insight** | Emitting explicit domain events and routing follow-on actions through a process manager keeps use cases focused while preserving existing behavior. |
| **Suggested Action** | Keep cross-aggregate progression in process managers and add event/action tests whenever new lifecycle automation is introduced. |
| **Applies To** | src/application/story_lifecycle.rs, src/application/voyage_epic_lifecycle.rs, src/application/process_manager.rs |
| **Observed At** | 2026-03-02T04:31:37Z |
| **Score** | 0.84 |
| **Confidence** | 0.92 |
| **Applied** | Introduced DomainEvent + DomainProcessManager and rewired lifecycle services to emit events. |

## Observations

Process-manager planning tests made the orchestration refactor deterministic and easy to verify without brittle filesystem transition setup.
The main risk was ordering: voyage auto-start must happen before the story transition to satisfy existing voyage start gates.
Running targeted start/accept/voyage tests before the full suite quickly caught and resolved that regression.


---

