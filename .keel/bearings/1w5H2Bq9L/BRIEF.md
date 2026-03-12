# Semantic Search and Ranking in Keel — Brief

## Hypothesis

Implementing semantic search with a pure-Rust, in-process embedding and vector
search stack will materially improve discovery on large Keel boards without
breaking the standalone, statically linked distribution model.

## Problem Space

Current search in Keel is limited to simple case-insensitive substring matching
on IDs and titles. That misses semantic context, fails to rank near matches by
relevance, and makes large boards progressively harder to navigate as the
artifact graph grows.

## Success Criteria

- [ ] Search queries can return semantically related results, not just literal
      substring matches.
- [ ] Relevance ranking improves operator discovery without introducing an
      external service dependency.
- [ ] The chosen approach remains compatible with a statically linked Keel
      binary on supported platforms.

## Open Questions

- Should model weights download on first run or ship with a compact default
  model?
- Is an in-memory index sufficient, or should embeddings be cached on disk to
  reduce startup cost?
- What memory footprint is acceptable for hundreds of in-memory embeddings?

## Research Findings

### [FastEmbed-rs](https://github.com/Anush008/fastembed-rs)
- **Pros**: Most popular, high performance, wide model support.
- **Pure Rust**: Supports `candle` as a backend to avoid C++ dependencies
  (`onnxruntime`).
- **Linking**: Can produce a statically linked binary.

### [VectorLite](https://github.com/v0-capital/vectorlite)
- **Pros**: All-in-one in-process vector store using `Candle`. Simplifies both
  embedding and indexing.
- **Linking**: Pure Rust, no external services.

### [zvec](https://github.com/alibaba/zvec)
- **Pros**: In-memory vector index by Alibaba, designed for high performance.
- **Note**: Needs to be paired with an embedding generator.

### [Embed-Anything](https://github.com/starlight-search/embed-anything)
- **Pros**: Minimalist, local-first embedding pipeline using `Candle`.
