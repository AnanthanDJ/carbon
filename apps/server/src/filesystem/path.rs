use std::fmt;

#[derive(Debug, Clone)]
pub struct Path {
    pub raw: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedPath {
    components: Vec<String>,
}

impl Path {
    pub fn new(path: impl Into<String>) -> Self {
        Self { raw: path.into() }
    }

    pub fn is_absolute(&self) -> bool {
        self.raw.starts_with('/')
    }

    pub fn components(&self) -> Vec<&str> {
        self.raw.split('/').filter(|s| !s.is_empty()).collect()
    }
}

impl ResolvedPath {
    pub fn root() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn components(&self) -> &[String] {
        &self.components
    }

    pub fn is_root(&self) -> bool {
        self.components.is_empty()
    }

    pub fn parent(&self) -> Self {
        if self.components.is_empty() {
            return Self::root();
        }

        let mut components = self.components.clone();
        components.pop();

        Self { components }
    }

    pub fn file_name(&self) -> Option<&str> {
        self.components.last().map(String::as_str)
    }

    pub fn join(&self, child: impl Into<String>) -> Self {
        let mut components = self.components.clone();
        components.push(child.into());

        Self { components }
    }

    pub fn depth(&self) -> usize {
        self.components.len()
    }

    pub fn starts_with(&self, other: &ResolvedPath) -> bool {
        self.components.starts_with(other.components())
    }
}

impl Default for ResolvedPath {
    fn default() -> Self {
        Self::root()
    }
}

impl From<&str> for ResolvedPath {
    fn from(path: &str) -> Self {
        resolve(&ResolvedPath::root(), path)
    }
}

impl fmt::Display for ResolvedPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.components.is_empty() {
            write!(f, "/")
        } else {
            write!(f, "/{}", self.components.join("/"))
        }
    }
}

pub fn resolve(cwd: &ResolvedPath, input: &str) -> ResolvedPath {
    let mut components = if input.starts_with('/') {
        Vec::new()
    } else {
        cwd.components().to_vec()
    };

    for part in input.split('/') {
        match part {
            "" | "." => {}

            ".." => {
                components.pop();
            }

            other => {
                components.push(other.to_string());
            }
        }
    }

    ResolvedPath { components }
}
