# Core Changes - Software Design Description

> GOAL-02, GOAL-01

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage completes the hard cutover from actor booleans to role taxonomies for queue pull, queue labelling, and manual-accept authorization while preserving the existing two-lane workflow.

## Architecture

- `src/domain/model/taxonomy.rs` owns parsing and capability matching for `role/specialization:tags` inputs.
- `src/cli/command_tree.rs` and `src/cli/runtime.rs` accept `--role`, reject legacy queue booleans, and forward parsed taxonomies into command handlers.
- `src/cli/commands/management/next.rs` plus `src/cli/commands/management/next_support/algorithm.rs` determine queue lane and story eligibility from the parsed role.
- `src/cli/presentation/flow/display.rs`, help text, and drift tests rename Human/Agent queue labels to Management/Execution consistently.
- `src/application/story_lifecycle.rs` and `src/cli/commands/management/story/accept.rs` replace the old human override with manager-role authorization for manual acceptance.

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Queue mapping | Hardcode `manager` -> Management, `engineer` -> Execution for now | Simplest path to role-based routing before we introduce fully dynamic capability mapping. |
| CLI flags | `--role <TAXONOMY>` replaces `--human` and `--agent` | Forces explicit capability declaration. |
| Legacy flags | Reject with explicit migration guidance instead of runtime aliases | Matches hard cutover policy and keeps failure modes deterministic. |
| Manual acceptance | Only `manager/*` roles may accept stories requiring manual verification | Keeps subjective approval in the management lane. |
