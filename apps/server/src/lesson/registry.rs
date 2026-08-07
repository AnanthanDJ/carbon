use std::collections::HashMap;

use anyhow::{Result, bail};

use super::{Lesson, LessonLoader};

/// In-memory registry of all loaded lessons.
///
/// This is loaded once at application startup and treated as immutable.
#[derive(Debug, Clone)]
pub struct LessonRegistry {
    lessons: HashMap<String, Lesson>,
}

impl LessonRegistry {
    /// Construct a registry from an existing lesson map.
    pub fn new(lessons: HashMap<String, Lesson>) -> Self {
        Self { lessons }
    }

    /// Get a lesson by ID.
    pub fn get(&self, id: &str) -> Option<&Lesson> {
        self.lessons.get(id)
    }

    /// Returns true if the lesson exists.
    pub fn contains(&self, id: &str) -> bool {
        self.lessons.contains_key(id)
    }

    /// Return all lessons.
    pub fn list(&self) -> Vec<&Lesson> {
        let mut lessons: Vec<_> = self.lessons.values().collect();

        // Stable ordering.
        lessons.sort_by(|a, b| a.id.cmp(&b.id));

        lessons
    }

    /// Iterate over every lesson.
    pub fn iter(&self) -> impl Iterator<Item = &Lesson> {
        self.lessons.values()
    }

    /// Number of loaded lessons.
    pub fn len(&self) -> usize {
        self.lessons.len()
    }

    /// Returns true if no lessons are loaded.
    pub fn is_empty(&self) -> bool {
        self.lessons.is_empty()
    }

    /// Returns the next lesson, if one exists.
    pub fn next_of(&self, lesson: &Lesson) -> Option<&Lesson> {
        lesson.next.as_ref().and_then(|id| self.lessons.get(id))
    }

    /// Ensure a lesson exists.
    pub fn require(&self, id: &str) -> Result<&Lesson> {
        self.get(id)
            .ok_or_else(|| anyhow::anyhow!("Lesson '{}' not found", id))
    }
}
