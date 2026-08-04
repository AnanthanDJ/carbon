# ADR-0005: Safe Terminal Approximation

**Status:** Accepted

## Context

Real terminals expose beginners to unnecessary complexity and potentially destructive commands.

The project is intended as a stepping stone rather than a replacement.

## Decision

The platform will intentionally expose only the concepts required during a beginner's first terminal experience.

The terminal is not intended to be **POSIX-complete**.

## Consequences

### Advantages

- Reduced intimidation
- Easier lesson authoring
- Deterministic behaviour

### Trade-offs

- Advanced users quickly outgrow the platform