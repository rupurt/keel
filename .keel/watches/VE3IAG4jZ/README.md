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

## Canonical Routine Anchors

Recurring operational topics now live in the surviving watch-scoped stories:

- `VEvT3qXAi` Explore Speculative Decoding for Transit Messages
- `VEvd69Hyf` Eliminate Report Tail Friction
- `VEvd69Hyg` Ask About Keel System Audio Feedback Support For Transitions
- `VEvT3qXAl` Daily Status Surface Progress
- `VEvd69Iyh` Bridge Engine And VCS Via Auto Staging
