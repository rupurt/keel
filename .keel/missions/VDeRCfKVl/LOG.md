# Simulation Kernel and Reactive Architecture - Decision Log

<!-- Append entries below. Each entry is an H2 with ISO timestamp. -->
<!-- Use `keel mission digest` to compress older entries when this file grows large. -->

## 2026-03-12T04:26:55

Created mission VDeRCfKVl for the simulation-kernel architecture extension. Linked bearing VDeRKA7fo for architecture research and epic VDeRV9CAo for the first implementation line. Charter now constrains the work to an extension of DDD and hexagonal architecture rather than a replacement.

## 2026-03-12T04:33:12

Advanced bearing VDeRKA7fo from exploring to ready. Research and assessment confirm that Keel should extend DDD and hexagonal architecture with a small internal simulation kernel rather than replace the architecture. Recommended first refactor slices: explicit reactor extraction from process_manager, shared reference-time simulation context for temporal evaluation, and a shared projection pipeline for flow/next steering. Epic VDeRV9CAo remains the implementation line; do not lay a duplicate epic from the bearing.
