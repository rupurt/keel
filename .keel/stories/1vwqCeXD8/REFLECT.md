# Reflection - Implement Filesystem Adapter Layer

## Knowledge

<!--
Institutional memory is mandatory. Capture what you learned during implementation.
Format:
### L001: Title
| Field | Value |
|-------|-------|
| **Category** | code/testing/process/architecture |
| **Context** | describe when this applies |
| **Insight** | the fundamental discovery |
| **Suggested Action** | what to do next time |
| **Applies To** | file patterns or components |
| **Observed At** | RFC3339 timestamp (e.g. 2026-02-22T12:00:00Z) |
| **Score** | 0.0-1.0 (impact significance) |
| **Confidence** | 0.0-1.0 (insight quality) |
| **Applied** | |
-->

### L001: Frontmatter-rewrite adapters preserve markdown parity with low migration risk

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Implementing filesystem repository/document adapters over existing `.keel` markdown files without changing domain/application behavior. |
| **Insight** | Parsing existing frontmatter, serializing updated typed frontmatter, and reattaching the original body provides a practical parity-preserving persistence strategy while introducing port-based boundaries. |
| **Suggested Action** | Reuse this adapter pattern for future repository migrations and add command-level integration points incrementally to avoid broad behavior shifts. |
| **Applies To** | `src/infrastructure/fs_adapters.rs`, `src/application/ports.rs`, markdown-backed aggregate repositories |
| **Observed At** | 2026-03-02T18:05:22Z |
| **Score** | 0.84 |
| **Confidence** | 0.95 |
| **Applied** | story `1vwqCeXD8` |

## Observations

Adding adapter-level tests against `TestBoardBuilder` made behavior parity verification fast and deterministic.
The main friction was strict `dead_code` quality gates in a staged migration; targeted allowances were needed until application services consume the new adapter directly.
