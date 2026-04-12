---
# system-managed
id: VGbHgz6nT
status: done
created_at: 2026-04-12T10:52:49
updated_at: 2026-04-12T11:14:06
# authored
title: Add Hub Login Logout Info And Server Backend Config
type: feat
operator-signal:
scope: VGbHeuTMW/VGbHgxwnS
index: 1
started_at: 2026-04-12T10:58:29
completed_at: 2026-04-12T11:14:06
---

# Add Hub Login Logout Info And Server Backend Config

## Summary

Add the first usable multiplayer-auth slice in Keel: a Hub-backed `keel auth`
surface plus explicit server-backend configuration that can carry Keeper and
Hub coordinates while preserving the existing filesystem default.

## Acceptance Criteria

- [x] [SRS-01/AC-01] `keel auth login` exchanges Hub credentials for a Hub-issued session and persists a reusable local session record. <!-- verify: cargo test -p keel login_persists_session_and_redacts_human_output, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] `keel auth info` and `keel auth logout` inspect and revoke the current Hub-backed session without printing bearer tokens in normal output. <!-- verify: cargo test -p keel cli::commands::setup::auth::tests::, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-03/AC-01] `keel.toml` supports `storage.backend = "server"` with explicit Keeper and Hub endpoint fields while keeping filesystem as the default backend. <!-- verify: cargo test -p keel-core load_from_file_parses_storage_section, SRS-03:start:end, proof: ac-3.log-->
- [x] [SRS-04/AC-01] `keel config show` and the authored docs render the effective auth and storage contract for operators. <!-- verify: cargo test -p keel config_show_renders_server_backend_and_configured_auth_path, SRS-04:start:end, proof: ac-4.log-->
- [x] [SRS-NFR-01/AC-01] The local-filesystem workflow still works without a Hub account or Keeper endpoint. <!-- verify: cargo test -p spoke-auth load_auth_context_returns_local_system_when_no_session_exists, SRS-NFR-01:start:end, proof: ac-5.log-->
- [x] [SRS-NFR-02/AC-01] Human-facing auth output redacts secret session material. <!-- verify: cargo test -p keel info_verifies_saved_session_without_printing_token, SRS-NFR-02:start:end, proof: ac-6.log-->
- [x] [SRS-NFR-03/AC-01] The persisted auth session format stays provider-neutral enough for future non-credential Hub sign-in flows. <!-- verify: cargo test -p spoke-auth session_record_round_trips_without_losing_provider_neutral_shape, SRS-NFR-03:start:end, proof: ac-7.log-->
