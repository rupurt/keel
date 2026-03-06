---
created_at: 2026-03-02T09:33:26
---

# Knowledge - 1vwq9Zf67

> Automated synthesis of story reflections.

## Story Knowledge

## Story: Introduce Domain Events And Process Managers (1vwqCeVSm)

### 1vyDuwiVQ: Event-First Cross-Aggregate Orchestration Preserves Boundaries

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Story start/accept and voyage completion previously embedded cross-aggregate calls inside lifecycle services. |
| **Insight** | Emitting explicit domain events and routing follow-on actions through a process manager keeps use cases focused while preserving existing behavior. |
| **Suggested Action** | Keep cross-aggregate progression in process managers and add event/action tests whenever new lifecycle automation is introduced. |
| **Applies To** | src/application/story_lifecycle.rs, src/application/voyage_epic_lifecycle.rs, src/application/process_manager.rs |
| **Applied** | Introduced DomainEvent + DomainProcessManager and rewired lifecycle services to emit events. |



---

## Story: Rewire Command Handlers To Use Cases (1vwqCfPpe)

### 1vyDuwnad: Enforce policy invariants in application services, not command handlers

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Story and voyage lifecycle commands need consistent enforcement of manual-verification acceptance and requirements-coverage gating. |
| **Insight** | Putting enforcement in application services centralizes lifecycle invariants and avoids drift across multiple command entrypoints. |
| **Suggested Action** | Keep command handlers thin and add service-level tests for every lifecycle policy; use architecture contract tests to block direct transition orchestration from handlers. |
| **Applies To** | `src/application/story_lifecycle.rs`, `src/application/voyage_epic_lifecycle.rs`, `src/commands/{story,voyage,epic}/*.rs`, `src/architecture_contract_tests.rs` |
| **Applied** | story `1vwqCfPpe` |



---

## Story: Implement Voyage And Epic Lifecycle Use Cases (1vwqCejs5)

### 1vyDuwgcV: Keep Lifecycle Command Handlers As Thin Adapters

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Refactoring voyage and epic lifecycle commands to align with application-service orchestration boundaries |
| **Insight** | Moving orchestration into a dedicated application service lets command modules stay stable adapters while preserving behavior through existing command tests |
| **Suggested Action** | Add use-case methods first, then delegate command `run` entrypoints to those methods and update cross-command callsites to service APIs |
| **Applies To** | src/application/*.rs; src/commands/voyage/*.rs; src/commands/epic/*.rs |
| **Applied** | 1vwqCejs5 |



---

## Story: Implement Story Lifecycle Use Cases (1vwqCe5T0)

### 1vyDuwDTq: Thin Command Adapters Preserve Behavior During Refactors

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Extracting orchestration out of CLI command handlers while keeping existing workflow behavior stable |
| **Insight** | Moving orchestration to an application service is low-risk when command handlers become thin pass-through adapters and existing command tests remain the compatibility suite. |
| **Suggested Action** | For future migrations, extract service logic first, then convert command files to wrappers and keep legacy helper behavior behind `#[cfg(test)]` shims only where needed. |
| **Applies To** | src/application/story_lifecycle.rs, src/commands/story/{start,submit,accept,reject,ice,thaw}.rs |
| **Applied** | yes |



---

## Synthesis

### t2UGk6lpx: Event-First Cross-Aggregate Orchestration Preserves Boundaries

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Story start/accept and voyage completion previously embedded cross-aggregate calls inside lifecycle services. |
| **Insight** | Emitting explicit domain events and routing follow-on actions through a process manager keeps use cases focused while preserving existing behavior. |
| **Suggested Action** | Keep cross-aggregate progression in process managers and add event/action tests whenever new lifecycle automation is introduced. |
| **Applies To** | src/application/story_lifecycle.rs, src/application/voyage_epic_lifecycle.rs, src/application/process_manager.rs |
| **Linked Knowledge IDs** | 1vyDuwiVQ |
| **Score** | 0.84 |
| **Confidence** | 0.92 |
| **Applied** | Introduced DomainEvent + DomainProcessManager and rewired lifecycle services to emit events. |

### LaLCqrPQZ: Enforce policy invariants in application services, not command handlers

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Story and voyage lifecycle commands need consistent enforcement of manual-verification acceptance and requirements-coverage gating. |
| **Insight** | Putting enforcement in application services centralizes lifecycle invariants and avoids drift across multiple command entrypoints. |
| **Suggested Action** | Keep command handlers thin and add service-level tests for every lifecycle policy; use architecture contract tests to block direct transition orchestration from handlers. |
| **Applies To** | `src/application/story_lifecycle.rs`, `src/application/voyage_epic_lifecycle.rs`, `src/commands/{story,voyage,epic}/*.rs`, `src/architecture_contract_tests.rs` |
| **Linked Knowledge IDs** | 1vyDuwnad |
| **Score** | 0.90 |
| **Confidence** | 0.96 |
| **Applied** | story `1vwqCfPpe` |

### r1jV4LrNV: Keep Lifecycle Command Handlers As Thin Adapters

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Refactoring voyage and epic lifecycle commands to align with application-service orchestration boundaries |
| **Insight** | Moving orchestration into a dedicated application service lets command modules stay stable adapters while preserving behavior through existing command tests |
| **Suggested Action** | Add use-case methods first, then delegate command `run` entrypoints to those methods and update cross-command callsites to service APIs |
| **Applies To** | src/application/*.rs; src/commands/voyage/*.rs; src/commands/epic/*.rs |
| **Linked Knowledge IDs** | 1vyDuwgcV |
| **Score** | 0.82 |
| **Confidence** | 0.88 |
| **Applied** | 1vwqCejs5 |

### SrUkwz3sZ: Thin Command Adapters Preserve Behavior During Refactors

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Extracting orchestration out of CLI command handlers while keeping existing workflow behavior stable |
| **Insight** | Moving orchestration to an application service is low-risk when command handlers become thin pass-through adapters and existing command tests remain the compatibility suite. |
| **Suggested Action** | For future migrations, extract service logic first, then convert command files to wrappers and keep legacy helper behavior behind `#[cfg(test)]` shims only where needed. |
| **Applies To** | src/application/story_lifecycle.rs, src/commands/story/{start,submit,accept,reject,ice,thaw}.rs |
| **Linked Knowledge IDs** | 1vyDuwDTq |
| **Score** | 0.90 |
| **Confidence** | 0.91 |
| **Applied** | yes |

