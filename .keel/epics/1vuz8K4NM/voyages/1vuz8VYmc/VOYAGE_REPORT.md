# VOYAGE REPORT: Policy Module and Queue Semantics

## Voyage Metadata
- **ID:** 1vuz8VYmc
- **Epic:** 1vuz8K4NM
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 5/5 stories complete

## Implementation Narrative
### Define Queue Policy Module and Threshold Constants
- **ID:** 1vuz963fU
- **Status:** done

#### Summary
Create the canonical queue policy module that defines threshold constants, queue categories, and derivation helpers used by both `keel next` and `keel flow`.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Add a queue policy module that defines canonical thresholds and decision categories for next and flow. <!-- verify: manual, SRS-01:start -->
- [x] [SRS-01/AC-02] Replace duplicated threshold constants in next and flow call sites with policy exports. <!-- verify: manual, SRS-01:continues -->
- [x] [SRS-01/AC-03] Add unit tests that lock policy defaults and helper behavior. <!-- verify: manual, SRS-01:end -->

#### Verified Evidence
- [ac-01-03.txt](../../../../stories/1vuz963fU/EVIDENCE/ac-01-03.txt)

### Update Architecture and Command Docs for Queue Policy
- **ID:** 1vuz97CCg
- **Status:** done

#### Summary
Update architecture and command documentation so queue policy thresholds, derivation order, and human/agent boundaries are explicitly consistent with implemented behavior.

#### Acceptance Criteria
- [x] [SRS-05/AC-01] Update `ARCHITECTURE.md` sections for 2-queue flow and system-state derivation to match canonical policy values. <!-- verify: manual, SRS-05:start -->
- [x] [SRS-05/AC-02] Update command/help documentation to reflect human-mode queue boundaries and policy-driven decision behavior. <!-- verify: manual, SRS-05:continues -->
- [x] [SRS-05/AC-03] Add or update documentation consistency checks/tests that prevent threshold and terminology drift. <!-- verify: manual, SRS-05:end -->

### Align Flow Derivation Bottleneck Messaging and Tests
- **ID:** 1vuz97Joy
- **Status:** done

#### Summary
Align flow-state derivation and bottleneck messaging with the same queue policy used by `next` so thresholds and state labels do not drift.

#### Acceptance Criteria
- [x] [SRS-04/AC-01] Update flow derivation and bottleneck threshold checks to use shared policy constants. <!-- verify: manual, SRS-04:start -->
- [x] [SRS-04/AC-02] Ensure flow assessment messaging reflects the same classification boundaries used by `next`. <!-- verify: manual, SRS-04:continues -->
- [x] [SRS-04/AC-03] Add cross-module tests for boundary conditions that assert consistent `next` and `flow` classification results. <!-- verify: manual, SRS-04:end -->

### Enforce Human Next Queue Boundary
- **ID:** 1vuz97Ynz
- **Status:** done

#### Summary
Enforce the actor boundary that human-mode `keel next` never returns implementation work and only surfaces human-queue actions.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] Update human-mode selection logic so `calculate_next(..., agent_mode=false, ...)` cannot emit `NextDecision::Work`. <!-- verify: manual, SRS-03:start -->
- [x] [SRS-03/AC-02] Ensure human-mode outcomes are restricted to human queue decision kinds only. <!-- verify: manual, SRS-03:continues -->
- [x] [SRS-03/AC-03] Add tests covering mixed queue states where human mode previously surfaced implementation work. <!-- verify: manual, SRS-03:end -->

### Refactor Next and Flow to Use Queue Policy
- **ID:** 1vuz97nO2
- **Status:** done

#### Summary
Refactor queue-selection and flow-health paths to consume the shared policy APIs instead of local literals and ad-hoc ordering logic.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] Refactor `src/next/algorithm.rs` to use policy helpers for blocked, accept, research, planning, and work ordering decisions. <!-- verify: manual, SRS-02:start -->
- [x] [SRS-02/AC-02] Refactor relevant `src/flow/*` decision points to consume policy constants instead of inline literals. <!-- verify: manual, SRS-02:continues -->
- [x] [SRS-02/AC-03] Add regression tests proving policy-driven behavior and removal of inline threshold usage in decision paths. <!-- verify: manual, SRS-02:end -->


