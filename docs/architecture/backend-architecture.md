# Backend Architecture

> **Status:** Active
>
> **Audience:** Backend Developers
>
> **Project:** Carbon
>
> **Version:** v1 (Buildathon)

---

# Overview

The Carbon backend provides the application's business logic.

It is responsible for:

- Authentication
- Session management
- Virtual filesystem
- Terminal execution
- Lesson runtime
- Persistence

The backend intentionally contains **all application logic** while remaining independent of the frontend.

---

# High-Level Architecture

```text
                    HTTP API
                        │
                Axum Route Handlers
                        │
                        ▼
                  Service Layer
        ┌────────────┼────────────┐
        ▼            ▼            ▼
 Authentication  Filesystem   Lesson Runtime
        │            │            │
        └────────────┼────────────┘
                     ▼
             Repository Layer
                     │
                     ▼
                  SQLite
```

---

# Project Structure

```
apps/server/

├── app/
│   ├── config.rs
│   ├── state.rs
│   └── mod.rs
│
├── auth/
│
├── filesystem/
│
├── lesson/
│
├── repository/
│
├── routes/
│
├── models/
│
└── main.rs
```

Each module has a single responsibility.

---

# Request Lifecycle

Every request follows the same pipeline.

```text
HTTP Request
      │
      ▼
Route Handler
      │
      ▼
Authentication (if required)
      │
      ▼
Service Layer
      │
      ▼
Repository Layer
      │
      ▼
SQLite
      │
      ▼
JSON Response
```

Business logic never exists inside route handlers.

---

# AppState

Application-wide services are created once during startup and shared through `AppState`.

Current services include:

- Configuration
- SQLite pool
- Authentication service
- Filesystem service
- Lesson service
- Lesson runtime service

Routes access these services through Axum's shared state.

---

# Service Layer

The service layer contains all business logic.

Current services include:

## Authentication Service

Responsible for:

- user registration
- login
- session creation
- authentication
- current user lookup

---

## Filesystem Service

Responsible for:

- path resolution
- file creation
- directory creation
- deletion
- navigation
- file reading

The filesystem is completely virtual and isolated per user.

---

## Lesson Service

Responsible for:

- loading lesson definitions
- indexing lessons
- exposing lesson metadata

Lesson definitions are loaded during application startup.

---

## Lesson Runtime Service

Responsible for:

- validating commands
- tracking attempts
- recording progress
- advancing lessons
- interacting with the filesystem

The runtime is executed after terminal commands whenever lesson validation is required.

---

# Repository Layer

Repositories abstract database access.

Services never communicate with SQLite directly.

Current repositories include:

```
UserRepository

FilesystemRepository

LessonRepository
```

SQLite implementations satisfy these interfaces.

This separation allows services to remain storage-independent.

---

# Authentication

Carbon currently uses **server-side session tokens**.

Workflow:

```text
Register/Login
        │
        ▼
Session created
        │
        ▼
Random token generated
        │
        ▼
Stored in SQLite
        │
        ▼
Returned to client
```

Authenticated requests include:

```
Authorization: Bearer <token>
```

The backend authenticates every protected request before executing business logic.

---

# Terminal Execution

Terminal commands are processed through a dedicated execution pipeline.

```text
Command
    │
    ▼
Parser
    │
    ▼
Dispatcher
    │
    ▼
Filesystem Service
    │
    ▼
Lesson Runtime
    │
    ▼
Terminal Response
```

Each command executes against the authenticated user's virtual filesystem.

---

# Virtual Filesystem

Every registered user receives an isolated filesystem.

Example:

```
/

└── home
```

Operations performed by one user never affect another user's filesystem.

Filesystem state is persisted in SQLite.

---

# Lesson Runtime

Lesson progression is backend-controlled.

After each terminal command:

```text
Command executed
        │
        ▼
Lesson validation
        │
        ▼
Attempt recorded
        │
        ▼
Progress updated
        │
        ▼
Next lesson unlocked
```

The frontend simply renders the returned lesson state.

---

# Content

Educational content is separated from business logic.

```
content/

├── lessons/
├── docs/
├── glossary/
└── mascot/
```

Current responsibilities:

| Directory | Consumer |
|-----------|----------|
| lessons | Backend |
| docs | Frontend |
| glossary | Frontend |
| mascot | Frontend |

This keeps educational content editable without modifying backend code.

---

# Database

SQLite stores:

- users
- sessions
- filesystem nodes
- lesson progress
- lesson attempts

Repositories are responsible for translating between database rows and domain models.

---

# Error Handling

Services return domain errors.

Routes convert these into appropriate HTTP status codes.

Typical responses include:

- 400 Bad Request
- 401 Unauthorized
- 404 Not Found
- 500 Internal Server Error

This keeps HTTP concerns separate from business logic.

---

# Design Principles

The backend follows several architectural principles.

## Separation of Concerns

Routes handle HTTP.

Services contain business logic.

Repositories handle persistence.

---

## Dependency Inversion

Business logic depends on repository traits rather than concrete SQLite implementations.

---

## Single Responsibility

Each module has one clearly defined responsibility.

---

## Stateless HTTP

Authentication state is stored in server-side sessions.

Each request contains the information required for authentication.

---

## Content-Driven Learning

Lessons are defined as content rather than compiled into application logic.

This allows lesson updates without modifying the execution engine.

---

# Future Improvements

Potential future enhancements include:

- JWT authentication
- Logout endpoint
- Session expiration
- Middleware-based authentication
- Command history
- WebSocket terminal sessions
- Multi-device session management
- Additional repository implementations

These improvements should extend the current architecture without changing the existing service boundaries.
