# ADR-0009: Content as Data

**Status:** Accepted

## Context

Educational content changes much more frequently than application logic.

## Decision

Lessons, glossary entries, documentation, and mascot dialogue will be stored as Markdown/YAML/TOML content files.

The database stores only user state.

## Consequences

### Advantages

- Version-controlled lessons
- Easier collaboration
- No database migrations for content

### Trade-offs

- Requires a content loader