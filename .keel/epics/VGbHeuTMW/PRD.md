# Hub Session Auth And Server Backend Wiring - Product Requirements

## Problem Statement

Keel only works against the local file backend today. Multiplayer Keeper requires a configurable remote backend contract plus first-class Hub-backed authentication so Keel can operate against Keeper APIs using user accounts instead of local-only OS identity.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Let a Keel operator authenticate with a Spoke Hub user account from the CLI and persist a reusable session for future Keeper-backed command transport. | `keel auth login`, `keel auth info`, and `keel auth logout` work against Hub-backed sessions and are covered by automated tests. | First multiplayer slice shipped |
| GOAL-02 | Define an explicit remote backend configuration contract that points Keel at Keeper while preserving the local filesystem backend as the default. | `keel config show` and `keel.toml` expose a stable server-backend shape with Keeper and Hub coordinates. | Config contract shipped |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Keeper Operator | A human or agent running Keel against a shared Keeper deployment instead of only a local `.keel` directory. | Authenticate once with a Hub account and point Keel at a Keeper API without losing the local-only workflow. |
| Local Repo Operator | A human or agent continuing to use Keel entirely against local files. | Keep the local filesystem backend working unchanged while multiplayer capabilities are added. |

## Scope

### In Scope

- [SCOPE-01] Add a first-class `keel auth` command family for login, logout, and session inspection using Hub-issued sessions.
- [SCOPE-02] Persist authenticated session state locally and reuse it when resolving the execution actor for later commands.
- [SCOPE-03] Extend the config contract so `storage.backend = "server"` can carry Keeper and Hub connection details alongside the existing filesystem default.
- [SCOPE-04] Surface the new auth and storage settings in docs and `keel config show`.

### Out of Scope

- [SCOPE-90] Routing every existing Keel command through Keeper HTTP instead of the local board loader.
- [SCOPE-91] Implementing browser callback or device-code OAuth flows for third-party identity providers.
- [SCOPE-92] Replacing or deprecating the local filesystem backend.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Keel SHALL expose `auth login`, `auth logout`, and `auth info` commands that work with Hub-backed user sessions. | GOAL-01 | must | Operators need an explicit CLI entrypoint for account-backed multiplayer access. |
| FR-02 | Keel SHALL persist authenticated session state locally and reuse it when resolving the execution actor for later commands. | GOAL-01 | must | Future Keeper API calls need a durable authenticated identity rather than local-only OS user fallback. |
| FR-03 | Keel SHALL expose an explicit server-backend configuration contract for Keeper and Hub endpoints while keeping filesystem as the default backend. | GOAL-02 | must | Remote transport must be configurable without breaking local use. |
| FR-04 | Keel SHALL render the effective auth and storage configuration in operator-facing documentation surfaces. | GOAL-01, GOAL-02 | should | Operators need to inspect and debug the active multiplayer wiring. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | The new auth and storage paths SHALL preserve the current local-filesystem workflow with no required account or Keeper dependency. | GOAL-02 | must | Multiplayer support cannot regress single-repo operation. |
| NFR-02 | Human-facing auth commands SHALL avoid printing bearer tokens or other secret session material in normal output. | GOAL-01 | must | Session auth introduces secret-handling risk. |
| NFR-03 | The persisted auth/session contract SHALL stay compatible with future non-credential Hub sign-in flows. | GOAL-01 | should | The first slice should not hard-code credential login as the only forever path. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Hub session auth | CLI tests plus manual command proofs | Story-level tests and recorded auth outputs |
| Storage contract | Config serialization and render tests | `keel config show` output plus config tests |
| Regression safety | Existing repository quality gate | fmt, clippy, nextest, and `keel health` |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Hub’s current credential-backed session API is acceptable as the first CLI login transport until browser-friendly OAuth or device-code flows are added. | The CLI auth flow might need a different handshake before Keeper rollout. | Keep the stored session contract provider-neutral and revisit when Hub exposes a CLI-native OAuth path. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Which Keeper routes will become the first read/write remote board adapter after auth and config land? | Epic owner | Open |
| Should Keel store auth sessions in the XDG config directory by default or a stronger OS credential store later? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] A Keel operator can log into Hub, inspect session identity, and log out from the CLI.
- [ ] `keel.toml` can express both filesystem and server backends without ambiguity.
- [ ] The local-filesystem default path remains unchanged and covered by tests.
<!-- END SUCCESS_CRITERIA -->
