# Frontend Integration Guide

> **Status:** Active
>
> **Audience:** Frontend Developers
>
> **Project:** Carbon
>
> **Version:** v1 (Buildathon)

---

# Overview

Carbon follows a strict separation between the frontend and backend.

The backend owns all application logic.

The frontend is responsible only for rendering the user interface and communicating with the backend through the HTTP API.

The frontend must never duplicate backend logic.

---

# Responsibility Split

## Frontend Responsibilities

The frontend is responsible for:

- Authentication UI
- Hero page
- Terminal interface
- Cursor handling
- Keyboard input
- Rendering command output
- Lesson interface
- Documentation viewer
- Mascot interface
- Progress indicators
- Theme management
- Navigation
- Animations

The frontend should **never**:

- execute terminal commands
- parse commands
- validate lessons
- determine lesson completion
- manage filesystem state
- modify lesson progression
- interpret terminal semantics

---

## Backend Responsibilities

The backend is responsible for:

- Authentication
- Session management
- Command parsing
- Command execution
- Virtual filesystem
- Lesson validation
- Lesson progression
- Persistence
- Business logic

The backend is the single source of truth.

---

# High-Level Architecture

```text
                Browser

        React + TypeScript
               │
               ▼
        HTTP JSON API
               │
               ▼
        Carbon Backend
               │
     ┌─────────┴─────────┐
     ▼                   ▼
Filesystem         Lesson Runtime
     │                   │
     └─────────┬─────────┘
               ▼
            SQLite
```

---

# Authentication Flow

Authentication uses **server-side session tokens**.

## Register

```
POST /auth/register
```

↓

Returns

```json
{
    "success": true,
    "token": "<session-token>"
}
```

---

## Login

```
POST /auth/login
```

↓

Returns

```json
{
    "success": true,
    "token": "<session-token>"
}
```

---

## Store Token

After login or registration, store the returned session token.

Example:

```text
localStorage["token"] = token
```

or

```text
sessionStorage["token"] = token
```

The frontend decides which storage strategy is appropriate.

---

## Authenticated Requests

Every authenticated request must include:

```
Authorization: Bearer <token>
```

Example:

```
Authorization: Bearer
8d22d3d7-a2b8-4d9b-b8aa-7f66b13db97d
```

---

## Current User

Immediately after authentication, fetch:

```
GET /auth/me
```

The response contains the authenticated user.

Example:

```json
{
    "id": "...",
    "username": "alice"
}
```

The frontend should use this information wherever the current user is displayed.

---

# Terminal Flow

Every terminal command follows the same lifecycle.

```text
User types command
        │
        ▼
POST /terminal
        │
        ▼
Backend parses command
        │
        ▼
Command executed
        │
        ▼
Filesystem updated
        │
        ▼
Lesson validated
        │
        ▼
Progress updated
        │
        ▼
Response returned
        │
        ▼
Frontend updates UI
```

---

# Terminal Request

```
POST /terminal
```

Headers

```
Authorization: Bearer <token>
```

Body

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
    "cwd": "/home",
    "lesson": {
        "completed": true,
        "current": { },
        "next": { }
    }
}
```

The frontend should always trust the backend response.

---

# Current Working Directory

The backend returns the updated working directory after every command.

Example:

```json
{
    "cwd": "/projects"
}
```

The terminal prompt should always display this value.

Never attempt to compute the current directory on the frontend.

---

# Lesson Progress

Lesson progression is completely backend-driven.

After every terminal command, the backend may return updated lesson information.

Example:

```json
{
    "lesson": {
        "completed": true,
        "current": { ... },
        "next": { ... }
    }
}
```

The frontend should:

- display completion
- update lesson UI
- transition to the next lesson

The frontend should never determine whether a lesson passed.

---

# Documentation

Educational documentation is **not** served through the backend.

Instead, the frontend reads documentation directly from:

```text
content/docs/
```

Advantages:

- no unnecessary API
- easier editing
- instant updates during development
- backend remains focused on business logic

---

# Lesson Content

Lesson definitions are also stored in:

```text
content/lessons/
```

The backend loads these files during startup.

The frontend should treat lesson files as content only and rely on the backend for runtime state.

---

# Glossary

Beginner terminology is located in:

```text
content/glossary/
```

The frontend may load glossary entries directly.

---

# Mascot

Mux dialogue is stored under:

```text
content/mascot/
```

The mascot is entirely frontend-driven.

The backend has no knowledge of mascot conversations.

---

# Theme System

Themes are entirely a frontend concern.

The backend should never receive theme information.

Changing themes must not affect API behavior.

---

# Error Handling

Display backend error messages exactly as returned.

Examples include:

```text
Unknown command

No such file or directory

Directory not empty

File already exists

Not a directory

Not a file
```

Avoid rewriting backend messages.

---

# Initial Application Flow

```text
Open application
        │
        ▼
Display hero page
        │
        ▼
Register or Login
        │
        ▼
Store session token
        │
        ▼
GET /auth/me
        │
        ▼
Load terminal interface
        │
        ▼
GET /lessons/current
        │
        ▼
Load lesson UI
        │
        ▼
User begins learning
```

---

# Backend Endpoints

| Method | Endpoint | Purpose |
|---------|----------|---------|
| POST | `/auth/register` | Register a user |
| POST | `/auth/login` | Login |
| GET | `/auth/me` | Current authenticated user |
| POST | `/terminal` | Execute terminal command |
| GET | `/lessons` | List lessons |
| GET | `/lessons/{id}` | Retrieve lesson |
| GET | `/lessons/current` | Current lesson |
| GET | `/health` | Health check |

---

# Design Principles

The frontend should remain a presentation layer.

The backend owns:

- authentication
- terminal semantics
- virtual filesystem
- lesson validation
- lesson progression
- persistence

The frontend owns:

- user experience
- rendering
- interaction
- accessibility
- animation
- presentation

Maintaining this separation keeps the architecture modular, predictable, and easy to evolve.

---

# Future Improvements

The current architecture is designed to support future enhancements without changing the frontend/backend contract.

Potential future additions include:

- logout endpoint
- WebSocket terminal sessions
- command history
- terminal autocomplete
- achievements
- XP and progression
- multiplayer classrooms
- plugin support

These features should extend the existing API rather than replace it.
