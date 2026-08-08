# Terminal Architecture

> **Status:** Active
>
> **Audience:** Backend Developers

---

# Overview

The Terminal subsystem is responsible for receiving user commands, parsing them, dispatching execution, and returning results to the client.

It intentionally does **not** contain filesystem logic or lesson progression logic. Those responsibilities are delegated to dedicated services.

---

# Responsibilities

The terminal subsystem is responsible for:

* accepting terminal commands
* parsing command input
* dispatching commands
* maintaining terminal sessions
* returning command output

It is **not** responsible for:

* filesystem persistence
* lesson validation
* lesson progression
* authentication

---

# Architecture

```text
Browser

↓

POST /terminal

↓

Route

↓

Command Parser

↓

Command Dispatcher

↓

Command Implementation

↓

CommandResult

↓

Lesson Runtime

↓

Response
```

---

# Components

## TerminalSession

Represents a user's active terminal state.

Stores:

* user ID
* current working directory

The session is passed to every command execution.

---

## ParsedCommand

Represents a parsed terminal command.

Example

```text
mkdir projects
```

becomes

```text
name = "mkdir"

args = ["projects"]
```

The parser performs only lexical parsing.

It does not validate command semantics.

---

## Command Dispatcher

Receives a `ParsedCommand` and invokes the appropriate command implementation.

Example

```text
mkdir

↓

cmd_mkdir()
```

Unknown commands return an execution error.

---

## CommandResult

Represents the result of a command.

Current fields:

* stdout

Additional metadata (such as cwd and lesson state) is added by higher layers before being returned to the frontend.

---

# Supported Commands

Current implementation:

* echo
* clear
* help
* pwd
* ls
* cd
* mkdir
* touch
* cat
* rm
* rmdir

---

# Command Lifecycle

```text
Input

↓

Parser

↓

Dispatcher

↓

Filesystem

↓

CommandResult

↓

Lesson Runtime

↓

HTTP Response
```

---

# Design Principles

* Commands remain small and focused.
* Filesystem logic belongs in `FilesystemService`.
* Lesson progression belongs in `LessonRuntimeService`.
* Parser performs no business logic.
* Dispatcher performs no persistence.

---

# Future Work

Potential additions:

* command history
* aliases
* autocomplete
* piping
* redirection
* shell variables
* interactive commands
* WebSocket transport
