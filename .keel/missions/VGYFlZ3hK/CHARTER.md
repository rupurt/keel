# Keeper Managed Janitor Transition - Charter

Archetype: Bridging

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Plan the first board-ready slice that lets Keeper assume janitor posture over a Keel board, carry janitor-authenticated stewardship through native Keel lifecycle surfaces, and route GitHub janitor work through the Spoke connector boundary. | board: VGYFmJEuH |

## Constraints

- Keep Keel authoritative for planning truth; Keeper may call Keel commands but must not mutate `.keel` state out-of-band.
- Treat `janitor` as Keeper posture, not as a replacement for Keel board-role routing.
- Use GitHub as the first janitor connector without baking GitHub-only semantics into the enduring custody model.
- Keep the first rollout scoped to janitor posture; driver/navigator expansion remains follow-on work.

## Halting Rules

- DO NOT halt while epic `VGYFmJEuH` lacks a planned voyage or executable story for the janitor custody boundary.
- YIELD to human before broadening the contract beyond janitor posture or beyond the first GitHub connector surface.
- HALT when the janitor transition is captured in a planned epic/voyage/story stack and only implementation prioritization remains.
