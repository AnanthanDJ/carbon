# ADR-0006: Semantic Terminal

**Status:** Accepted

## Context

Traditional terminals render streams of text.

The project requires:

- Clickable commands
- Interactive output
- Glossary popups
- Command descriptions

## Decision

The terminal will render structured semantic objects instead of raw terminal text.

### Pipeline

```text
Keyboard
   ↓
Lexer
   ↓
Parser
   ↓
AST
   ↓
Executor
   ↓
Structured Output
   ↓
React Components
```

Every rendered token contains metadata.

## Consequences

### Advantages

- Interactive commands
- Interactive errors
- Glossary support
- Mobile-friendly interactions

### Trade-offs

- More custom implementation
- Not a true terminal emulator