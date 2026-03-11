# Temporal Pull and Business Process Automation - Charter

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Define the `Routine` entity for recurring work blueprints. | board: VDakm8eVW |
| MG-02 | Implement temporal gating and countdowns in `keel next`. | board: VDakmCGYi |
| MG-03 | Implement `keel pulse` for non-interactive automation triggers. | board: VDakmG8cH |
| MG-04 | Add the `scheduled` lane to `keel flow`. | board: VDakmG8cH |
| MG-05 | Author `GUIDE.md` for Business Process Automation. | board: VDakmJodq |

## Constraints

- Routines must maintain strict SRS traceability.
- `keel pulse` must be idempotent and safe for frequent cron execution.
- Maintain backward compatibility with existing story models.

## Halting Rules
- DO NOT halt while any of MG-01 through MG-05 lacks either a planned voyage or a submitted delivery story under its scoped epic.
- YIELD to human before introducing long-running background services, external schedulers, or persisted schema changes that require migration work.
- HALT when the Routine model, temporal gating, pulse entry point, scheduled flow lane, and automation guide each have board-backed completion evidence and no open migration story remains.
