# Horizon and Roadmap View - Charter

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Add an explicit horizon or roadmap mode that makes future work legible through priority, dependencies, and proceed-vs-park posture instead of requiring users to infer roadmap state from raw bearing lists. | board: VDiHw85WK |

## Constraints

- Reuse canonical bearing and dependency data in v1 rather than introducing a second backlog model in parallel.
- Keep the roadmap readable in both interactive and static CLI contexts.
- Surface proceed, park, and blocked posture explicitly instead of burying them in scores alone.
- Preserve deterministic output so harnesses can snapshot and diff the roadmap surface.

## Halting Rules

- DO NOT halt while understanding horizon work still requires hopping across multiple commands or reading raw bearing bundles by hand.
- HALT when Keel exposes a canonical roadmap view with priority, dependencies, and proceed-vs-park posture drawn from board data.
- YIELD to human if a dedicated horizon entity type is required beyond a bearing-backed roadmap view.
