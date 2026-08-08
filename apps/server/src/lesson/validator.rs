use crate::{
    filesystem::FilesystemService,
    models::NodeKind,
    repository::FilesystemRepository,
    terminal::{CommandResult, ParsedCommand, TerminalSession},
};

use super::{Lesson, Validator};

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub passed: bool,
    pub message: Option<String>,
    pub completed: bool,
}

impl ValidationResult {
    pub fn success() -> Self {
        Self {
            passed: true,
            message: None,
            completed: true,
        }
    }

    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            passed: false,
            message: Some(message.into()),
            completed: false,
        }
    }
}

pub struct LessonValidator;

impl LessonValidator {
    pub async fn validate<R>(
        lesson: &Lesson,
        filesystem: &FilesystemService<R>,
        session: &TerminalSession,
        command: &ParsedCommand,
        result: &CommandResult,
    ) -> ValidationResult
    where
        R: FilesystemRepository,
    {
        match &lesson.mission.validator {
            Validator::ExactCommand { command: expected } => {
                let mut actual = command.name.clone();

                if !command.args.is_empty() {
                    actual.push(' ');
                    actual.push_str(&command.args.join(" "));
                }

                if actual.trim() == expected.trim() {
                    ValidationResult::success()
                } else {
                    ValidationResult::failure(format!("Expected '{}', got '{}'.", expected, actual))
                }
            }

            Validator::ExpectedOutput { output: expected } => {
                if result.stdout.trim() == expected.trim() {
                    ValidationResult::success()
                } else {
                    ValidationResult::failure(format!(
                        "Expected output:\n{}\n\nGot:\n{}",
                        expected,
                        result.stdout.trim()
                    ))
                }
            }

            Validator::CurrentDirectory { path } => {
                if session.cwd.as_path() == *path {
                    ValidationResult::success()
                } else {
                    ValidationResult::failure(format!(
                        "Expected current directory '{}', got '{}'.",
                        path,
                        session.cwd.as_path()
                    ))
                }
            }

            Validator::FileExists { path } => match filesystem.resolve_node(session, path).await {
                Ok(node) => {
                    if matches!(node.kind, NodeKind::File) {
                        ValidationResult::success()
                    } else {
                        ValidationResult::failure(format!("'{}' exists but is not a file.", path))
                    }
                }

                Err(_) => ValidationResult::failure(format!("Expected file '{}' to exist.", path)),
            },

            Validator::DirectoryExists { path } => {
                match filesystem.resolve_node(session, path).await {
                    Ok(node) => {
                        if matches!(node.kind, NodeKind::Directory) {
                            ValidationResult::success()
                        } else {
                            ValidationResult::failure(format!(
                                "'{}' exists but is not a directory.",
                                path
                            ))
                        }
                    }

                    Err(_) => ValidationResult::failure(format!(
                        "Expected directory '{}' to exist.",
                        path
                    )),
                }
            }

            Validator::FileContains { path, text } => match filesystem.cat(session, path).await {
                Ok(contents) => {
                    if contents.contains(text) {
                        ValidationResult::success()
                    } else {
                        ValidationResult::failure(format!(
                            "File '{}' does not contain '{}'.",
                            path, text
                        ))
                    }
                }

                Err(e) => ValidationResult::failure(e.to_string()),
            },
        }
    }
}
