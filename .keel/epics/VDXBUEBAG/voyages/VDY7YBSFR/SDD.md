# Public Library Surface - SDD

## Overview

This design describes the reorganization of `src/lib.rs` to serve as a clean library entry point.

## Architecture

We will restructure `src/lib.rs` to use `pub mod` for the layers we want to expose.

### Proposed `lib.rs` structure:

```rust
pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod read_model;

// Facade exports for common tasks
pub use application::story_lifecycle::StoryLifecycleService;
// ...
```

## Detailed Design

### API Surface

We will explicitly mark which modules and types are part of the public API using Rust's visibility modifiers (`pub`). We will avoid exporting `src/cli` as it contains terminal-specific logic and `clap` dependencies.

## Data Flow

1.  External crate adds `keel` as a dependency.
2.  External crate imports `keel::application::StoryLifecycleService`.
3.  External crate implements `keel::domain::port::BoardStore`.
4.  External crate initializes the service with its custom store.

## Security & Performance

- **Visibility Control**: By keeping the CLI layer private, we ensure that external users don't accidentally depend on internal terminal styling or command parsing logic.
