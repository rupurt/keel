# Knowledge Graph and Drift Cartography - Charter

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Deliver the canonical knowledge graph kernel that unifies board entities, authored artifacts, knowledge units, project documents, and source code, plus the deterministic cache and structural drift inputs the rest of the mission will reuse. | board: VDfufyl6w |
| MG-02 | Deliver the interactive/static `keel knowledge graph` experience and surface the structural drift coefficient through graph-adjacent read experiences such as topology and show commands. | board: VDg0dAPVS |

## Constraints

- Extend the existing `BoardGraph`, knowledge, and topology surfaces instead of introducing parallel graph models or duplicate relationship scanners.
- Structural graph relationships and doctor checks must remain deterministic and explainable; semantic edges are advisory only and must never affect doctor outcomes.
- Keep the implementation pure local Rust with a repo-local cache under `.keel/cache/knowledge-graph/` and a lightweight Candle-backed embedding path; no remote services or nondeterministic online dependencies.
- Preserve a harness-safe `--static` path alongside interactive terminal rendering and keep static output stable enough for snapshot tests.

## Halting Rules

- DO NOT halt while any mission epic linked to an `MG-*` board goal lacks a planned voyage, executable story, or unresolved board-backed requirement.
- YIELD to human before introducing heavyweight local models or cache formats that materially change Keel's build/runtime footprint beyond the lightweight Candle embedding path.
- HALT when epics `VDfufyl6w` and `VDg0dAPVS` land the canonical knowledge graph kernel, deterministic cache substrate, interactive/static `keel knowledge graph` surface, and structural drift coefficient visibility for topology/show-style experiences with doctor-clean evidence.
