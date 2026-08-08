# Repository Layer

> **Status:** Active
>
> **Audience:** Backend Developers

---

# Overview

The repository layer abstracts persistent storage from application logic.

Services depend only on repository traits.

SQLite is simply one implementation.

---

# Architecture

```text
Service

↓

Repository Trait

↓

SQLite Repository

↓

SQLite
```

---

# Current Repositories

## UserRepository

Responsible for:

* user lookup
* user creation
* authentication queries

---

## FilesystemRepository

Responsible for:

* filesystem node lookup
* child lookup
* creation
* deletion
* updates

---

## LessonRepository

Responsible for:

* current lesson
* lesson progress
* lesson attempts
* lesson completion

---

## Session Management

UserRepository is also responsible for:

* creating sessions
* looking up sessions
* authenticating session tokens

---

# Why Repository Traits?

Repository traits allow:

* easier testing
* storage independence
* clean separation of concerns
* future database implementations

The service layer never depends directly on SQL.

---

# Design Principles

Repositories should:

* perform persistence only
* avoid business logic
* expose domain-oriented methods
* remain database-specific
