# Reflection - Align CLI Contracts For Creation Commands

## Knowledge

### L001: Keep CLI contract updates end-to-end

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | When changing creation command flags and required inputs |
| **Insight** | Command tree flags, runtime mappers, and user-facing suggestion strings drift unless updated in the same slice. |
| **Suggested Action** | Pair every CLI contract edit with parser rejection tests for removed flags and updates to generated command hints. |
| **Applies To** | src/cli/command_tree.rs, src/cli/runtime.rs, src/cli_tests.rs, src/cli/presentation/flow/next_up.rs |
| **Observed At** | 2026-03-03T16:09:32Z |
| **Score** | 0.82 |
| **Confidence** | 0.90 |
| **Applied** | yes |

## Observations

Updated the creation command contract with hard-cutover tests first, then aligned command tree/action enums/runtime wiring. The main friction point was that AC evidence recording requires verify annotations to exist first, so those annotations need to be added before running `keel story record`.
