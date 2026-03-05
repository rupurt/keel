---
source_type: Voyage
source: epics/1vwq96cpt/voyages/1vwq9RqCe/KNOWLEDGE.md
created_at: 2026-03-02T10:07:49
---

### hpVUqQ88C: Frontmatter-rewrite adapters preserve markdown parity with low migration risk

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Implementing filesystem repository/document adapters over existing `.keel` markdown files without changing domain/application behavior. |
| **Insight** | Parsing existing frontmatter, serializing updated typed frontmatter, and reattaching the original body provides a practical parity-preserving persistence strategy while introducing port-based boundaries. |
| **Suggested Action** | Reuse this adapter pattern for future repository migrations and add command-level integration points incrementally to avoid broad behavior shifts. |
| **Applies To** | `src/infrastructure/fs_adapters.rs`, `src/application/ports.rs`, markdown-backed aggregate repositories |
| **Linked Knowledge IDs** | 1vyDuwPS4 |
| **Observed At** |  |
| **Score** | 0.84 |
| **Confidence** | 0.95 |
| **Applied** | story `1vwqCeXD8` |
