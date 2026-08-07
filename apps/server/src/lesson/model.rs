use serde::{Deserialize, Serialize};

/// A single lesson loaded from `content/lessons/**/*.yaml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    /// Stable lesson identifier (e.g. "c1l1").
    pub id: String,

    /// Lesson title displayed to the learner.
    pub title: String,

    /// Difficulty level.
    pub difficulty: Difficulty,

    /// Estimated completion time in minutes.
    pub estimated_minutes: u32,

    /// Story shown before the lesson begins.
    pub story: String,

    /// The mission the learner must complete.
    pub mission: Mission,

    /// Educational explanation shown alongside the lesson.
    pub explanation: String,

    /// Example command and output.
    pub example: Example,

    /// Key takeaways.
    pub learning_points: Vec<String>,

    /// Message shown after successful completion.
    pub victory_message: String,

    /// Reward granted after completion.
    pub reward: Reward,

    /// ID of the next lesson, if any.
    pub next: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mission {
    pub title: String,
    pub description: String,
    pub objective: Objective,
    pub validator: Validator,

    #[serde(default)]
    pub hints: Vec<String>,

    #[serde(default)]
    pub common_mistakes: Vec<CommonMistake>,

    pub challenge: Option<Challenge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objective {
    pub concept: String,
    pub command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    pub command: String,
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonMistake {
    pub mistake: String,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Challenge {
    pub description: String,
    pub example: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    pub xp: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Validator {
    ExactCommand { command: String },

    ExpectedOutput { output: String },

    CurrentDirectory { path: String },

    FileExists { path: String },

    DirectoryExists { path: String },

    FileContains { path: String, text: String },
}
