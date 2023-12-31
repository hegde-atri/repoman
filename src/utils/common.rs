use std::path::Path;

/// Returns true if provided path is a git directory.
pub fn is_repo(dir: &Path) -> bool {
    return dir.join(".git").is_dir();
}
