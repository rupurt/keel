# Reflection - Define Verification Technique Catalog Model

## Knowledge

### L001: Catalog Entries Should Be Declarative And Sorted By ID
| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Defining a technique bank that will be extended by config/autodetection stories |
| **Insight** | A stable schema plus ID-sorted built-ins gives deterministic output and a predictable merge base for later override/ranking stages. |
| **Suggested Action** | Keep all built-ins in one constructor and enforce sort-by-ID before returning catalog vectors. |
| **Applies To** | `src/read_model/verification_techniques.rs`, upcoming config merge/recommendation modules |
| **Observed At** | 2026-03-04T18:23:41Z |
| **Score** | 0.81 |
| **Confidence** | 0.91 |
| **Applied** | yes |

## Observations

Separating model fields by concern (configuration, detection, ranking, rendering) made AC coverage straightforward.
The deterministic-order test is a low-cost guard that will prevent flaky recommendation output later.
