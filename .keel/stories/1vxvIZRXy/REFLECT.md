# Reflection - Refactor Config Show Into Technique Flag Matrix

## Knowledge

### L001: Prefer direct status flags over aggregated recommendation blocks
| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Config introspection commands where automation depends on deterministic machine-readable state |
| **Insight** | A per-technique flag matrix (`detected`, `disabled`, `active`) is a better contract boundary than mixed narrative sections because it cleanly separates inventory from recommendation logic. |
| **Suggested Action** | Keep config/read commands focused on canonical state and move advisory ranking/commentary to dedicated recommend commands. |
| **Applies To** | `src/cli/commands/setup/config.rs`, `src/read_model/verification_techniques.rs` |
| **Observed At** | 2026-03-04T23:34:00Z |
| **Score** | 0.86 |
| **Confidence** | 0.90 |
| **Applied** | yes |

## Observations

- The old `config show` renderer was tightly coupled to recommendation and scoring output, so introducing a structured projection object simplified both text and JSON rendering paths.
- Verification evidence recording is sensitive to shell tokenization in `--msg`; compact messages avoid accidental argument splitting in automation scripts.
