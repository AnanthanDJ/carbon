# Authentication Architecture

> **Status:** Active
>
> **Audience:** Backend Developers
>
> **Project:** Carbon
>
> **Version:** v1 (Buildathon)

---

# Overview

Carbon authenticates users using **server-side session tokens**.

Unlike JWT-based authentication, session state is stored on the server. Clients receive an opaque session token after successful authentication and include it in future requests.

The authentication subsystem is responsible for:

- user registration
- password verification
- session creation
- session validation
- current user retrieval

---

# Design Goals

The authentication system is designed to be:

- simple
- deterministic
- easy to debug
- appropriate for a local educational environment

It intentionally avoids unnecessary complexity while remaining easy to replace with JWT or another authentication mechanism in the future.

---

# Components

```text
Browser
    │
    ▼
Authentication Routes
    │
    ▼
AuthService
    │
    ▼
UserRepository
    │
    ▼
SQLite
```

---

# Authentication Flow

## Registration

```
POST /auth/register
```

Request

```json
{
    "username": "alice",
    "password": "password123"
}
```

Workflow

```text
Validate request
        │
        ▼
Check username availability
        │
        ▼
Hash password
        │
        ▼
Create user
        │
        ▼
Create root filesystem
        │
        ▼
Create home directory
        │
        ▼
Generate session
        │
        ▼
Return session token
```

Successful response

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

Workflow

```text
Find user
      │
      ▼
Verify password
      │
      ▼
Generate random session token
      │
      ▼
Store session
      │
      ▼
Return token
```

Response

```json
{
    "success": true,
    "token": "<session-token>"
}
```

---

# Session Authentication

Authenticated requests include:

```
Authorization: Bearer <token>
```

Example

```
Authorization: Bearer
5aad8867-fda9-4867-b462-507be4a76bb0
```

The backend performs the following steps:

```text
Read Authorization header
        │
        ▼
Extract token
        │
        ▼
Lookup session
        │
        ▼
Retrieve user
        │
        ▼
Continue request
```

If authentication fails:

```
401 Unauthorized
```

is returned.

---

# Current User

```
GET /auth/me
```

Returns information about the authenticated user.

Example

```json
{
    "id": "7b2c74b2-66d7-4c58-89b6-0a46b04fdd6d",
    "username": "alice"
}
```

This endpoint allows the frontend to initialize user state after login.

---

# Password Storage

Passwords are never stored in plaintext.

During registration:

```text
Password
    │
    ▼
Password Hasher
    │
    ▼
Password Hash
    │
    ▼
Database
```

During login:

```text
Password
      │
      ▼
Verify Hash
      │
      ▼
Authenticated
```

---

# Session Storage

Sessions are stored in SQLite.

Each session contains:

- session token
- user identifier
- creation timestamp

The session token is the only credential sent by the client after authentication.

---

# User Filesystem

Successful registration automatically creates a personal filesystem.

Initial structure:

```text
/

└── home
```

Each user receives an isolated filesystem that is independent of every other user.

---

# Responsibilities

## Auth Routes

Responsible for:

- parsing requests
- returning HTTP responses
- status codes

---

## AuthService

Responsible for:

- registration
- login
- session creation
- authentication
- current user lookup

---

## UserRepository

Responsible for:

- user lookup
- user creation
- session creation
- session lookup

Repositories contain no business logic.

---

# Error Handling

Typical authentication failures include:

| Status | Meaning |
|---------|---------|
|400|Username already exists|
|401|Invalid username or password|
|401|Missing authentication token|
|401|Invalid session|

Internal failures return:

```
500 Internal Server Error
```

---

# Security Notes

Current implementation:

- Passwords are hashed before storage.
- Session tokens are randomly generated UUIDs.
- Clients never receive password hashes.
- Authentication state is maintained on the server.

This design is intentionally simple for the buildathon while supporting multiple concurrent users.

---

# Future Improvements

Potential future enhancements include:

- JWT authentication
- Session expiration
- Logout endpoint
- Refresh tokens
- Role-based authorization
- Rate limiting
- Multi-device session management
- OAuth providers

These additions should build on the existing authentication boundaries without changing the public API.
