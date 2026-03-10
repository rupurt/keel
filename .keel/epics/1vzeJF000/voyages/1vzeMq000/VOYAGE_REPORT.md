# VOYAGE REPORT: CLI Commands

## Voyage Metadata
- **ID:** 1vzeMq000
- **Epic:** 1vzeJF000
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 5/5 stories complete

## Implementation Narrative
### Mission Transition Commands
- **ID:** 1vzeUT000
- **Status:** done

#### Summary
Implement mission lifecycle transition commands: pause, achieve, verify, abandon.

#### Acceptance Criteria
- [x] [SRS-07/AC-01] `keel mission pause <id>` transitions Active → Paused <!-- verify: test, SRS-07:start:end -->
- [x] [SRS-08/AC-01] `keel mission achieve <id>` transitions Active → Achieved when all board-verifiable goals are met <!-- verify: test, SRS-08:start:end -->
- [x] [SRS-08/AC-02] Achievement is rejected when any board-verifiable goal is unmet, with diagnostic output <!-- verify: test, SRS-08:start:end -->
- [x] [SRS-09/AC-01] `keel mission verify <id>` transitions Achieved → Verified (terminal) <!-- verify: test, SRS-09:start:end -->
- [x] [SRS-10/AC-01] `keel mission abandon <id>` transitions Active or Paused → Abandoned (terminal) <!-- verify: test, SRS-10:start:end -->

### Mission Log And Digest
- **ID:** 1vzeUU000
- **Status:** done

#### Summary
Implement LOG.md append and digest commands.

#### Acceptance Criteria
- [x] [SRS-11/AC-01] `keel mission log <id> --entry "<text>"` appends timestamped entry to LOG.md <!-- verify: test, SRS-11:start:end -->
- [x] [SRS-12/AC-01] `keel mission digest <id>` compresses older entries into summary block at top of LOG.md <!-- verify: test, SRS-12:start:end -->

### Mission New Command
- **ID:** 1vzeVP000
- **Status:** done

#### Summary
Implement `keel mission new` command that creates .keel/missions/<id>/ directory with README.md, CHARTER.md scaffold, and LOG.md scaffold.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] `keel mission new "<title>"` creates mission directory under .keel/missions/ <!-- verify: test, SRS-01:start:end -->
- [x] [SRS-01/AC-02] Created README.md has frontmatter with id, title, status=defining, created_at <!-- verify: test, SRS-01:start:end -->
- [x] [SRS-01/AC-03] Created CHARTER.md has Goals table, Constraints, and Halting Rules scaffold sections <!-- verify: test, SRS-01:start:end -->
- [x] [SRS-01/AC-04] Created LOG.md has initial scaffold with header <!-- verify: test, SRS-01:start:end -->

### Mission Show And List Commands
- **ID:** 1vzeVT000
- **Status:** done

#### Summary
Implement `keel mission show` and `keel mission list` commands for mission visibility.

#### Acceptance Criteria
- [x] [SRS-05/AC-01] `keel mission show <id>` displays title, status, goals, child entities, and LOG summary <!-- verify: test, SRS-05:start:end -->
- [x] [SRS-06/AC-01] `keel mission list` displays all missions with id, title, status, and child count <!-- verify: test, SRS-06:start:end -->
- [x] [SRS-05/AC-02] Show command supports --json output <!-- verify: test, SRS-05:start:end -->

### Mission Refine And Activate
- **ID:** 1vzeVa000
- **Status:** done

#### Summary
Implement `keel mission refine` for iterative CHARTER.md goal elicitation and `keel mission activate` to transition from Defining to Active.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] `keel mission refine <id>` returns next question when CHARTER.md is incomplete <!-- verify: test, SRS-02:start:end -->
- [x] [SRS-02/AC-02] `keel mission refine <id>` returns "ready" signal when CHARTER.md is complete <!-- verify: test, SRS-02:start:end -->
- [x] [SRS-03/AC-01] `keel mission refine <id> --answer "<text>"` records answer into CHARTER.md and returns next question or ready <!-- verify: test, SRS-03:start:end -->
- [x] [SRS-04/AC-01] `keel mission activate <id>` transitions Defining → Active, gated on CHARTER Goals having at least one authored MG-XX row <!-- verify: test, SRS-04:start:end -->


