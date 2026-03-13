# Accelerated Bearing Source Capture - Charter

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Add a source-capture workflow that can ingest a URL and auto-populate title, retrieval date, and provenance so bearing evidence collection becomes faster and more consistent. | board: VDiHwULir |

## Constraints

- Preserve the existing evidence contract and keep captured metadata reviewable and editable after ingestion.
- Autocapture should fill only clearly derivable fields and leave ambiguity visible instead of inventing data.
- Keep manual source entry available for offline, non-web, or sensitive provenance cases.
- Normalize provenance and timestamp fields so doctor, assessment, and later automation can trust the captured record shape.

## Halting Rules

- DO NOT halt while adding a web source still requires repetitive manual entry of title, retrieval date, and provenance.
- HALT when Keel can ingest a URL into bearing evidence with normalized metadata and a clean manual follow-up path for notes or corrections.
- YIELD to human if remote fetch behavior or source-capture security policy requires a broader product decision before implementation.
