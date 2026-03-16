# VOYAGE REPORT: Bearing Dependency Primitives

## Voyage Metadata
- **ID:** VE1vAyNzt
- **Epic:** VDiHwLLfY
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 3/3 stories complete

## Implementation Narrative
### Add Depends On Field to Bearing Frontmatter
- **ID:** VE1vOqhch
- **Status:** done

#### Summary
Add an optional `depends_on` field to BearingFrontmatter so operators can declare explicit dependency edges between bearings. Update the test fixture to support the new field.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] BearingFrontmatter deserializes `depends_on: [BRG-A, BRG-B]` from bearing README.md and exposes it as `Option<Vec<String>>`. <!-- verify: cargo test -p keel-core bearing_frontmatter_deserializes_depends_on, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Bearings without `depends_on` in frontmatter load with `None` (backward compatible). <!-- verify: cargo test -p keel-core bearing_frontmatter_handles_defaults, SRS-01:start:end, proof: ac-2.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VE1vOqhch/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VE1vOqhch/EVIDENCE/ac-2.log)

### Validate Bearing Dependencies in Doctor
- **ID:** VE1vQc4Lh
- **Status:** done

#### Summary
Add a `check_bearing_dependencies` diagnostic that validates all `depends_on` references exist, detects cycles via DFS, and flags self-references. Register the check under the Sensory subsystem in `keel doctor`.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] Doctor flags an error when `depends_on` contains a bearing ID that does not exist on the board. <!-- verify: cargo test -p keel-core doctor_flags_dangling_depends_on, SRS-02:start:end -->
- [x] [SRS-03/AC-01] Doctor flags an error when the dependency graph contains a cycle. <!-- verify: cargo test -p keel-core doctor_flags_cyclic_depends_on, SRS-03:start:end -->
- [x] [SRS-03/AC-02] Doctor flags an error when a bearing references itself in `depends_on`. <!-- verify: cargo test -p keel-core doctor_flags_self_reference_in_depends_on, SRS-03:start:end -->
- [x] [SRS-NFR-01/AC-01] Dependency validation scales linearly with bearing count. <!-- verify: cargo test -p keel-core dependency_validation_scales_linearly, SRS-NFR-01:start:end -->

### Factor Dependency State into Bearing Sort Order
- **ID:** VE1vTf5Yq
- **Status:** done

#### Summary
Extend bearing sort order in `bearing list` and `next` to demote bearings whose `depends_on` targets are not in a terminal state. Bearings with unresolved dependencies sort below those that are ready to research.

#### Acceptance Criteria
- [x] [SRS-04/AC-01] `bearing list` sorts bearings with unresolved dependencies below bearings whose dependencies are all terminal. <!-- verify: cargo test -p keel bearing_list_demotes_unresolved_dependencies, SRS-04:start:end -->


