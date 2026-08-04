# ADR-0007: Virtual Filesystem

**Status:** Accepted

## Context

Only beginner filesystem concepts are required.

Implementing a real filesystem would unnecessarily increase complexity.

## Decision

Implement a virtual filesystem supporting only:

- Directories
- Regular files

No support for:

- Permissions
- Symbolic links
- Devices
- Ownership
- Timestamps

## Consequences

### Advantages

- Simpler implementation
- Deterministic lessons
- Lightweight

### Trade-offs

- Not compatible with arbitrary shell behaviour