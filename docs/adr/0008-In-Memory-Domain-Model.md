# ADR-0008: In-Memory Domain Model

**Status:** Accepted

## Context

Filesystem operations occur frequently during lessons.

Persisting an entire serialized tree after every command would be inefficient.

## Decision

Maintain the filesystem as an in-memory tree.

Persist only node-level changes.

SQLite stores nodes using an adjacency-list representation.

## Consequences

### Advantages

- Efficient updates
- Clean domain model
- Future database portability

### Trade-offs

- Requires reconstruction when loading sessions