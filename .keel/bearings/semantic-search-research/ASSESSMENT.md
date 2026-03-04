---
id: semantic-search-research
---

# Semantic Search and Ranking in Keel — Assessment

## Scoring Factors

| Factor | Score | Rationale |
|--------|-------|-----------|
| Impact | 4 | Significant improvement in knowledge retrieval. |
| Confidence | 5 | Proven libraries and patterns for semantic search in Rust. |
| Effort | 3 | Moderate effort to integrate libraries and add search CLI. |
| Risk | 2 | Minimal risk; fallback to current fuzzy search is easy. |

## Analysis

### Opportunity Cost
Developing semantic search delays other features like graph visualization improvements or better ADR management, but the ROI in knowledge discovery is high.

### Dependencies
- **Model weights**: Reliable mechanism for model weight distribution (e.g., downloading to `.keel/cache/models/`).
- **Rust Toolchain**: Statically linked binary requires careful dependency management to avoid dynamic libraries (especially for `fastembed-rs` + `candle`).

### Alternatives Considered
- **Standard fuzzy search (current)**: Simple but misses semantic context (e.g., searching for "bug" doesn't find "crash").
- **External database (Qdrant/Milvus)**: Overkill for Keel's standalone philosophy and introduces infrastructure overhead.

## Recommendation

[x] Proceed → convert to epic
[ ] Park → revisit later
[ ] Decline → document learnings
