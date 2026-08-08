# Database Schema

> **Status:** Active
>
> **Audience:** Backend Developers
>
> **Project:** Carbon
>
> **Version:** v1 (Buildathon)

---

# Overview

Carbon uses SQLite as its primary datastore.

The database stores persistent application state while all business logic remains inside the service layer.

Current responsibilities include:

- user accounts
- authentication sessions
- virtual filesystem
- lesson progression
- lesson attempts

---

# Entity Relationship Diagram

```text
                  Users
                    │
        ┌───────────┼───────────┐
        ▼           ▼           ▼
   Sessions   Filesystem    Lesson Progress
                                │
                                ▼
                         Lesson Attempts
```

Each user owns their own filesystem, authentication sessions, and lesson progress.

---

# Tables

Current schema consists of five primary tables.

```
users
sessions
filesystem_nodes
lesson_progress
lesson_attempts
```

---

# Users

Stores registered user accounts.

## Fields

| Column | Type | Description |
|--------|------|-------------|
| id | TEXT | UUID |
| username | TEXT | Unique username |
| password_hash | TEXT | Password hash |
| created_at | TEXT | RFC3339 timestamp |

---

## Example

```text
id            8bda...
username      alice
password_hash $argon2...
created_at    2026-08-08T17:32:41Z
```

---

# Sessions

Stores active authentication sessions.

Each successful login creates a new session.

## Fields

| Column | Type | Description |
|--------|------|-------------|
| token | TEXT | Session token |
| user_id | TEXT | User UUID |
| created_at | TEXT | RFC3339 timestamp |

---

## Purpose

Session lookup is performed for every authenticated request.

Workflow:

```text
Authorization Header
        │
        ▼
Session Lookup
        │
        ▼
Authenticated User
```

---

# Filesystem Nodes

Stores the virtual filesystem.

Each record represents either:

- file
- directory

---

## Fields

| Column | Type | Description |
|--------|------|-------------|
| id | TEXT | UUID |
| parent_id | TEXT | Parent node |
| user_id | TEXT | Owner |
| name | TEXT | Node name |
| kind | TEXT | File or Directory |
| content | TEXT | File contents |

---

## Example Tree

```text
/

└── home
    ├── notes.txt
    └── projects
```

Internally this is represented through parent-child relationships.

---

# Lesson Progress

Tracks the user's overall lesson progression.

## Fields

| Column | Type | Description |
|--------|------|-------------|
| user_id | TEXT | User UUID |
| lesson_id | TEXT | Lesson identifier |
| completed | INTEGER | Completion flag |
| current | INTEGER | Current lesson flag |
| started_at | TEXT | RFC3339 timestamp |
| completed_at | TEXT | RFC3339 timestamp (nullable) |

---

## Purpose

Allows users to resume learning across sessions.

---

# Lesson Attempts

Records every lesson validation attempt.

## Fields

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER | Primary key |
| user_id | TEXT | User UUID |
| lesson_id | TEXT | Lesson identifier |
| command | TEXT | Executed command |
| successful | INTEGER | Validation result |
| attempted_at | TEXT | RFC3339 timestamp |

---

# Relationships

```text
User
 │
 ├── Sessions
 │
 ├── Filesystem Nodes
 │
 ├── Lesson Progress
 │
 └── Lesson Attempts
```

All user-owned records reference the user's UUID.

---

# Identifiers

Carbon uses UUIDs for persistent identifiers.

Examples:

```text
7dca4ec0-f2d3-4f26-98fb-5737b5a0d2d4
```

UUIDs are used for:

- users
- filesystem nodes
- session tokens

Lesson identifiers are defined by lesson content.

---

# Persistence Flow

Typical request lifecycle:

```text
HTTP Request
      │
      ▼
Route
      │
      ▼
Service
      │
      ▼
Repository
      │
      ▼
SQLite
      │
      ▼
Response
```

The database is accessed only through repositories.

---

# Design Principles

## Service-Oriented

Business logic never appears inside SQL queries.

---

## Repository-Based

Services depend on repository traits rather than SQLite directly.

---

## User Isolation

Every user-owned table includes a `user_id`.

Queries are scoped to the authenticated user.

---

## Content Separation

Educational content is not stored in the database.

Instead:

```text
content/

├── lessons/
├── docs/
├── glossary/
└── mascot/
```

SQLite stores only runtime state.

---

# Future Improvements

Potential schema additions include:

- command history
- bookmarks
- achievements
- user preferences
- lesson analytics
- audit logs
- filesystem metadata
- multiple active terminals

These additions should extend the schema without changing the existing relationships.
