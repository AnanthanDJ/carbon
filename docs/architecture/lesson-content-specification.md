# Lesson Content Specification

> **Status:** Active
>
> **Audience:** Lesson Authors & Backend Developers
>
> **Applies to:** `content/lessons`

---

# Overview

Lessons define the educational content of Carbon.

Each lesson is represented as a YAML file describing:

* lesson metadata
* learning objectives
* expected user actions
* validation rules
* rewards
* hints

Lessons are immutable content files loaded during backend startup.

---

# Directory Structure

```text
content/

└── lessons/
    ├── chapter1/
    │   ├── c1l1.yaml
    │   ├── c1l2.yaml
    │   └── ...
    │
    ├── chapter2/
    │
    └── ...
```

Each lesson should exist as its own YAML file.

---

# Loading

All lesson files are loaded during application startup.

```text
Backend Startup

↓

LessonLoader

↓

LessonRegistry

↓

LessonService

↓

LessonRuntime
```

Once loaded, lessons are treated as read-only.

---

# Lesson Structure

Example:

```yaml
id: c1l1

title: Hello Terminal

description: >
  Learn your first terminal command.

chapter: 1

order: 1

mission:
  description: Print "Hello, Terminal!"

  validator:
    type: exact_command
    command: echo "Hello, Terminal!"

reward:
  xp: 10

hints:
  - Remember that echo prints text.
```

---

# Required Fields

| Field       | Description                        |
| ----------- | ---------------------------------- |
| id          | Unique lesson identifier           |
| title       | Lesson title                       |
| description | Lesson description                 |
| chapter     | Chapter number                     |
| order       | Lesson ordering within the chapter |
| mission     | Learning objective                 |
| reward      | Lesson reward                      |

---

# Lesson ID

Lesson IDs should remain stable.

Recommended format:

```text
c1l1

c1l2

c1l3

c2l1
```

Pattern:

```text
c<chapter>l<lesson>
```

Examples

```text
c1l4

c2l8

c4l12
```

---

# Mission

A mission describes what the learner must accomplish.

Example

```yaml
mission:
  description: Create a directory named projects.
```

Each lesson contains exactly one mission.

---

# Validators

The validator determines whether the learner has successfully completed the lesson.

Every lesson contains exactly one validator.

---

## Exact Command

Requires the learner to execute a specific command.

```yaml
validator:
  type: exact_command

  command: pwd
```

---

## Expected Output

Requires command output to exactly match.

```yaml
validator:
  type: expected_output

  output: Hello
```

---

## Current Directory

Requires the learner to be inside a specific directory.

```yaml
validator:
  type: current_directory

  path: /projects
```

---

## File Exists

Succeeds when the specified file exists.

```yaml
validator:
  type: file_exists

  path: hello.txt
```

---

## Directory Exists

Succeeds when the specified directory exists.

```yaml
validator:
  type: directory_exists

  path: projects
```

---

## File Contains

Succeeds when a file contains the expected text.

```yaml
validator:
  type: file_contains

  path: hello.txt

  text: Hello, Carbon!
```

---

# Rewards

Rewards are granted upon successful lesson completion.

Current implementation:

```yaml
reward:
  xp: 10
```

Future versions may include:

```yaml
reward:
  xp: 10

  badges:
    - first-command

  achievements:
    - unix-beginner
```

---

# Hints

Hints assist learners without revealing the answer.

Example

```yaml
hints:
  - echo prints text to the terminal.
  - Strings containing spaces should be wrapped in quotes.
```

Hints should gradually guide the learner.

---

# Writing Guidelines

Lessons should:

* teach one concept
* use real Unix terminology
* avoid unnecessary complexity
* build upon previous lessons

A lesson should introduce exactly one new idea whenever possible.

---

# Validation Philosophy

Validators should verify learning outcomes rather than memorization.

Good examples:

```text
Create a directory.

Navigate into it.

Print the current directory.

Create a file.
```

Avoid combining unrelated objectives in a single lesson.

---

# Lesson Progression

Lessons are completed sequentially.

```text
Lesson 1

↓

Lesson 2

↓

Lesson 3

↓

...
```

The backend determines progression.

Lesson files do not reference subsequent lessons directly.

---

# Best Practices

* Keep lesson descriptions concise.
* Use meaningful file and directory names.
* Prefer realistic examples.
* Introduce concepts incrementally.
* Minimize ambiguity in objectives.

---

# Common Mistakes

Avoid lessons that:

* require multiple unrelated commands
* depend on unspecified filesystem state
* rely on implementation details
* introduce several new concepts simultaneously

---

# Future Extensions

The lesson format is intentionally extensible.

Potential future additions include:

```yaml
difficulty: beginner

estimated_time: 3

prerequisites:
  - c1l2

common_mistakes:
  - message: "You forgot the quotes."

mascot:
  intro: "Let's learn echo!"
  success: "Great job!"
  failure: "Almost there."

resources:
  - docs/echo.md
```

These fields should remain optional so that existing lessons continue to work without modification.

---

# Design Principles

The lesson format should remain:

* Human-readable
* Version-controllable
* Easy to author
* Backend-independent
* Extensible without breaking existing lessons

Lesson content should describe **what** the learner needs to accomplish, while the backend is responsible for determining **whether** it has been accomplished.

