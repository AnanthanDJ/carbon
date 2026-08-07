use std::path::Path;

use anyhow::Result;

use super::{Lesson, LessonLoader, LessonRegistry};

/// High-level interface for lesson content.
///
/// Lessons are immutable and loaded once during application startup.
#[derive(Debug, Clone)]
pub struct LessonService {
    registry: LessonRegistry,
}

impl LessonService {
    /// Load all lessons from disk.
    pub fn load(root: impl AsRef<Path>) -> Result<Self> {
        let lessons = LessonLoader::load(root)?;
        let registry = LessonRegistry::new(lessons);

        Ok(Self { registry })
    }

    /// Returns every lesson.
    pub fn list(&self) -> Vec<&Lesson> {
        self.registry.list()
    }

    /// Returns a lesson by ID.
    pub fn get(&self, id: &str) -> Option<&Lesson> {
        self.registry.get(id)
    }

    /// Returns the next lesson, if one exists.
    pub fn next(&self, lesson: &Lesson) -> Option<&Lesson> {
        self.registry.next_of(lesson)
    }

    /// Returns true if the lesson exists.
    pub fn contains(&self, id: &str) -> bool {
        self.registry.contains(id)
    }

    /// Number of loaded lessons.
    pub fn len(&self) -> usize {
        self.registry.len()
    }

    /// Returns true if no lessons are loaded.
    pub fn is_empty(&self) -> bool {
        self.registry.is_empty()
    }
}
