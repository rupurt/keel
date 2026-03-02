### L001: Unified enforcement reduces command-side complexity
Wiring `enforce_transition` into `story start` ensures that state legality and domain gates are checked in one pass, removing the need for manual check sequences in command handlers.

### L002: Consistent error formatting
Using `format_enforcement_error` ensures that all state transitions report issues with the same structure, improving CLI predictability for both humans and agents.
