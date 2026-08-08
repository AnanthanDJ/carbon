# Lesson Content Specification

> **Status:** Active
>
> **Audience:** Content Authors
>
> **Project:** Carbon
>
> **Version:** v1 (Buildathon)

---

# Overview

Lessons are defined as content rather than application code.

This allows new lessons to be added, modified, or reorganized without changing the backend.

The lesson runtime loads lesson definitions during application startup and validates user actions against them.

---

# Design Goals

Lessons should be:

- beginner-friendly
- incremental
- interactive
- deterministic
- documentation-driven

A lesson should teach **one concept at a time**.

---

# Content Location

All lesson definitions are stored under:

```text
content/
└── lessons/
```

The backend loads every lesson in this directory during startup.

---

# Lesson Lifecycle

```text
Lesson Definition
        │
        ▼
Loaded at startup
        │
        ▼
Indexed
        │
        ▼
Presented to user
        │
        ▼
Terminal interaction
        │
        ▼
Validation
        │
        ▼
Progress recorded
        │
        ▼
Next lesson unlocked
```

The frontend never validates lesson completion.

---

# Lesson Structure

Each lesson should define:

- unique identifier
- title
- objective
- description
- expected command or validation rule
- hints
- documentation references
- next lesson (if applicable)

A lesson represents a single learning objective.

---

# Lesson Principles

Every lesson should introduce only one new concept.

Good examples:

- printing text
- current directory
- listing files
- creating directories
- reading files

Avoid combining multiple unrelated concepts into a single lesson.

---

# Validation

Lesson completion is determined entirely by the backend.

Typical validation process:

```text
User executes command
        │
        ▼
Terminal execution
        │
        ▼
Lesson runtime
        │
        ▼
Validation
        │
        ▼
Success or failure
```

Validation may depend on:

- command name
- command arguments
- filesystem state
- command result

The frontend should treat backend validation as authoritative.

---

# Hints

Lessons may provide progressive hints.

Hints should:

- encourage experimentation
- explain concepts
- avoid revealing the full solution immediately

Example progression:

```text
Hint 1

Think about which command creates a directory.


Hint 2

The command begins with "mk..."


Hint 3

Try:

mkdir projects
```

Hints should become increasingly specific.

---

# Documentation References

Lessons should reference relevant documentation.

Example:

```text
Related Documentation

mkdir
filesystem
paths
```

The frontend may use these references to open documentation from:

```text
content/docs/
```

This keeps lessons concise while encouraging exploration.

---

# Glossary References

Lessons introducing new terminology should also reference glossary entries.

Example:

```text
directory
path
working directory
```

Glossary content is stored under:

```text
content/glossary/
```

---

# Learning Philosophy

Carbon is documentation-first.

Lessons should encourage users to consult documentation rather than memorize commands.

A successful lesson should increase confidence, not simply test recall.

---

# Lesson Progression

Lessons are intended to be completed sequentially.

Typical progression:

```text
echo

↓

pwd

↓

ls

↓

cd

↓

mkdir

↓

touch

↓

cat

↓

rm

↓

rmdir
```

Each lesson builds upon concepts introduced earlier.

---

# Attempt Recording

Every validation attempt may be recorded by the backend.

Information recorded can include:

- lesson
- executed command
- success or failure
- timestamp

This data is used to track learning progress and allow users to resume where they left off.

---

# Content Guidelines

When writing lessons:

✔ Introduce one concept.

✔ Keep explanations concise.

✔ Encourage experimentation.

✔ Reference documentation.

✔ Provide progressive hints.

✔ Assume no prior terminal experience.

Avoid:

- long paragraphs
- multiple objectives
- hidden requirements
- unexplained terminology

---

# Example Learning Flow

```text
Display lesson
        │
        ▼
User reads objective
        │
        ▼
User experiments
        │
        ▼
Backend validates
        │
        ▼
Lesson completed
        │
        ▼
Next lesson begins
```

This interaction loop is the core learning model of Carbon.

---

# Design Principles

Lessons should prioritize:

1. Confidence over memorization.
2. Experimentation over imitation.
3. One concept at a time.
4. Documentation before answers.
5. Progressive guidance.
6. Immediate feedback.

These principles should guide every lesson added to Carbon.

---

# Future Improvements

Potential future enhancements include:

- multiple validation strategies
- optional challenges
- branching lesson paths
- achievements
- lesson categories
- difficulty levels
- localization
- instructor-authored lesson packs

The lesson format should remain content-driven so new learning material can be added without modifying backend code.
