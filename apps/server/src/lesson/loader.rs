use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use walkdir::WalkDir;

use super::Lesson;

/// Loads all lessons from the content directory.
pub struct LessonLoader;

impl LessonLoader {
    /// Recursively load every `.yaml` lesson file under `root`.
    pub fn load(root: impl AsRef<Path>) -> Result<HashMap<String, Lesson>> {
        let mut lessons = HashMap::new();

        for entry in WalkDir::new(root) {
            let entry = entry?;

            if !entry.file_type().is_file() {
                continue;
            }

            if entry.path().extension().and_then(|e| e.to_str()) != Some("yaml") {
                continue;
            }

            let lesson = Self::load_file(entry.path())?;

            if lessons.insert(lesson.id.clone(), lesson).is_some() {
                bail!("Duplicate lesson id");
            }
        }

        Ok(lessons)
    }

    fn load_file(path: &Path) -> Result<Lesson> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        let lesson: Lesson = serde_yaml::from_str(&contents)
            .with_context(|| format!("Failed to parse {}", path.display()))?;

        Ok(lesson)
    }
}
