# Content Organization

> **Status:** Active
>
> **Audience:** Content Authors & Frontend Developers

---

# Overview

Educational content is stored outside the database.

Content is version-controlled alongside the application and loaded directly from the project directory.

---

# Directory Structure

```text
content/

├── lessons/
│
├── docs/
│
├── mascot/
│
├── assets/
│
└── themes/
```

---

# lessons/

Contains lesson definitions.

Each lesson is represented by a YAML file describing:

* metadata
* objectives
* validator
* rewards
* hints

Lessons are loaded by the backend during startup.

---

# docs/

Contains educational documentation for terminal commands.

Examples:

```text
echo.md

pwd.md

ls.md

mkdir.md
```

The frontend reads these files directly and renders them for the learner.

The backend does not expose documentation through an API.

---

# mascot/

Contains mascot dialogue and narrative content.

Possible content includes:

* introductions
* encouragement
* hints
* celebration messages
* contextual dialogue

The frontend is responsible for rendering mascot interactions.

---

# assets/

Contains static resources used by the frontend.

Examples include:

* illustrations
* icons
* images
* audio
* animations

---

# themes/

Contains theme definitions and visual customization resources.

Theme data is consumed by the frontend only.

---

# Backend vs Frontend Ownership

| Directory  |           Backend          |          Frontend         |
| ---------- | :------------------------: | :-----------------------: |
| `lessons/` | ✓ Loads lesson definitions | ✓ Displays lesson content |
| `docs/`    |              ✗             |    ✓ Reads and renders    |
| `mascot/`  |              ✗             |    ✓ Reads and renders    |
| `assets/`  |              ✗             |      ✓ Uses directly      |
| `themes/`  |              ✗             |    ✓ Applies UI styling   |

---

# Design Principles

The `content/` directory is intended to be:

* Human-readable
* Version-controllable
* Independent of the database
* Easy for non-backend contributors to edit
* Extensible without requiring backend changes

Only lesson definitions are interpreted by the backend. All other content is treated as static resources owned by the frontend.

