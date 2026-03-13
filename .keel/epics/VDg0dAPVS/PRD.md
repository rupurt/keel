# Interactive Knowledge Graph Surfaces - Product Requirements

## Problem Statement

The mission now has a deterministic knowledge graph kernel, but there is no interactive/static keel knowledge graph surface or drift visibility on topology/show commands, so operators still cannot explore the world model or see structural drift in daily command output.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Give operators and mission stewards a first-class `keel knowledge graph` surface for exploring the canonical world model. | The command exposes stable interactive and static flows backed by the canonical graph projection. | 100% of knowledge-graph command paths use the canonical projection/cache substrate |
| GOAL-02 | Make structural drift visible in day-to-day terminal surfaces instead of leaving it buried in one-off diagnostics. | Topology and show-style read surfaces expose a stable drift signal and supporting context sourced from the graph kernel. | Drift signal available in automated CLI/read-model coverage and visible in shipped surfaces |
| GOAL-03 | Keep graph-facing surfaces deterministic, offline, and architecturally convergent. | Equivalent inputs produce stable static output and adjacent commands consume shared graph/drift read models. | Static graph rendering and reused drift projection stay deterministic across repeated runs |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Operator | The person executing work and navigating command-line surfaces throughout the day. | An interactive graph that makes the project world legible without reading dozens of files. |
| Mission Steward | The planner or harness coordinating broad work across missions, epics, and voyages. | A stable static graph and drift signal that highlight where intent and reality are diverging. |
| Architect | The reviewer checking whether command surfaces and projections converge on one relationship model. | Confidence that knowledge-graph experiences reuse the canonical projection instead of inventing new scanners. |

## Scope

### In Scope

- [SCOPE-01] Interactive `keel knowledge graph` rendering with a terminal-friendly default mode and a deterministic `--static` path for harnesses and snapshots.
- [SCOPE-02] Reuse of the canonical knowledge-graph projection/cache for rendering, focus, zoom, and graph summaries instead of introducing duplicate command-local scanners.
- [SCOPE-03] Structural drift coefficient rendering and supporting context in graph-adjacent surfaces such as topology and show-style views.
- [SCOPE-04] Deterministic output contracts and viewport-safe interactive behavior suitable for local use and automated proof capture.

### Out of Scope

- [SCOPE-05] Treating semantic neighborhoods or embedding similarity as doctor gates, mission gates, or lifecycle invariants.
- [SCOPE-06] Remote embedding providers, cloud-hosted search, or nondeterministic online graph enrichment.
- [SCOPE-07] Symbol-level code graphs or advanced clustering beyond file-level world-map rendering.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Implement `keel knowledge graph` as an interactive whole-project world map backed by the canonical knowledge-graph projection. | GOAL-01, GOAL-03 | must | The mission needs a real exploration surface, not just the underlying graph substrate. |
| FR-02 | Provide a deterministic `--static` knowledge-graph rendering suitable for harnesses, snapshots, and low-context review. | GOAL-01, GOAL-03 | must | The graph must be usable in automation and documentation flows, not only on live TTYs. |
| FR-03 | Surface the structural drift coefficient and supporting mismatch context inside graph-adjacent read surfaces, including a distinct topology radar treatment and compact show-style summaries. | GOAL-02, GOAL-03 | must | Drift only changes behavior when people can actually see it during routine command use. |
| FR-04 | Reuse shared graph/drift read models from topology/show-style commands rather than introducing command-local relationship logic. | GOAL-02, GOAL-03 | should | The graph effort only pays off if adjacent surfaces stop rebuilding their own partial views. |
| FR-05 | Preserve semantic proximity as advisory only and keep interactive rendering correct when semantic cache data is absent or stale. | GOAL-01, GOAL-03 | must | The UI should remain useful and deterministic even before embedding work arrives. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Equivalent inputs must produce stable static graph output, drift metrics, and surface summaries across repeated runs. | GOAL-02, GOAL-03 | must | Static graph output and drift surfaces need snapshot-friendly determinism. |
| NFR-02 | All rendering and drift surfacing must remain fully local/offline and must degrade gracefully when semantic cache artifacts are missing. | GOAL-01, GOAL-03 | must | The user explicitly wants local Rust execution with no remote dependency for these surfaces. |
| NFR-03 | Interactive graph rendering must fit the terminal viewport and fall back safely when a TTY is unavailable. | GOAL-01 | must | Exploration should feel usable rather than chaotic on real terminals and harnesses. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Interactive/static command surface | CLI tests, renderer tests, and terminal smoke proofs | Story-level command proofs plus `vhs` or terminal smoke evidence |
| Drift surfacing | Read-model tests and CLI output assertions for topology/show-style commands | Story-level command proofs plus `cargo test` |
| Repo safety | Full hygiene and doctor checks on the real board | `just quality`, `just test`, `just doctest`, `just keel doctor` |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| The current knowledge-graph kernel exposes enough structural information to drive useful first-render surfaces without a semantic neighborhood pass. | The first graph view may feel sparse, but it should still unblock exploration and drift visibility. | Build the first surface on structural graph data and evaluate before expanding semantic work. |
| A single follow-on epic can reasonably carry both the graph command surface and drift visibility work as long as the voyage decomposition stays narrow. | The epic could grow too broad if stories are not sliced carefully. | Use one voyage with tightly scoped stories and verify command-level proofs before expanding scope. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| How much detail should the interactive graph reveal at each zoom level before the view becomes noisy? | Epic owner | Open |
| Which show-style commands should surface the drift coefficient first to maximize value without clutter? | Epic owner | Resolved: start with mission, epic, and voyage show commands plus topology |
| Whether a single voyage can land the first graph surface and drift visibility cleanly, or whether a follow-on voyage will be needed after the first story set. | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] `keel knowledge graph` can render a whole-project interactive graph and a deterministic `--static` graph from the canonical projection.
- [ ] The structural drift coefficient is visible in graph-adjacent read surfaces such as topology and show commands.
- [ ] Static graph output, drift summaries, and reused graph projections remain deterministic across repeated runs.
- [ ] `just keel doctor`, `just quality`, `just test`, and `just doctest` remain clean after the new graph surfaces land.
<!-- END SUCCESS_CRITERIA -->
