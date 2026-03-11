# Core Storage Traits - SDD

## Overview

This design introduces the "Ports" for Keel's hexagonal architecture. We will define traits that represent the abstract capability of storing and retrieving Keel entities and the board aggregate.

## Architecture

We will introduce a new module: `src/domain/port/mod.rs` (or similar). This module will contain the trait definitions.

### Component Diagram

```text
┌───────────────────┐      ┌──────────────────┐      ┌─────────────────────────┐
│  Application      │      │      Domain      │      │     Infrastructure      │
│    Services       │─────▶│      Ports       │◀─────│        Adapters         │
└───────────────────┘      └──────────────────┘      └─────────────────────────┘
                                     │                            │
                                     ▼                            ▼
                           ┌──────────────────┐      ┌─────────────────────────┐
                           │   Domain Models  │      │   FileSystem / Database │
                           └──────────────────┘      └─────────────────────────┘
```

## Detailed Design

### `BoardStore` Trait

```rust
pub trait BoardStore {
    fn load(&self) -> Result<Board>;
    fn save(&self, board: &Board) -> Result<()>;
}
```

### `EntityStore<T>` Trait

```rust
pub trait EntityStore<T: Entity> {
    fn get(&self, id: &str) -> Result<T>;
    fn put(&self, entity: &T) -> Result<()>;
    fn list(&self) -> Result<Vec<T>>;
    fn delete(&self, id: &str) -> Result<()>;
}
```

## Data Flow

1.  The CLI initializes a concrete `FileSystemAdapter` (in a future epic).
2.  The adapter is injected into the `StoryLifecycleService`.
3.  The service calls `store.load()` or `store.put(story)` without knowing it's hitting the disk.

## Security & Performance

- **Security**: The ports themselves are abstract. Security (e.g. file permissions) is the responsibility of the concrete adapter.
- **Performance**: Traits should use `Result` to allow adapters to report I/O latency or failures.
