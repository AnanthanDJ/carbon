# Filesystem Architecture

> **Status:** Active
>
> **Audience:** Backend Developers
>
> **Project:** Carbon
>
> **Version:** v1 (Buildathon)

---

# Overview

Carbon provides every user with an independent virtual filesystem.

The filesystem behaves similarly to a Unix filesystem while remaining completely isolated from the host operating system.

All filesystem operations are performed against data stored in SQLite.

---

# Goals

The filesystem is designed to:

- mimic basic Unix filesystem behavior
- isolate users from one another
- remain deterministic
- support lesson validation
- avoid interacting with the host machine

No command executed inside Carbon can modify the developer's real filesystem.

---

# High-Level Architecture

```text
Terminal Command
        │
        ▼
Command Dispatcher
        │
        ▼
Filesystem Service
        │
        ▼
Filesystem Repository
        │
        ▼
SQLite
```

Business logic lives inside `FilesystemService`.

Persistence lives inside `FilesystemRepository`.

---

# Filesystem Structure

Each registered user automatically receives a filesystem.

Initial structure:

```text
/

└── home
```

Each user owns their own tree.

Example:

```text
User A

/

└── home
    ├── notes
    └── projects


User B

/

└── home
    ├── downloads
    └── practice
```

Operations performed by one user never affect another.

---

# Storage Model

Filesystem nodes are persisted in SQLite.

Each node stores:

- unique identifier
- owner (user ID)
- parent identifier
- name
- node type
- file contents (for files)

The filesystem is reconstructed from these records.

---

# Node Types

Carbon currently supports two node types.

## Directory

Directories may contain child nodes.

Example

```text
projects/
```

---

## File

Files store text content.

Example

```text
notes.txt
```

---

# Filesystem Service

The service layer implements all filesystem behavior.

Responsibilities include:

- path resolution
- directory navigation
- file creation
- directory creation
- deletion
- file reading
- validation

Services do not communicate directly with SQLite.

---

# Repository Layer

The repository is responsible only for persistence.

Typical operations include:

- create node
- update node
- delete node
- lookup node
- lookup children

Repositories never contain filesystem rules.

---

# Path Resolution

Commands may use:

```text
.
..
/
relative paths
absolute paths
```

The filesystem service resolves these paths before executing an operation.

Example

```text
cd ../projects
```

↓

```text
/home/projects
```

The frontend never performs path resolution.

---

# Current Working Directory

Each terminal session maintains a current working directory.

Example

```text
/home
```

After every successful command, the backend returns the updated working directory.

Example response

```json
{
    "cwd": "/home/projects"
}
```

The frontend should always display this value.

---

# Supported Operations

Current implementation supports:

- pwd
- ls
- cd
- mkdir
- touch
- cat
- rm
- rmdir

Additional commands can be added without changing the filesystem architecture.

---

# Command Lifecycle

```text
User Command
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
Repository
        │
        ▼
SQLite
        │
        ▼
Command Result
```

Lesson validation occurs after filesystem execution.

---

# Isolation

Every filesystem query is scoped to the authenticated user.

Conceptually:

```text
User ID
    │
    ▼
Filesystem Query
    │
    ▼
Only that user's nodes
```

This prevents users from accessing or modifying another user's files.

---

# Error Handling

Common filesystem errors include:

```text
No such file or directory

File already exists

Directory already exists

Directory not empty

Not a directory

Not a file
```

These messages are returned to the terminal unchanged.

---

# Design Principles

The filesystem follows several principles.

## Virtual

The host filesystem is never modified.

---

## User Isolation

Every user has an independent filesystem.

---

## Service-Oriented

Filesystem behavior belongs in `FilesystemService`.

Persistence belongs in `FilesystemRepository`.

---

## Deterministic

Given the same filesystem state and command, the result should always be identical.

---

## Unix-Inspired

Carbon intentionally mirrors familiar Unix filesystem behavior wherever practical.

---

# Future Improvements

Potential future additions include:

- move (`mv`)
- copy (`cp`)
- symbolic links
- recursive deletion
- file permissions
- hidden files
- timestamps
- metadata
- search
- command history integration

These enhancements should extend the current filesystem model without changing its core architecture.
