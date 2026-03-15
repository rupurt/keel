# Formalize VSDD and Harden Verification Infrastructure - Charter

Archetype: Strategic
## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Formalize Verified Spec Driven Development (VSDD) in documentation. | manual |
| MG-02 | Implement structured verification commands (argv + cwd). | board: VDZKYMeNQ |
| MG-03 | Implement failure diagnostics and story audit. | board: VDZKYQ9RC |
| MG-04 | Add first-class operator_signal fields. | board: VDZKYTeQK |
| MG-05 | Automate lifecycle transitions for voyages and epics. | board: VDZKYX7TX |

## Constraints

- Ensure backward compatibility for existing raw-string verification markers.
- Maintain strict Hexagonal layer boundaries.

## Halting Rules

- DO NOT halt while any MG-* goal has unfinished board work
- HALT when all MG-* goals with `board:` verification are satisfied
- YIELD to human when only `metric:` or `manual:` goals remain
