use ansi_term::Colour;
use ansi_term::Style;

use crate::utils::command;
use std::{
    env::current_dir,
    io::{self, Write},
    path::{Path, PathBuf},
};

/// Prints git status either `pwd` or provided path
///
/// # Arguments
///
/// * `p` - An `Option<&Path>` to run the command `git status` in
pub fn git_status(p: Option<&Path>) {
    // decide the path
    let pwd = get_pwd();
    let path: &Path = match p {
        Some(v) => v,
        None => pwd.as_path(),
    };

    // print the status with path red and bold.
    println!("{}", Colour::Red.bold().paint(path.to_str().unwrap()));

    match command::exec("git status", path) {
        Ok(o) => io::stdout().write_all(&o.stdout).unwrap(),
        Err(err) => println!("Error: {}", err),
    };
}

/// Checks if provided path contains a git repository
fn is_git(path: &Path) -> bool {
    todo!();
}

/// Attempts to get the current working directory as a PathBuf
fn get_pwd() -> PathBuf {
    return match std::env::current_dir() {
        Ok(v) => PathBuf::from(v),
        Err(err) => panic!("Couldn't find current dir: {}", err),
    };
}
