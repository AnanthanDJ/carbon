# Filesystem Architecture

> **Status:** Active
>
> **Audience:** Backend Developers

---

# Overview

Carbon implements a virtual Unix-like filesystem for every user.

All filesystem operations are performed through `FilesystemService`.

The service is the single source of truth for path resolution and filesystem manipulation.

---

# Architecture

```text
Terminal Command

↓

FilesystemService

↓

FilesystemRepository

↓

SQLite
```

---

# Path Resolution

Every path is converted into a `ResolvedPath`.

Supported syntax:

```text
/

.

..

relative/path

absolute/path
```

Examples

```text
cwd = /home

notes.txt

↓

/home/notes.txt
```

```text
cd ..

↓

parent directory
```

---

# Filesystem Operations

Current operations:

* pwd
* ls
* cd
* mkdir
* touch
* cat
* rm
* rmdir

Each operation validates the requested action before updating the repository.

---

# Node Model

Every filesystem object is represented by a `FilesystemNode`.

A node may be:

* file
* directory

Directories have no content.

Files store textual content.

---

# Repository

Filesystem persistence is abstracted through `FilesystemRepository`.

Implementations are responsible for:

* node lookup
* child lookup
* creation
* deletion
* updates

Business logic remains inside `FilesystemService`.

---

# Design Principles

* Unix-inspired behavior
* per-user isolation
* normalized path resolution
* repository abstraction
* storage independence
