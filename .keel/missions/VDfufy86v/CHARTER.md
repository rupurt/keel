# Knowledge Graph and Drift Cartography - Charter

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Deliver a canonical knowledge graph that unifies board entities, authored artifacts, knowledge units, project documents, and source code, then use that graph to power interactive/static world views and structural drift visibility across Keel surfaces. | board: VDfufyl6w |

## Constraints

- Extend the existing `BoardGraph`, knowledge, and topology surfaces instead of introducing parallel graph models or duplicate relationship scanners.
- Structural graph relationships and doctor checks must remain deterministic and explainable; semantic edges are advisory only and must never affect doctor outcomes.
- Keep the implementation pure local Rust with a repo-local cache under `.keel/cache/knowledge-graph/` and a lightweight Candle-backed embedding path; no remote services or nondeterministic online dependencies.
- Preserve a harness-safe `--static` path alongside interactive terminal rendering and keep static output stable enough for snapshot tests.

## Halting Rules

- DO NOT halt while epic `VDfufyl6w` lacks a planned voyage, executable story, or unresolved board-backed requirement.
- YIELD to human before introducing heavyweight local models or cache formats that materially change Keel's build/runtime footprint beyond the lightweight Candle embedding path.
- HALT when epic `VDfufyl6w` lands a canonical knowledge graph, deterministic cache substrate, interactive/static `keel knowledge graph` surface, and structural drift coefficient visibility for topology/show-style experiences with doctor-clean evidence.
