# Reflection - Define Planning Show Output Contracts

## Knowledge

### L001: Centralized show projections reduce drift
| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Multiple show commands were independently parsing PRD/SRS/story evidence with diverging placeholder and ordering rules. |
| **Insight** | A shared read-model projection layer stabilizes data contracts, keeps ordering deterministic, and lets renderers remain thin. |
| **Suggested Action** | Add new planning/read surfaces by extending `read_model::planning_show` first, then adapt renderer output only. |
| **Applies To** | `src/read_model/planning_show.rs`, `src/cli/commands/management/*/show.rs` |
| **Observed At** | 2026-03-04T19:47:27Z |
| **Score** | 0.84 |
| **Confidence** | 0.89 |
| **Applied** | yes |

## Observations

Moving parsing/build logic into one module made it straightforward to keep epic, voyage, and story behavior aligned while preserving existing renderer tests.
The main friction was shell argument handling for `story record --msg` through `just keel`; punctuation and spaces were split by the shell wrapper, so compact message tokens were used.
Deterministic ordering checks were easiest to validate by comparing projections from two equivalent boards created with reversed story insertion and artifact creation order.
