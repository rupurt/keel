---
created_at: 2026-03-03T09:13:53
---

# Reflection - Enforce Terminal Story Coherency In Doctor

## Knowledge

### 1vyDuwdbL: Stage-gate scaffold checks to avoid noisy early warnings

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Adding scaffold/default text diagnostics to doctor checks |
| **Insight** | Stage filtering is critical: terminal-only checks avoid penalizing in-progress drafting while still hard-failing review-complete states. |
| **Suggested Action** | Reuse a shared unresolved-pattern detector and explicitly gate by story stage (`needs-human-verification`, `done`) for terminal coherency rules. |
| **Applies To** | src/cli/commands/diagnostics/doctor/checks/stories.rs, src/infrastructure/validation/structural.rs |
| **Observed At** | 2026-03-03T17:13:33Z |
| **Score** | 0.85 |
| **Confidence** | 0.91 |
| **Applied** | yes |

## Observations

The implementation was straightforward once the existing unresolved-pattern helper was reused. The main practical effect is that doctor now surfaces a backlog of previously hidden terminal-story scaffold issues, which matches the hard-enforcement intent and will require a follow-up cleanup pass of existing `.keel` artifacts.
