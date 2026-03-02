# Reflection - Implement Hard Migration Command for Canonical Schema

### L001: Preflight validation prevents partial migrations
Collecting all unsupported status tokens before writing ensures hard migration fails safely and keeps board files unchanged on error.

### L002: Path-scoped entity classification keeps rewrites precise
Classifying story/voyage/epic README paths before applying mappings avoids accidental status normalization outside canonical schema surfaces.
