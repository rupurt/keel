# Reflection - Escalate Unresolved Scaffold Checks To Doctor Errors

## Knowledge

### L001: Report Pattern And Severity From One Shared Placeholder Extractor

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Multiple doctor checks were independently detecting TODO/tokens and emitting inconsistent warning messages. |
| **Insight** | A shared unresolved-pattern extractor enables deterministic detection and allows every check to emit the same actionable `pattern: ...` output while enforcing error severity. |
| **Suggested Action** | Route all new scaffold/default-text checks through the shared extractor and assert severity/message structure in unit tests. |
| **Applies To** | `src/infrastructure/validation/structural.rs`, `src/cli/commands/diagnostics/doctor/checks/*.rs` |
| **Observed At** | 2026-03-03T05:58:00Z |
| **Score** | 0.84 |
| **Confidence** | 0.91 |
| **Applied** | yes |

## Observations

- Converting only severity was not enough; AC-02 needed explicit offending-pattern strings in the rendered doctor message.
- Keeping the old warning-oriented assertions would silently permit rollback, so explicit hard-cutover tests were necessary.
