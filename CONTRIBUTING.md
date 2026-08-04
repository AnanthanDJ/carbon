# Contributing to Carbon

Thank you for contributing to **Carbon**.

Carbon exists to help beginners overcome the fear of the terminal through guided experimentation, documentation-first learning, and thoughtful engineering. Every contribution should reinforce that mission.

---

# Our Philosophy

Before adding a feature, ask yourself:

> **Does this help users become more confident using a real terminal?**

If the answer is "no", reconsider whether it belongs in Carbon.

Carbon is intentionally:

* a stepping stone, not a replacement for Linux
* documentation-first
* deterministic
* beginner-focused
* built to be outgrown

---

# Repository Structure

```
apps/
    server/        Rust backend
    web/           React frontend

content/
    lessons/       Interactive lessons
    docs/          Command documentation
    glossary/      Beginner-friendly terminology
    mascot/        Mux dialogue

docs/
    adr/           Architecture Decision Records
    architecture/  Design documentation
    research/      User research
    testing/       Test plans & reports
```

---

# Development Workflow

1. Create a new branch.

```
feature/terminal-parser
fix/filesystem-path
docs/glossary
content/navigation-lessons
```

2. Keep commits focused.

3. Open a pull request.

4. Request a review before merging.

---

# Commit Messages

Carbon follows Conventional Commits.

Examples:

```
feat(terminal): implement mkdir command
feat(lesson): add navigation chapter
fix(filesystem): resolve relative path bug
refactor(storage): simplify repository layer

docs(adr): add semantic terminal decision
docs(research): summarize beginner survey

content(lessons): add filesystem navigation
content(glossary): explain PATH variable

test(parser): add lexer edge cases
```

Avoid commit messages such as:

```
update
changes
fix
misc
```

---

# Coding Guidelines

## Backend

* Keep modules focused.
* Prefer composition over global state.
* Keep business logic independent of storage.
* Run `cargo fmt` before committing.
* Ensure `cargo clippy` passes.
* Write tests whenever practical.

## Frontend

* Prefer reusable components.
* Keep presentation separate from business logic.
* Avoid deeply nested component trees.
* Design mobile-first.

---

# Content Guidelines

Lessons should encourage discovery rather than memorization.

A lesson should:

* introduce one concept
* have one clear objective
* encourage experimentation
* reference documentation
* provide progressive hints

Avoid long explanations.

Teach by interaction whenever possible.

---

# Documentation

Every new command should include:

* command documentation
* glossary updates (if new terminology is introduced)
* lesson integration (when appropriate)

---

# Mux

Mux is not an AI assistant.

Mux exists to:

* encourage curiosity
* explain terminology
* provide progressive hints
* celebrate genuine progress

Mux should **never** immediately provide the solution unless the lesson explicitly allows it.

---

# Architecture Decisions

Any significant design decision should be documented as an ADR.

Examples:

* introducing a new storage backend
* changing lesson format
* modifying the terminal architecture
* introducing AI features
* changing the learning model

---

# Testing

Please test before opening a pull request.

Consider:

* parser behaviour
* filesystem operations
* lesson progression
* mobile responsiveness
* accessibility
* edge cases

Bug reports and testing notes should be placed under:

```
docs/testing/
```

---

# Pull Request Checklist

* [ ] Code builds successfully
* [ ] Tests pass
* [ ] Code is formatted
* [ ] Documentation updated (if required)
* [ ] Lessons updated (if required)
* [ ] ADR added (for architectural changes)
* [ ] Commit history is clean

---

# Core Principles

When in doubt, optimize for:

1. Clarity over cleverness.
2. Confidence over memorization.
3. Experimentation over imitation.
4. Documentation over hidden knowledge.
5. Simplicity over unnecessary complexity.

---

# Final Note

Carbon is successful when users no longer need it.

Every contribution should help beginners gain the confidence to leave Carbon and explore a real terminal on their own.

Build thoughtfully.
