# VOYAGE REPORT: Explain Roles Lanes And Next Routing

## Voyage Metadata
- **ID:** VF8hkVGjy
- **Epic:** VF8hiVofm
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 2/2 stories complete

## Implementation Narrative
### Add Roles Surface For Workflow Topology
- **ID:** VF8hnWZs1
- **Status:** done

#### Summary
Add a direct roles surface so workflow topology stops living mainly inside config output and implied `next` behavior.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] `keel roles` exposes configured role families, default lanes, contracts, and lane behavior in a concise human-readable surface. <!-- verify: manual, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] `keel roles --json` exposes stable machine-readable role and lane data. <!-- verify: manual, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-02/AC-01] The role inspection output is deterministic for the same config. <!-- verify: manual, SRS-NFR-02:start:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VF8hnWZs1/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VF8hnWZs1/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/VF8hnWZs1/EVIDENCE/ac-3.log)

### Explain Next Routing With Canonical Role Context
- **ID:** VF8hnXWs0
- **Status:** done

#### Summary
Teach `keel next` to explain its routing decision from canonical topology and role-context data so role-scoped pulls are legible instead of magical.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] `keel next --explain` surfaces the resolved lane, queue type, and role-context contract for the selected role. <!-- verify: manual, SRS-03:start:end, proof: ac-1.log-->
- [x] [SRS-04/AC-01] The explanation is derived from workflow-topology and role-context projections rather than duplicated local heuristics. <!-- verify: manual, SRS-04:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-01/AC-01] Explanation output does not change the underlying next-decision behavior. <!-- verify: manual, SRS-NFR-01:start:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VF8hnXWs0/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VF8hnXWs0/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/VF8hnXWs0/EVIDENCE/ac-3.log)


