use ansi_term::Colour;
use ansi_term::Style;

use crate::utils::command;
use crate::utils::common;
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
    let path = common::get_cwd(p);

    if common::is_repo(path.as_path()) {
        // print the status with path red and bold.
        println!("{}", Colour::Blue.bold().paint(path.to_str().unwrap()));

        match command::exec("git status", path.as_path()) {
            Ok(o) => io::stdout().write_all(&o.stdout).unwrap(),
            Err(err) => println!("Error: {}", err),
        };
    } else {
        println!("{}", Colour::Red.bold().paint("Not a git directory"));
    }
}
