# Authentication Architecture

> **Status:** Active
>
> **Audience:** Backend Developers

---

# Overview

Authentication is handled by `AuthService`.

It is responsible for:

* registration
* login
* password verification
* JWT generation
* initial user setup

---

# Registration Flow

```text
Register

↓

Validate Request

↓

Hash Password

↓

Create User

↓

Create Root Directory

↓

Assign First Lesson

↓

Generate JWT

↓

Return Response
```

---

# Login Flow

```text
Login

↓

Lookup User

↓

Verify Password

↓

Generate JWT

↓

Return Token
```

---

# Password Storage

Passwords are never stored in plaintext.

The backend stores Argon2 password hashes.

---

# JWT

Authenticated requests include:

```text
Authorization: Bearer <token>
```

Routes requiring authentication validate the JWT before executing application logic.

---

# Responsibilities

Authentication owns:

* identity
* password verification
* token generation

It does **not** own:

* filesystem logic
* lesson progression
* command execution
