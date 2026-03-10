# Core Changes - Software Design Description

> GOAL-02, GOAL-01

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage replaces the hardcoded "human" and "agent" queue concepts with a role-based authorization system, mapping "management" to humans/advanced-agents and "execution" to standard agents.

## Architecture

We will port the `RoleTaxonomy` struct and parser from the `vibes` repository into `src/domain/model/taxonomy.rs`.

`keel next` will be updated to accept a `--role` string, parse it, and route the pull request to the appropriate internal queue based on the base role (e.g., `manager` -> management queue, `engineer` -> execution queue).

`keel story accept` will be updated to take a `--role` string instead of a `--human` boolean flag. It will parse the role and enforce that `manager/*` roles are required for stories with manual verification criteria.

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Queue mapping | Hardcode `manager` -> Management, `engineer` -> Execution for now | Simplest path to role-based routing before we introduce fully dynamic capability mapping. |
| CLI flags | `--role <TAXONOMY>` replaces `--human` and `--agent` | Forces explicit capability declaration. |
