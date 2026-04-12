# Ship Hub Session CLI And Remote Backend Config - SRS

## Summary

Epic: VGbHeuTMW
Goal: Add the first multiplayer Keeper slice in Keel: a server backend configuration contract plus Hub-backed login/logout/info commands that persist an authenticated session for future remote API calls.

## Scope

### In Scope

- [SCOPE-01] Add a `keel auth` command family with `login`, `logout`, and `info` actions backed by Hub-issued sessions.
- [SCOPE-02] Persist a reusable local auth session record and reuse it when resolving authenticated execution context.
- [SCOPE-03] Extend the storage configuration contract to distinguish filesystem and server backends and carry Keeper plus Hub endpoint coordinates.
- [SCOPE-04] Surface the effective auth and storage contract in config and operator-facing docs.

### Out of Scope

- [SCOPE-90] Proxying all existing board reads and writes through Keeper API routes.
- [SCOPE-91] Browser callback or device-code OAuth flows for third-party identity providers.
- [SCOPE-92] Any migration that removes or weakens the local filesystem backend.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The voyage SHALL add a `keel auth login` flow that exchanges Hub credentials for a Hub-issued bearer session and persists the resulting local session record for future CLI use. | SCOPE-01, SCOPE-02 | FR-01 | automated |
| SRS-02 | The voyage SHALL add `keel auth info` and `keel auth logout` flows that inspect the current Hub-backed session and revoke it through the Hub session API. | SCOPE-01, SCOPE-02 | FR-02 | automated |
| SRS-03 | The voyage SHALL extend the config contract so the storage backend can be either `filesystem` or `server`, with explicit Keeper and Hub connection fields for the server path. | SCOPE-03 | FR-03 | automated |
| SRS-04 | The voyage SHALL surface the effective auth and storage configuration through `keel config show` and the authored docs. | SCOPE-03, SCOPE-04 | FR-04 | automated |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | The new auth and storage contract SHALL preserve the default local-filesystem workflow so Keel still works without a Hub account or Keeper endpoint. | SCOPE-03 | NFR-01 | automated |
| SRS-NFR-02 | Human-facing auth output SHALL avoid printing bearer tokens or other secret session material in normal output. | SCOPE-01, SCOPE-02 | NFR-02 | automated |
| SRS-NFR-03 | The persisted auth session format SHALL remain provider-neutral enough to support future non-credential Hub sign-in flows without redoing the CLI contract. | SCOPE-01, SCOPE-02 | NFR-03 | automated |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
