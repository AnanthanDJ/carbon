# Frontend Integration Guide

> **Status:** Active
>
> **Audience:** Frontend Developers
>
> **Applies to:** Carbon Frontend + Backend Integration

---

# Overview

Carbon follows a clear separation of responsibilities between the frontend and backend.

The backend is responsible for command execution, lesson progression, persistence, authentication, and virtual filesystem management.

The frontend is responsible for rendering the user interface and presenting backend data to the learner.

The frontend **does not implement terminal semantics**.

---

# High-Level Architecture

```text
                Browser

        ┌──────────────────────┐
        │                      │
        │      React App       │
        │                      │
        └──────────┬───────────┘
                   │
           HTTP JSON API
                   │
                   ▼
        ┌──────────────────────┐
        │                      │
        │   Carbon Backend     │
        │                      │
        └──────┬───────────────┘
               │
        ┌──────┴──────────┐
        ▼                 ▼
 Filesystem Service   Lesson Runtime
        │                 │
        └────────┬────────┘
                 ▼
              SQLite
```

---

# Responsibility Split

## Frontend

The frontend is responsible for:

* Terminal UI
* Cursor handling
* Keyboard input
* Rendering command output
* Lesson UI
* Mascot UI
* Documentation rendering
* Animations
* Progress indicators
* Theme management

The frontend should never:

* execute commands
* validate lessons
* manipulate lesson progress
* implement filesystem logic
* determine command correctness

---

## Backend

The backend is responsible for:

* Parsing commands
* Executing commands
* Managing terminal sessions
* Managing the virtual filesystem
* Lesson validation
* Lesson progression
* Authentication
* Database persistence

---

# Static Content

The frontend reads educational content directly from the project.

```text
content/

├── docs/
│
├── lessons/
│
├── mascot/
│
└── assets/
```

These files are **not served through the backend API**.

Reasons:

* Faster development
* No unnecessary API endpoints
* Easier content editing
* Backend remains focused on business logic

---

# Terminal Flow

Every command follows the same lifecycle.

```text
User types command

↓

POST /terminal

↓

Backend parses command

↓

Backend executes command

↓

Filesystem updated

↓

Lesson validated

↓

Lesson progress updated

↓

Response returned

↓

Frontend renders output
```

---

# Terminal Request

```
POST /terminal
```

Request

```json
{
    "command": "mkdir projects"
}
```

---

# Terminal Response

```json
{
    "stdout": "",
    "cwd": "/",

    "lesson": {
        "completed": true,

        "current": {
            ...
        },

        "next": {
            ...
        }
    }
}
```

The frontend should always trust the backend response.

Do not attempt to determine whether a lesson was completed.

---

# Lesson Rendering

The frontend receives lesson information from the backend.

It is responsible for displaying:

* title
* description
* objectives
* hints
* rewards
* completion state

The backend decides:

* whether the lesson passed
* when progression occurs
* what the next lesson is

---

# Filesystem

The frontend never stores filesystem state.

Whenever the user executes a command, the backend becomes the single source of truth.

Current working directory is returned after every command.

Example:

```json
{
    "cwd": "/projects"
}
```

The prompt should update accordingly.

---

# Error Handling

The backend returns human-readable error messages.

Examples:

```text
No such file or directory.

Directory not empty.

File already exists.

Unknown command.
```

The frontend should display these messages without modification.

---

# Lesson Progress

The frontend should never infer lesson completion.

Instead, it should use the returned lesson information.

Example:

```text
lesson.completed == true
```

This indicates that the backend has already:

* validated the lesson
* recorded the attempt
* updated progress
* advanced to the next lesson (if applicable)

---

# Authentication

Authentication is handled entirely by the backend.

The frontend should simply:

* log in
* store the JWT
* send the JWT with authenticated requests

Authorization decisions are never made on the frontend.

---

# Design Principles

The frontend should remain a presentation layer.

Business logic belongs in the backend.

This allows:

* consistent behaviour
* easier testing
* simpler maintenance
* platform-independent execution

---

# Current Backend Endpoints

| Method | Endpoint           | Purpose                            |
| ------ | ------------------ | ---------------------------------- |
| POST   | `/terminal`        | Execute a terminal command         |
| GET    | `/lessons`         | List all lessons                   |
| GET    | `/lessons/{id}`    | Retrieve a lesson by ID            |
| GET    | `/lessons/current` | Retrieve the user's current lesson |
| GET    | `/health`          | Server health check                |

---

# Future Integrations

The architecture is designed to support future features without changing the frontend/backend contract.

Potential additions include:

* WebSocket terminal transport
* Command history
* Terminal autocomplete
* Interactive lessons
* Rich terminal output
* Achievement system
* XP and leveling
* Multiplayer classrooms

These features should extend the existing API rather than replace it.

