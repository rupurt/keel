---
created_at: 2026-03-06T08:01:59
---

# Reflection - Author Epic Workflow Dogfood Tapes

## Knowledge

- [1vyXcz000](../../knowledge/1vyXcz000.md) Use hidden setup blocks and dynamic ID discovery in VHS planning flows

## Observations

The first tape draft was structurally correct but not valid VHS because escaped double quotes broke `Type` parsing. Rewriting the shell commands to use single-quoted CLI arguments and hidden setup blocks made the tape both valid and easier to read.

The other important friction point was proof recording. `just keel` expands arguments in a way that is fine for normal commands but not for `story record --cmd ...`, so evidence capture needed the direct `cargo run -- story record ...` path to preserve compound shell commands.
