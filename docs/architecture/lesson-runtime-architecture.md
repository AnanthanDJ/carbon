# Lesson Runtime Architecture

> **Status:** Active
>
> **Audience:** Backend Developers
>
> **Subsystem:** Lesson Engine

---

# Overview

The Lesson Runtime is responsible for validating learner actions and progressing them through the course.

It connects the terminal, virtual filesystem, lesson definitions, and database into a single execution pipeline.

The runtime does **not** execute commands. Command execution is handled by the terminal subsystem. The runtime only evaluates the result of those commands.

---

# Responsibilities

The Lesson Runtime is responsible for:

* determining the user's current lesson
* validating completed commands
* recording lesson attempts
* marking lessons as complete
* advancing users to the next lesson
* returning lesson progression information to the frontend

It is **not** responsible for:

* parsing commands
* executing commands
* filesystem operations
* rendering lesson content

---

# High-Level Flow

```text
User enters command
        │
        ▼
Terminal Parser
        │
        ▼
Command Dispatcher
        │
        ▼
Filesystem Service
        │
        ▼
Command Result
        │
        ▼
Lesson Runtime
        │
        ▼
Lesson Validator
        │
        ▼
Lesson Repository
        │
        ▼
SQLite
```

---

# Runtime Lifecycle

Every terminal command follows the same lifecycle.

```text
Receive command

↓

Parse command

↓

Execute command

↓

Generate CommandResult

↓

Load current lesson

↓

Validate lesson

↓

Record attempt

↓

Complete lesson (if successful)

↓

Advance lesson

↓

Return LessonOutcome
```

---

# LessonRuntimeService

The runtime is implemented by `LessonRuntimeService`.

Responsibilities include:

* retrieving the current lesson
* starting lessons
* completing lessons
* advancing progression
* validating terminal commands

The runtime coordinates the other lesson components but does not contain lesson-specific validation logic.

---

# Current Lesson

The runtime first determines the learner's active lesson.

```text
User

↓

users_current_lesson

↓

LessonService

↓

Lesson
```

If no lesson is active, validation is skipped.

---

# Validation

Validation is delegated to `LessonValidator`.

```text
Lesson

+

TerminalSession

+

ParsedCommand

+

CommandResult

↓

ValidationResult
```

This separation keeps validation rules independent from progression logic.

---

# Supported Validators

The runtime currently supports:

* ExactCommand
* ExpectedOutput
* CurrentDirectory
* FileExists
* DirectoryExists
* FileContains

Each validator checks one aspect of the learner's terminal state.

---

# Recording Attempts

Every validation attempt is recorded.

```text
LessonAttempt

↓

lesson_attempts
```

Stored information includes:

* user
* lesson
* executed command
* success or failure
* timestamp

Attempts are recorded regardless of whether validation succeeds.

---

# Successful Completion

When validation succeeds:

```text
Current Lesson

↓

lesson_progress

↓

completed

↓

Advance

↓

users_current_lesson
```

The runtime updates both progress and the active lesson.

---

# Failed Validation

If validation fails:

```text
Record Attempt

↓

Return Failure

↓

Remain on Current Lesson
```

No lesson progression occurs.

The learner may retry immediately.

---

# LessonOutcome

After validation, the runtime returns a `LessonOutcome`.

```text
LessonOutcome

completed

current

next
```

Example:

```text
completed = true

current = echo

next = pwd
```

or

```text
completed = false

current = mkdir

next = null
```

---

# Interaction with Filesystem

Some validators inspect the virtual filesystem.

```text
LessonValidator

↓

FilesystemService

↓

FilesystemRepository

↓

SQLite
```

Examples include:

* FileExists
* DirectoryExists
* FileContains

The validator never queries SQLite directly.

---

# Interaction with Terminal

The runtime receives terminal execution results.

Inputs include:

* current terminal session
* parsed command
* command output

The runtime does not execute commands itself.

---

# Interaction with LessonService

Lesson definitions are loaded once during startup.

```text
LessonLoader

↓

LessonRegistry

↓

LessonService
```

The runtime only performs lookups.

Lesson files remain immutable during execution.

---

# Interaction with Repository Layer

Persistence is handled through repository interfaces.

```text
LessonRuntime

↓

LessonRepository

↓

SQLite Implementation

↓

Database
```

This keeps the runtime independent of the storage backend.

---

# Error Handling

Validation failures are **not** treated as runtime errors.

Example:

```text
User executes incorrect command

↓

Validation fails

↓

Attempt recorded

↓

Lesson remains active

↓

Return completed = false
```

Only infrastructure failures (database, repository, etc.) return runtime errors.

---

# Design Principles

The Lesson Runtime follows these principles:

* Single responsibility
* Stateless execution
* Repository abstraction
* Immutable lesson definitions
* Deterministic validation
* Database-backed progression

This separation keeps lesson content, validation, persistence, and command execution independent while allowing them to work together through well-defined interfaces.

