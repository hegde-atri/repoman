use ansi_term::Colour;
use ha_utils::cmd::exec;
use ha_utils::cmd::get_pwd;

use std::{
    io::{self, Write},
    path::Path,
};

use crate::utils::actions::branch::get_branch;
use crate::utils::common::is_repo;

pub fn git_status_oneline(p: Option<&Path>) {
    let path = get_pwd(p);

    if is_repo(path.as_path()) {
        // print the status with path red and bold.
        println!(
            "{} - {}",
            Colour::Cyan.paint(get_branch(&path.as_path())),
            Colour::dimmed(Colour::White).paint(path.to_str().unwrap())
        );
    } else {
        println!("{}", Colour::Red.bold().paint("Not a git directory"));
    }
}

/// Prints git status either `pwd` or provided path
///
/// # Arguments
///
/// * `p` - An `Option<&Path>` to run the command `git status` in
pub fn git_status(p: Option<&Path>) {
    // decide the path
    let path = get_pwd(p);

    if is_repo(path.as_path()) {
        // print the status with path red and bold.
        println!("{}", Colour::Blue.bold().paint(path.to_str().unwrap()));

        match exec("git status", path.as_path()) {
            Ok(o) => io::stdout().write_all(&o.stdout).unwrap(),
            Err(err) => println!("Error: {}", err),
        };
    } else {
        println!("{}", Colour::Red.bold().paint("Not a git directory"));
    }
}
