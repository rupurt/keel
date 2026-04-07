# Mission Request Command Surface Research — Brief

## Hypothesis

Keel should expose a native `mission request` command family so Keeper and other
programs can compose mission-request parsing, validation, drafting, application,
and acknowledgement without embedding provider-specific logic in Keel core.

## Problem Space

The current Keeper and keeper-cli surfaces are too thin to ingest formal mission
requests from external providers. Without a native Keel CLI surface, each
provider worker would need to reimplement normalization, validation, and
application behavior, which would fracture the planning contract and weaken
replayability.

## Success Criteria

- [ ] Define the canonical `keel mission request template|parse|validate|draft|apply|ack` command family.
- [ ] Define a provider-neutral request envelope that can be piped over stdin/stdout.
- [ ] Define the minimum required inputs for GitHub issue activation and later provider expansion.
- [ ] Define how Keeper and non-Keeper automation invoke the same Keel surface.

## Open Questions

- How should `keel mission request apply` behave when a request is exploratory rather than implementation-ready?
- Which fields should be required on stdin versus derivable from provider metadata?
- Should `ack` emit only provider-facing content or also a canonical audit record payload?
