# VOYAGE REPORT: Epic Topology Terminal View

## Voyage Metadata
- **ID:** 1vyWIM000
- **Epic:** 1vyWIF000
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 4/4 stories complete

## Implementation Narrative
### Build Epic Topology Projection
- **ID:** 1vyWNb000
- **Status:** done

#### Summary
Create the canonical epic-topology projection that composes epic, voyage, story, drift, and dependency signals into one deterministic read model for the new command surface.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Add a canonical epic-topology projection that normalizes epic, voyage, and story nodes into deterministic order using existing board and planning read models. <!-- verify: cargo test --lib topology_projection_builds_epic_voyage_story_graph, SRS-01:start, proof: ac-1.log -->
- [x] [SRS-01/AC-03] [SRS-NFR-01/AC-01] Equivalent board states produce the same node ordering, grouping, and annotation order in the topology projection. <!-- verify: cargo test --lib topology_projection_is_deterministic_across_board_loads, SRS-NFR-01:start:end, SRS-01:continues, proof: ac-2.log -->
- [x] [SRS-01/AC-04] [SRS-NFR-03/AC-01] The projection sources lineage and dependency inputs through canonical planning and traceability helpers instead of duplicate parsing logic. <!-- verify: cargo test --lib topology_projection_reuses_canonical_read_models, SRS-NFR-03:start:end, SRS-01:end, proof: ac-3.log -->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vyWNb000/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vyWNb000/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vyWNb000/EVIDENCE/ac-2.log)

### Add Topology Command And Filters
- **ID:** 1vyWNd000
- **Status:** done

#### Summary
Introduce the `keel topology` command path, epic targeting, and done-visibility controls so the new projection is reachable from the terminal in the intended human workflow.

#### Acceptance Criteria
- [x] [SRS-01/AC-02] `keel topology --epic <id>` resolves the target epic and renders the topology projection through a dedicated informational command path. <!-- verify: cargo test --lib topology_command_invokes_epic_projection, SRS-01:end, proof: ac-1.log -->
- [x] [SRS-02/AC-01] The command defaults to a focused view that hides done voyages and stories while preserving planned and in-progress flow. <!-- verify: cargo test --lib topology_command_hides_done_by_default, SRS-02:start, proof: ac-2.log -->
- [x] [SRS-02/AC-02] An explicit done-visibility option reveals done voyages and stories without changing the default operational view. <!-- verify: cargo test --lib topology_command_includes_done_when_requested, SRS-02:end, proof: ac-3.log -->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vyWNd000/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vyWNd000/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vyWNd000/EVIDENCE/ac-2.log)

### Render Topology Drift Hotspots
- **ID:** 1vyWNi000
- **Status:** done

#### Summary
Render the topology tree and inline hotspot annotations so operators can immediately see where planning or execution drift is accumulating inside the epic flow.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] The topology renderer highlights epic and voyage drift hotspots from scope lineage issues and uncovered PRD or SRS requirement coverage. <!-- verify: cargo test --lib topology_renderer_surfaces_scope_and_coverage_hotspots, SRS-03:start, proof: ac-1.log -->
- [x] [SRS-03/AC-02] The topology renderer highlights story-level blockage from unmet dependencies and missing proof or verification coverage. <!-- verify: cargo test --lib topology_renderer_surfaces_dependency_and_proof_gaps, SRS-03:end, proof: ac-2.log -->
- [x] [SRS-03/AC-03] [SRS-NFR-02/AC-01] The renderer remains readable at common terminal widths by collapsing or summarizing detail instead of producing ambiguous layout. <!-- verify: vhs tapes/topology-epic-drift.tape, SRS-NFR-02:start:end, SRS-03:continues, proof: ac-3.gif -->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vyWNi000/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vyWNi000/EVIDENCE/ac-2.log)
![ac-3.gif](../../../../stories/1vyWNi000/EVIDENCE/ac-3.gif)
![record-cli.gif](../../../../stories/1vyWNi000/EVIDENCE/record-cli.gif)

### Surface Topology Knowledge And Horizon Warnings
- **ID:** 1vyWNn000
- **Status:** done

#### Summary
Attach scoped knowledge and forward-looking commentary to the topology output so operators can see what execution has already taught the system and what to watch next.

#### Acceptance Criteria
- [x] [SRS-04/AC-01] The topology view surfaces scoped recent insights and pending unapplied knowledge as annotations relevant to the epic flow. <!-- verify: cargo test --lib topology_annotations_surface_scoped_knowledge, SRS-04:start:end, proof: ac-1.log -->
- [x] [SRS-05/AC-01] Horizon commentary surfaces verification debt and throughput or ETA risk using deterministic board signals. <!-- verify: cargo test --lib topology_horizon_surfaces_verification_and_eta_risk, SRS-05:start, proof: ac-2.log -->
- [x] [SRS-05/AC-02] Horizon commentary surfaces tech or process debt heuristics from scoped knowledge signals and labels recommendations as advisory. <!-- verify: llm-judge, SRS-05:continues, proof: ac-3.log -->
- [x] [SRS-05/AC-03] [SRS-NFR-03/AC-02] Knowledge and horizon derivation reuse existing scanner and navigator helpers rather than duplicate parsing paths. <!-- verify: cargo test --lib topology_horizon_reuses_knowledge_helpers, SRS-NFR-03:end, SRS-05:end, proof: ac-4.log -->

#### Verified Evidence
- [ac-4.log](../../../../stories/1vyWNn000/EVIDENCE/ac-4.log)
- [ac-1.log](../../../../stories/1vyWNn000/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vyWNn000/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vyWNn000/EVIDENCE/ac-2.log)
- [llm-judge-horizon-commentary-surfaces-tech-or-process-debt-heuristics-from-scoped-knowledge-signals-and-labels-recommendations-as-advisory.txt](../../../../stories/1vyWNn000/EVIDENCE/llm-judge-horizon-commentary-surfaces-tech-or-process-debt-heuristics-from-scoped-knowledge-signals-and-labels-recommendations-as-advisory.txt)


