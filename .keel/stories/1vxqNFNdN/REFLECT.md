# Reflection - Implement Project Autodetection And Recommendation Engine

## Knowledge

### L001: Deterministic ranking requires total-order tie breaks
| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Recommendation scores can tie across techniques when confidence and keyword matches are equivalent. |
| **Insight** | Deterministic ordering is guaranteed only when ranking sorts by score and then by stable id as a total-order tie breaker. |
| **Suggested Action** | Keep recommendation outputs sorted by `(score desc, id asc)` and normalize lists/sets before scoring. |
| **Applies To** | `src/read_model/verification_techniques.rs` |
| **Observed At** | 2026-03-04T20:49:05Z |
| **Score** | 0.80 |
| **Confidence** | 0.90 |
| **Applied** | yes |

## Observations

Modeling signal detection and ranking in the read-model layer made it easy to test determinism without invoking CLI paths.
Confidence/hint aggregation is easiest to reason about when the detector emits normalized tokens and deduplicated artifact markers.
Story submission still depends on AC reference regex constraints, so AC labels using NFR IDs should be normalized before submit gates.
