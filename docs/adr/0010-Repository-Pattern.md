# ADR-0010: Repository Pattern

**Status:** Accepted

## Context

The application should not depend directly on SQLite.

## Decision

Storage will be abstracted behind repository traits.

Business logic depends only on repository interfaces.

SQLite is an implementation detail.

## Consequences

### Advantages

- Storage independence
- Easier testing
- Future PostgreSQL support

### Trade-offs

- Additional abstraction layer