# Document Downstream Keel Adoption And Upgrade Workflow - Charter

Archetype: Strategic

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Add public docs that teach downstream repositories how to use Keel as their project-management engine through `AGENTS.md` and `INSTRUCTIONS.md`, and how to upgrade Keel while safely syncing upstream instruction changes. | board: VF2RJfiKo |

## Constraints

- Keep the docs OSS-facing and vendor-neutral.
- Document the real downstream adaptation pattern used by `port`.
- Treat upgrades and instruction sync as an operational workflow, not as a promise of automated migration tooling.

## Halting Rules

- DO NOT halt while the public docs still lack downstream guidance for adapting `AGENTS.md` and `INSTRUCTIONS.md` or lack an explicit upgrade-and-sync workflow.
- HALT when the downstream adoption docs, upgrade guidance, and linked board work are all landed and closed.
- YIELD to human if the requested guidance requires policy about commercial hosted offerings or downstream automation beyond the OSS documentation scope.
