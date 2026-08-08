# Backend Architecture

> **Status:** Active
>
> **Audience:** Backend Developers
>
> **Project:** Carbon Server

---

# Overview

The Carbon backend is responsible for executing terminal commands, managing the virtual filesystem, validating lessons, tracking user progression, and persisting application state.

The backend intentionally contains **all business logic**, while the frontend acts purely as a presentation layer.

The architecture follows a layered design centered around services and repository abstractions, allowing core logic to remain independent of the underlying database.

---

# High-Level Architecture

```text
                    HTTP Requests
                          │
                          ▼
                     Axum Router
                          │
                          ▼
                      Route Layer
                          │
                          ▼
                    Service Layer
          ┌───────────┼─────────────┐
          ▼           ▼             ▼
     AuthService FilesystemService LessonRuntimeService
                          │
                          ▼
                  Repository Traits
                          │
                          ▼
               SQLite Repository Layer
                          │
                          ▼
                       SQLite
```

---

# Project Structure

```text
apps/server
│
├── app/
│   ├── config.rs
│   ├── router.rs
│   ├── state.rs
│   └── startup.rs
│
├── auth/
│
├── filesystem/
│
├── lesson/
│
├── models/
│
├── repository/
│
├── routes/
│
├── terminal/
│
└── main.rs
```

---

# Architectural Layers

## Route Layer

The route layer exposes the HTTP API.

Responsibilities:

* deserialize requests
* invoke services
* serialize responses
* return HTTP status codes

Routes should contain minimal business logic.

---

## Service Layer

The service layer contains the application's business logic.

Current services include:

* AuthService
* FilesystemService
* LessonService
* LessonRuntimeService

Services coordinate repositories but remain independent of SQLite.

---

## Repository Layer

Repositories abstract persistence.

Business logic communicates only with repository traits.

Example:

```text
FilesystemService

↓

FilesystemRepository

↓

SqliteFilesystemRepository
```

This separation allows storage implementations to be replaced without affecting application logic.

---

## Model Layer

Models represent the application's core domain objects.

Examples include:

* User
* FilesystemNode
* LessonProgress
* LessonAttempt
* TerminalSession

Models remain storage-agnostic whenever possible.

---

# Major Components

## Authentication

Responsible for:

* registration
* login
* password hashing
* JWT generation
* user initialization

Passwords are stored using Argon2.

---

## Terminal

The terminal subsystem handles:

* command parsing
* command dispatch
* session management
* command execution

The terminal does **not** determine lesson progression.

---

## Virtual Filesystem

The virtual filesystem provides an isolated Unix-like environment for each user.

Supported operations include:

* pwd
* ls
* cd
* mkdir
* touch
* cat
* rm
* rmdir

Filesystem state is stored in SQLite.

---

## Lesson System

Lessons are loaded from the filesystem during startup.

```text
content/lessons

↓

LessonLoader

↓

LessonRegistry

↓

LessonService
```

Lessons remain immutable during runtime.

---

## Lesson Runtime

The runtime validates learner actions after command execution.

Responsibilities:

* retrieve current lesson
* validate command
* record attempt
* update progress
* advance learner

The runtime never executes commands itself.

---

# Request Lifecycle

A typical terminal request follows this sequence.

```text
Client

↓

POST /terminal

↓

Route

↓

Command Parser

↓

Command Dispatcher

↓

Filesystem Service

↓

Lesson Runtime

↓

Repositories

↓

SQLite

↓

Response
```

---

# Dependency Flow

Dependencies flow in one direction.

```text
Routes

↓

Services

↓

Repositories

↓

SQLite
```

Lower layers never depend on higher layers.

For example:

* repositories never call services
* services never call routes
* models never depend on Axum

This keeps the architecture modular and testable.

---

# State Management

Application-wide state is stored in `AppState`.

Current state includes:

* configuration
* database pool
* authentication service
* filesystem service
* lesson service
* lesson runtime

All services are initialized once during application startup and shared across requests.

---

# Content Loading

Static educational content is loaded during startup.

```text
content/

├── lessons/

├── docs/

├── mascot/

└── assets/
```

Only lesson definitions are consumed by the backend.

Documentation and UI assets are intended for direct frontend consumption.

---

# Error Handling

Errors propagate through the service layer using `anyhow::Result`.

Domain-specific failures (such as invalid paths or missing files) are represented by custom error types and converted into appropriate HTTP responses.

---

# Design Principles

The backend follows several architectural principles:

### Separation of Concerns

Each subsystem has a clearly defined responsibility.

### Repository Pattern

Persistence remains independent of business logic.

### Immutable Lesson Content

Lesson definitions are loaded once and never modified at runtime.

### Stateless HTTP Layer

Routes perform request handling only.

Application state lives within services and the database.

### Backend as Source of Truth

The backend owns:

* terminal semantics
* filesystem state
* lesson progression
* validation
* persistence

The frontend renders backend state without duplicating business logic.

---

# Future Evolution

The architecture is designed to accommodate future enhancements without significant restructuring.

Potential additions include:

* WebSocket terminal sessions
* Command history
* Terminal autocompletion
* Achievement system
* Leaderboards
* Multiplayer classrooms
* Additional repository implementations (e.g., PostgreSQL)
* Filesystem snapshots
* Plugin-based command extensions

These features can be added by extending the service layer while preserving the existing architecture and API contracts.

