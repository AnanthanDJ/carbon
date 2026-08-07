use crate::terminal::{CommandResult, ParsedCommand, TerminalSession};

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
    pub fn validate(
        lesson: &Lesson,
        session: &TerminalSession,
        command: &ParsedCommand,
        result: &CommandResult,
    ) -> ValidationResult {
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

            Validator::FileExists { .. } => {
                ValidationResult::failure("FileExists validator not implemented.")
            }

            Validator::DirectoryExists { .. } => {
                ValidationResult::failure("DirectoryExists validator not implemented.")
            }

            Validator::FileContains { .. } => {
                ValidationResult::failure("FileContains validator not implemented.")
            }
        }
    }
}
