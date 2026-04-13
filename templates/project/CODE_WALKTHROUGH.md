# {{project_name}} Code Walkthrough

This document orients contributors and agents to the source layout, key abstractions, and data flows in the {{project_name}} codebase. For governance philosophy see [CONSTITUTION.md](CONSTITUTION.md); for architectural contracts see [ARCHITECTURE.md](ARCHITECTURE.md); for coordination and Mission Stack rules see [PROTOCOL.md](PROTOCOL.md).

## Repository Layout

Describe the top-level directory structure and what each area is responsible for.

- What is the build system and how are modules/packages organized?
- Where does production code live vs tests, scripts, and configuration?
- Which directories are entry points and which are internal?

## Key Abstractions

Document the core types, services, or modules that a contributor must understand before making changes.

- What are the primary domain objects and where are they defined?
- What are the main interfaces or traits that compose the system?
- How does data flow from input to output through the layers?

## State and Lifecycle

If the system manages stateful entities, describe the key state machines and transitions.

- What states can entities be in?
- What triggers transitions and what are the side effects?
- Where is state persisted and how is it loaded?

## Command / Request Flow

Trace a representative operation from entry point to completion.

- Where does user input enter the system?
- What validation or enforcement happens before execution?
- How are results persisted and presented back?
- Where do external or Mission Stack requests get normalized before local board mutations?

## Configuration

Describe the key configuration surfaces and their effects on runtime behavior.

- Where does configuration live and what format does it use?
- What are the most important settings a new contributor should know about?
- How does configuration affect routing, behavior, or output?

## Where to Look

Provide a quick-reference table mapping common tasks to starting points in the code.

| I want to... | Start here |
|---------------|-----------|
| Understand the domain model | |
| Add a new command or endpoint | |
| Change how output renders | |
| Modify validation rules | |
| Add a new entity or resource | |
| Change state transition behavior | |
