# Reflection - Implement Keel.toml Technique Configuration Overrides

## Knowledge

### 1vyDuwSon: Advisory parser keeps keel.toml resilient
| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Technique overrides need richer schema while core config loading should not fail when optional override blocks are malformed. |
| **Insight** | Parsing overrides from raw TOML with per-field diagnostics allows invalid entries to be ignored safely without blocking normal command behavior. |
| **Suggested Action** | Keep optional/advanced config surfaces advisory by default, then merge validated entries into canonical models with explicit diagnostics. |
| **Applies To** | `src/read_model/verification_techniques.rs` |
| **Observed At** | 2026-03-04T20:13:39Z |
| **Score** | 0.82 |
| **Confidence** | 0.88 |
| **Applied** | yes |

## Observations

The typed override model and deterministic merge path were straightforward once precedence was centralized in a single merge function.
Schema validation is easiest to maintain when unknown keys and type mismatches produce path-specific diagnostics instead of hard parse failure.
NFR-tagged AC refs still require lifecycle-gate-compatible formatting in story metadata, so submission should be validated before commit.
