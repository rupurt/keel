# Observational Knowledge Synthesis - System Design Document

> Automate the aggregation of story reflections into voyage knowledge artifacts.

**Epic:** [1vv7YWzw2](../../README.md) | **SRS:** [SRS.md](SRS.md)

## System Overview

This voyage implements the automated synthesis of story reflections into voyage-level knowledge artifacts.

## Components

### Synthesis Engine
- Create a new module or service to handle the aggregation of `REFLECT.md` files.
- The engine will scan the stories in a voyage and collect all reflection insights.

### Command Integration
- Update `src/commands/voyage/done.rs` to call the synthesis engine before completing the voyage.
- Generate or update a `KNOWLEDGE.md` file in the voyage's directory.

## Constraints & Considerations

- Synthesis should be observational and not modify the original `REFLECT.md` files.
- The `KNOWLEDGE.md` file should be well-structured and easy for both humans and agents to read.
