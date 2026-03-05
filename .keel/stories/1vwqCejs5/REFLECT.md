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

### 1vyDuwgcV: Keep Lifecycle Command Handlers As Thin Adapters

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
