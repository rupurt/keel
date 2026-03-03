# Reflection - Remove Stage and Status Compatibility Aliases

### Note 001: Alias removal is safest as a compile-driven codemod
Replacing compatibility aliases with canonical state-machine types across modules was most reliable when done mechanically and validated immediately with `cargo check` to catch stragglers.

### Note 002: Canonical-type migrations need both code and metadata cleanup
Removing alias modules required updating stale comments and board metadata checks so the migration is coherent in code, docs, and doctor gates.
