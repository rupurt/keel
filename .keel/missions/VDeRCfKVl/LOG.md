# Simulation Kernel and Reactive Architecture - Decision Log

<!-- Append entries below. Each entry is an H2 with ISO timestamp. -->
<!-- Use `keel mission digest` to compress older entries when this file grows large. -->

## 2026-03-12T04:26:55

Created mission VDeRCfKVl for the simulation-kernel architecture extension. Linked bearing VDeRKA7fo for architecture research and epic VDeRV9CAo for the first implementation line. Charter now constrains the work to an extension of DDD and hexagonal architecture rather than a replacement.

## 2026-03-12T04:33:12

Advanced bearing VDeRKA7fo from exploring to ready. Research and assessment confirm that Keel should extend DDD and hexagonal architecture with a small internal simulation kernel rather than replace the architecture. Recommended first refactor slices: explicit reactor extraction from process_manager, shared reference-time simulation context for temporal evaluation, and a shared projection pipeline for flow/next steering. Epic VDeRV9CAo remains the implementation line; do not lay a duplicate epic from the bearing.

## 2026-03-12T04:40:25

Planned voyage VDeUIiB3Q Explicit Lifecycle Reactors under epic VDeRV9CAo. Narrowed the first implementation slice to the reactor seam only: explicit reactor contracts, story lifecycle reactors, and voyage-completion event wiring. Deferred shared simulation-context and projection unification to later voyages. Created and thawed stories VDeUNOfrU, VDeUNP4rV, and VDeUNRFtq for operator execution.

## 2026-03-12T04:46:42

Story VDeUNOfrU sealed: explicit reactor registry and deterministic planning order landed in process_manager; proofs captured with targeted reactor and architecture tests; next slice is VDeUNP4rV for story-accepted automation.

## 2026-03-12T04:51:01

Story VDeUNP4rV sealed: process_manager lifecycle automation is now split into explicit story-started, story-accepted, and voyage-completed reactors; the malformed AC proof command was corrected and evidence captured; next slice is VDeUNRFtq to emit real voyage.completed events.
