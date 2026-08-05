use crate::filesystem::{ResolvedPath, path::resolve};

#[test]
fn resolve_root() {
    assert_eq!(
        resolve(&ResolvedPath::root(), "hello").to_string(),
        "/hello",
    );
}

#[test]
fn resolve_parent() {
    assert_eq!(
        resolve(
            &resolve(&ResolvedPath::root(), "/home/projects"),
            "../notes"
        )
        .to_string(),
        "/home/notes",
    );
}

#[test]
fn resolve_absolute() {
    assert_eq!(
        resolve(&resolve(&ResolvedPath::root(), "/home"), "/etc").to_string(),
        "/etc",
    );
}

#[test]
fn resolve_root_parent() {
    assert_eq!(resolve(&ResolvedPath::root(), "../../..",).to_string(), "/",);
}
