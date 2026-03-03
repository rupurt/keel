### Note 001: Queue policy drift prevention
Centralizing queue thresholds and classification helpers into one module removes duplicated literals and keeps `next`, flow bottleneck summaries, and flow state derivation coherent.

### Note 002: Threshold semantics must be explicit
A small naming mismatch (`>=` vs `>`) can change behavior materially. Encoding policy through category helpers makes boundary semantics clear and testable.
