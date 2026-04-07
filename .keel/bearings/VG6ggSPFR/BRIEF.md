# Keeper Provider Mission Request Ingress Research — Brief

## Hypothesis

Keeper should own provider polling, activation detection, normalization, and
acknowledgement for formal mission requests while delegating request semantics
and planning mutation to native Keel commands.

## Problem Space

The Keeper architecture already defines reactor inboxes, connector ingress, and
provider-facing routing, but it does not yet define a formal mission-request
intake flow. Without that split, provider-specific parsing will leak into Keel
or Keeper workers will drift on how requests are normalized and replayed.

## Success Criteria

- [ ] Define the GitHub issue activation rule and its normalization path into a canonical mission request envelope.
- [ ] Define the boundary between Keeper provider polling and Keel mission-request commands.
- [ ] Define how provider revisions, retries, and acknowledgements remain replayable.
- [ ] Define the first ingress worker responsibilities for GitHub issues.

## Open Questions

- Should GitHub issue edits create new normalized revisions or supersede prior drafts?
- What evidence should Keeper persist locally versus refer to by provider reference?
- Which acknowledgements belong in provider comments versus reactor-private audit streams?
