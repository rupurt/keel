---
id: 1w5H2Bq9L
---

# Semantic Search and Ranking in Keel — Evidence

## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | academic | manual:prior-art-review | https://arxiv.org/abs/1706.03762 | 2017-06-12 | 2026-03-04 | high | medium | Transformer-era embedding approaches make local semantic ranking viable without external services. |
| SRC-02 | web | manual:github-review | https://github.com/Anush008/fastembed-rs | 2026-03-04 | 2026-03-04 | high | high | `fastembed-rs` provides a pure Rust path for local embedding generation through Candle-backed execution. |
| SRC-03 | web | manual:github-review | https://github.com/v0-capital/vectorlite | 2026-03-04 | 2026-03-04 | medium | high | `vectorlite` demonstrates an in-process vector store that keeps indexing and retrieval local to the CLI runtime. |
| SRC-04 | social | manual:community-signal | https://news.ycombinator.com/ | 2026-03-04 | 2026-03-04 | low | high | Practitioner discussion trends still emphasize lightweight local-first search and ranking workflows for developer tooling. |

## Technical Research

### Feasibility
Implementing semantic search in a standalone CLI without an external database is highly feasible thanks to the Rust ecosystem's deep learning frameworks like `Candle`.

### Existing Solutions

- **[FastEmbed-rs](https://github.com/Anush008/fastembed-rs)**:
    - **Pros**: Most popular, high performance, wide model support.
    - **Pure Rust**: Supports `candle` as a backend to avoid C++ dependencies (`onnxruntime`).
    - **Linking**: Can produce a statically linked binary.
- **[VectorLite](https://github.com/v0-capital/vectorlite)**:
    - **Pros**: All-in-one in-process vector store using `Candle`. Simplifies both embedding and indexing.
    - **Linking**: Pure Rust, no external services or DLLs.
- **[zvec](https://github.com/alibaba/zvec)**:
    - **Pros**: In-memory vector index by Alibaba, designed for high performance.
    - **Note**: Needs to be paired with an embedding generator (like `fastembed-rs` or `candle` directly).
- **[Embed-Anything](https://github.com/starlight-search/embed-anything)**:
    - **Pros**: Minimalist, local-first embedding pipeline using `Candle`.

### Proof of Concepts
For a board with hundreds or even a few thousand documents, a simple in-memory linear scan of cosine similarities is fast enough for sub-millisecond search times, eliminating the need for complex HNSW indices.

## Key Findings

1. **Pure Rust is possible**: Using `Candle` allows us to avoid any dynamic linking (DLLs/Shared Objects) for ML models, ensuring cross-platform portability for a single static binary.
2. **all-MiniLM-L6-v2**: This model is the ideal balance of size (~80MB) and performance for semantic search on documentation.
3. **Integration Strategy**: We can reuse the existing `parser.rs` to extract document bodies and generate embeddings during the board loading phase.

## Unknowns

- **Model Distribution**: How should we distribute the model weights? Download on first run (with cache) or embed in the binary (increases binary size significantly)?
- **Resource Usage**: Memory impact of keeping several hundred high-dimensional embeddings in memory on very large boards.
