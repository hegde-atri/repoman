#![allow(unused)]

mod args;
mod utils;

use args::RepoArgs;
use clap::Parser;

use crate::utils::command;

fn main() {
    let args = RepoArgs::parse();
    println!("{:?}", args);
    command::exec("ls");
}
