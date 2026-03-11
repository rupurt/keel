---
id: VDY8VYlbu
title: Implement EntityStore For FileSystemAdapter
type: feat
status: backlog
created_at: 2026-03-10T23:45:00
scope: VDXBUAn7a/VDY8Js8As
index: 3
updated_at: 2026-03-11T02:31:36
---

# Implement EntityStore For FileSystemAdapter

## Summary

Implement the `EntityStore<T>` trait for Keel entities, providing CRUD operations on the local filesystem.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] `EntityStore<T>::get` retrieves an entity by its ID. <!-- verify: cargo test -p keel filesystem_entity_store, SRS-02:start -->
- [ ] [SRS-02/AC-02] `EntityStore<T>::list` returns all entities of a given type. <!-- verify: cargo test -p keel filesystem_entity_store, SRS-02:continues -->
- [ ] [SRS-02/AC-03] `EntityStore<T>::put` and `delete` correctly modify the disk state. <!-- verify: cargo test -p keel filesystem_entity_store, SRS-02:end -->
