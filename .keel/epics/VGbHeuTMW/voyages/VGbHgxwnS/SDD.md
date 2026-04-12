# Ship Hub Session CLI And Remote Backend Config - Software Design Description

> Add the first multiplayer Keeper slice in Keel: a server backend configuration contract plus Hub-backed login/logout/info commands that persist an authenticated session for future remote API calls.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage ships the first multiplayer Keeper boundary in Keel without
pretending the full remote board adapter already exists. The local filesystem
path stays canonical by default. Alongside it, Keel gains:

- a server-backend configuration contract with explicit Keeper and Hub
  coordinates;
- a persisted Hub session record stored on the operator machine;
- a `keel auth` command family that can log in, inspect the current session,
  and revoke it;
- automatic reuse of the persisted session when resolving the execution actor.

That gives later Keeper transport work a stable authenticated identity and
configuration contract to build on.

## Context & Boundaries

### In Scope

- session-oriented CLI auth against Spoke Hub;
- local persistence of the current Hub session;
- config fields for `filesystem` versus `server` storage;
- config rendering and docs for those new fields.

### Out of Scope

- replacing the existing board loader and transition engine with Keeper HTTP;
- inventing a browser callback or device-code flow that Hub does not yet expose
  for CLI use;
- removing local auth-less operation.

```text
operator
  |
  v
keel auth login/info/logout
  |
  v
Hub HTTP endpoints
  /auth/login
  /protected/me
  /sessions/{id}/revoke
  |
  v
local auth session file
  |
  v
spoke-auth execution context
  |
  v
future Keeper API client

keel.toml
  storage.backend = filesystem | server
  storage.server.keeper_base_url
  storage.server.hub_base_url
```

## Dependencies

<!-- External systems, libraries, services this design relies on -->

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `crates/hub/src/main.rs` in `spoke` | sibling repo code | Defines `/auth/login`, `/protected/me`, and `/sessions/{session_id}/revoke` response contracts | current workspace |
| `crates/keeper/src/lib.rs` in `spoke` | sibling repo code | Defines the early Keeper HTTP service boundary that future remote Keel transport will target | current workspace |
| `crates/spoke-auth/src/lib.rs` | local code | Owns execution-context loading and actor identity | current repo |
| `crates/keel-core/src/infrastructure/config.rs` | local code | Owns layered config parsing plus `storage.backend` | current repo |
| `crates/keel-cli/src/cli/runtime.rs` | local code | Central command dispatch and auth-file resolution | current repo |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| First login transport | Use Hub credential login (`POST /auth/login`) for the first CLI session path | Hub exposes it today and it produces the same bearer/session model that future OAuth-backed flows will use |
| Session persistence | Store a local structured session record, not just a bare JWT file path | Logout and info both need session metadata such as `session_id`, provider, and endpoint base URL |
| Runtime auth reuse | Load the default saved session automatically when `--auth-file` is omitted | Later Keeper-backed commands need auth to become ambient rather than a per-command manual flag |
| Remote backend contract | Add explicit `storage.server` coordinates instead of overloading the existing backend enum alone | Server mode needs Keeper and Hub URLs and should remain inspectable |
| Local path preserved | Keep filesystem as the default backend and do not switch command transport in this voyage | The remote adapter is a follow-on change with a much larger blast radius |

## Architecture

The design splits into four layers:

1. `spoke-auth` session model
   - parses either a saved structured session file or the legacy no-auth path;
   - returns an authenticated execution context when a saved session exists.
2. `keel auth` CLI surface
   - handles login, info, and logout;
   - talks to Hub over HTTP;
   - writes or removes the structured session file.
3. config model
   - exposes `filesystem` and `server` backends;
   - carries Keeper and Hub URLs for the server backend;
   - renders those values through `keel config show`.
4. future remote transport seam
   - not implemented yet in this voyage;
   - will read the same config/auth contracts when Keeper-backed command routing is added.

## Components

| Component | Purpose | Behavior |
|-----------|---------|----------|
| `StoredAuthSession` | Canonical local auth/session record | Stores base URLs, access token, session id, provider identity, and scopes without exposing them in normal human output |
| `spoke-auth::load_auth_context` | Resolve execution actor | Uses explicit `--auth-file` when given, otherwise the default saved session if present, else falls back to the local OS user |
| `keel auth login` | Create a Hub session | Sends credential login to Hub, fetches current identity if needed, and persists the session record |
| `keel auth info` | Inspect the current session | Reads the local session and optionally confirms identity against `/protected/me` |
| `keel auth logout` | Revoke and clear session state | Calls Hub revoke on the saved `session_id` and deletes the local record |
| `storage.server` config | Describe future Keeper transport | Holds Keeper and Hub base URLs so later remote calls know where to go |

## Interfaces

Hub HTTP contracts used in this slice:

- `POST /auth/login`
  - request: `email`, `password`, optional `state`, optional `application_slug`
  - response includes `access_token`, `session_id`, `provider`, `scopes`
- `GET /protected/me`
  - request: `Authorization: Bearer <access_token>`
  - response includes `account_id`, `provider`, `provider_subject`
- `POST /sessions/{session_id}/revoke`
  - request: bearer token plus JSON body with `reason`
  - response: `204 No Content`

Local session file contract:

- carries `hub_base_url`
- carries `access_token`
- carries `session_id`
- carries identity summary such as `account_id`, `provider`, `provider_subject`
- carries scopes and timestamp metadata

## Data Flow

1. Operator runs `keel auth login`.
2. CLI resolves the effective Hub URL from config or default.
3. CLI exchanges credentials with Hub `POST /auth/login`.
4. CLI stores the returned session data in the local session file and prints a
   redacted success summary.
5. A later Keel command starts.
6. `spoke-auth::load_auth_context` loads the saved session record when present
   and returns an authenticated execution context instead of local OS identity.
7. `keel auth info` reads the same record and can validate it with
   `/protected/me`.
8. `keel auth logout` revokes the session using `session_id` and removes the
   local file.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Hub login fails due to bad credentials | non-2xx from `/auth/login` | Return a clear auth error and do not persist anything | Retry with correct credentials |
| Saved session file is malformed | session parse error | Refuse authenticated mode and explain how to log in again or remove the file | Re-run `keel auth login` or `keel auth logout --local-only` if added later |
| Saved session has been revoked or expired | `/protected/me` or revoke returns unauthorized | Report session invalid and clear or replace it on next login | Run `keel auth login` again |
| Server backend selected without Keeper or Hub URL | config validation/render failure | Surface missing required fields in config and docs/tests | Fix `keel.toml` and rerun |
| Local user has no auth session | no session file found | Fall back to existing local system actor behavior | Keep using filesystem mode or log in later |
