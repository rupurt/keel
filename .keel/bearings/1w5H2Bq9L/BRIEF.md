# Semantic Search and Ranking in Keel — Brief

## Hypothesis
Implementing semantic search using a pure-Rust, in-process embedding and vector store will significantly improve knowledge discovery in Keel without sacrificing its standalone, statically-linked nature.

## Problem Space
Current search in Keel is limited to simple case-insensitive substring matching on IDs and Titles. This misses semantic context (e.g., searching for "bug" doesn't find "crash") and doesn't rank results by relevance, making it difficult to navigate large boards as they grow.

## Success Criteria
- [ ] Sub-millisecond search performance for typical board sizes (hundreds of documents).
- [ ] No external database or server required.
- [ ] Statically linked binary for Linux, Mac, and Windows (no DLLs).
- [ ] Results ranked by semantic relevance (cosine similarity).
- [ ] Pure Rust implementation (via `Candle` backend).

## Open Questions
- **Model Distribution**: Should we download weights on first run or embed a tiny model in the binary?
- **Index Persistence**: Is an in-memory index sufficient, or should we cache embeddings on disk to speed up startup?
- **Resource Usage**: What is the memory footprint of keeping several hundred high-dimensional embeddings in memory?

## Research Findings

### [FastEmbed-rs](https://github.com/Anush008/fastembed-rs)
- **Pros**: Most popular, high performance, wide model support.
- **Pure Rust**: Supports `candle` as a backend to avoid C++ dependencies (`onnxruntime`).
- **Linking**: Can produce a statically linked binary.

### [VectorLite](https://github.com/v0-capital/vectorlite)
- **Pros**: All-in-one in-process vector store using `Candle`. Simplifies both embedding and indexing.
- **Linking**: Pure Rust, no external services.

### [zvec](https://github.com/alibaba/zvec)
- **Pros**: In-memory vector index by Alibaba, designed for high performance.
- **Note**: Needs to be paired with an embedding generator.

### [Embed-Anything](https://github.com/starlight-search/embed-anything)
- **Pros**: Minimalist, local-first embedding pipeline using `Candle`.
