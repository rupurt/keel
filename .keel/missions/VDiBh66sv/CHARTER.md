# Bearing Research Contract Alignment - Charter

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Align research templates, parsing, `bearing show`, and doctor so authored sections like `## Feasibility` are interpreted consistently and section mismatches fail with exact, actionable guidance. | board: VDiHwCUZt |

## Constraints

- Keep one canonical section contract for bearing research artifacts instead of allowing template/parser drift.
- Prefer precise mismatch reporting over fuzzy acceptance that hides authored drift.
- If normalization is allowed, make it explicit and deterministic rather than heuristic.
- Preserve the hard-cutover policy for stale scaffold text and legacy section ambiguity.

## Halting Rules

- DO NOT halt while `bearing show`, doctor, and authored templates can disagree about whether research content is complete.
- HALT when the research template contract and diagnostics agree exactly, and failures point to the specific missing or mismatched section names.
- YIELD to human if existing bearings require a one-time migration policy for renamed legacy sections.
