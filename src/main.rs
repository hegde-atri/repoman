#![allow(unused)]

mod args;
mod utils;

use args::RepoArgs;
use clap::Parser;
use std::{
    io::{self, Write},
    path::Path,
};

use crate::utils::{actions, command, dirs};

fn main() {
    let args = RepoArgs::parse();
    println!("{:?}\n", args);

    let mut path = Path::new("/home/mizuuu/repos/personal/rust/repo");
    actions::git_status(Some(path));
    dirs::get_repos(Some(path));

    // match command::exec("ex", Some("--icons"), path) {
    //     Ok(v) => io::stdout().write_all(&v.stdout).unwrap(),
    //     Err(err) => println!("Error: {}", err),
    // };
}
