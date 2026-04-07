# Keeper Provider Mission Request Ingress - Charter

Archetype: Strategic

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Define the Keeper-side provider ingress flow that detects formal mission requests, normalizes provider payloads, invokes Keel commands, and records replayable acknowledgement evidence. | board: VG6ggSPFR |

## Constraints

- Default the first provider flow to GitHub issues without baking GitHub-specific parsing into Keel core.
- Preserve deterministic replay inputs so polling, retries, and provider edits can be audited without hidden state.

## Halting Rules

- Halt after the provider ingress contract is captured with clear boundaries between Keeper polling and Keel request semantics.
- Yield to human review before expanding ingestion beyond GitHub issues or introducing connector-specific branching in the canonical request model.
