# Refactor Speccy Crate Surface And Module Layout - Decision Log

<!-- Append entries below. Each entry is an H2 with ISO timestamp. -->
<!-- Use `keel mission digest` to compress older entries when this file grows large. -->

## 2026-03-26T13:49:13

Completed the two-pass speccy refactor: split the crate into focused modules, reduced the public rendering API to core entrypoints plus RenderOptions, cut Keel over to the smaller surface, and documented stable extension points versus host-owned responsibilities. Verified with fmt, clippy, and cargo nextest run.

## 2026-03-26T13:49:13

Mission achieved by local system user 'alex'
