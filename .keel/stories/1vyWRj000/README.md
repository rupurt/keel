---
id: 1vyWRj000
title: Build Tape Runner And Reset Harness
type: feat
status: done
created_at: 2026-03-06T06:46:31
updated_at: 2026-03-06T07:46:46
scope: 1vyWLl000/1vyWNL000
index: 2
started_at: 2026-03-06T07:32:30
completed_at: 2026-03-06T07:46:46
---

# Build Tape Runner And Reset Harness

## Summary

Build the canonical local entrypoint that resets the secondary workspace, runs named dogfood scenarios, and reports failures without making the suite part of default CI or pre-commit paths.

## Acceptance Criteria

- [x] [SRS-02/AC-01] A local opt-in runner executes named dogfood scenarios from the secondary workspace and reports actionable failure context. <!-- verify: cargo test -p keel dogfood_runner_executes_named_scenarios, SRS-02:start:end, proof: ac-1.log-->
- [x] [SRS-NFR-03/AC-01] The runner remains absent from default CI and pre-commit workflows. <!-- verify: cargo test -p keel dogfood_runner_is_opt_in_and_not_wired_into_default_checks, SRS-NFR-03:start:end, proof: ac-2.log-->
