use std::path::{Path, PathBuf};

/// Returns true if provided path is a git directory.
pub fn is_repo(dir: &Path) -> bool {
    return dir.join(".git").is_dir();
}

/// Returns a Pathbuf of current working dir or the dir if provided.
pub fn get_pwd(dir: Option<&Path>) -> PathBuf {
    let pwd = match std::env::current_dir() {
        Ok(v) => PathBuf::from(v),
        Err(err) => panic!("Couldn't find current dir: {}", err),
    };

    return match dir {
        Some(v) => v.to_path_buf(),
        None => pwd,
    };
}
