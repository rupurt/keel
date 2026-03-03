### Note 001: Legality and gates should be separate outputs before classification
Returning legality findings and gate findings as distinct fields makes enforcement decisions easier to reason about and keeps debugging focused when a transition fails.

### Note 002: Blocking policy should be explicit at evaluation time
Modeling strict/runtime/reporting blocking modes directly in enforcement policy removes ad-hoc severity filtering and creates deterministic behavior across runtime and diagnostic paths.
