# Storage Backend Configuration - SDD

## Overview

This design describes the addition of storage-related configuration options to Keel. We will use the existing layered configuration system to support project-level, user-level, and environment-level overrides.

## Architecture

We will update the `Config` struct in `src/infrastructure/config.rs`.

### Configuration Schema

```toml
[storage]
backend = "filesystem" # options: filesystem, server (future)

[storage.filesystem]
# backend-specific options could go here
```

## Detailed Design

### Storage Factory

We will introduce a `StorageFactory` or similar utility that:
1.  Takes the resolved `Config`.
2.  Initializes the appropriate `BoardStore` and `EntityStore` implementations.
3.  Returns them as `Arc<dyn Port>`.

## Data Flow

1.  CLI starts.
2.  Configuration is loaded and resolved.
3.  `StorageFactory` is called with the config.
4.  Storage ports are injected into application services.

## Security & Performance

- **Environment Overrides**: Allows sensitive settings (like API keys for future remote backends) to be passed via env vars instead of being committed to `keel.toml`.
