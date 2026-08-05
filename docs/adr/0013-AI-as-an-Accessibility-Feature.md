# ADR-0013: AI as an Accessibility Feature, Not a Teaching Engine

**Status:** Accepted

## Context

AI assistants are increasingly common in educational software, and judges may expect AI features. However, the project's educational objective is to cultivate independent problem-solving, documentation literacy, and confidence using a terminal.

## Decision

Carbon will not rely on AI as its primary teaching mechanism.

The core learning experience will use deterministic lessons, structured hints, and documentation-first guidance.

If AI is introduced in future versions, it will be limited to accessibility-enhancing tasks such as:

- Simplifying documentation
- Explaining unfamiliar terminology
- Rephrasing error messages
- Adapting explanations to different reading levels

AI must never replace the discovery process or become the default source of answers.

## Consequences

### Advantages

- Preserves the educational philosophy
- Ensures consistent lesson quality
- Prevents answer dependency
- Keeps the learning experience reproducible

### Trade-offs

- Requires more carefully authored educational content
- Forgoes some open-ended conversational capabilities