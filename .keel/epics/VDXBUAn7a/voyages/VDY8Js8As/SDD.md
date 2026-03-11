# Filesystem Storage Implementation - SDD

## Overview

This设计 document describes the implementation of the `FileSystemAdapter`.

## Architecture

The adapter will live in `src/infrastructure/storage/filesystem.rs`. It will wrap the existing `load_board` and `parse_frontmatter` functions to provide a trait-compliant interface.

## Detailed Design

### Adapter Structure

```rust
pub struct FileSystemAdapter {
    root: PathBuf,
}

impl BoardStore for FileSystemAdapter {
    fn load(&self) -> Result<Board> {
        // delegates to loader::load_board
    }
}

// ...
```

### Entity Store Implementation

We will use generics or concrete impls for each entity type to satisfy the `EntityStore<T>` trait.

## Data Flow

1.  Application Service requests an entity by ID.
2.  `FileSystemAdapter` resolves the ID to a file path.
3.  `FileSystemAdapter` reads the file, parses frontmatter, and returns the entity.

## Security & Performance

- **File Access**: The adapter uses the standard library's `std::fs` for all operations.
- **Caching**: Future versions of this adapter could implement internal caching to optimize multiple reads.
