# ADR-0001: Mobile-First Design

**Status:** Accepted

## Context

The platform targets complete beginners, many of whom may only have access to a smartphone. The project itself originated from learning programming through Termux on a low-end Android device before transitioning to a ThinkPad running Arch Linux.

Traditional terminal learning platforms assume desktop environments with physical keyboards, large screens, and hover interactions.

## Decision

The platform will be designed **mobile-first**.

Desktop support will be derived from the mobile experience rather than the reverse.

The UI must support:

- Touch interactions
- Portrait layouts
- Small screens
- Virtual keyboards

Hover interactions must always have an equivalent tap or long-press interaction.

## Consequences

### Advantages

- Accessible to more beginners
- Lightweight
- Easier transition from phone to desktop

### Trade-offs

- Desktop-specific interactions are intentionally limited
- UI design requires careful information density