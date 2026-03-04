# Knowledge - 1vxqN5jnA

> Automated synthesis of story reflections.

## Story: Define Verification Technique Catalog Model (1vxqNFaR9)

### L001: Catalog Entries Should Be Declarative And Sorted By ID

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Defining a technique bank that will be extended by config/autodetection stories |
| **Insight** | A stable schema plus ID-sorted built-ins gives deterministic output and a predictable merge base for later override/ranking stages. |
| **Suggested Action** | Keep all built-ins in one constructor and enforce sort-by-ID before returning catalog vectors. |
| **Applies To** | `src/read_model/verification_techniques.rs`, upcoming config merge/recommendation modules |
| **Applied** | yes |



---

## Story: Implement Keel.toml Technique Configuration Overrides (1vxqNFJOf)

### L001: Advisory parser keeps keel.toml resilient

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Technique overrides need richer schema while core config loading should not fail when optional override blocks are malformed. |
| **Insight** | Parsing overrides from raw TOML with per-field diagnostics allows invalid entries to be ignored safely without blocking normal command behavior. |
| **Suggested Action** | Keep optional/advanced config surfaces advisory by default, then merge validated entries into canonical models with explicit diagnostics. |
| **Applies To** | `src/read_model/verification_techniques.rs` |
| **Applied** | yes |



---

## Story: Surface Technique Recommendations In Planning Shows (1vxqNFHpk)

### L001: Centralized recommendation projection keeps show commands coherent

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Multiple read commands need consistent recommendation output while using different local data sources. |
| **Insight** | A shared recommendation report model plus per-command input extraction avoids drift between epic/voyage/story rendering. |
| **Suggested Action** | Add new recommendation behavior in `verification_techniques` first, then wire each show command through the same renderer helper. |
| **Applies To** | `src/read_model/verification_techniques.rs`, `src/cli/commands/management/*/show.rs` |
| **Applied** | yes |



---

## Story: Implement Project Autodetection And Recommendation Engine (1vxqNFNdN)

### L001: Deterministic ranking requires total-order tie breaks

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Recommendation scores can tie across techniques when confidence and keyword matches are equivalent. |
| **Insight** | Deterministic ordering is guaranteed only when ranking sorts by score and then by stable id as a total-order tie breaker. |
| **Suggested Action** | Keep recommendation outputs sorted by `(score desc, id asc)` and normalize lists/sets before scoring. |
| **Applies To** | `src/read_model/verification_techniques.rs` |
| **Applied** | yes |



---

