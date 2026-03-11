# Dependency Injection for Services - SDD

## Overview

This设计 document describes the refactoring of application services to use Storage Ports. We will move away from static methods that take `board_dir` and instead use instance-based services with injected dependencies.

## Architecture

We will adopt a classic dependency injection pattern. Services will hold `Arc<dyn Port>` to allow sharing and thread-safety (required for parallel CLI operations).

### Component Diagram

```text
┌─────────────────────────┐
│      CLI Commands       │
└────────────┬────────────┘
             │ (injects)
             ▼
┌─────────────────────────┐      ┌─────────────────────────┐
│ Application Services    │─────▶│      Domain Ports       │
└─────────────────────────┘      └─────────────────────────┘
                                              ▲
                                              │ (implements)
                                 ┌────────────┴────────────┐
                                 │ Infrastructure Adapters │
                                 └─────────────────────────┘
```

## Detailed Design

### Service Refactoring

Example for `StoryLifecycleService`:

```rust
pub struct StoryLifecycleService {
    board_store: Arc<dyn BoardStore>,
    story_store: Arc<dyn EntityStore<Story>>,
    // ... other ports as needed
}

impl StoryLifecycleService {
    pub fn new(board_store: Arc<dyn BoardStore>, story_store: Arc<dyn EntityStore<Story>>) -> Self {
        Self { board_store, story_store }
    }

    pub fn start(&self, id: &str, version: Option<u64>) -> Result<()> {
        let board = self.board_store.load()?;
        // ... logic
    }
}
```

### Dependency Resolution

In the CLI layer (`src/cli/command_tree.rs` or individual command handlers):

1.  Identify `board_dir`.
2.  Initialize `FileSystemAdapter`.
3.  Wrap in `Arc`.
4.  Initialize required services.
5.  Call service methods.

## Data Flow

1.  User runs `keel story start FEAT-01`.
2.  CLI entry point resolves configuration.
3.  CLI creates `FileSystemAdapter` for the project board.
4.  CLI creates `StoryLifecycleService` with the adapter.
5.  CLI calls `service.start("FEAT-01")`.

## Security & Performance

- **Arc usage**: Ensures that multiple commands or parallel verification runs can safely share the same storage backend.
- **Lazy Loading**: Adapters should ensure they don't reload the entire board if only a single entity is needed (if possible).
