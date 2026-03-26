---
id: VE3IAG4jZ
title: Standard Operations
limit_hours: 12
---

# Standard Operations

This is a **Watch** constraint. It limits the duration of missions that reference it.

## Consolidated Legacy Coverage

`Standard Operations` is now the canonical home for recurring operational pressure that was previously modeled through the retired project-operations epics `VDseuzIFg` and `VE3KrOPS`.

- Watch platform delivery: `VE3IkbIgn` implemented watch loading and board integration; `VE3IkgNlQ` delivered the `keel watch` CLI surface and watch rendering.
- Pacemaker protocol: `VE3IklYoe` codified mandatory heartbeat updates in `INSTRUCTIONS.md`.
- Operational guardrails: `VDyhMtYBO` added backlog overload limits and `VDzfi1DvX` added human notification hooks for charged capacity.
- Routine reliability: `VE3z3roGf`, `VE3z4kLeK`, and `VE3z5SB01` added terminal-scope rejection, doctor scope coherence checks, and per-routine pulse outcome reporting.

## Active Routine Anchor

Only one live watch-scoped routine topic remains after auditing the routine backlog for duplicate shipped work:

- `VEvT3qXAi` Explore Speculative Decoding for Transit Messages

This remaining topic is open research pressure rather than completed operational delivery. If it graduates from recurring review into concrete work, it should likely move into a bearing-led research track instead of staying on the watch indefinitely.

## Retired Routine Anchors

The following watch-scoped routine anchors were retired on `2026-03-26` after confirming that their blueprint intent had already shipped under earlier scopes. Their current watch-scoped materializations were iced and the stale routines were removed so they can no longer create false strategic pressure.

- `VEvd69Hyf` Eliminate Report Tail Friction — covered by `VE56ttUZW` and voyage `VE5NSQ2T2`
- `VEvd69Hyg` Ask About Keel System Audio Feedback Support For Transitions — covered by `VE56ttUZX` and voyage `VE5NSQ2T2`
- `VEvT3qXAl` Daily Status Surface Progress — covered by `VDtx8IX2L` and epic `VDm4ld6EX`
- `VEvd69Iyh` Bridge Engine And VCS Via Auto Staging — covered by `VE56ttVZY` and voyage `VE5NSQ2T2`
