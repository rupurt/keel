---
created_at: 2026-03-05T16:16:10
---

# Reflection - Replace Epic Goal CLI With Problem Input

## Knowledge

- [1vyIq5M2c](../../knowledge/1vyIq5M2c.md) Verify Annotation Chains Only Materialize One Requirement Token

## Observations

The CLI contract change itself was small, but it exposed two downstream contracts that needed to stay in sync: the drift test for creation-surface tokens and the voyage evidence-chain annotations in planning stories.

The biggest surprise was that voyage evidence checks only retain one requirement-phase token per acceptance criterion. Reordering the mixed `SRS-*` and `SRS-NFR-*` markers was enough to clear the voyage warnings without changing runtime behavior.
