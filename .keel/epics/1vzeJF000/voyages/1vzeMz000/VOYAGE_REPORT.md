# VOYAGE REPORT: Flow Integration

## Voyage Metadata
- **ID:** 1vzeMz000
- **Epic:** 1vzeJF000
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 4/4 stories complete

## Implementation Narrative
### Refine Completeness Analysis
- **ID:** 1vzeUf000
- **Status:** done

#### Summary
Implement CHARTER.md completeness analysis for the refine command's question generation.

#### Acceptance Criteria
- [x] [SRS-05/AC-01] Completeness analysis checks Goals, Constraints, and Halting Rules sections for authored content <!-- verify: test, SRS-05:start:end -->
- [x] [SRS-05/AC-02] Analysis generates contextual questions for missing or incomplete sections <!-- verify: test, SRS-05:start:end -->
- [x] [SRS-05/AC-03] Analysis requires at least one board-type verification goal as baseline <!-- verify: test, SRS-05:start:end -->

### Agents Template Mission Workflow
- **ID:** 1vzeUg000
- **Status:** done

#### Summary
Update AGENTS.md template to document mission workflow for harnesses.

#### Acceptance Criteria
- [x] [SRS-06/AC-01] AGENTS.md template includes Mission workflow section documenting new/refine/activate loop <!-- verify: manual, SRS-06:start:end -->
- [x] [SRS-06/AC-02] Autonomous Delivery Policy updated to reference mission entity as authoritative objective <!-- verify: manual, SRS-06:start:end -->

### Mission Aware Keel Next
- **ID:** 1vzeVn000
- **Status:** done

#### Summary
Make `keel next --agent` mission-aware so it recommends work creation when queue empty but mission incomplete.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] `keel next --agent` returns mission recommendation when no stories ready but active mission has unmet goals <!-- verify: test, SRS-01:start:end -->
- [x] [SRS-02/AC-01] Recommendation includes mission context: unmet goal summary and suggested action type <!-- verify: test, SRS-02:start:end -->

### Mission Progress In Keel Flow
- **ID:** 1vzeVr000
- **Status:** done

#### Summary
Add mission-level progress section to `keel flow` output.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] `keel flow` includes mission progress section when active missions exist <!-- verify: test, SRS-03:start:end -->
- [x] [SRS-04/AC-01] `keel flow` mission section is omitted when no missions exist (backward compatible) <!-- verify: test, SRS-04:start:end -->


