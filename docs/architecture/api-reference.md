# API Reference

> **Status:** Active
>
> **Audience:** Frontend Developers
>
> **Base URL**
>
> ```
> http://localhost:3000
> ```

---

# Overview

The Carbon backend exposes a small REST API responsible for terminal execution, lesson progression, authentication, and application state.

All responses are JSON.

The backend is considered the single source of truth for:

* terminal execution
* virtual filesystem
* lesson progression
* user authentication
* persistent state

---

# Authentication

Authenticated endpoints require a JWT.

```
Authorization: Bearer <token>
```

Unauthenticated requests should receive:

```http
401 Unauthorized
```

---

# POST /terminal

Executes a terminal command inside the user's virtual terminal session.

---

## Request

```json
{
    "command": "mkdir projects"
}
```

---

## Successful Response

```json
{
    "stdout": "",

    "cwd": "/",

    "lesson": {
        "completed": true,

        "current": {
            "id": "c1l1",
            "title": "Hello Terminal"
        },

        "next": {
            "id": "c1l2",
            "title": "pwd"
        }
    }
}
```

---

## Validation Failure

```json
{
    "stdout": "",

    "cwd": "/",

    "lesson": {
        "completed": false,

        "current": {
            "id": "c1l1",
            "title": "Hello Terminal"
        },

        "next": null
    }
}
```

---

## Command Error

```json
{
    "stdout": "No such file or directory.",

    "cwd": "/projects",

    "lesson": null
}
```

The command failed before lesson validation.

---

# GET /lessons

Returns every lesson loaded by the backend.

---

## Response

```json
[
    {
        "id": "c1l1",
        "title": "Hello Terminal",
        "description": "...",
        "mission": {
            ...
        }
    },

    {
        "id": "c1l2",
        "title": "pwd",
        "description": "...",
        "mission": {
            ...
        }
    }
]
```

---

# GET /lessons/{id}

Returns a single lesson.

---

## Example

```
GET /lessons/c1l1
```

---

## Response

```json
{
    "id": "c1l1",
    "title": "Hello Terminal",
    "description": "...",

    "mission": {
        ...
    }
}
```

---

## Not Found

```http
404 Not Found
```

---

# GET /lessons/current

Returns the user's currently active lesson.

---

## Response

```json
{
    "id": "c1l4",

    "title": "mkdir",

    "description": "...",

    "mission": {
        ...
    }
}
```

---

# GET /health

Simple health endpoint.

Useful for frontend startup checks.

---

## Response

```json
{
    "status": "ok"
}
```

---

# Terminal Lifecycle

Every command follows the same sequence.

```text
Frontend

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

Attempt recorded

↓

Lesson progressed

↓

JSON response

↓

Frontend renders response
```

---

# Response Fields

## stdout

Human-readable command output.

Example

```json
{
    "stdout": "hello world"
}
```

---

## cwd

Current working directory after command execution.

Example

```json
{
    "cwd": "/projects"
}
```

Always update the terminal prompt using this value.

---

## lesson

Present only when lesson evaluation occurs.

Fields:

| Field     | Description                              |
| --------- | ---------------------------------------- |
| completed | Whether the current lesson was completed |
| current   | Current lesson information               |
| next      | Next lesson if progression occurred      |

---

# HTTP Status Codes

| Code | Meaning                              |
| ---- | ------------------------------------ |
| 200  | Request completed successfully       |
| 400  | Invalid request or malformed command |
| 401  | Authentication required              |
| 404  | Requested resource not found         |
| 500  | Internal server error                |

---

# Error Handling

The backend returns human-readable messages whenever possible.

Examples:

```
Unknown command.

No such file or directory.

Directory already exists.

File already exists.

Directory not empty.

Cannot remove directory with rm.

Cannot read a directory.
```

The frontend should display these messages directly.

---

# Frontend Responsibilities

The frontend should:

* execute terminal requests
* display stdout
* update cwd
* update lesson UI
* render lesson content
* render documentation
* maintain terminal history

The frontend should **not**:

* validate commands
* determine lesson completion
* manipulate filesystem state
* update lesson progress
* infer terminal semantics

---

# API Stability

The API is designed around stable contracts.

Future additions may introduce new endpoints or response fields, but existing endpoint behavior should remain backwards compatible whenever possible.

