# Trusted Consumer Scheduling For External Ingress - Charter

Archetype: Bridging

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Define the board-ready contract that turns normalized external ingress into authored Keel demand and limits scheduling or mission-request application to trusted consumers. | board: VGYPeZj64 |

## Constraints

- Keep `ping` and `poke` conversational; structured planning ingress must use a typed, replayable contract instead of free-form chat messages.
- Reuse the provider-neutral mission request envelope and acknowledgement split already captured by the earlier mission-request planning slices.
- Let Keel-native reactors learn about new work through Keel communication or application-reactor surfaces, not through provider-owned queue state or direct connector mutation.
- Preserve explicit provenance, replay identity, and trust boundaries for any consumer allowed to schedule or apply board mutations.

## Halting Rules

- DO NOT halt while any MG-* goal has unfinished board work
- HALT when all MG-* goals with `board:` verification are satisfied
- YIELD to human when choosing between multiple equally valid trust or scheduling policies would change operator-facing behavior
