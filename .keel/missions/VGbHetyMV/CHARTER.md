# Remote Keeper Backend And Hub Auth For Keel - Charter

Archetype: Bridging

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Define and ship the first Keeper-backed multiplayer slice in Keel without breaking the local filesystem path: explicit server-backend configuration, persisted Hub session auth, and a CLI auth surface for login, logout, and session inspection. | board: VGbHeuTMW |

## Constraints

- Preserve the local filesystem backend as the default and fully supported path.
- Do not claim that all Keel commands are remote-capable until a Keeper-backed board adapter exists.
- Persist auth state locally without printing bearer tokens in normal human output.
- Keep the config and auth contracts provider-neutral enough to support future browser OAuth or non-credential Hub sign-in flows.

## Halting Rules

- DO NOT halt while epic `VGbHeuTMW` lacks a planned voyage and executable story for the first Hub-auth plus server-backend slice.
- YIELD to human before changing command transport from the local board loader to Keeper-backed HTTP for existing lifecycle commands.
- HALT when the first multiplayer auth/config slice is captured in executable board work and the remaining decisions are about follow-on remote command coverage.
