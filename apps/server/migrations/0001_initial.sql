PRAGMA foreign_keys = ON;

CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE filesystem_nodes (
    id TEXT PRIMARY KEY,
    parent_id TEXT,
    user_id TEXT NOT NULL,

    name TEXT NOT NULL,
    kind TEXT NOT NULL CHECK(kind IN ('file', 'directory')),
    content TEXT,

    FOREIGN KEY(parent_id)
        REFERENCES filesystem_nodes(id)
        ON DELETE CASCADE,

    FOREIGN KEY(user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);

CREATE TABLE lesson_progress (
    user_id TEXT NOT NULL,
    lesson_id TEXT NOT NULL,

    completed INTEGER NOT NULL DEFAULT 0,

    PRIMARY KEY(user_id, lesson_id),

    FOREIGN KEY(user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);

CREATE TABLE lesson_attempts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    user_id TEXT NOT NULL,
    lesson_id TEXT NOT NULL,

    command TEXT NOT NULL,
    successful INTEGER NOT NULL,

    created_at TEXT NOT NULL,

    FOREIGN KEY(user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);

CREATE INDEX idx_filesystem_parent
ON filesystem_nodes(parent_id);

CREATE INDEX idx_filesystem_user
ON filesystem_nodes(user_id);

CREATE INDEX idx_attempt_user
ON lesson_attempts(user_id);

CREATE INDEX idx_attempt_lesson
ON lesson_attempts(lesson_id);
