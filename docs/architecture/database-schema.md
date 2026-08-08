# Database Schema

> **Status:** Active
>
> **Audience:** Backend Developers
>
> **Database:** SQLite

---

# Overview

Carbon stores all persistent application state in SQLite.

The database is responsible for:

* user accounts
* virtual filesystem
* lesson progression
* lesson history

Educational content such as lessons and documentation are **not** stored in the database. They are loaded from the `content/` directory during application startup.

---

# Entity Relationship Diagram

```text
                    users
                      │
          ┌───────────┼────────────┐
          │           │            │
          ▼           ▼            ▼
filesystem_nodes  lesson_progress  users_current_lesson
          │
          │
          ▼
lesson_attempts
```

---

# Tables

The current schema consists of five primary tables.

| Table                | Purpose                   |
| -------------------- | ------------------------- |
| users                | User accounts             |
| filesystem_nodes     | Virtual filesystem        |
| lesson_progress      | Lesson completion history |
| lesson_attempts      | Validation history        |
| users_current_lesson | Current active lesson     |

---

# users

Stores user accounts.

```sql
CREATE TABLE users (
    id TEXT PRIMARY KEY,

    username TEXT NOT NULL UNIQUE,

    password_hash TEXT NOT NULL,

    created_at TEXT NOT NULL
);
```

---

## Fields

| Field         | Type | Description                |
| ------------- | ---- | -------------------------- |
| id            | TEXT | User UUID                  |
| username      | TEXT | Unique username            |
| password_hash | TEXT | Argon2 password hash       |
| created_at    | TEXT | Account creation timestamp |

---

# filesystem_nodes

Stores the virtual filesystem.

Directories and files share the same table.

```sql
CREATE TABLE filesystem_nodes (
    id TEXT PRIMARY KEY,

    parent_id TEXT,

    user_id TEXT NOT NULL,

    name TEXT NOT NULL,

    kind TEXT NOT NULL,

    content TEXT
);
```

---

## Fields

| Field     | Description                          |
| --------- | ------------------------------------ |
| id        | Node UUID                            |
| parent_id | Parent directory                     |
| user_id   | Owner                                |
| name      | File or directory name               |
| kind      | file or directory                    |
| content   | File contents (NULL for directories) |

---

## Root Directory

Every user owns a single root directory.

```text
/
```

The root directory has:

```text
parent_id = NULL
```

---

## Directory Tree

Example:

```text
/

├── home
│   └── notes.txt
│
├── projects
│
└── hello.txt
```

Every node references its parent through `parent_id`.

---

# lesson_progress

Tracks completed lessons.

```sql
CREATE TABLE lesson_progress (
    user_id TEXT NOT NULL,

    lesson_id TEXT NOT NULL,

    status TEXT NOT NULL DEFAULT 'in_progress',

    started_at TEXT NOT NULL,

    completed_at TEXT,

    PRIMARY KEY (user_id, lesson_id)
);
```

---

## Fields

| Field        | Description          |
| ------------ | -------------------- |
| user_id      | User UUID            |
| lesson_id    | Lesson identifier    |
| status       | Current lesson state |
| started_at   | Lesson start time    |
| completed_at | Completion time      |

---

## Status Values

Current values:

```text
in_progress

completed
```

Additional states may be introduced in future versions.

---

# lesson_attempts

Records every lesson validation attempt.

```sql
CREATE TABLE lesson_attempts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    user_id TEXT NOT NULL,

    lesson_id TEXT NOT NULL,

    command TEXT NOT NULL,

    successful INTEGER NOT NULL,

    created_at TEXT NOT NULL
);
```

---

## Fields

| Field      | Description        |
| ---------- | ------------------ |
| id         | Attempt identifier |
| user_id    | User UUID          |
| lesson_id  | Lesson identifier  |
| command    | Executed command   |
| successful | Validation result  |
| created_at | Timestamp          |

---

## Purpose

Attempt history allows the backend to:

* record learner progress
* collect analytics
* inspect failures
* support future replay features

Every validation attempt is stored, regardless of success.

---

# users_current_lesson

Stores the learner's active lesson.

```sql
CREATE TABLE users_current_lesson (
    user_id TEXT PRIMARY KEY,

    lesson_id TEXT NOT NULL,

    updated_at TEXT NOT NULL
);
```

---

## Fields

| Field      | Description           |
| ---------- | --------------------- |
| user_id    | User UUID             |
| lesson_id  | Active lesson         |
| updated_at | Last change timestamp |

---

## Why Separate This Table?

Although the current lesson could theoretically be derived from `lesson_progress`, maintaining a dedicated table provides:

* constant-time lookup
* simpler queries
* cleaner runtime logic

This avoids scanning lesson history for every command execution.

---

# Data Flow

Typical lesson progression:

```text
User executes command

↓

lesson_attempts
(record validation)

↓

lesson_progress
(mark completed)

↓

users_current_lesson
(advance)
```

---

# Filesystem Flow

Filesystem operations update `filesystem_nodes`.

Example:

```text
mkdir projects

↓

Insert directory node

↓

filesystem_nodes
```

Deleting a file:

```text
rm notes.txt

↓

Delete node

↓

filesystem_nodes
```

---

# Relationships

```text
users

    │

    ├──────────────► filesystem_nodes.user_id

    │

    ├──────────────► lesson_progress.user_id

    │

    ├──────────────► lesson_attempts.user_id

    │

    └──────────────► users_current_lesson.user_id
```

Lesson identifiers reference lesson YAML files by their `id` field rather than a separate database table.

---

# Design Decisions

The schema follows several principles:

* UUIDs for stable identifiers
* One table for both files and directories
* Immutable lesson definitions stored outside the database
* Repository abstraction between application logic and SQLite
* Simple schema optimized for readability and maintainability

---

# Future Extensions

Potential future additions include:

## achievements

```text
Store unlocked achievements.
```

## user_settings

```text
Theme

Accessibility

Editor preferences
```

## command_history

```text
Persistent terminal history.
```

## lesson_checkpoints

```text
Support resumable lessons.
```

These additions can be introduced without modifying the existing tables or breaking current functionality.

