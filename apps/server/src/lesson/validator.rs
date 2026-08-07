use crate::terminal::TerminalSession;

use super::{Lesson, Validator};

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub passed: bool,
    pub message: Option<String>,
}

impl ValidationResult {
    pub fn success() -> Self {
        Self {
            passed: true,
            message: None,
        }
    }

    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            passed: false,
            message: Some(message.into()),
        }
    }
}

pub struct LessonValidator;

impl LessonValidator {
    pub fn validate(
        lesson: &Lesson,
        _session: &TerminalSession,
        command: &str,
        output: &str,
    ) -> ValidationResult {
        match &lesson.mission.validator {
            Validator::ExactCommand { command: expected } => {
                if command.trim() == expected.trim() {
                    ValidationResult::success()
                } else {
                    ValidationResult::failure(format!("Expected command '{}'.", expected))
                }
            }

            Validator::ExpectedOutput { output: expected } => {
                if output.trim() == expected.trim() {
                    ValidationResult::success()
                } else {
                    ValidationResult::failure("Output did not match expected output.")
                }
            }

            Validator::CurrentDirectory { .. }
            | Validator::FileExists { .. }
            | Validator::DirectoryExists { .. }
            | Validator::FileContains { .. } => {
                ValidationResult::failure("Validator not implemented.")
            }
        }
    }
}
