# Reflection - Route Story and Voyage Commands Through Unified Enforcer

### L001: Enforcer intent must match command semantics
`story start` needed `Restart` intent for rejected stories to preserve existing behavior while still routing through unified enforcement.

### L002: Policy flags cleanly preserve command-specific behavior
Passing `require_requirements_coverage: !force` for voyage start retained force bypass semantics without duplicating gate logic in command handlers.
