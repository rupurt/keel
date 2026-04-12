---
# system-managed
id: VGYPhvZ7N
status: backlog
created_at: 2026-04-11T23:05:50
updated_at: 2026-04-11T23:10:26
# authored
title: Specify Reactor Awareness And Trusted Consumer Scheduling
type: feat
operator-signal:
scope: VGYPeZj64/VGYPh3luG
index: 1
---

# Specify Reactor Awareness And Trusted Consumer Scheduling

## Summary

Define the first contract that turns normalized external ingress into
reactor-visible Keel demand and reserves scheduling plus mission-request
application for trusted consumers rather than direct janitor or connector lane
pulls.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] The story defines a staged ingress record that carries the normalized request or work envelope, replay identity, trust context, and scheduling state before any planning mutation. <!-- verify: manual, SRS-01:start:end -->
- [ ] [SRS-02/AC-01] The story defines how Keel-native reactors or read models become aware of staged ingress through communication or application-reactor mechanisms rather than connector-owned direct mutation. <!-- verify: manual, SRS-02:start:end -->
- [ ] [SRS-03/AC-01] The story defines the trusted-consumer boundary for scheduling and mission-request `apply`, including which actors may observe, schedule, acknowledge, or escalate. <!-- verify: manual, SRS-03:start:end -->
- [ ] [SRS-04/AC-01] The story names the first `keel` and `spoke` surfaces required to land staged ingress, reactor awareness, and trusted-consumer scheduling. <!-- verify: manual, SRS-04:start:end -->
- [ ] [SRS-NFR-01/AC-01] The story preserves deterministic replay and deduplication from provider revision through scheduling, `apply`, and acknowledgement. <!-- verify: manual, SRS-NFR-01:start:end -->
- [ ] [SRS-NFR-02/AC-01] The story keeps conversational comms distinct from structured planning ingress and does not overload free-form `ping` and `poke`. <!-- verify: manual, SRS-NFR-02:start:end -->
- [ ] [SRS-NFR-03/AC-01] The story keeps GitHub-specific parsing and acknowledgement transport outside the provider-neutral trusted-consumer core. <!-- verify: manual, SRS-NFR-03:start:end -->
