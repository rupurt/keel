# Reflection - Add Hard Cutover Regression Coverage

## Knowledge

### 1vyDuwFj5: Assert check identity and severity for hard-cutover gates

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | Updating terminal artifact coherence enforcement for doctor and story transitions |
| **Insight** | Message-only assertions can pass even if a hard error silently downgrades to a warning; check-id plus severity assertions prevent this regression class |
| **Suggested Action** | For each enforcement rule, add at least one integration test that asserts both `check_id` and `severity` |
| **Applies To** | `src/cli/commands/diagnostics/doctor/mod.rs`, `src/domain/state_machine/gating.rs` |
| **Observed At** | 2026-03-03T19:30:00Z |
| **Score** | 0.84 |
| **Confidence** | 0.91 |
| **Applied** | yes |

## Observations

- Doctor and transition logic already emitted `Severity::Error`; the main gap was regression precision in tests.
- `story record` requires `verify` annotations to exist first, so AC lines without verify comments must be normalized before evidence capture.
- Running `just quality` and `just test` after tightening assertions validated there was no impact to unrelated checks.
