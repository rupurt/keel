---
id: 1w5H2Bq9L
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

## Findings
- Local semantic ranking is feasible without external infrastructure when we pair embedding generation with in-process indexing [SRC-01][SRC-02][SRC-03]
- The workflow direction matches current operator expectations for lightweight, local-first developer tooling [SRC-04]

## Opportunity Cost
Developing semantic search delays other features like graph visualization improvements or better ADR management, but the ROI in knowledge discovery is high [SRC-02][SRC-04].

## Dependencies
- **Model weights**: Reliable mechanism for model weight distribution (e.g., downloading to `.keel/cache/models/`) [SRC-02].
- **Rust Toolchain**: Statically linked binary requires careful dependency management to avoid dynamic libraries (especially for `fastembed-rs` + `candle`) [SRC-02][SRC-03].

## Alternatives Considered
- **Standard fuzzy search (current)**: Simple but misses semantic context (e.g., searching for "bug" doesn't find "crash") [SRC-04].
- **External database (Qdrant/Milvus)**: Overkill for Keel's standalone philosophy and introduces infrastructure overhead [SRC-03].

## Recommendation

[x] Proceed → convert to epic [SRC-01][SRC-02][SRC-03][SRC-04]
[ ] Park → revisit later [SRC-04]
[ ] Decline → document learnings [SRC-03]
