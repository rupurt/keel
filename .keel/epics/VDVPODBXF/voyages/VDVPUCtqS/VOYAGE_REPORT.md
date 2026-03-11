# VOYAGE REPORT: Role and Lane Config Contract

## Voyage Metadata
- **ID:** VDVPUCtqS
- **Epic:** VDVPODBXF
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 5/5 stories complete

## Implementation Narrative
### Add Workflow Topology Config Model
- **ID:** VDVRaQp66
- **Status:** done

#### Summary
Add the config schema and effective-topology resolver that seed default roles and lanes, carry lane behavior, and compile selector inputs into one canonical source catalog.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] [SRS-NFR-01/AC-01] Add workflow-topology config structs and seeded default resolution so boards with no topology overrides still resolve `manager`/`operator` roles and `management`/`delivery` lanes. <!-- verify: cargo test -p keel workflow_topology_, SRS-01:start:end, SRS-NFR-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] `keel config show` renders the effective seeded defaults, configured role families, lane definitions, and exact overrides rather than only raw authored fragments. <!-- verify: cargo test -p keel config_show_workflow_topology_, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-03/AC-01] Lane config captures `description`, ordered `include`/`exclude`, `parallel`, `manual_accept`, and `priority` fields and compiles selector inputs against the canonical source catalog. <!-- verify: cargo test -p keel workflow_topology_lane_config_, SRS-03:start:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VDVRaQp66/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/VDVRaQp66/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/VDVRaQp66/EVIDENCE/ac-2.log)

### Route Next Through Configured Lanes
- **ID:** VDVRaRR5x
- **Status:** done

#### Summary
Replace hardcoded `manager` and `engineer` queue routing in `keel next` with topology-driven lane resolution and lane capability checks.

#### Acceptance Criteria
- [x] [SRS-04/AC-01] `keel next --role <taxonomy>` resolves configured base role families to their default lanes and rejects unknown families with guidance based on configured default role examples. <!-- verify: cargo test -p keel next_role_topology_, SRS-04:start:end, proof: ac-1.log-->
- [x] [SRS-05/AC-01] [SRS-NFR-02/AC-01] `keel next --parallel` is allowed only for lanes with `parallel = true`, and repeated resolution of the same role/config yields identical lane and capability results. <!-- verify: cargo test -p keel next_parallel_topology_, SRS-05:start:end, SRS-NFR-02:start:end, proof: ac-2.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VDVRaRR5x/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VDVRaRR5x/EVIDENCE/ac-2.log)

### Validate Topology Selectors And Overlap
- **ID:** VDVRaSf5y
- **Status:** done

#### Summary
Add hard-fail validation for topology integrity so bad defaults, bad references, bad selectors, or cross-lane overlap are caught before routing and rendering drift.

#### Acceptance Criteria
- [x] [SRS-09/AC-01] `keel doctor` fails on missing defaults, bad role-to-lane references, unknown selectors, and cross-lane overlap. <!-- verify: cargo test -p keel doctor_topology_, SRS-09:start:end, proof: ac-1.log-->
- [x] [SRS-09/AC-02] [SRS-NFR-03/AC-01] Selector compilation surfaces precise hard failures and never silently drops invalid or unknown patterns. <!-- verify: cargo test -p keel workflow_topology_selector_errors_, SRS-09:continues:end, SRS-NFR-03:start:end, proof: ac-2.log-->

### Authorize Acceptance And Templates Through Topology
- **ID:** VDVRaSs5w
- **Status:** done

#### Summary
Move manual acceptance authorization and role-context template selection onto the resolved topology so configured roles and exact overrides drive behavior.

#### Acceptance Criteria
- [x] [SRS-06/AC-01] Manual acceptance checks use the resolved lane's `manual_accept` capability instead of literal `manager/*` matching. <!-- verify: cargo test -p keel story_accept_topology_, SRS-06:start:end, proof: ac-1.log-->
- [x] [SRS-07/AC-01] Role context and guidance resolve from configured base roles, with exact `role_overrides` taking precedence when the full taxonomy matches. <!-- verify: cargo test -p keel role_context_topology_, SRS-07:start:end, proof: ac-2.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VDVRaSs5w/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VDVRaSs5w/EVIDENCE/ac-2.log)

### Render Dynamic Lanes In Flow
- **ID:** VDVRaTB5z
- **Status:** done

#### Summary
Expose the effective topology in `flow` by rendering configured lane cards and ordering from the resolved lane definitions.

#### Acceptance Criteria
- [x] [SRS-08/AC-01] `keel flow` renders configured lanes in deterministic `priority` order and counts only work selected by each lane's resolved sources. <!-- verify: bash stories/VDVRaTB5z/scripts/render-workflow-topology-flow.sh, SRS-08:start:end, proof: ac-1.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VDVRaTB5z/EVIDENCE/ac-1.log)


