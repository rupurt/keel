# Reflection - Remove Legacy State and Status Deserializers

### Note 001: Hard-cut deserializers should fail with explicit replacement guidance
Legacy tokens are safest when rejected with deterministic, canonical replacement hints so migration gaps are obvious and actionable.

### Note 002: Canonical-state migrations require fixture cleanup beyond parser units
Once aliases are removed, integration fixtures that still emit legacy epic status values fail quickly, so test data must be normalized in the same change.
