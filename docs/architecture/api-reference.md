# API Reference

> **Status:** Active
>
> **Audience:** Frontend Developers
>
> **Project:** Carbon Server
>
> **Version:** v1 (Buildathon)

---

# Overview

Carbon exposes a small HTTP JSON API.

The backend is responsible for:

- authentication
- terminal execution
- virtual filesystem
- lesson progression
- persistence

The frontend is responsible only for presentation.

---

# Base URL

Development:

```
http://localhost:3000
```

---

# Authentication

Carbon currently uses **server-side session tokens**.

Successful authentication returns an opaque session token.

Example:

```json
{
    "success": true,
    "token": "8d22d3d7-a2b8-4d9b-b8aa-7f66b13db97d"
}
```

Authenticated requests must include:

```
Authorization: Bearer <token>
```

Example:

```
Authorization: Bearer 8d22d3d7-a2b8-4d9b-b8aa-7f66b13db97d
```

---

# Authentication Endpoints

## Register

```
POST /auth/register
```

Creates a new user.

### Request

```json
{
    "username": "alice",
    "password": "password123"
}
```

### Response

```json
{
    "success": true,
    "token": "<session-token>"
}
```

### Errors

| Status | Meaning |
|---------|---------|
|400|Username already exists|

---

## Login

```
POST /auth/login
```

Authenticates an existing user.

### Request

```json
{
    "username": "alice",
    "password": "password123"
}
```

### Response

```json
{
    "success": true,
    "token": "<session-token>"
}
```

### Errors

| Status | Meaning |
|---------|---------|
|401|Invalid username or password|

---

## Current User

```
GET /auth/me
```

Returns information about the authenticated user.

### Headers

```
Authorization: Bearer <token>
```

### Response

```json
{
    "id": "2d3e47d5-1df6-49cb-b8a7-fdb63d99b2dd",
    "username": "alice"
}
```

### Errors

| Status | Meaning |
|---------|---------|
|401|Missing or invalid session|

---

# Terminal

## Execute Command

```
POST /terminal
```

Executes a terminal command.

### Headers

```
Authorization: Bearer <token>
```

### Request

```json
{
    "command": "mkdir projects"
}
```

### Response

```json
{
    "stdout": "",
    "cwd": "/home",
    "lesson": {
        "completed": true,
        "current": {
            "...": "..."
        },
        "next": {
            "...": "..."
        }
    }
}
```

### Response Fields

| Field | Description |
|-------|-------------|
|stdout|Command output|
|cwd|Current working directory after execution|
|lesson|Updated lesson state (if applicable)|

### Errors

Typical failures include:

```
Unknown command
No such file or directory
File already exists
Directory not empty
Not a directory
Not a file
```

---

# Lessons

## Current Lesson

```
GET /lessons/current
```

Returns the learner's active lesson.

### Headers

```
Authorization: Bearer <token>
```

### Response

```json
{
    "...": "lesson data"
}
```

---

## List Lessons

```
GET /lessons
```

Returns every available lesson.

### Response

```json
[
    {
        "...": "..."
    }
]
```

---

## Get Lesson

```
GET /lessons/{id}
```

Returns a single lesson.

### Response

```json
{
    "...": "..."
}
```

---

# Health

## Health Check

```
GET /health
```

### Response

```json
{
    "status": "ok"
}
```

---

# Authentication Flow

```text
Register/Login
        │
        ▼
Receive session token
        │
        ▼
Store token
(localStorage or sessionStorage)
        │
        ▼
Include

Authorization: Bearer <token>

with every authenticated request
        │
        ▼
Backend authenticates request
        │
        ▼
Request executes
```

---

# HTTP Status Codes

| Code | Meaning |
|------|---------|
|200|Success|
|400|Invalid request|
|401|Authentication failed|
|404|Resource not found|
|405|Method not allowed|
|500|Internal server error|

---

# API Principles

Carbon intentionally exposes a minimal API.

The backend is the single source of truth for:

- authentication
- terminal semantics
- filesystem state
- lesson validation
- lesson progression
- persistence

The frontend should never duplicate backend logic.

---

# Future Endpoints

The current API is intentionally small.

Future additions may include:

- logout
- command history
- filesystem metadata
- achievements
- user statistics
- WebSocket terminal sessions

These additions should extend the existing API without breaking compatibility.
